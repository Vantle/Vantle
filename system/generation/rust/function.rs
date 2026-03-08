use std::collections::HashMap;
use std::path::Path;

use proc_macro2::Span;
use serde_json::{Map, Value};
use syn::{Expr, FnArg, Signature, Type};

use component::generation::rust::{error::Error, schema::Case, types::Callable};
use literal::expression;

#[derive(Debug, Clone)]
struct Instance {
    parameters: HashMap<syn::Ident, Value>,
    returns: HashMap<String, Value>,
}

pub struct Inputs<'a> {
    pub parameters: &'a HashMap<String, Value>,
    pub returns: &'a HashMap<String, Value>,
    pub functions: &'a HashMap<String, Signature>,
}

pub struct Registration {
    pub name: String,
    pub tags: Vec<String>,
    pub parameters: Value,
    pub expected: Value,
    pub statements: Vec<syn::Stmt>,
    pub actuals: syn::Expr,
    pub comparisons: Vec<syn::Stmt>,
}

pub fn build(
    case: &Case,
    target: &Callable,
    tags: &[String],
    inputs: &Inputs,
    content: &str,
    path: &str,
) -> Result<Registration, Box<Error>> {
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

    let function = &target.qualified;

    let parameters = shadowed(inputs.parameters, &case.parameters);
    let returns = shadowed(inputs.returns, &case.returns);

    let tags = merge(tags, &case.tags);

    let validated = validate(signature, &parameters, &returns, content, path)?;

    let (statements, actuals, comparisons) = generate(
        function,
        signature,
        &validated.parameters,
        &validated.returns,
        path,
    )?;

    let name = target.qualified.replace("::", ".");
    let serialized = serde_json::to_value(&parameters)
        .expect("parameters are already Value and always serializable");
    let returns =
        serde_json::to_value(&returns).expect("returns are already Value and always serializable");

    Ok(Registration {
        name,
        tags,
        parameters: serialized,
        expected: returns,
        statements,
        actuals,
        comparisons,
    })
}

fn validate(
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    returns: &HashMap<String, Value>,
    content: &str,
    path: &str,
) -> Result<Instance, Box<Error>> {
    let expected = signature
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(typed) => match &*typed.pat {
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

    Ok(Instance {
        parameters: parameters
            .iter()
            .map(|(k, v)| (syn::Ident::new(k, Span::call_site()), v.clone()))
            .collect::<HashMap<_, _>>(),
        returns: returns.clone(),
    })
}

fn shadow(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(rewrites) => {
            if !matches!(target, Value::Object(_)) {
                *target = Value::Object(Map::default());
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

fn generate(
    function: &str,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
    path: impl AsRef<Path>,
) -> Result<(Vec<syn::Stmt>, syn::Expr, Vec<syn::Stmt>), Box<Error>> {
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

    let expression: Expr = syn::parse_str(function).map_err(|e| {
        Box::new(Error::Untargetable {
            name: format!(
                "Failed to parse function name '{function}' as valid Rust expression: {e}. \
                 Ensure the function name is a valid Rust identifier or path."
            ),
        })
    })?;

    let statement: syn::Stmt = syn::parse_quote! {
        let #variable = #expression(#(#arguments),*);
    };
    code.push(statement);

    let actuals = recording(parameters, expected)?;
    let comparisons = comparison(signature, parameters, expected, &path);

    Ok((code.statements, actuals, comparisons))
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

fn recording(
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
) -> Result<syn::Expr, Box<Error>> {
    let mut expr: syn::Expr = syn::parse_quote! { function::Actuals::default() };

    if expected.contains_key(keyword::result().key) {
        let key = keyword::result().key;
        let variable = keyword::result().variable;
        expr = syn::parse_quote! { #expr.record(#key, &#variable)? };
    }

    for parameter in expected.keys() {
        if parameter != keyword::result().key {
            let identifier = parameters
                .keys()
                .find(|ident| ident.to_string() == *parameter)
                .ok_or_else(|| {
                    Box::new(Error::Missing {
                        field: format!("parameter identifier for '{parameter}'"),
                        context: "recording parameters".to_string(),
                    })
                })?;
            let key = parameter.as_str();
            expr = syn::parse_quote! { #expr.record(#key, &#identifier)? };
        }
    }

    Ok(expr)
}

fn comparison(
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
    path: impl AsRef<Path>,
) -> Vec<syn::Stmt> {
    let mut statements = Vec::new();

    if let (Some(value), syn::ReturnType::Type(_, ty)) =
        (expected.get(keyword::result().key), &signature.output)
    {
        let variable = keyword::result().variable;
        let expectation = expression(ty.as_ref(), value, &path);
        let key = keyword::result().key;
        let stmt: syn::Stmt = syn::parse_quote! {
            if !utility::equal(&#variable, &#expectation) {
                return Err(Box::new(function::error::Error::mismatch(
                    actuals.clone(),
                    format!("expected {}: {:?}, got: {:?}", #key, #expectation, #variable),
                )));
            }
        };
        statements.push(stmt);
    }

    for (key, value) in expected {
        if key == keyword::result().key {
            continue;
        }
        if let Some((ident, _)) = parameters.iter().find(|(i, _)| i.to_string() == *key) {
            let ty = signature.inputs.iter().find_map(|input| match input {
                FnArg::Typed(pat) => match &*pat.pat {
                    syn::Pat::Ident(i) if i.ident == *ident => Some(pat.ty.as_ref()),
                    _ => None,
                },
                FnArg::Receiver(_) => None,
            });
            if let Some(ty) = ty {
                let inner = match ty {
                    Type::Reference(r) => r.elem.as_ref(),
                    _ => ty,
                };
                let expectation = expression(inner, value, &path);
                let label = key.as_str();
                let stmt: syn::Stmt = syn::parse_quote! {
                    if !utility::equal(&#ident, &#expectation) {
                        return Err(Box::new(function::error::Error::mismatch(
                            actuals.clone(),
                            format!("expected {}: {:?}, got: {:?}", #label, #expectation, #ident),
                        )));
                    }
                };
                statements.push(stmt);
            }
        }
    }

    statements
}
