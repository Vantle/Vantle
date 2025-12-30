use clap::Parser;
use miette::{Diagnostic, Result};
use record::info;
use std::net::SocketAddr;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Invalid address: {address}")]
    #[diagnostic(code(portal::address), help("Use format host:port"))]
    Address {
        address: String,
        #[source]
        source: std::net::AddrParseError,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Server(#[from] server::error::Error),
}

#[derive(Parser)]
#[command(name = "portal")]
#[command(about = "Observation server for trace streaming")]
#[command(version)]
struct Arguments {
    #[arg(short, long, default_value = "127.0.0.1:50051")]
    address: String,

    #[arg(short, long, default_value = "4096")]
    capacity: usize,

    #[arg(long, help = "Store traces to URI (e.g., file:///tmp/traces)")]
    store: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    let address: SocketAddr = args.address.parse().map_err(|source| Error::Address {
        address: args.address.clone(),
        source,
    })?;

    let mut assembler = server::Portal::assembler().capacity(args.capacity);

    if let Some(store) = args.store {
        let path = store.strip_prefix("file://").unwrap_or(&store);
        assembler = assembler.storage(PathBuf::from(path));
    }

    let portal = assembler.assemble();

    info!("listening on {address}");

    server::serve(address, portal).await?;

    Ok(())
}
