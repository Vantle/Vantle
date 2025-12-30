pub use error;

use cached::proc_macro::once;
use error::Error;
use observe::trace;
use syntect::parsing::SyntaxDefinition;

#[once(result = true)]
#[trace(channels = [core])]
pub fn syntax() -> error::Result<SyntaxDefinition> {
    let path = "Molten/resource/system/graph/syntax.yaml";
    let content = std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_string(),
        source,
    })?;
    SyntaxDefinition::load_from_str(&content, false, None).map_err(|source| Error::Parse {
        path: path.to_string(),
        details: source.to_string(),
    })
}
