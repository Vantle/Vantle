use std::io::{Read, Seek, SeekFrom};

use observe::trace;
use record::debug;

use rule::{Lambda, Rules as Data};
use translator::Translation;

type Result<T> = error::Result<T>;

pub trait Rules {
    fn consume<Source, T>(&self, source: T) -> Result<Translation<u8>>
    where
        T: Translator<Source, u8>;
}

impl Rules for Data<u8> {
    #[trace(channels = [core])]
    fn consume<Source, T>(&self, mut source: T) -> Result<Translation<u8>>
    where
        T: Translator<Source, u8>,
    {
        source.consume(&*self.filter, &*self.terminator, self.limiter)
    }
}

pub trait Translator<Source, Element> {
    fn consume(
        &mut self,
        filter: &Lambda<Element>,
        terminator: &Lambda<Element>,
        limit: Option<usize>,
    ) -> Result<Translation<Element>>;
}

impl<Source: Read + Seek> Translator<Source, u8> for Source {
    #[trace(channels = [core])]
    fn consume(
        &mut self,
        filter: &Lambda<u8>,
        terminator: &Lambda<u8>,
        limiter: Option<usize>,
    ) -> Result<Translation<u8>> {
        let buffered = buffer::translate(self, filter, terminator, limiter)?;

        let consumed = i64::try_from(buffered.elements.len()).unwrap_or(i64::MAX);
        let bound = i64::try_from(buffered.consumable).unwrap_or(i64::MAX);
        let terminal = self.seek(SeekFrom::Current(consumed - bound))?;

        debug!("Source position set to terminal: {:?}", terminal);

        Ok(Translation::new(
            buffered.initial,
            terminal,
            buffered.elements,
        ))
    }
}
