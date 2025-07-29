use serde_json::Value;
use syn::Type;

pub fn expression(ty: &Type, value: &Value) -> syn::Expr {
    let inner = match ty {
        Type::Reference(type_ref) => type_ref.elem.as_ref(),
        _ => ty,
    };

    let string = value.to_string();

    syn::parse_quote! {
        serde_json::from_str::<#inner>(#string).unwrap_or_else(|failure| {
            let source = #string;
            let target = stringify!(#inner);

            let position = {
                let row = failure.line();
                let column = failure.column();

                let rows: Vec<&str> = source.lines().collect();
                let content = if row > 0 && row <= rows.len() {
                    rows[row - 1]
                } else {
                    "<content not found>"
                };

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
