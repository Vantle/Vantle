use serde_json::Value;
use syn::Type;

/// Generate a serde_json deserialize expression for a type and JSON value.
pub fn expression(ty: &Type, value: &Value) -> syn::Expr {
    // For references use the inner type so we can borrow later.
    let inner = match ty {
        Type::Reference(type_ref) => type_ref.elem.as_ref(),
        _ => ty,
    };

    // Convert the JSON value to a string representation for the generated code
    let string = value.to_string();

    // Use parse_quote to generate the expression directly with AST nodes
    // Include enhanced error reporting with precise source location
    syn::parse_quote! {
        serde_json::from_str::<#inner>(#string).unwrap_or_else(|failure| {
            let source = #string;
            let target = stringify!(#inner);

            // Format failure with source position information
            let position = {
                let row = failure.line();
                let column = failure.column();

                // Extract the problematic row from source
                let rows: Vec<&str> = source.lines().collect();
                let content = if row > 0 && row <= rows.len() {
                    rows[row - 1]
                } else {
                    "<content not found>"
                };

                // Create a visual indicator for the failure position
                let indicator = if column > 0 {
                    format!("{}^", " ".repeat(column.saturating_sub(1)))
                } else {
                    "^".to_string()
                };

                format!(
                    "\n📍 Failure at row {}, column {}:\n{}\n{}\n",
                    row, column, content, indicator
                )
            };

            // Create comprehensive failure message
            let message = format!(
                "🚨 JSON Deserialization Failed 🚨\n\
                 \n\
                 Target: {}\n\
                 Failure: {}{}\n\
                 💡 Tip: Check that your JSON structure matches the expected target format.\n\
                 \n\
                 📄 Complete Source:\n{}\n",
                target,
                failure,
                position,
                source
            );

            eprintln!("{}", message);
            panic!("JSON deserialization failed - see detailed failure above")
        })
    }
}
