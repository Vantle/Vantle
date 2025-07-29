use component::graph::attribute::{Assembler, Attribute, Category, Value};
use syntax::syntax;
use translator::Translation;

use log::debug;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected element `{0}` recieved in context transition")]
    Unexpected(String),
    #[error("Context yields undefined state")]
    Undefined,
    #[error("Expected element `{0}` not defined in context")]
    Incomplete(String),
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
    fn module(self) -> Result<Attribute<Sink>>;
    fn construct(self) -> Result<Attribute<Sink>>;
    fn attribute(self) -> Result<Attribute<Sink>>;
    fn context(self) -> Result<Attribute<Sink>>;
    fn group(self) -> Result<Attribute<Sink>>;
    fn partition(self) -> Result<Attribute<Sink>>;
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
    use translator::rule;
    use translator::Translation;

    use log::debug;

    pub fn space<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Translation<u8> {
        let skipped = Translation::rules()
            .terminator(rule::glyph())
            .consume(source.by_ref())
            .expect("Failed to consume space");
        debug!("Advance: {:?}", skipped.length());
        skipped
    }

    pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Translation<u8> {
        Translation::rules()
            .limiter(1)
            .consume(source.by_ref())
            .expect("Failed to consume next element")
    }
}

mod view {
    use translator::Translation;

    pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Translation<u8> {
        Translation::rules()
            .limiter(1)
            .view(source.by_ref())
            .expect("Failed to view next element")
    }
}

impl<Source: std::io::Read + std::io::Seek> Constructor<String> for &mut Source {
    fn module(self) -> Result<Attribute<String>> {
        debug!("===> Module");
        let mut assembler = Assembler::<String>::new(Category::Group);
        loop {
            consume::space(self.by_ref());

            let next = Translation::rules().limiter(1).view(self.by_ref());
            match next {
                Ok(result) => {
                    let transition = result.elements().transition();
                    match transition {
                        Control::Undefined => break,
                        _ => {
                            self.construct()
                                .map(|construct| assembler.then(construct))?;
                        }
                    }
                }
                Err(error) => match error.kind() {
                    std::io::ErrorKind::UnexpectedEof => break,
                    _ => todo!(),
                },
            }
        }
        let terminal = self.stream_position().unwrap();
        debug!("{:?}", terminal);
        Ok(assembler.assemble())
    }

    fn construct(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref());
        let next = view::next(self.by_ref());
        debug!("======> Construct with {:?}", next.characterize());
        let attribute = match next.elements().transition() {
            Control::Attribute => self.attribute(),
            Control::Continuation => self.attribute(),
            Control::Context(State::Initial) => self.context(),
            Control::Group(State::Initial) => self.group(),
            Control::Partition => self.partition(),
            Control::Void => Err(Error::Unexpected(next.characterize().parsed())),
            Control::Undefined => Err(Error::Undefined),
            _ => Err(Error::Incomplete(next.characterize().parsed())),
        };
        debug!("<======= Construct {:#?}", attribute.as_ref().unwrap());
        attribute
    }

    fn attribute(self) -> Result<Attribute<String>> {
        let skipped = consume::space(self.by_ref());
        debug!("===> Attribute");
        let initial = skipped.terminal();
        debug!("Initial: {:?}", initial);
        let mut assembler = Assembler::<String>::empty();
        let mut value = String::new();
        loop {
            let next = view::next(self.by_ref());
            let transition = next.elements().transition();
            match transition {
                Control::Attribute => {
                    consume::next(self.by_ref())
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
                    consume::next(self.by_ref());
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
        consume::space(self.by_ref());
        let next = consume::next(self.by_ref());
        debug!("===> Context with {:?}", next.characterize());
        debug!("Initial: {:?}", self.stream_position());
        let mut assembler = Assembler::<String>::new(Category::Context);
        loop {
            let next = view::next(self.by_ref());
            let transition = next.elements().transition();
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
                    consume::next(self.by_ref());
                    break;
                }
                Control::Group(State::Terminal) => {
                    return Err(Error::Unexpected(next.characterize().parsed()))
                }
                Control::Undefined => break,
                Control::Void => {
                    consume::space(self.by_ref());
                }
            }
        }
        let terminal = self.stream_position().unwrap();
        debug!("Terminal: {:?}", terminal);
        Ok(assembler.assemble())
    }

    fn group(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref());

        let next = consume::next(self.by_ref());
        debug!("===> Group with {:?}", next.characterize());
        let initial = self.stream_position().unwrap();
        debug!("Initial: {:?}", initial);

        let mut assembler = Assembler::<String>::new(Category::Group);
        loop {
            let next = view::next(self.by_ref());
            let transition = next.elements().transition();
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
                    let consumed = consume::next(self.by_ref());
                    debug!("{:?}", consumed);
                    break;
                }
                Control::Context(State::Terminal) => {
                    return Err(Error::Unexpected(next.characterize().parsed()))
                }
                Control::Undefined => break,
                Control::Void => {
                    consume::space(self.by_ref());
                }
            }
        }
        let terminal = self.stream_position().unwrap();
        debug!("Terminal: {:?}", terminal);
        Ok(assembler.assemble())
    }

    fn partition(self) -> Result<Attribute<String>> {
        consume::space(self.by_ref());
        debug!("===> Partition");
        let next = consume::next(self.by_ref());
        assert!(
            next.elements().transition() == Control::Partition,
            "Partition was called in a non-partition state."
        );
        debug!("Partition with {:?}", next.characterize());
        debug!("<=== Partition");
        Ok(Assembler::new(Category::Partition).assemble())
    }
}
