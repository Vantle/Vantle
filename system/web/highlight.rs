use element::Language;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};

pub struct Highlighter {
    defaults: SyntaxSet,
    custom: SyntaxSet,
    theme: syntect::highlighting::Theme,
}

impl Highlighter {
    #[must_use]
    pub fn new() -> Self {
        let defaults = SyntaxSet::load_defaults_newlines();
        let custom = match load() {
            Ok(definition) => {
                let mut builder = SyntaxSetBuilder::new();
                builder.add(definition);
                builder.build()
            }
            Err(_) => SyntaxSet::new(),
        };

        let set = ThemeSet::load_defaults();
        let theme = set
            .themes
            .get("base16-ocean.dark")
            .or_else(|| set.themes.values().next())
            .cloned()
            .unwrap_or_default();

        Self {
            defaults,
            custom,
            theme,
        }
    }

    pub fn highlight(&self, code: &str, language: Language) -> miette::Result<String> {
        let name = language.name();
        let extension = language.extension();

        if let Some(syntax) = self
            .defaults
            .find_syntax_by_token(name)
            .or_else(|| self.defaults.find_syntax_by_extension(extension))
        {
            return highlighted_html_for_string(code, &self.defaults, syntax, &self.theme).map_err(
                |_| {
                    miette::Report::new(error::Error::Highlight {
                        language: name.into(),
                    })
                },
            );
        }

        if let Some(syntax) = self
            .custom
            .find_syntax_by_token(name)
            .or_else(|| self.custom.find_syntax_by_extension(extension))
        {
            return highlighted_html_for_string(code, &self.custom, syntax, &self.theme).map_err(
                |_| {
                    miette::Report::new(error::Error::Highlight {
                        language: name.into(),
                    })
                },
            );
        }

        let plain = self.defaults.find_syntax_plain_text();
        highlighted_html_for_string(code, &self.defaults, plain, &self.theme).map_err(|_| {
            miette::Report::new(error::Error::Highlight {
                language: name.into(),
            })
        })
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

fn load() -> std::io::Result<SyntaxDefinition> {
    let path = "Molten/resource/system/graph/syntax.yaml";
    let content = std::fs::read_to_string(path)?;
    SyntaxDefinition::load_from_str(&content, false, None)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
