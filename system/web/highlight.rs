use element::Language;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxDefinition, SyntaxReference, SyntaxSet, SyntaxSetBuilder};
use syntect::util::LinesWithEndings;

pub struct Highlighter {
    defaults: SyntaxSet,
    custom: SyntaxSet,
}

impl Highlighter {
    #[must_use]
    pub fn new() -> Self {
        let defaults = SyntaxSet::load_defaults_newlines();
        let custom = match load() {
            Some(definition) => {
                let mut builder = SyntaxSetBuilder::new();
                builder.add(definition);
                builder.build()
            }
            None => SyntaxSet::new(),
        };

        Self { defaults, custom }
    }

    pub fn highlight(&self, code: &str, language: Language) -> miette::Result<String> {
        let (syntax, set) = self.resolve(language);

        let mut generator = ClassedHTMLGenerator::new_with_class_style(
            syntax,
            set,
            ClassStyle::SpacedPrefixed { prefix: "syntax-" },
        );

        for line in LinesWithEndings::from(code) {
            let _ = generator.parse_html_for_line_which_includes_newline(line);
        }

        Ok(format!(
            "<pre><code class=\"language-{}\">{}</code></pre>",
            language.name(),
            generator.finalize()
        ))
    }

    fn resolve(&self, language: Language) -> (&SyntaxReference, &SyntaxSet) {
        let name = language.name();
        let extension = language.extension();

        if let Some(syntax) = self
            .defaults
            .find_syntax_by_token(name)
            .or_else(|| self.defaults.find_syntax_by_extension(extension))
        {
            return (syntax, &self.defaults);
        }

        if let Some(syntax) = self
            .custom
            .find_syntax_by_token(name)
            .or_else(|| self.custom.find_syntax_by_extension(extension))
        {
            return (syntax, &self.custom);
        }

        (self.defaults.find_syntax_plain_text(), &self.defaults)
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

fn load() -> Option<SyntaxDefinition> {
    let content = include_str!("../../Molten/resource/system/graph/syntax.yaml");
    SyntaxDefinition::load_from_str(content, false, None).ok()
}
