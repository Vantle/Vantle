use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Initial,
    Terminal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Control {
    Attribute,
    Context(State),
    Group(State),
    Partition,
    Continuation,
    Void,
    Undefined,
}

pub struct Source(pub source::Source);

impl Source {
    pub fn path<P>(path: P) -> resource::Result<Self>
    where
        P: AsRef<Path>,
    {
        source::Source::path(path, language::molten()).map(Source)
    }

    #[must_use]
    pub fn string<S>(string: S) -> Self
    where
        S: AsRef<str>,
    {
        Source(source::Source::string(string, language::molten()))
    }
}
