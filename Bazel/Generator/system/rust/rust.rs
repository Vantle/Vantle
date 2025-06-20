use component::error::{Error, Result};
use component::schema::Cases;
use std::collections::{HashMap, HashSet};
use syn::{
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    File, Item, ItemFn, ItemMod, Signature,
};

use imports::ImportTrie;

// Type aliases for clarity
type FunctionSignatures = HashMap<String, Signature>;
type TypeNames = HashSet<String>;
type TestCode = HashMap<String, Vec<String>>;
type ImportSet = HashMap<String, HashSet<String>>;
type TestCounters = HashMap<String, usize>;

/// Generate Rust test code from a template and test cases.
pub fn generate(template: &str, data: &Cases) -> Result<String> {
    let mut ast = syn::parse_file(template).map_err(Error::from)?;

    let context = Context::from_ast(&ast);
    let (tests, imports) = process_cases(data, &context, &ast)?;

    inject_tests(&mut ast, tests, imports, &context);
    Ok(prettyplease::unparse(&ast))
}

/// Contextual information extracted from the AST.
struct Context {
    functions: FunctionSignatures,
    types: TypeNames,
    template_imports: ImportSet,
}

impl Context {
    fn from_ast(ast: &File) -> Self {
        let mut collector = AstCollector::default();
        collector.visit_file(ast);

        Self {
            functions: collector.functions,
            types: collector.types,
            template_imports: collector.imports,
        }
    }
}

/// Process all test cases and generate test code.
fn process_cases(data: &Cases, context: &Context, ast: &File) -> Result<(TestCode, ImportSet)> {
    let mut tests = TestCode::new();
    let mut imports = context.template_imports.clone();
    let mut counters = TestCounters::new();

    for function in &data.functions {
        for case in &function.cases {
            let inputs = test::BuildInputs {
                default_parameters: &function.parameters,
                default_returns: &function.returns,
                functions: &context.functions,
                ast,
            };

            let test_results = test::build(
                case,
                &function.function,
                &function.tags,
                &inputs,
                &mut counters,
            )?;

            for (module_path, code) in test_results {
                tests.entry(module_path.clone()).or_default().push(code);
                imports
                    .entry(module_path)
                    .or_default()
                    .insert(function.function.qualified.clone());
            }
        }
    }

    Ok((tests, imports))
}

/// Inject generated tests into the AST.
fn inject_tests(ast: &mut File, tests: TestCode, imports: ImportSet, context: &Context) {
    let mut injector = TestInjector {
        tests,
        imports,
        context,
        current_path: Vec::new(),
    };
    injector.visit_file_mut(ast);
}

/// Collects information from the AST.
#[derive(Default)]
struct AstCollector {
    functions: FunctionSignatures,
    types: TypeNames,
    imports: ImportSet,
    module_path: Vec<String>,
}

impl Visit<'_> for AstCollector {
    fn visit_item_fn(&mut self, item: &ItemFn) {
        let name = item.sig.ident.to_string();
        let qualified_name = self.qualified_name(&name);

        self.functions.insert(name, item.sig.clone());
        self.functions.insert(qualified_name, item.sig.clone());

        visit::visit_item_fn(self, item);
    }

    fn visit_item_struct(&mut self, item: &syn::ItemStruct) {
        self.types.insert(item.ident.to_string());
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &syn::ItemEnum) {
        self.types.insert(item.ident.to_string());
        visit::visit_item_enum(self, item);
    }

    fn visit_item_type(&mut self, item: &syn::ItemType) {
        self.types.insert(item.ident.to_string());
        visit::visit_item_type(self, item);
    }

    fn visit_item_use(&mut self, item: &syn::ItemUse) {
        let current_module = self.current_module();

        // Extract imported names
        if let Some(imported) = Self::extract_import_name(item) {
            self.imports
                .entry(current_module)
                .or_default()
                .insert(imported);
        }

        visit::visit_item_use(self, item);
    }

    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.module_path.push(item.ident.to_string());
        visit::visit_item_mod(self, item);
        self.module_path.pop();
    }
}

impl AstCollector {
    fn qualified_name(&self, name: &str) -> String {
        if self.module_path.is_empty() {
            name.to_string()
        } else {
            format!("{}::{}", self.module_path.join("::"), name)
        }
    }

    fn current_module(&self) -> String {
        self.module_path.join("::")
    }

    fn extract_import_name(item: &syn::ItemUse) -> Option<String> {
        // Convert the use item to a temporary file for pretty-printing
        let temp_file = File {
            shebang: None,
            attrs: vec![],
            items: vec![Item::Use(item.clone())],
        };

        let use_string = prettyplease::unparse(&temp_file);
        use_string
            .lines()
            .find(|line| line.trim().starts_with("use "))
            .and_then(|line| Self::parse_import_from_line(line.trim()))
    }

    fn parse_import_from_line(line: &str) -> Option<String> {
        let content = line.strip_prefix("use ")?.strip_suffix(';')?.trim();

        if content.contains('{') {
            // Group imports are complex, skip for now
            None
        } else if let Some(as_pos) = content.find(" as ") {
            // Renamed import: use the alias
            Some(content[as_pos + 4..].trim().to_string())
        } else if content.starts_with("std::") {
            // Standard library: preserve full path
            Some(content.to_string())
        } else {
            // Other imports: use the last segment
            content.split("::").last().map(|s| s.to_string())
        }
    }
}

/// Injects tests into the AST.
struct TestInjector<'a> {
    tests: TestCode,
    imports: ImportSet,
    context: &'a Context,
    current_path: Vec<String>,
}

impl VisitMut for TestInjector<'_> {
    fn visit_item_mod_mut(&mut self, item: &mut ItemMod) {
        self.current_path.push(item.ident.to_string());
        let current_module = self.current_module();

        visit_mut::visit_item_mod_mut(self, item);

        // Inject tests if this module has any
        if let Some((_, items)) = &mut item.content {
            if let Some(test_module) = self.create_test_module(&current_module) {
                items.push(test_module);
            }
        }

        self.current_path.pop();
    }

    fn visit_file_mut(&mut self, file: &mut File) {
        visit_mut::visit_file_mut(self, file);

        // Inject root-level tests
        if let Some(test_module) = self.create_test_module("") {
            file.items.push(test_module);
        }
    }
}

impl TestInjector<'_> {
    fn current_module(&self) -> String {
        self.current_path.join("::")
    }

    fn create_test_module(&self, module_path: &str) -> Option<Item> {
        let tests = self.tests.get(module_path)?;
        if tests.is_empty() {
            return None;
        }

        let imports = self.imports.get(module_path).cloned().unwrap_or_default();
        Some(Self::build_test_module(
            tests,
            &imports,
            &self.context.functions,
            &self.context.types,
        ))
    }

    fn build_test_module(
        tests: &[String],
        template_and_target_imports: &HashSet<String>,
        available_functions: &FunctionSignatures,
        available_types: &TypeNames,
    ) -> Item {
        let test_items: Vec<Item> = tests
            .iter()
            .filter_map(|code| syn::parse_str(code).ok())
            .collect();

        // Analyze test code to find used symbols
        let test_code = tests.join("\n");
        let used_types = Self::find_used_types(&test_code, available_types);
        let used_functions = Self::find_used_functions(&test_code, available_functions);

        // Build import statements
        let mut import_trie = ImportTrie::default();

        for import_name in template_and_target_imports {
            if Self::should_import(
                import_name,
                &used_functions,
                available_functions,
                available_types,
            ) {
                let full_path = if import_name.starts_with("std::") {
                    import_name.clone()
                } else {
                    format!("crate::{}", import_name)
                };
                import_trie.insert(&full_path);
            }
        }

        // Add used types
        for type_name in &used_types {
            import_trie.insert(&format!("crate::{}", type_name));
        }

        let import_statements = import_trie.to_use_statements();

        syn::parse_quote! {
            #[cfg(test)]
            mod tests {
                #(#import_statements)*
                #(#test_items)*
            }
        }
    }

    fn should_import(
        import_name: &str,
        used_functions: &HashSet<String>,
        available_functions: &FunctionSignatures,
        available_types: &TypeNames,
    ) -> bool {
        import_name.starts_with("std::")
            || (available_functions.contains_key(import_name)
                && used_functions.contains(import_name))
            || available_types.contains(import_name)
            || used_functions.contains(import_name)
    }

    fn find_used_types(test_code: &str, available_types: &TypeNames) -> HashSet<String> {
        available_types
            .iter()
            .filter(|type_name| {
                test_code.contains(&format!("{}::", type_name))
                    || test_code.contains(&format!("{} {{", type_name))
            })
            .cloned()
            .collect()
    }

    fn find_used_functions(
        test_code: &str,
        available_functions: &FunctionSignatures,
    ) -> HashSet<String> {
        available_functions
            .keys()
            .filter(|function_name| {
                let simple_name = function_name.split("::").last().unwrap_or(function_name);
                test_code.contains(&format!("{}(", simple_name))
            })
            .cloned()
            .collect()
    }
}
