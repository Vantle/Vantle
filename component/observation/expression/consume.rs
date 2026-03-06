use error::Error;
use translate::Rules;
use translator::Translation;

pub type Result<T> = std::result::Result<T, Error>;

pub fn space<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    Ok(Translation::rules()
        .terminator(rule::glyph())
        .consume(source.by_ref())?)
}

pub fn next<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<Translation<u8>> {
    Ok(Translation::rules().limiter(1).consume(source.by_ref())?)
}

pub fn name<Source: std::io::Read + std::io::Seek>(mut source: Source) -> Result<String> {
    let translation = Translation::rules()
        .terminator(|b: u8| !b.is_ascii_alphanumeric())
        .consume(source.by_ref())?;
    Ok(translation.characterize().parsed())
}
