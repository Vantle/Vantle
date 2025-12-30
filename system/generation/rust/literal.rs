use std::path::Path;

use serde_json::Value;
use syn::Type;

#[must_use]
pub fn expression(ty: &Type, value: &Value, source: impl AsRef<Path>) -> syn::Expr {
    let inner = match ty {
        Type::Reference(type_ref) => type_ref.elem.as_ref(),
        _ => ty,
    };

    let string = value.to_string();
    let path = source
        .as_ref()
        .canonicalize()
        .unwrap_or_else(|_| source.as_ref().to_path_buf())
        .display()
        .to_string();

    syn::parse_quote! {
        serde_json::from_str::<#inner>(#string).map_err(|failure| {
            runtime::Runtime::deserialization(
                stringify!(#inner),
                #path,
                #string,
                failure
            )
        })?
    }
}
