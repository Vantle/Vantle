use std::collections::HashMap;
use std::path::Path;

use proc_macro2::Span;
use serde_json::{Map, Value};
use syn::{FnArg, Signature, Type};

use quote::format_ident;
use syn::Expr;

use component::generation::rust::error::Error;
use component::generation::rust::schema::Case;
use component::generation::rust::types::Callable;
use literal::expression;

type Counters = HashMap<String, usize>;

#[derive(Debug, Clone)]
struct Instance {
    parameters: HashMap<syn::Ident, Value>,
    returns: HashMap<String, Value>,
}

#[derive(Debug)]
struct Test {
    case: Instance,
}

pub struct Inputs<'a> {
    pub parameters: &'a HashMap<String, Value>,
    pub returns: &'a HashMap<String, Value>,
    pub functions: &'a HashMap<String, Signature>,
}

pub fn build(
    case: &Case,
    target: &Callable,
    tags: &[String],
    inputs: &Inputs,
    counters: &mut Counters,
    content: &str,
    path: &str,
) -> Result<Vec<(String, syn::Item)>, Box<Error>> {
    let signature = inputs.functions.get(&target.qualified).ok_or_else(|| {
        let functions = inputs.functions.keys().cloned().collect::<Vec<String>>();
        let suggestion =
            similarity::nearest(&target.qualified, &functions).unwrap_or_default();

        Box::new(Error::Untargetable {
            name: format!(
                "Available functions: [{}]{}\n\nTip: Check that the function name matches exactly with a function in your template file.",
                functions.join(", "),
                suggestion
            ),
        })
    })?;

    let function = &target.name;
    let module = &target.module;

    let parameters = shadowed(inputs.parameters, &case.parameters);
    let returns = shadowed(inputs.returns, &case.returns);

    let tags = merge(tags, &case.tags);

    let test = validate(signature, &parameters, &returns, content, path)?;

    let name = identify(function, &tags, counters);

    let item = generate(
        &name,
        function,
        signature,
        &test.case.parameters,
        &test.case.returns,
        path,
    )?;

    Ok(vec![(module.clone(), item)])
}

fn validate(
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    returns: &HashMap<String, Value>,
    content: &str,
    path: &str,
) -> Result<Test, Box<Error>> {
    let expected = signature
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(ident) => Some(ident.ident.clone()),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    for name in &expected {
        if !parameters.contains_key(name) {
            let provided = parameters.keys().cloned().collect::<Vec<String>>();

            return Err(Box::new(Error::Missing {
                field: name.clone(),
                context: format!(
                    "test case parameters for function signature.\n\nProvided parameters: [{}]\n\nTip: Add the missing parameter '{}' to your test case.",
                    provided.join(", "),
                    name
                ),
            }));
        }
    }

    for name in parameters.keys() {
        if !expected.contains(name) {
            let pattern = format!("\"{name}\"");
            let position = content.find(&pattern);

            return Err(Box::new(Error::extraneous(
                path,
                content.to_string(),
                position.map(|pos| (pos, pattern.len())),
                name,
                &signature.ident.to_string(),
                &expected,
            )));
        }
    }

    let mut keys = vec![keyword::result().key.to_string()];
    keys.extend(expected.iter().cloned());

    for name in returns.keys() {
        if !keys.contains(name) {
            let pattern = format!("\"{name}\"");
            let location = content.find(&pattern);

            let suggestion = similarity::nearest(name, &keys).unwrap_or_default();

            let ident = &signature.ident;
            let keys = keys
                .iter()
                .map(|r| format!("  • {r}"))
                .collect::<Vec<_>>()
                .join("\n");
            let help = format!(
                "Return value '{name}' is not expected by function '{ident}'.\n\nExpected return values for this function:\n{keys}{suggestion}\n\nTip: Remove the extra return value '{name}' from your test case or check the function signature."
            );

            return Err(Box::new(Error::test(
                path,
                content.to_string(),
                location.map(|pos| (pos, pattern.len())),
                &format!("Invalid return value '{name}'"),
                &help,
            )));
        }
    }

    if returns.is_empty() {
        return Err(Box::new(Error::Missing {
            field: "return validation".to_string(),
            context: "test case - at least one return check was required".to_string(),
        }));
    }

    let case = Instance {
        parameters: parameters
            .iter()
            .map(|(k, v)| (syn::Ident::new(k, Span::call_site()), v.clone()))
            .collect(),
        returns: returns.clone(),
    };

    Ok(Test { case })
}

fn shadow(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(rewrites) => {
            if !matches!(target, Value::Object(_)) {
                *target = Value::Object(Map::new());
            }
            if let Value::Object(object) = target {
                for (key, overwrite) in rewrites {
                    if overwrite.is_null() {
                        object.remove(key);
                    } else {
                        let entry = object.entry(key.clone()).or_insert(Value::Null);
                        shadow(entry, overwrite);
                    }
                }
            }
        }
        _ => {
            *target = patch.clone();
        }
    }
}

fn shadowed(
    defaults: &HashMap<String, Value>,
    overrides: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    let mut merged = defaults.clone();

    for (key, value) in overrides {
        match merged.get_mut(key) {
            Some(existing) => {
                shadow(existing, value);
            }
            None => {
                merged.insert(key.clone(), value.clone());
            }
        }
    }

    merged
}

fn merge(function: &[String], case: &[String]) -> Vec<String> {
    let mut tags = function.to_vec();
    tags.extend_from_slice(case);
    tags
}

fn identify(test: &str, tags: &[String], counters: &mut Counters) -> String {
    let mut name = vec![test.to_string()];

    if !tags.is_empty() {
        name.extend(tags.iter().cloned());
    }

    let base = name.join("_");

    let counter = counters.entry(base.clone()).or_insert(0);
    let result = format!("{base}_{counter}");
    *counter += 1;

    result
}

fn generate(
    name: &str,
    function: &str,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
    path: impl AsRef<Path>,
) -> Result<syn::Item, Box<Error>> {
    let mut code = Code::new();

    let arguments = parameterize(&mut code, signature, parameters, &path)?;

    let required = expected.contains_key(keyword::result().key)
        || expected
            .iter()
            .any(|(k, _)| k == &keyword::result().variable.to_string());
    let variable = if required {
        keyword::result().variable.clone()
    } else {
        syn::Ident::new("_result", Span::call_site())
    };

    let expression: Expr = syn::parse_str(function).unwrap_or_else(|e| {
        panic!(
            "Failed to parse function name '{function}' as valid Rust expression: {e}. \
             Ensure the function name is a valid Rust identifier or path."
        )
    });

    let statement: syn::Stmt = syn::parse_quote! {
        let #variable = #expression(#(#arguments),*);
    };
    code.push(statement);

    assertion(&mut code, signature, parameters, expected, name, &path)?;

    let identifier = format_ident!("{}", name);
    let block = code.block();

    let item: syn::ItemFn = syn::parse_quote! {
        #[test]
        fn #identifier() -> miette::Result<()> #block
    };

    Ok(syn::Item::Fn(item))
}

struct Code {
    statements: Vec<syn::Stmt>,
}

impl Code {
    fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    fn push(&mut self, statement: syn::Stmt) {
        self.statements.push(statement);
    }

    fn assert(&mut self, left: &syn::Expr, right: &syn::Expr, test: &str, name: &str) {
        let statement: syn::Stmt = syn::parse_quote! {
            assert(&#left, &#right, #test, #name);
        };
        self.statements.push(statement);
    }

    fn block(self) -> syn::Block {
        let statements = self.statements;
        syn::parse_quote!({
            #(#statements)*
            Ok(())
        })
    }
}

fn parameterize(
    body: &mut Code,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    path: impl AsRef<Path>,
) -> Result<Vec<syn::Expr>, Box<Error>> {
    let mut arguments = Vec::<syn::Expr>::new();

    for input in &signature.inputs {
        if let FnArg::Typed(pattern) = input {
            let name = match &*pattern.pat {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => continue,
            };

            let value = parameters.get(&name).ok_or_else(|| {
                Box::new(Error::Missing {
                    field: name.to_string(),
                    context: keyword::parameters().key.to_string(),
                })
            })?;

            let literal = expression(pattern.ty.as_ref(), value, &path);

            let mutable = matches!(
                pattern.ty.as_ref(),
                Type::Reference(r) if r.mutability.is_some()
            );

            let statement: syn::Stmt = if mutable {
                syn::parse_quote! { let mut #name = #literal; }
            } else {
                syn::parse_quote! { let #name = #literal; }
            };
            body.push(statement);

            let argument: syn::Expr = match pattern.ty.as_ref() {
                Type::Reference(r) => {
                    if r.mutability.is_some() {
                        syn::parse_quote! { &mut #name }
                    } else {
                        syn::parse_quote! { &#name }
                    }
                }
                _ => syn::parse_quote! { #name },
            };

            arguments.push(argument);
        }
    }

    Ok(arguments)
}

fn assertion(
    code: &mut Code,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
    test: &str,
    path: impl AsRef<Path>,
) -> Result<(), Box<Error>> {
    if let Some(returns) = expected.get(keyword::result().key) {
        let literal = match &signature.output {
            syn::ReturnType::Type(_, ty) => expression(ty.as_ref(), returns, &path),
            syn::ReturnType::Default => syn::parse_quote! { () },
        };
        let variable = &keyword::result().variable;
        let expression: syn::Expr = syn::parse_quote! { #variable };
        code.assert(
            &expression,
            &literal,
            test,
            &keyword::result().variable.to_string(),
        );
    }

    for (parameter, result) in expected {
        if parameter != keyword::result().key {
            let identifier = parameters
                .keys()
                .find(|ident| ident.to_string() == *parameter)
                .ok_or_else(|| {
                    Box::new(Error::Missing {
                        field: format!("parameter identifier for '{parameter}'"),
                        context: "assertion parameters".to_string(),
                    })
                })?;

            let ty = signature
                .inputs
                .iter()
                .find_map(|arg| match arg {
                    FnArg::Typed(t) => {
                        let name = match &*t.pat {
                            syn::Pat::Ident(ident) => ident.ident.to_string(),
                            _ => return None,
                        };
                        if name == *parameter {
                            Some(&*t.ty)
                        } else {
                            None
                        }
                    }
                    FnArg::Receiver(_) => None,
                })
                .ok_or_else(|| {
                    Box::new(Error::Missing {
                        field: format!("type for parameter '{parameter}'"),
                        context: "function signature".to_string(),
                    })
                })?;

            let expected = expression(ty, result, &path);
            let expression: syn::Expr = syn::parse_quote! { #identifier };
            code.assert(&expression, &expected, test, parameter);
        }
    }

    Ok(())
}
