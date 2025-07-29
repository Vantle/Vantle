use log::debug;

use std::fmt::Debug;
use std::io::SeekFrom;
use std::io::{Error, Read, Seek};

#[derive(Debug, PartialEq, Eq)]
pub struct Translation<Element> {
    initial: u64,
    terminal: u64,
    elements: Vec<Element>,
}

impl<Element: std::fmt::Debug> Translation<Element> {
    pub fn new(initial: u64, terminal: u64, translation: Vec<Element>) -> Self {
        Self {
            initial,
            terminal,
            elements: translation,
        }
    }

    pub fn initial(&self) -> u64 {
        self.initial
    }

    pub fn terminal(&self) -> u64 {
        self.terminal
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn rules() -> Rules<Element> {
        Rules::default()
    }
}

impl<Element: std::fmt::Debug> std::fmt::Display for Translation<Element> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Translated from {} to {} with a total length of {} with per element size {} bits and value of {:?}.",
            self.initial(),
            self.terminal(),
            self.elements().len(),
            std::mem::size_of::<Element>(),
            self.elements()
        )
    }
}

impl Translation<u8> {
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
    pub fn parsed(&self) -> String {
        self.elements().iter().collect::<String>()
    }
}

pub type Rule<Element> = dyn Fn(Element) -> bool + 'static;

pub mod rule {
    use crate::Rule;

    pub mod terminate {
        use crate::Rule;
        pub fn none<Element>() -> Box<Rule<Element>> {
            Box::new(|_: Element| false)
        }
    }

    pub fn glyph() -> Box<Rule<u8>> {
        Box::new(|element: u8| !element.is_ascii_whitespace())
    }

    pub fn is(value: u8) -> Box<Rule<u8>> {
        Box::new(move |element: u8| value == element)
    }

    pub fn not(value: u8) -> Box<Rule<u8>> {
        Box::new(move |element: u8| value != element)
    }

    pub mod filter {
        use crate::Rule;

        pub fn none<Element>() -> Box<Rule<Element>> {
            Box::new(|_: Element| true)
        }
    }
}

pub struct Rules<Element> {
    filter: Box<Rule<Element>>,
    terminator: Box<Rule<Element>>,
    limiter: Option<usize>,
}

impl<Element> Rules<Element> {
    pub fn filter(mut self, filter: impl Fn(Element) -> bool + 'static) -> Self {
        self.filter = Box::new(filter);
        self
    }

    pub fn terminator(mut self, terminator: impl Fn(Element) -> bool + 'static) -> Self {
        self.terminator = Box::new(terminator);
        self
    }

    pub fn limiter(mut self, limiter: usize) -> Self {
        self.limiter = Some(limiter);
        self
    }
}

impl<Element> Default for Rules<Element> {
    fn default() -> Self {
        Self {
            filter: rule::filter::none(),
            terminator: rule::terminate::none(),
            limiter: None,
        }
    }
}

impl Rules<u8> {
    pub fn view<Source>(
        &self,
        mut source: impl Translator<Source, u8>,
    ) -> Result<Translation<u8>, Error> {
        source.view(self.filter.as_ref(), self.terminator.as_ref(), self.limiter)
    }
    pub fn consume<Source>(
        &self,
        mut source: impl Translator<Source, u8>,
    ) -> Result<Translation<u8>, Error> {
        source.consume(self.filter.as_ref(), self.terminator.as_ref(), self.limiter)
    }
}

pub trait Translator<Source, Element> {
    fn view(
        &mut self,
        filter: &Rule<Element>,
        terminator: &Rule<Element>,
        limit: Option<usize>,
    ) -> Result<Translation<Element>, Error>;
    fn consume(
        &mut self,
        filter: &Rule<Element>,
        terminator: &Rule<Element>,
        limit: Option<usize>,
    ) -> Result<Translation<Element>, Error>;
}

impl<Source: Read + Seek> Translator<Source, u8> for Source {
    fn view(
        &mut self,
        filter: &Rule<u8>,
        terminator: &Rule<u8>,
        limiter: Option<usize>,
    ) -> Result<Translation<u8>, Error> {
        let initial = self.stream_position()?;
        let remaining = self.seek(SeekFrom::End(0))? - initial;
        let reset = self.seek(SeekFrom::Start(initial))?;
        debug!("Reset to {:?}", reset);

        let consumable = std::cmp::min(remaining as usize, limiter.unwrap_or(remaining as usize));

        let mut buffer = vec![0u8; consumable];

        self.read_exact(buffer.as_mut_slice())?;

        let translation = buffer
            .iter()
            .take_while(|&&element| !terminator(element))
            .filter(|&&element| filter(element))
            .cloned()
            .collect::<Vec<u8>>();

        debug!(
            "Translation with rules and limit of {:?} capped at consumable {:?}: {:?}",
            limiter, consumable, translation
        );

        let terminal = initial + translation.len() as u64;
        self.seek(SeekFrom::Current(-(consumable as i64)))?;

        debug!("Source position reset to initial: {:?}", initial);

        Ok(Translation::new(initial, terminal, translation))
    }

    fn consume(
        &mut self,
        filter: &Rule<u8>,
        terminator: &Rule<u8>,
        limiter: Option<usize>,
    ) -> Result<Translation<u8>, Error> {
        let initial = self.stream_position()?;
        let remaining = self.seek(SeekFrom::End(0))? - initial;
        let reset = self.seek(SeekFrom::Start(initial))?;
        debug!("Reset to {:?}", reset);

        let consumable = std::cmp::min(remaining as usize, limiter.unwrap_or(remaining as usize));

        let mut buffer = vec![0u8; consumable];

        self.read_exact(buffer.as_mut_slice())?;

        let translation = buffer
            .iter()
            .take_while(|&&element| !terminator(element))
            .filter(|&&element| filter(element))
            .cloned()
            .collect::<Vec<u8>>();

        debug!(
            "Translation with rules and limit of {:?} capped at consumable {:?}: {:?}",
            limiter, consumable, translation
        );

        let consumed = translation.len() as i64;
        let terminal = self.seek(SeekFrom::Current(consumed - (consumable as i64)))?;

        debug!("Source position set to terminal: {:?}", terminal);

        Ok(Translation::new(initial, terminal, translation))
    }
}
