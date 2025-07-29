//! Test generation for Rust code.
//!
//! This module generates Rust test functions from JSON case definitions,
//! handling test naming and assertion generation.

use component::generation::rust::error::Error;
use component::generation::rust::schema::Case;
use component::generation::rust::types::Callable;

use literal::expression;

use proc_macro2::Span;

use serde_json::{Map, Value};
use std::collections::HashMap;
use syn::{FnArg, Signature, Type};

/// A single test case with concrete values.
#[derive(Debug, Clone)]
struct Instance {
    /// The parameter values for this test case, keyed by identifier.
    parameters: HashMap<syn::Ident, Value>,
    /// The expected return values for this test case.
    returns: HashMap<String, Value>,
}

/// Validated test case data ready for code generation.
#[derive(Debug)]
struct Test {
    /// The test case.
    case: Instance,
}

/// Type alias for test name counters.
type Counters = HashMap<String, usize>;

/// Input context for building test cases.
pub struct Inputs<'a> {
    pub parameters: &'a HashMap<String, Value>,
    pub returns: &'a HashMap<String, Value>,
    pub functions: &'a HashMap<String, Signature>,
}

/// Build test cases from a case definition.
///
/// This function generates test code for each test case.
///
/// # Returns
/// A vector of (module_path, test_item) pairs.
pub fn build(
    case: &Case,
    target: &Callable,
    tags: &[String],
    inputs: &Inputs,
    counters: &mut Counters,
    content: &str,
    path: &str,
) -> Result<Vec<(String, syn::Item)>, Box<Error>> {
    // Look up the target function signature using the full path
    let signature = inputs.functions.get(&target.qualified).ok_or_else(|| {
        let functions: Vec<String> = inputs.functions.keys().cloned().collect();
        Box::new(Error::NotFound {
            name: format!(
                "Available functions: [{}]\n\nTip: Check that the function name matches exactly with a function in your template file.",
                functions.join(", ")
            ),
        })
    })?;

    // Use the pre-parsed module path and function name
    let function = &target.name; // Simple name for both test naming and function calls
    let module = &target.module;

    // Merge parameters and returns with defaults
    let parameters = shadowed(inputs.parameters, &case.parameters);
    let returns = shadowed(inputs.returns, &case.returns);

    // Merge tags
    let tags = merge(tags, &case.tags);

    // Validate the test case
    let test = validate(signature, &parameters, &returns, content, path)?;

    // Generate test name
    let name = identify(function, &tags, counters);

    // Generate test code
    let item = generate(
        &name,
        function,
        signature,
        &test.case.parameters,
        &test.case.returns,
    )?;

    Ok(vec![(module.clone(), item)])
}

/// Validate test case parameters and returns.
///
/// This function validates that all required parameters are present
/// and returns structured test case data with enhanced error messages.
fn validate(
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    returns: &HashMap<String, Value>,
    content: &str,
    path: &str,
) -> Result<Test, Box<Error>> {
    // Collect expected parameter names from signature
    let expected: Vec<String> = signature
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(ident) => Some(ident.ident.clone()),
                _ => None,
            },
            _ => None,
        })
        .map(|ident| ident.to_string())
        .collect();

    // Validate that all function parameters have values
    for name in &expected {
        if !parameters.contains_key(name) {
            let provided: Vec<String> = parameters.keys().cloned().collect();

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

    // Validate that no extra parameters are provided
    for name in parameters.keys() {
        if !expected.contains(name) {
            // Find the exact position of the parameter in the JSON
            let pattern = format!("\"{}\"", name);
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

    // Collect expected return keys: "()" for function return value + parameter names for mutable params
    let mut keys = vec![keyword::result().key.to_string()];
    keys.extend(expected.iter().cloned());

    // Validate that no extra return values are provided
    for name in returns.keys() {
        if !keys.contains(name) {
            // Find the exact position of the return key in the JSON
            let pattern = format!("\"{}\"", name);
            let location = content.find(&pattern);

            let help = format!(
                "Return value '{}' is not expected by function '{}'.\n\nExpected return values for this function:\n{}\n\nTip: Remove the extra return value '{}' from your test case or check the function signature.",
                name,
                signature.ident,
                keys.iter()
                    .map(|r| format!("  • {}", r))
                    .collect::<Vec<_>>()
                    .join("\n"),
                name
            );

            return Err(Box::new(Error::test(
                path,
                content.to_string(),
                location.map(|pos| (pos, pattern.len())),
                &format!("Invalid return value '{}'", name),
                &help,
            )));
        }
    }

    // Validate that returns are not empty (you must validate at least one output)
    if returns.is_empty() {
        return Err(Box::new(Error::Missing {
            field: "return validation".to_string(),
            context: "test case - at least one return check was required".to_string(),
        }));
    }

    // Create test case
    let case = Instance {
        parameters: parameters
            .iter()
            .map(|(k, v)| (syn::Ident::new(k, Span::call_site()), v.clone()))
            .collect(),
        returns: returns.clone(),
    };

    Ok(Test { case })
}

/// Recursively merge `patch` into `target` following JSON Merge-Patch (RFC 7396) semantics.
///
/// * If both values are objects, keys from `patch` are merged into `target`.
///   * A key whose value is `null` in `patch` removes that key from `target`.
///   * Otherwise the value is merged recursively.
/// * In all other cases the value from `patch` overwrites `target`.
fn shadow(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(rewrites) => {
            // Ensure target is an object; if not, replace it first
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
            // For non-object patches, replace target completely (RFC 7396 semantics)
            *target = patch.clone();
        }
    }
}

/// Deep-merge two JSON maps.
///
/// `overrides` is merged into a clone of `defaults`, returning the merged map.
/// This allows nested maps to be combined instead of blindly overwritten.
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

/// Merge function tags with case tags.
fn merge(function: &[String], case: &[String]) -> Vec<String> {
    let mut tags = function.to_vec();
    tags.extend_from_slice(case);
    tags
}

/// Generate a unique test function name.
fn identify(test: &str, tags: &[String], counters: &mut Counters) -> String {
    // Start with the base function name; rely on suffixes/tags/counters for uniqueness
    let mut name = vec![test.to_string()];

    // Add tags
    if !tags.is_empty() {
        name.extend(tags.iter().cloned());
    }

    // Create the base test name
    let base = name.join("_");

    // Always add counter starting with 0 to avoid function name shadowing
    let counter = counters.entry(base.clone()).or_insert(0);
    let result = format!("{}_{}", base, counter);
    *counter += 1;

    result
}

/// Generate the test function item.
fn generate(
    name: &str,
    function: &str,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
) -> Result<syn::Item, Box<Error>> {
    let mut code = Code::new();

    // Generate parameter declarations and collect call arguments
    let arguments = parameterize(&mut code, signature, parameters)?;

    // Determine if we need the result variable
    let required = expected.contains_key(keyword::result().key)
        || expected
            .iter()
            .any(|(k, _)| k == &keyword::result().variable.to_string());
    let variable = if required {
        keyword::result().variable.clone()
    } else {
        syn::Ident::new("_result", Span::call_site())
    };

    // Generate the function call
    {
        use syn::Expr;

        let expression: Expr = syn::parse_str(function).unwrap_or_else(|e| {
            panic!(
                "Failed to parse function name '{}' as valid Rust expression: {}. \
                 Ensure the function name is a valid Rust identifier or path.",
                function, e
            )
        });

        let statement: syn::Stmt = syn::parse_quote! {
            let #variable = #expression(#(#arguments),*);
        };
        code.push(statement);
    }

    // Generate assertions - pass the test name
    assertion(&mut code, signature, parameters, expected, name)?;

    // Build the complete test function
    use quote::format_ident;

    let identifier = format_ident!("{}", name);
    let block = code.block();

    let item: syn::ItemFn = syn::parse_quote! {
        #[test]
        fn #identifier() #block
    };

    Ok(syn::Item::Fn(item))
}

/// Helper for building test function bodies.
struct Code {
    statements: Vec<syn::Stmt>,
}

impl Code {
    fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    /// Push a pre-built statement into the body.
    fn push(&mut self, statement: syn::Stmt) {
        self.statements.push(statement);
    }

    /// Helper to append an assertion statement using expressions directly.
    fn assert(&mut self, left: syn::Expr, right: syn::Expr, test: &str, name: &str) {
        let statement: syn::Stmt = syn::parse_quote! {
            assert(#left, #right, #test, #name);
        };
        self.statements.push(statement);
    }

    /// Convert the collected statements into a `syn::Block`.
    fn block(self) -> syn::Block {
        let statements = self.statements;
        syn::parse_quote!({ #(#statements)* })
    }
}

/// Generate parameter declarations and return call arguments.
fn parameterize(
    body: &mut Code,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
) -> Result<Vec<syn::Expr>, Box<Error>> {
    let mut arguments: Vec<syn::Expr> = Vec::new();

    for input in &signature.inputs {
        if let FnArg::Typed(pattern) = input {
            let name = match &*pattern.pat {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => continue, // Skip non-identifier patterns
            };

            let value = parameters.get(&name).ok_or_else(|| {
                Box::new(Error::Missing {
                    field: name.to_string(),
                    context: keyword::parameters().key.to_string(),
                })
            })?;

            // Generate the literal value
            let literal = expression(pattern.ty.as_ref(), value);

            // Check if this is a mutable reference parameter
            let mutable = matches!(
                pattern.ty.as_ref(),
                Type::Reference(r) if r.mutability.is_some()
            );

            // Generate the declaration - using identifier directly
            let statement: syn::Stmt = if mutable {
                syn::parse_quote! { let mut #name = #literal; }
            } else {
                syn::parse_quote! { let #name = #literal; }
            };
            body.push(statement);

            // Generate the call argument using the identifier directly
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

/// Generate assertions for the test.
fn assertion(
    code: &mut Code,
    signature: &Signature,
    parameters: &HashMap<syn::Ident, Value>,
    expected: &HashMap<String, Value>,
    test: &str,
) -> Result<(), Box<Error>> {
    // Assert on return value if specified
    if let Some(returns) = expected.get(keyword::result().key) {
        let literal = match &signature.output {
            syn::ReturnType::Type(_, ty) => expression(ty.as_ref(), returns),
            syn::ReturnType::Default => syn::parse_quote! { () },
        };
        let variable = &keyword::result().variable;
        let expression: syn::Expr = syn::parse_quote! { #variable };
        code.assert(
            expression,
            literal,
            test,
            &keyword::result().variable.to_string(),
        );
    }

    // Assert on modified parameters
    for (parameter, result) in expected {
        if parameter != keyword::result().key {
            // Find the parameter identifier that matches this string
            let identifier = parameters
                .keys()
                .find(|ident| ident.to_string() == *parameter)
                .ok_or_else(|| {
                    Box::new(Error::Missing {
                        field: format!("parameter identifier for '{}'", parameter),
                        context: "assertion parameters".to_string(),
                    })
                })?;

            // Find the parameter type
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
                    _ => None,
                })
                .ok_or_else(|| {
                    Box::new(Error::Missing {
                        field: format!("type for parameter '{}'", parameter),
                        context: "function signature".to_string(),
                    })
                })?;

            let expected = expression(ty, result);
            let expression: syn::Expr = syn::parse_quote! { #identifier };
            code.assert(expression, expected, test, parameter);
        }
    }

    Ok(())
}
