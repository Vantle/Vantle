use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Invalid socket address: {address}")]
    #[diagnostic(
        code(observation::server::address),
        help("Use format host:port (e.g., grpc://localhost:50051)")
    )]
    Address {
        address: String,
        #[source]
        source: std::net::AddrParseError,
    },

    #[error("No source address configured")]
    #[diagnostic(
        code(observation::client::source),
        help("Call .source(address) before .connect()")
    )]
    Source,

    #[error("Connection failed: {details}")]
    #[diagnostic(
        code(observation::client::connection),
        help("Ensure the host is running")
    )]
    Connection { details: String },

    #[error("Failed to create record file: {path}")]
    #[diagnostic(
        code(observation::record::create),
        help("Check directory permissions and disk space")
    )]
    Create {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to open record file: {path}")]
    #[diagnostic(
        code(observation::record::open),
        help("Ensure the file exists and is readable")
    )]
    Open {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write record: {path}")]
    #[diagnostic(code(observation::record::write), help("Check disk space"))]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read record: {path}")]
    #[diagnostic(
        code(observation::record::read),
        help("Ensure the file is not corrupted")
    )]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid record format: {path}")]
    #[diagnostic(
        code(observation::record::format),
        help("The file may be corrupted or in an incompatible format")
    )]
    Format {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Server initialization failed")]
    #[diagnostic(code(observation::server::transport), help("Check port availability"))]
    Server {
        #[source]
        source: tonic::transport::Error,
    },

    #[error("Connection not found: {handle}")]
    #[diagnostic(
        code(observation::peer::handle),
        help("Verify the handle was returned from sink() or source()")
    )]
    Handle { handle: u64 },
}
