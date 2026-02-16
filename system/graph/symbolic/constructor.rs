use component::graph::attribute::Attribute;
use component::graph::symbolic::constructor::Source;
use parse::Constructor;

pub fn rust<T>(source: T) -> miette::Result<syn::File>
where
    T: AsRef<str>,
{
    syn::parse_file(source.as_ref()).map_err(|e| miette::miette!("{e}"))
}

pub fn molten<T>(source: T) -> miette::Result<Attribute<String>>
where
    T: AsRef<str>,
{
    Source::string(source.as_ref()).module().map_err(Into::into)
}

pub fn json<T>(source: T) -> miette::Result<serde_json::Value>
where
    T: AsRef<str>,
{
    serde_json::from_str(source.as_ref()).map_err(|e| miette::miette!("{e}"))
}

pub fn python<T>(source: T) -> miette::Result<tree_sitter::Tree>
where
    T: AsRef<str>,
{
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .map_err(|e| miette::miette!("{e}"))?;
    parser
        .parse(source.as_ref(), None)
        .ok_or_else(|| miette::miette!("failed to parse python"))
}

pub fn bash<T>(source: T) -> miette::Result<tree_sitter::Tree>
where
    T: AsRef<str>,
{
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .map_err(|e| miette::miette!("{e}"))?;
    parser
        .parse(source.as_ref(), None)
        .ok_or_else(|| miette::miette!("failed to parse bash"))
}
