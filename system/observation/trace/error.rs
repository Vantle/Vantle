use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("invalid channel expression in sink URI: {expression}")]
    #[diagnostic(
        code(trace::sink::channels),
        help("use . (and), , (or), ! (not), () (group) — e.g. core,http.!debug")
    )]
    Channels {
        expression: String,
        #[source]
        source: expression::Sourced,
    },

    #[error("failed to initialize tracing subscriber: {details}")]
    #[diagnostic(
        code(trace::subscriber),
        help("ensure no other subscriber is installed")
    )]
    Subscriber { details: String },

    #[error("failed to create trace file: {path}")]
    #[diagnostic(code(trace::file), help("check directory permissions and disk space"))]
    File {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid sink address: {address}")]
    #[diagnostic(
        code(trace::sink::parse),
        help("use scheme://path (e.g., log:///tmp/trace.jsonl, http://127.0.0.1:3000)")
    )]
    Parse {
        address: String,
        #[source]
        source: url::ParseError,
    },

    #[error("unsupported sink scheme: {scheme}")]
    #[diagnostic(
        code(trace::sink::scheme),
        help(
            "supported schemes: log, chrome, grpc, http (e.g., log:///tmp/trace.jsonl, http://127.0.0.1:3000)"
        )
    )]
    Scheme { scheme: String },

    #[error("sink address missing host: {address}")]
    #[diagnostic(
        code(trace::sink::host),
        help("provide host and port (e.g., grpc://127.0.0.1:50051)")
    )]
    Host { address: String },

    #[error("sink address missing port: {address}")]
    #[diagnostic(
        code(trace::sink::port),
        help("provide host and port (e.g., grpc://127.0.0.1:50051)")
    )]
    Port { address: String },

    #[error("failed to spawn trace thread")]
    #[diagnostic(code(trace::thread), help("system may be out of resources"))]
    Thread,

    #[error("failed to connect to sink at {address}")]
    #[diagnostic(code(trace::connect), help("verify server is running and reachable"))]
    Connect {
        address: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
