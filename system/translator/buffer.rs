use std::io::{Read, Seek, SeekFrom};

use record::debug;

use rule::Lambda;

pub type Result<T> = error::Result<T>;

pub struct Buffered {
    pub initial: u64,
    pub consumable: usize,
    pub elements: Vec<u8>,
}

pub fn translate<Source: Read + Seek>(
    source: &mut Source,
    filter: &Lambda<u8>,
    terminator: &Lambda<u8>,
    limiter: Option<usize>,
) -> Result<Buffered> {
    let initial = source.stream_position()?;
    let remaining = source.seek(SeekFrom::End(0))? - initial;
    let reset = source.seek(SeekFrom::Start(initial))?;
    debug!("Reset to {:?}", reset);

    let remaining = usize::try_from(remaining).unwrap_or(usize::MAX);
    let consumable = limiter.map_or(remaining, |limit| std::cmp::min(remaining, limit));

    let mut buffer = vec![0u8; consumable];

    source.read_exact(buffer.as_mut_slice())?;

    let elements = buffer
        .iter()
        .take_while(|&&element| !terminator(element))
        .filter(|&&element| filter(element))
        .copied()
        .collect::<Vec<u8>>();

    debug!(
        "Translation with rules and limit of {:?} capped at consumable {:?}: {:?}",
        limiter, consumable, elements
    );

    Ok(Buffered {
        initial,
        consumable,
        elements,
    })
}
