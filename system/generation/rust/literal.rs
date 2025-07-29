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
    syn::parse_quote! {
        serde_json::from_str::<#inner>(#string).unwrap()
    }
}
