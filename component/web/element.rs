use span::Fragment;

pub enum Element {
    Tag {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<Element>,
    },
    Text(String),
    Span(Vec<Fragment>),
    Raw(String),
    Code {
        source: Source,
        language: Language,
    },
    Inject {
        name: String,
    },
    Markdown {
        name: String,
    },
}

pub enum Source {
    File(String),
    Inline(String),
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
            Self::Python | Self::Starlark => "py",
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
}
