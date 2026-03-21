use std::borrow::Cow;

use language::Language;

pub const HEADINGS: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];

#[derive(Clone)]
pub struct Location {
    pub source: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone)]
pub enum Element {
    Tag {
        name: Cow<'static, str>,
        attributes: Vec<(String, String)>,
        children: Vec<Element>,
    },
    Void {
        name: Cow<'static, str>,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    Raw(String),
    Code {
        content: String,
        language: Language,
        location: Option<Location>,
    },
}

pub fn merge(attributes: &mut Vec<(String, String)>, key: &str, value: &str) {
    if let Some(existing) = attributes.iter_mut().find(|(k, _)| k == key) {
        if key == "class" && existing.1.split_whitespace().any(|c| c == value) {
            return;
        }
        existing.1.push(' ');
        existing.1.push_str(value);
    } else {
        attributes.push((key.into(), value.into()));
    }
}
