#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("failed to serialize hypergraph state")]
    #[diagnostic(
        code(observation::state::serialize),
        help("ensure all types implement Serialize")
    )]
    Serialize {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to deserialize hypergraph state")]
    #[diagnostic(
        code(observation::state::deserialize),
        help("snapshot may be corrupted or from incompatible version")
    )]
    Deserialize {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to compress hypergraph state")]
    #[diagnostic(code(observation::state::compress), help("compression error occurred"))]
    Compress {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to decompress hypergraph state")]
    #[diagnostic(
        code(observation::state::decompress),
        help("snapshot may be corrupted or not compressed")
    )]
    Decompress {
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
