use miette::{Diagnostic, NamedSource, SourceOffset};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Runtime {
    #[error("deserialization failed for `{target}`")]
    #[diagnostic(code(runtime::deserialization))]
    Deserialization {
        target: String,

        #[source_code]
        json: NamedSource<String>,

        #[label("here")]
        location: SourceOffset,

        #[help]
        help: String,

        #[source]
        cause: serde_json::Error,
    },
}

impl Runtime {
    #[must_use]
    pub fn deserialization(
        target: &str,
        source: &str,
        json: impl Into<String>,
        error: serde_json::Error,
    ) -> Self {
        let json = json.into();

        let offset = {
            let line = error.line().saturating_sub(1);
            let column = error.column().saturating_sub(1);

            let mut offset = 0;
            for (index, row) in json.lines().enumerate() {
                if index == line {
                    offset += column;
                    break;
                }
                offset += row.len() + 1;
            }
            offset
        };

        let help = format!(
            "In file: {source}\nExpected type: {target}\nCheck that your structure matches the expected target format."
        );

        Self::Deserialization {
            target: target.to_string(),
            json: NamedSource::new(source, json).with_language("json"),
            location: SourceOffset::from(offset),
            help,
            cause: error,
        }
    }
}
