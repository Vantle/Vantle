use std::io::{Read, Seek, SeekFrom};

use observe::trace;
use record::debug;

use rule::{Lambda, Rules as Data};
use translator::Translation;

pub type Result<T> = error::Result<T>;

pub trait Rules {
    fn view<Source, T>(&self, source: T) -> Result<Translation<u8>>
    where
        T: Translator<Source, u8>;
}

impl Rules for Data<u8> {
    #[trace(channels = [core])]
    fn view<Source, T>(&self, mut source: T) -> Result<Translation<u8>>
    where
        T: Translator<Source, u8>,
    {
        source.view(&*self.filter, &*self.terminator, self.limiter)
    }
}

pub trait Translator<Source, Element> {
    fn view(
        &mut self,
        filter: &Lambda<Element>,
        terminator: &Lambda<Element>,
        limit: Option<usize>,
    ) -> Result<Translation<Element>>;
}

impl<Source: Read + Seek> Translator<Source, u8> for Source {
    #[trace(channels = [core])]
    fn view(
        &mut self,
        filter: &Lambda<u8>,
        terminator: &Lambda<u8>,
        limiter: Option<usize>,
    ) -> Result<Translation<u8>> {
        let initial = self.stream_position()?;
        let remaining = self.seek(SeekFrom::End(0))? - initial;
        let reset = self.seek(SeekFrom::Start(initial))?;
        debug!("Reset to {:?}", reset);

        let remaining = usize::try_from(remaining).unwrap_or(usize::MAX);
        let consumable = limiter.map_or(remaining, |limit| std::cmp::min(remaining, limit));

        let mut buffer = vec![0u8; consumable];

        self.read_exact(buffer.as_mut_slice())?;

        let translation = buffer
            .iter()
            .take_while(|&&element| !terminator(element))
            .filter(|&&element| filter(element))
            .copied()
            .collect::<Vec<u8>>();

        debug!(
            "Translation with rules and limit of {:?} capped at consumable {:?}: {:?}",
            limiter, consumable, translation
        );

        let terminal = initial + u64::try_from(translation.len()).unwrap_or(u64::MAX);
        self.seek(SeekFrom::Start(initial))?;

        debug!("Source position reset to initial: {:?}", initial);

        Ok(Translation::new(initial, terminal, translation))
    }
}
