use language::Language;

pub fn treesitter(
    source: impl AsRef<str>,
    language: Language,
) -> miette::Result<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    let grammar: tree_sitter::Language = match language {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::Python | Language::Starlark => tree_sitter_python::LANGUAGE.into(),
        Language::Bash => tree_sitter_bash::LANGUAGE.into(),
        Language::Json => tree_sitter_json::LANGUAGE.into(),
        _ => {
            return Err(error::Error::Unsupported {
                language: language.name().into(),
            }
            .into());
        }
    };
    parser
        .set_language(&grammar)
        .map_err(|e| error::Error::Grammar {
            detail: e.to_string(),
        })?;
    parser
        .parse(source.as_ref(), None)
        .ok_or_else(|| error::Error::Parse.into())
}
