//! Keywords used throughout the test generation system.
//!
//! This module centralizes all magic string literals to avoid duplication
//! and improve maintainability.

/// Result-related keyword for test generation.
pub struct Result {
    /// JSON key used to represent function return values in test cases.
    pub key: &'static str,
    /// Variable identifier used for capturing function results.
    pub variable: syn::Ident,
}

/// Parameter-related keyword for test generation.
pub struct Parameters {
    /// JSON key used to represent function parameters in test cases.
    pub key: &'static str,
    /// Variable identifier used for capturing function parameters.
    pub variable: syn::Ident,
}

/// Get the result keyword configuration.
pub fn result() -> Result {
    Result {
        key: "()",
        variable: syn::Ident::new("result", proc_macro2::Span::call_site()),
    }
}

/// Get the parameters keyword configuration.
pub fn parameters() -> Parameters {
    Parameters {
        key: "parameters",
        variable: syn::Ident::new("parameters", proc_macro2::Span::call_site()),
    }
}
