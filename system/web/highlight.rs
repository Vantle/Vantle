pub use bash;
pub use escape;
pub use json;
pub use molten;
pub use rust;
pub use starlark;
pub use walker;

use element::Language;

pub fn highlight<T>(code: T, language: Language) -> miette::Result<String>
where
    T: AsRef<str>,
{
    let source = code.as_ref();
    match language {
        Language::Rust => {
            if let Ok(ast) = constructor::rust(source) {
                rust::rust(&ast)
            } else {
                let wrapped = format!("fn __snippet__() {{\n{source}\n}}");
                if let Ok(ast) = constructor::rust(&wrapped) {
                    Ok(rust::snippet(&ast))
                } else {
                    Ok(escape::escape(source))
                }
            }
        }
        Language::Molten => match constructor::molten(source) {
            Ok(ast) => molten::molten(&ast),
            Err(_) => Ok(escape::escape(source)),
        },
        Language::Json => match constructor::json(source) {
            Ok(value) => json::json(&value, 61),
            Err(_) => Ok(escape::escape(source)),
        },
        Language::Starlark => match constructor::python(source) {
            Ok(tree) => Ok(walker::highlight(&tree, source, starlark::classify)),
            Err(_) => Ok(escape::escape(source)),
        },
        Language::Bash => match constructor::bash(source) {
            Ok(tree) => Ok(walker::highlight(&tree, source, bash::classify)),
            Err(_) => Ok(escape::escape(source)),
        },
        _ => Ok(escape::escape(source)),
    }
}
