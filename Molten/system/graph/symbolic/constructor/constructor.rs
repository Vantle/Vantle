use component::graph::attribute::{Assembler, Attribute, Category, Value};
use syntax::syntax;
use system::language;
use translator::Translation;

use log::debug;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::io::Cursor;
use std::path::Path;
use thiserror::Error;

pub type Result<T> = miette::Result<T, Error>;

pub struct Source {
    cursor: Cursor<Vec<u8>>,
    source: NamedSource<String>,
}

impl Source {
    pub fn path(path: impl AsRef<Path>) -> resource::Result<Self> {
        let path = path.as_ref();
        let content = resource::stringify(path)?;
        Ok(Source {
            cursor: Cursor::new(content.as_bytes().into()),
            source: NamedSource::new(path.display().to_string(), content)
                .with_language(system::language::molten()),
        })
    }

    pub fn string(string: impl AsRef<str>) -> resource::Result<Self> {
        Ok(Source {
            cursor: Cursor::new(string.as_ref().as_bytes().into()),
            source: NamedSource::new("stdin", string.as_ref().to_string())
                .with_language(system::language::molten()),
        })
    }

    fn sourced(&self, error: Error) -> Sourced {
        Sourced {
            source: self.source.clone().with_language(language::molten()),
            error,
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Unexpected element `{token}` expected {expected} in {context}")]
    #[diagnostic(
        code(constructor::unexpected),
        help("Element is not valid in this context")
    )]
    Unexpected {
        token: String,
        expected: String,
        context: String,
        #[label("Unexpected token here")]
        span: SourceSpan,
    },

    #[error("Context yields undefined state")]
    #[diagnostic(
        code(constructor::undefined),
        help("The parser reached an undefined state - check for malformed context syntax")
    )]
    Undefined {
        #[label("Undefined state here")]
        span: SourceSpan,
    },

    #[error("Expected element `{token}` not defined in context")]
    #[diagnostic(
        code(constructor::incomplete),
        help("The context is missing required elements")
    )]
    Incomplete {
        token: String,
        #[label("Incomplete context here")]
        span: SourceSpan,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Translator(#[from] translator::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug, Diagnostic)]
#[error("{error}")]
pub struct Sourced {
    #[source]
    #[diagnostic_source]
    error: Error,
    #[source_code]
    source: NamedSource<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Initial,
    Terminal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Control {
    Attribute,
    Context(State),
    Group(State),
    Partition,
    Continuation,
    Void,
    Undefined,
}

trait Transition {
    fn transition(self) -> Control;
}

pub trait Constructor<Sink: Value> {
    type Error;
    fn module(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
    fn construct(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
    fn attribute(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
    fn context(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
    fn group(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
    fn partition(self) -> std::result::Result<Attribute<Sink>, Self::Error>;
}

impl Transition for u8 {
    fn transition(self) -> Control {
        match self {
            syntax::context::INITIAL => Control::Context(State::Initial),
            syntax::group::INITIAL => Control::Group(State::Initial),
            syntax::PARTITION => Control::Partition,
            syntax::CONTINUATION => Control::Continuation,
            syntax::group::TERMINAL => Control::Group(State::Terminal),
            syntax::context::TERMINAL => Control::Context(State::Terminal),
            value => match value.is_ascii_whitespace() {
                true => Control::Void,
                false => Control::Attribute,
            },
        }
    }
}

impl Transition for Option<u8> {
    fn transition(self) -> Control {
        self.map(|value| value.transition())
            .unwrap_or(Control::Undefined)
    }
}

impl Transition for &[u8] {
    fn transition(self) -> Control {
        self.first()
            .map(|value| value.transition())
            .unwrap_or(Control::Undefined)
    }
}

mod consume {
    use crate::Result;
    use log::debug;
    use translator::rule;
    use translator::Translation;

    pub fn space<Source: std::io::Read + std::io::Seek>(
        mut source: Source,
    ) -> Result<Translation<u8>> {
        let skipped = Translation::rules()
            .terminator(rule::glyph())
            .consume(source.by_ref())?;
        debug!("Advance: {:?}", skipped.length());
        Ok(skipped)
    }

    pub fn next<Source: std::io::Read + std::io::Seek>(
        mut source: Source,
    ) -> Result<Translation<u8>> {
        Ok(Translation::rules().limiter(1).consume(source.by_ref())?)
    }
}

mod view {
    use crate::Result;
    use translator::Translation;

    pub fn next<Source: std::io::Read + std::io::Seek>(
        mut source: Source,
    ) -> Result<Translation<u8>> {
        Ok(Translation::rules().limiter(1).view(source.by_ref())?)
    }
}

impl<Source: std::io::Read + std::io::Seek> Constructor<String> for &mut Source {
    type Error = Error;

    fn module(self) -> Result<Attribute<String>> {
        debug!("===> Module");
        let mut assembler = Assembler::<String>::new(Category::Group);
        loop {
            consume::space(self.by_ref())?;

            let result = Translation::rules().limiter(1).view(self.by_ref())?;

            match result.elements().transition() {
                Control::Undefined => break,
                _ => {
                    self.construct().map(|c| assembler.then(c))?;
                }
            }
        }
        let terminal = self.stream_position()?;
        debug!("{:?}", terminal);
        Ok(assembler.assemble())
    }

    fn construct(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref())?;
        let next = view::next(self.by_ref())?;
        let position = self.stream_position()?;
        debug!("======> Construct with {:?}", next.characterize());
        let attribute = match next.elements().transition() {
            Control::Attribute => self.attribute(),
            Control::Continuation => self.attribute(),
            Control::Context(State::Initial) => self.context(),
            Control::Group(State::Initial) => self.group(),
            Control::Partition => self.partition(),
            Control::Void => {
                let token = next.characterize().parsed();
                let length = token.len();
                Err(Error::Unexpected {
                    token,
                    expected: format!(
                        "'{}', '{}', '{}', or '{}'",
                        "Attribute",
                        syntax::context::INITIAL as char,
                        syntax::group::INITIAL as char,
                        syntax::PARTITION as char
                    ),
                    context: "Construct".to_string(),
                    span: (position as usize, length).into(),
                })
            }
            Control::Undefined => Err(Error::Undefined {
                span: (position as usize, 1).into(),
            }),
            _ => {
                let token = next.characterize().parsed();
                let length = token.len();
                Err(Error::Incomplete {
                    token,
                    span: (position as usize, length).into(),
                })
            }
        };
        debug!("<======= Construct {:#?}", attribute);
        attribute
    }

    fn attribute(self) -> Result<Attribute<String>> {
        let skipped = consume::space(self.by_ref())?;
        debug!("===> Attribute");
        let initial = skipped.terminal();
        debug!("Initial: {:?}", initial);
        let mut assembler = Assembler::<String>::empty();
        let mut value = String::new();
        loop {
            let next = view::next(self.by_ref())?;
            let transition = next.elements().transition();
            match transition {
                Control::Attribute => {
                    consume::next(self.by_ref())?
                        .elements()
                        .iter()
                        .for_each(|element| value.push(*element as char));
                }
                Control::Context(State::Initial) | Control::Group(State::Initial) => {
                    self.construct()
                        .map(|construct| assembler.then(construct))?;
                }
                Control::Context(State::Terminal) | Control::Group(State::Terminal) => break,
                Control::Partition => {
                    break;
                }
                Control::Void | Control::Continuation => {
                    consume::next(self.by_ref())?;
                    break;
                }
                Control::Undefined => break,
            }
        }
        let terminal = initial + value.len() as u64;
        debug!("Terminal: {:?}", terminal);
        debug!("<=== Attribute");
        Ok(assembler.category(Category::Attribute(value)).assemble())
    }

    fn context(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref())?;
        let next = consume::next(self.by_ref())?;
        debug!("===> Context with {:?}", next.characterize());
        debug!("Initial: {:?}", self.stream_position());
        let mut assembler = Assembler::<String>::new(Category::Context);
        loop {
            let next = view::next(self.by_ref())?;
            let transition = next.elements().transition();
            let position = self.stream_position().unwrap_or(0);
            match transition {
                Control::Attribute
                | Control::Context(State::Initial)
                | Control::Group(State::Initial)
                | Control::Continuation => {
                    self.construct()
                        .map(|construct| assembler.then(construct))?;
                }
                Control::Partition => {
                    self.partition()
                        .map(|partition| assembler.then(partition))?;
                }
                Control::Context(State::Terminal) => {
                    consume::next(self.by_ref())?;
                    break;
                }
                Control::Group(State::Terminal) => {
                    let token = next.characterize().parsed();
                    let length = token.len();
                    return Err(Error::Unexpected {
                        token,
                        expected: format!(
                            "{}, '{}', or '{}'",
                            "Attribute",
                            syntax::context::TERMINAL as char,
                            syntax::PARTITION as char
                        ),
                        context: "Context".to_string(),
                        span: (position as usize, length).into(),
                    });
                }
                Control::Undefined => break,
                Control::Void => {
                    consume::space(self.by_ref())?;
                }
            }
        }
        let terminal = self.stream_position()?;
        debug!("Terminal: {:?}", terminal);
        Ok(assembler.assemble())
    }

    fn group(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref())?;

        let next = consume::next(self.by_ref())?;
        debug!("===> Group with {:?}", next.characterize());
        let initial = self.stream_position()?;
        debug!("Initial: {:?}", initial);

        let mut assembler = Assembler::<String>::new(Category::Group);
        loop {
            let next = view::next(self.by_ref())?;
            let transition = next.elements().transition();
            let position = self.stream_position().unwrap_or(0);
            match transition {
                Control::Attribute
                | Control::Context(State::Initial)
                | Control::Group(State::Initial)
                | Control::Continuation => {
                    self.construct()
                        .map(|construct| assembler.then(construct))?;
                }
                Control::Partition => {
                    self.partition()
                        .map(|partition| assembler.then(partition))?;
                }
                Control::Group(State::Terminal) => {
                    let consumed = consume::next(self.by_ref())?;
                    debug!("{:?}", consumed);
                    break;
                }
                Control::Context(State::Terminal) => {
                    let token = next.characterize().parsed();
                    let length = token.len();
                    return Err(Error::Unexpected {
                        token,
                        expected: format!(
                            "{}, '{}', or '{}'",
                            "Attribute",
                            syntax::PARTITION as char,
                            syntax::group::TERMINAL as char,
                        ),
                        context: "Group".to_string(),
                        span: (position as usize, length).into(),
                    });
                }
                Control::Undefined => break,
                Control::Void => {
                    consume::space(self.by_ref())?;
                }
            }
        }
        let terminal = self.stream_position()?;
        debug!("Terminal: {:?}", terminal);
        Ok(assembler.assemble())
    }

    fn partition(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref())?;
        debug!("===> Partition");
        let next = consume::next(self.by_ref())?;
        let position = self.stream_position().unwrap_or(0);
        if next.elements().transition() != Control::Partition {
            let token = next.characterize().parsed();
            let length = token.len();
            return Err(Error::Unexpected {
                token,
                expected: format!("'{}'", syntax::PARTITION as char),
                context: "Partition".to_string(),
                span: (position as usize, length).into(),
            });
        }
        debug!("Partition with {:?}", next.characterize());
        debug!("<=== Partition");
        Ok(Assembler::new(Category::Partition).assemble())
    }
}

impl Constructor<String> for Source {
    type Error = Sourced;

    fn module(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.module().map_err(|e| self.sourced(e))
    }

    fn construct(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.construct().map_err(|e| self.sourced(e))
    }

    fn attribute(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.attribute().map_err(|e| self.sourced(e))
    }

    fn context(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.context().map_err(|e| self.sourced(e))
    }

    fn group(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.group().map_err(|e| self.sourced(e))
    }

    fn partition(mut self) -> miette::Result<Attribute<String>, Sourced> {
        self.cursor.partition().map_err(|e| self.sourced(e))
    }
}
