use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("unsupported language: {name}")]
#[diagnostic(
    code(language::unsupported),
    help("supported languages: rust, molten, bash, python, starlark, toml, yaml, json")
)]
pub struct Unsupported {
    name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
    Molten,
    Bash,
    Python,
    Starlark,
    Toml,
    Yaml,
    Json,
}

impl Language {
    #[must_use]
    pub fn extension(self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Molten => "magma",
            Self::Bash => "sh",
            Self::Python => "py",
            Self::Starlark => "bzl",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
            Self::Json => "json",
        }
    }

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Molten => "molten",
            Self::Bash => "bash",
            Self::Python => "python",
            Self::Starlark => "starlark",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
            Self::Json => "json",
        }
    }

    #[must_use]
    pub fn variant(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Molten => "Molten",
            Self::Bash => "Bash",
            Self::Python => "Python",
            Self::Starlark => "Starlark",
            Self::Toml => "Toml",
            Self::Yaml => "Yaml",
            Self::Json => "Json",
        }
    }

    pub fn parse(name: &str) -> miette::Result<Self> {
        match name {
            "rust" => Ok(Self::Rust),
            "starlark" => Ok(Self::Starlark),
            "python" => Ok(Self::Python),
            "bash" => Ok(Self::Bash),
            "json" => Ok(Self::Json),
            "molten" => Ok(Self::Molten),
            "toml" => Ok(Self::Toml),
            "yaml" => Ok(Self::Yaml),
            _ => Err(Unsupported { name: name.into() }.into()),
        }
    }
}
