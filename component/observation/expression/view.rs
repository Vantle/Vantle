use error::Error;
use translate::Rules;
use translator::Translation;

pub type Result<T> = std::result::Result<T, Error>;

pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    Ok(Translation::rules().limiter(1).view(source.by_ref())?)
}
