use observe::trace;
use record::debug;

use assemble::Assemble;
use component::graph::attribute::{Assembler, Attribute as Data, Category, Value};
use component::graph::symbolic::translator::Translation;
use constructor::{Control, State};
use error::{Error, Sourced};
use translator::view::Rules as ViewRules;

pub use consume;
pub use error;
pub use view;

pub type Result<T> = error::Result<T>;

pub trait Transition {
    fn transition(self) -> Control;
}

pub trait Constructor<Sink: Value> {
    type Error;
    fn module(self) -> std::result::Result<Data<Sink>, Self::Error>;
    fn construct(self) -> std::result::Result<Data<Sink>, Self::Error>;
    fn attribute(self) -> std::result::Result<Data<Sink>, Self::Error>;
    fn context(self) -> std::result::Result<Data<Sink>, Self::Error>;
    fn group(self) -> std::result::Result<Data<Sink>, Self::Error>;
    fn partition(self) -> std::result::Result<Data<Sink>, Self::Error>;
}

impl Transition for u8 {
    #[trace(channels = [core])]
    fn transition(self) -> Control {
        match self {
            syntax::context::INITIAL => Control::Context(State::Initial),
            syntax::group::INITIAL => Control::Group(State::Initial),
            syntax::PARTITION => Control::Partition,
            syntax::CONTINUATION => Control::Continuation,
            syntax::group::TERMINAL => Control::Group(State::Terminal),
            syntax::context::TERMINAL => Control::Context(State::Terminal),
            value => {
                if value.is_ascii_whitespace() {
                    Control::Void
                } else {
                    Control::Attribute
                }
            }
        }
    }
}

impl Transition for Option<u8> {
    #[trace(channels = [core])]
    fn transition(self) -> Control {
        self.map_or(Control::Undefined, Transition::transition)
    }
}

impl Transition for &[u8] {
    #[trace(channels = [core])]
    fn transition(self) -> Control {
        self.first()
            .copied()
            .map_or(Control::Undefined, Transition::transition)
    }
}

#[trace(channels = [core])]
fn void<Sink: Value>(assembler: &mut Assembler<Sink>, skipped: &Translation<u8>) {
    if skipped.length() > 0 {
        let _ = assembler.then(Assembler::new(Category::Void).assemble());
    }
}

impl<Source: std::io::Read + std::io::Seek> Constructor<String> for &mut Source {
    type Error = Error;

    #[trace(channels = [core])]
    fn module(self) -> Result<Data<String>> {
        debug!("===> Module");
        let mut assembler = Assembler::<String>::new(Category::Group);
        loop {
            let skipped = consume::space(self.by_ref())?;

            let result = Translation::rules().limiter(1).view(self.by_ref())?;

            if result.elements().transition() == Control::Undefined {
                break;
            }
            void(&mut assembler, &skipped);
            let _ = assembler.then(self.construct()?);
        }
        let terminal = self.stream_position()?;
        debug!("{:?}", terminal);
        Ok(assembler.assemble())
    }

    #[trace(channels = [core])]
    fn construct(self) -> Result<Data<String>> {
        consume::space(self.by_ref())?;
        let next = view::next(self.by_ref())?;
        let position = self.stream_position()?;
        debug!("======> Construct with {:?}", next.characterize());
        let attribute = match next.elements().transition() {
            Control::Attribute | Control::Continuation => self.attribute(),
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
                    span: (usize::try_from(position).unwrap_or(0), length).into(),
                })
            }
            Control::Undefined => Err(Error::Undefined {
                span: (usize::try_from(position).unwrap_or(0), 1).into(),
                context: "Construct".to_string(),
            }),
            _ => {
                let token = next.characterize().parsed();
                let length = token.len();
                Err(Error::Incomplete {
                    token,
                    span: (usize::try_from(position).unwrap_or(0), length).into(),
                    context: "Construct".to_string(),
                })
            }
        };
        debug!("<======= Construct {:#?}", attribute);
        attribute
    }

    #[trace(channels = [core])]
    fn attribute(self) -> Result<Data<String>> {
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
                    let _ = assembler.then(self.construct()?);
                }
                Control::Context(State::Terminal)
                | Control::Group(State::Terminal)
                | Control::Undefined => break,
                Control::Partition => {
                    break;
                }
                Control::Void | Control::Continuation => {
                    consume::next(self.by_ref())?;
                    break;
                }
            }
        }
        let terminal = initial + value.len() as u64;
        debug!("Terminal: {:?}", terminal);
        debug!("<=== Attribute");
        Ok(assembler.category(Category::Attribute(value)).assemble())
    }

    #[trace(channels = [core])]
    fn context(self) -> Result<Data<String>> {
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
                    let _ = assembler.then(self.construct()?);
                }
                Control::Partition => {
                    let _ = assembler.then(self.partition()?);
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
                        span: (usize::try_from(position).unwrap_or(0), length).into(),
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

    #[trace(channels = [core])]
    fn group(self) -> Result<Data<String>> {
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
                    let _ = assembler.then(self.construct()?);
                }
                Control::Partition => {
                    let _ = assembler.then(self.partition()?);
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
                        span: (usize::try_from(position).unwrap_or(0), length).into(),
                    });
                }
                Control::Undefined => break,
                Control::Void => {
                    let skipped = consume::space(self.by_ref())?;
                    void(&mut assembler, &skipped);
                }
            }
        }
        let terminal = self.stream_position()?;
        debug!("Terminal: {:?}", terminal);
        Ok(assembler.assemble())
    }

    #[trace(channels = [core])]
    fn partition(self) -> Result<Data<String>> {
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
                span: (usize::try_from(position).unwrap_or(0), length).into(),
            });
        }
        debug!("Partition with {:?}", next.characterize());
        debug!("<=== Partition");
        Ok(Assembler::new(Category::Partition).assemble())
    }
}

impl Constructor<String> for constructor::Source {
    type Error = Sourced;

    #[trace(channels = [core])]
    fn module(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .module()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }

    #[trace(channels = [core])]
    fn construct(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .construct()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }

    #[trace(channels = [core])]
    fn attribute(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .attribute()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }

    #[trace(channels = [core])]
    fn context(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .context()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }

    #[trace(channels = [core])]
    fn group(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .group()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }

    #[trace(channels = [core])]
    fn partition(mut self) -> miette::Result<Data<String>, Sourced> {
        self.cursor
            .partition()
            .map_err(|e| Sourced::wrap(self.source.clone(), e))
    }
}
