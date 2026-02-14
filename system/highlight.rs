use std::path::Path;

use owo_colors::{Rgb, Style, Styled};
use syntect::highlighting::{HighlightIterator, HighlightState};
use syntect::parsing::{ParseState, ScopeStack, SyntaxSet};

pub struct Syntax {
    set: SyntaxSet,
    theme: syntect::highlighting::Theme,
}

impl Syntax {
    pub fn new(set: SyntaxSet, theme: syntect::highlighting::Theme) -> Self {
        Self { set, theme }
    }

    fn detect<'a>(
        &'a self,
        source: &dyn miette::SpanContents<'_>,
    ) -> Option<&'a syntect::parsing::SyntaxReference> {
        source
            .language()
            .and_then(|language| self.set.find_syntax_by_name(language))
            .or_else(|| {
                source.name().and_then(|name| {
                    Path::new(name).extension().and_then(|extension| {
                        self.set
                            .find_syntax_by_extension(&extension.to_string_lossy())
                    })
                })
            })
            .or_else(|| {
                std::str::from_utf8(source.data())
                    .ok()
                    .and_then(|text| text.split('\n').next())
                    .and_then(|line| self.set.find_syntax_by_first_line(line))
            })
    }
}

impl miette::highlighters::Highlighter for Syntax {
    fn start_highlighter_state<'h>(
        &'h self,
        source: &dyn miette::SpanContents<'_>,
    ) -> Box<dyn miette::highlighters::HighlighterState + 'h> {
        match self.detect(source) {
            Some(reference) => {
                let highlighter = syntect::highlighting::Highlighter::new(&self.theme);
                let highlight = HighlightState::new(&highlighter, ScopeStack::new());
                let parse = ParseState::new(reference);
                Box::new(State {
                    set: &self.set,
                    highlighter,
                    highlight,
                    parse,
                })
            }
            None => Box::new(miette::highlighters::BlankHighlighterState),
        }
    }
}

struct State<'h> {
    set: &'h SyntaxSet,
    highlighter: syntect::highlighting::Highlighter<'h>,
    highlight: HighlightState,
    parse: ParseState,
}

impl miette::highlighters::HighlighterState for State<'_> {
    fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>> {
        match self.parse.parse_line(line, self.set) {
            Ok(operations) => {
                HighlightIterator::new(&mut self.highlight, &operations, line, &self.highlighter)
                    .map(|(style, text)| {
                        let foreground =
                            Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                        Style::new().color(foreground).style(text)
                    })
                    .collect::<Vec<_>>()
            }
            Err(_) => vec![Style::default().style(line)],
        }
    }
}
