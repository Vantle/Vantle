use component::generation::rust::error::Error;
use component::generation::rust::schema::Cases;

use std::collections::HashMap;
use syn::{
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    File, Ident, Item, ItemFn, ItemMod, Signature,
};

// Type aliases for clarity
type Source = HashMap<String, Vec<Item>>;
type Counters = HashMap<String, usize>;

/// Generate Rust test code from a template and test cases.
pub fn generate(
    template: &str,
    data: &Cases,
    content: &str,
    path: &str,
) -> Result<String, Box<Error>> {
    let mut ast = syn::parse_file(template).map_err(|error| Box::new(Error::from(error)))?;

    let context = Context::from_ast(&ast);
    let tests = process(data, &context, content, path)?;

    inject(&mut ast, tests);
    Ok(prettyplease::unparse(&ast))
}

/// Contextual information extracted from the AST.
struct Context {
    functions: HashMap<String, Signature>,
}

impl Context {
    fn from_ast(ast: &File) -> Self {
        let mut collector = Collector::default();
        collector.visit_file(ast);

        Self {
            functions: collector.functions,
        }
    }
}

/// Process all test cases and generate test code.
fn process(
    data: &Cases,
    context: &Context,
    content: &str,
    path: &str,
) -> Result<Source, Box<Error>> {
    let mut tests = Source::new();
    let mut counters = Counters::new();

    // Process each function, validating existence and generating tests in one pass
    for function in &data.functions {
        // Validate that the function exists in the template
        if !context.functions.contains_key(&function.function.qualified) {
            let functions: Vec<String> = context.functions.keys().cloned().collect();

            // Find the exact position of the function in the JSON with better context
            let name = &function.function.qualified;
            let pattern = format!("\"function\": \"{}\"", name);

            let (start, length) = if let Some(position) = content.find(&pattern) {
                // Find the start of the function name within the pattern
                let offset = position + pattern.find(name).unwrap_or(0);
                (offset, name.len())
            } else {
                // Fallback: just search for the function name
                let position = content.find(name).unwrap_or(0);
                (position, name.len())
            };

            // Create a beautiful error message with context
            let message = format!(
                "Available functions in template:\n{}\n\nTip: Check that the function name matches exactly with a function in your template file.\n\nExpected format: Use one of the available function names above.",
                functions.iter()
                    .map(|f| format!("  • {}", f))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            return Err(Box::new(Error::cases(
                path,
                content.to_string(),
                Some((start, length)),
                message,
            )));
        }

        // Process all test cases for this function
        for case in &function.cases {
            let inputs = test::Inputs {
                parameters: &function.parameters,
                returns: &function.returns,
                functions: &context.functions,
            };

            let results = test::build(
                case,
                &function.function,
                &function.tags,
                &inputs,
                &mut counters,
                content,
                path,
            )?;

            for (module, item) in results {
                tests.entry(module.clone()).or_default().push(item);
            }
        }
    }

    Ok(tests)
}

/// Inject generated tests into the AST.
fn inject(ast: &mut File, tests: Source) {
    let mut injector = Injector {
        tests,
        path: Vec::new(),
    };
    injector.visit_file_mut(ast);
}

/// Collects information from the AST.
#[derive(Default)]
struct Collector {
    functions: HashMap<String, Signature>,
    module: Vec<Ident>,
}

impl Visit<'_> for Collector {
    fn visit_item_fn(&mut self, item: &ItemFn) {
        let name = item.sig.ident.to_string();
        let qualified = self.qualified_name(&name);

        self.functions.insert(qualified, item.sig.clone());

        visit::visit_item_fn(self, item);
    }

    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.module.push(item.ident.clone());
        visit::visit_item_mod(self, item);
        self.module.pop();
    }
}

impl Collector {
    fn qualified_name(&self, name: &str) -> String {
        if self.module.is_empty() {
            name.to_string()
        } else {
            let path = self
                .module
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            format!("{}::{}", path, name)
        }
    }
}

/// Injects tests into the AST.
struct Injector {
    tests: Source,
    path: Vec<Ident>,
}

impl VisitMut for Injector {
    fn visit_item_mod_mut(&mut self, item: &mut ItemMod) {
        self.path.push(item.ident.clone());
        let module = self.module();

        visit_mut::visit_item_mod_mut(self, item);

        // Inject tests if this module has any
        if let Some((_, items)) = &mut item.content {
            if let Some(submodule) = self.create(&module) {
                items.push(submodule);
            }
        }

        self.path.pop();
    }

    fn visit_file_mut(&mut self, file: &mut File) {
        visit_mut::visit_file_mut(self, file);

        if let Some(submodule) = self.create("") {
            file.items.push(submodule);
        }
    }
}

impl Injector {
    fn module(&self) -> String {
        self.path
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    fn create(&self, module: &str) -> Option<Item> {
        let tests = self.tests.get(module)?;
        if tests.is_empty() {
            return None;
        }

        Some(Self::build(tests))
    }

    fn build(tests: &[Item]) -> Item {
        let items = tests;

        syn::parse_quote! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use vantle::test::utility::assert;

                #(#items)*
            }
        }
    }
}
