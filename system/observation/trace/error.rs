use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
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

    #[error("Invalid sink address: {address}")]
    #[diagnostic(
        code(trace::sink::parse),
        help("Use grpc://host:port (e.g., grpc://127.0.0.1:50051)")
    )]
    Parse {
        address: String,
        #[source]
        source: url::ParseError,
    },

    #[error("Unsupported sink scheme: {scheme}")]
    #[diagnostic(
        code(trace::sink::scheme),
        help(
            "Supported schemes: file, grpc (e.g., file:///tmp/trace.log, grpc://127.0.0.1:50051)"
        )
    )]
    Scheme { scheme: String },

    #[error("Sink address missing host: {address}")]
    #[diagnostic(
        code(trace::sink::host),
        help("Provide host and port (e.g., grpc://127.0.0.1:50051)")
    )]
    Host { address: String },

    #[error("Sink address missing port: {address}")]
    #[diagnostic(
        code(trace::sink::port),
        help("Provide host and port (e.g., grpc://127.0.0.1:50051)")
    )]
    Port { address: String },

    #[error("failed to spawn observation thread")]
    #[diagnostic(code(trace::thread), help("system may be out of resources"))]
    Thread,

    #[error("failed to create tokio runtime: {details}")]
    #[diagnostic(code(trace::runtime), help("check system resource limits"))]
    Runtime { details: String },

    #[error("failed to connect to sink at {address}")]
    #[diagnostic(code(trace::connect), help("verify server is running and reachable"))]
    Connect {
        address: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
