use std::io::Cursor;
use std::path::Path;

use miette::NamedSource;

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

pub struct Source {
    pub cursor: Cursor<Vec<u8>>,
    pub source: NamedSource<String>,
}

impl Source {
    pub fn path<P>(path: P) -> resource::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = resource::stringify(path)?;
        Ok(Source {
            cursor: Cursor::new(content.as_bytes().into()),
            source: NamedSource::new(path.display().to_string(), content)
                .with_language(language::molten()),
        })
    }

    #[must_use]
    pub fn string<S>(string: S) -> Self
    where
        S: AsRef<str>,
    {
        Source {
            cursor: Cursor::new(string.as_ref().as_bytes().into()),
            source: NamedSource::new("stdin", string.as_ref().to_string())
                .with_language(language::molten()),
        }
    }
}
