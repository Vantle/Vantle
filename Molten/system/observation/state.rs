pub use error::Error;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use hypergraph::Hypergraph;
use std::io::{Read, Write};
use stream::Snapshot;

const GZIP_HEADER: [u8; 2] = [0x1f, 0x8b];

pub struct Assembler<'a, T>
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    graph: &'a Hypergraph<T>,
    trigger: &'a str,
    compressed: bool,
}

impl<T> Assembler<'_, T>
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    #[must_use]
    pub fn compressed(mut self, enabled: bool) -> Self {
        self.compressed = enabled;
        self
    }

    pub fn assemble(self) -> error::Result<Snapshot> {
        let serialized =
            serde_json::to_vec(self.graph).map_err(|source| Error::Serialize { source })?;

        let state = if self.compressed {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(&serialized)
                .map_err(|source| Error::Compress { source })?;
            encoder
                .finish()
                .map_err(|source| Error::Compress { source })?
        } else {
            serialized
        };

        Ok(Snapshot::now(state, self.trigger.to_string()))
    }
}

#[must_use]
pub fn assembler<'a, T>(graph: &'a Hypergraph<T>, trigger: &'a str) -> Assembler<'a, T>
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    Assembler {
        graph,
        trigger,
        compressed: false,
    }
}

pub fn capture<T>(graph: &Hypergraph<T>, trigger: &str) -> error::Result<Snapshot>
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    assembler(graph, trigger).assemble()
}

pub fn restore<T>(snapshot: &Snapshot) -> error::Result<Hypergraph<T>>
where
    T: Clone + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned,
{
    let decompressed = if snapshot.state.starts_with(&GZIP_HEADER) {
        let mut decoder = GzDecoder::new(&snapshot.state[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|source| Error::Decompress { source })?;
        decompressed
    } else {
        snapshot.state.clone()
    };

    serde_json::from_slice(&decompressed).map_err(|source| Error::Deserialize { source })
}
