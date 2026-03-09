use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use parse::Constructor;

pub fn rust<T>(source: T) -> Result<syn::File, error::Error>
where
    T: AsRef<str>,
{
    syn::parse_file(source.as_ref()).map_err(|source| error::Error::Rust { source })
}

pub fn molten<T>(source: T) -> miette::Result<Attribute<String>>
where
    T: AsRef<str>,
{
    Source::string(source.as_ref()).module().map_err(Into::into)
}

pub fn json<T>(source: T) -> Result<serde_json::Value, error::Error>
where
    T: AsRef<str>,
{
    serde_json::from_str(source.as_ref()).map_err(|source| error::Error::Json { source })
}

pub fn python<T>(source: T) -> Result<tree_sitter::Tree, error::Error>
where
    T: AsRef<str>,
{
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .map_err(|source| error::Error::Language {
            language: "python",
            source,
        })?;
    parser
        .parse(source.as_ref(), None)
        .ok_or(error::Error::Parse { language: "python" })
}

pub fn bash<T>(source: T) -> Result<tree_sitter::Tree, error::Error>
where
    T: AsRef<str>,
{
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .map_err(|source| error::Error::Language {
            language: "bash",
            source,
        })?;
    parser
        .parse(source.as_ref(), None)
        .ok_or(error::Error::Parse { language: "bash" })
}

pub fn markdown<T>(source: T) -> Result<tree_sitter_md::MarkdownTree, error::Error>
where
    T: AsRef<str>,
{
    tree_sitter_md::MarkdownParser::default()
        .parse(source.as_ref().as_bytes(), None)
        .ok_or(error::Error::Parse {
            language: "markdown",
        })
}
