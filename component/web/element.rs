use language::Language;

use span::Fragment;

pub struct Location {
    pub source: String,
    pub start: usize,
    pub end: usize,
}

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
        content: String,
        language: Language,
        location: Option<Location>,
    },
    Shell {
        command: String,
    },
    Inject {
        name: String,
    },
    Markdown {
        name: String,
    },
}
