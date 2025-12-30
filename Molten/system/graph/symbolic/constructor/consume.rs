use observe::trace;
use record::debug;

use component::graph::symbolic::translator::Translation;
use error::Error;
use translator::consume::Rules;

pub type Result<T> = std::result::Result<T, Error>;

#[trace(channels = [core])]
pub fn space<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    let skipped = Translation::rules()
        .terminator(component::graph::symbolic::translator::rule::glyph())
        .consume(source.by_ref())?;
    debug!("Advance: {:?}", skipped.length());
    Ok(skipped)
}

#[trace(channels = [core])]
pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    Ok(Translation::rules().limiter(1).consume(source.by_ref())?)
}
