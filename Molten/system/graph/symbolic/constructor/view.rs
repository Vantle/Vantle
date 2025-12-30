use observe::trace;

use component::graph::symbolic::translator::Translation;
use error::Error;
use translator::view::Rules;

pub type Result<T> = std::result::Result<T, Error>;

#[trace(channels = [core])]
pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    Ok(Translation::rules().limiter(1).view(source.by_ref())?)
}
