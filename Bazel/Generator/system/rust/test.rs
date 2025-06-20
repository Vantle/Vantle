//! Test generation for Rust code.
//!
//! This module generates Rust test functions from JSON case definitions,
//! handling test naming and assertion generation.

use component::case::{Callable, Case};
use component::error::{Error, Result};
use literals::AsLiteral;
use serde_json::Value;
use std::collections::HashMap;
use syn::{File, FnArg, PatType, Signature, Type};

/// Extension trait for extracting parameter names from function arguments.
trait FnArgExt {
    /// Extract the parameter name from a function argument.
    fn parameter_name(&self) -> Option<String>;
}

impl FnArgExt for PatType {
    fn parameter_name(&self) -> Option<String> {
        match &*self.pat {
            syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
            _ => None,
        }
    }
}

/// A single test case with concrete values.
#[derive(Debug, Clone)]
struct TestCase {
    /// The parameter values for this test case.
    parameters: HashMap<String, Value>,
    /// The expected return values for this test case.
    returns: HashMap<String, Value>,
}

/// Validated test case data ready for code generation.
#[derive(Debug)]
struct TestCaseData {
    /// The test case.
    case: TestCase,
}

/// Type alias for test name counters.
type TestCounters = HashMap<String, usize>;

/// Input context for building test cases.
pub struct BuildInputs<'a> {
    pub default_parameters: &'a HashMap<String, Value>,
    pub default_returns: &'a HashMap<String, Value>,
    pub functions: &'a HashMap<String, Signature>,
    pub ast: &'a File,
}

/// Build test cases from a case definition.
///
/// This function generates test code for each test case.
///
/// # Returns
/// A vector of (module_path, test_code) pairs.
pub fn build(
    case: &Case,
    target: &Callable,
    function_tags: &[String],
    inputs: &BuildInputs,
    counters: &mut TestCounters,
) -> Result<Vec<(String, String)>> {
    // Look up the target function signature using the full path
    let signature = inputs
        .functions
        .get(&target.qualified)
        .ok_or_else(|| Error::NotFound {
            name: target.qualified.clone(),
        })?;

    // Use the pre-parsed module path and function name
    let function_name = &target.name;
    let module_path = &target.module;

    // Merge parameters and returns with defaults
    let parameters = merge_with_defaults(inputs.default_parameters, &case.parameters);
    let returns = merge_with_defaults(inputs.default_returns, &case.returns);

    // Merge tags
    let tags = merge_tags(function_tags, &case.tags);

    // Validate the test case
    let test_data = validate_test_case(signature, &parameters, &returns)?;

    // Generate test name
    let test_name = generate_test_name(
        function_name,
        &tags,
        0, // Only one case now
        counters,
    );

    // Generate test code
    let test_code = generate_test_function(
        &test_name,
        function_name,
        signature,
        &test_data.case.parameters,
        &test_data.case.returns,
        inputs.ast,
    )?;

    Ok(vec![(module_path.clone(), test_code)])
}

/// Validate test case parameters and returns.
///
/// This function validates that all required parameters are present
/// and returns structured test case data.
fn validate_test_case(
    sig: &Signature,
    params: &HashMap<String, Value>,
    returns: &HashMap<String, Value>,
) -> Result<TestCaseData> {
    // Validate that all function parameters have values
    for input in &sig.inputs {
        if let FnArg::Typed(pat_type) = input {
            if let Some(param_name) = pat_type.parameter_name() {
                if !params.contains_key(&param_name) {
                    return Err(Error::Missing {
                        field: param_name,
                        context: "test case parameters".to_string(),
                    });
                }
            }
        }
    }

    // Create test case
    let case = TestCase {
        parameters: params.clone(),
        returns: returns.clone(),
    };

    Ok(TestCaseData { case })
}

/// Merge two value maps, with the second taking precedence.
fn merge_with_defaults(
    defaults: &HashMap<String, Value>,
    overrides: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    let mut merged = defaults.clone();
    merged.extend(overrides.clone());
    merged
}

/// Merge function tags with case tags.
fn merge_tags(function_tags: &[String], case_tags: &[String]) -> Vec<String> {
    let mut tags = function_tags.to_vec();
    tags.extend_from_slice(case_tags);
    tags
}

/// Generate a unique test function name.
fn generate_test_name(
    base_name: &str,
    tags: &[String],
    case_idx: usize,
    counters: &mut TestCounters,
) -> String {
    // Always start with "test_" to avoid conflicts with the function being tested
    let mut name_parts = vec!["test".to_string(), base_name.to_string()];

    // Add tags
    if !tags.is_empty() {
        name_parts.extend(tags.iter().cloned());
    }

    // Add case index if needed (for multiple cases with same tags)
    if case_idx > 0 {
        name_parts.push(format!("case{}", case_idx));
    }

    // Create the base test name
    let base_test_name = name_parts.join("_");

    // Add counter to ensure uniqueness
    let counter_key = base_test_name.clone();
    let counter = counters.entry(counter_key).or_insert(0);
    let test_name = if *counter == 0 {
        base_test_name.clone()
    } else {
        format!("{}_{}", base_test_name, counter)
    };
    *counter += 1;

    test_name
}

/// Generate the test function code.
fn generate_test_function(
    test_name: &str,
    function_name: &str,
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    expected: &HashMap<String, Value>,
    ast: &File,
) -> Result<String> {
    let mut test_body = TestBody::new();

    // Generate parameter declarations and collect call arguments
    let call_args = generate_parameters(&mut test_body, signature, parameters, ast)?;

    // Generate the function call
    test_body.add_line(format!(
        "let result = {}({});",
        function_name,
        call_args.join(", ")
    ));

    // Generate assertions
    generate_assertions(&mut test_body, signature, parameters, expected, ast)?;

    // Build the complete test function
    Ok(format!(
        "#[test]\nfn {}() {{\n{}\n}}",
        test_name,
        test_body.build()
    ))
}

/// Helper for building test function bodies.
struct TestBody {
    lines: Vec<String>,
}

impl TestBody {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn add_line(&mut self, line: String) {
        self.lines.push(format!("    {}", line));
    }

    fn build(self) -> String {
        self.lines.join("\n")
    }
}

/// Generate parameter declarations and return call arguments.
fn generate_parameters(
    body: &mut TestBody,
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    ast: &File,
) -> Result<Vec<String>> {
    let mut call_args = Vec::new();

    for input in &signature.inputs {
        if let FnArg::Typed(pat_type) = input {
            if let Some(param_name) = pat_type.parameter_name() {
                let value = parameters.get(&param_name).ok_or_else(|| Error::Missing {
                    field: param_name.clone(),
                    context: "parameters".to_string(),
                })?;

                // Generate the literal value
                let literal = pat_type.ty.as_literal(value, ast);

                // Check if this is a mutable reference parameter
                let is_mut_ref = matches!(
                    pat_type.ty.as_ref(),
                    Type::Reference(r) if r.mutability.is_some()
                );

                // Generate the declaration
                body.add_line(format!(
                    "let {}{} = {};",
                    if is_mut_ref { "mut " } else { "" },
                    param_name,
                    literal
                ));

                // Generate the call argument
                let arg = match pat_type.ty.as_ref() {
                    Type::Reference(r) => {
                        if r.mutability.is_some() {
                            format!("&mut {}", param_name)
                        } else {
                            format!("&{}", param_name)
                        }
                    }
                    _ => param_name.clone(),
                };

                call_args.push(arg);
            }
        }
    }

    Ok(call_args)
}

/// Generate assertions for the test.
fn generate_assertions(
    body: &mut TestBody,
    signature: &Signature,
    parameters: &HashMap<String, Value>,
    expected: &HashMap<String, Value>,
    ast: &File,
) -> Result<()> {
    // Assert on return value if specified
    if let Some(expected_return) = expected.get("()") {
        let expected_literal = signature.output.as_literal(expected_return, ast);
        body.add_line(format!("assert_eq!(result, {});", expected_literal));
    }

    // Assert on modified parameters
    for (param_name, expected_value) in expected {
        if param_name != "()" && parameters.contains_key(param_name) {
            // Find the parameter type
            let param_type = signature
                .inputs
                .iter()
                .find_map(|arg| match arg {
                    FnArg::Typed(t) if t.parameter_name() == Some(param_name.clone()) => {
                        Some(&*t.ty)
                    }
                    _ => None,
                })
                .ok_or_else(|| Error::Missing {
                    field: format!("type for parameter '{}'", param_name),
                    context: "function signature".to_string(),
                })?;

            let expected_literal = param_type.as_literal(expected_value, ast);
            body.add_line(format!("assert_eq!({}, {});", param_name, expected_literal));
        }
    }

    Ok(())
}
