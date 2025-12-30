pub use rule;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Translation<Element> {
    pub initial: u64,
    pub terminal: u64,
    pub elements: Vec<Element>,
}

impl<Element: std::fmt::Debug> Translation<Element> {
    #[must_use]
    pub fn new(initial: u64, terminal: u64, translation: Vec<Element>) -> Self {
        Self {
            initial,
            terminal,
            elements: translation,
        }
    }

    #[must_use]
    pub fn initial(&self) -> u64 {
        self.initial
    }

    #[must_use]
    pub fn terminal(&self) -> u64 {
        self.terminal
    }

    #[must_use]
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.elements.len()
    }

    #[must_use]
    pub fn rules() -> rule::Rules<Element> {
        rule::Rules::default()
    }
}

impl<Element: std::fmt::Debug> std::fmt::Display for Translation<Element> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Translated from {} to {} with a total length of {} with per element size {} bytes and value of {:?}.",
            self.initial(),
            self.terminal(),
            self.elements().len(),
            std::mem::size_of::<Element>(),
            self.elements()
        )
    }
}

impl Translation<u8> {
    #[must_use]
    pub fn characterize(&self) -> Translation<char> {
        Translation {
            initial: self.initial(),
            terminal: self.terminal(),
            elements: self
                .elements()
                .iter()
                .map(|value| *value as char)
                .collect::<Vec<char>>(),
        }
    }
}

impl Translation<char> {
    #[must_use]
    pub fn parsed(&self) -> String {
        self.elements().iter().collect::<String>()
    }
}
