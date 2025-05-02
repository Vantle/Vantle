pub mod file {
    use serde;
    use serde::de::DeserializeOwned;
    use serde_json;

    use logging::debug;
    use resource::file::{Format, Meta};
    use std::borrow::Borrow;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    pub fn read<Value: DeserializeOwned>(meta: Meta) -> Value {
        let file = resource::file::read(meta.path());
        match meta.format() {
            Format::JSON => serde_json::from_reader(BufReader::new(file))
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
            format => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("{:#?} reading unsupported", format),
            )),
        }
        .unwrap_or_else(|error| panic!("Failed to deserialize {:#?} with {:#?}", meta, error))
    }

    pub fn write<Value: serde::Serialize>(meta: Meta, value: &Value) -> File {
        let file = resource::file::write(meta.path());
        match meta.format() {
            Format::JSON => serde_json::to_writer_pretty(BufWriter::new(file.borrow()), value)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
            format => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("{:#?} writing unsupported", format),
            )),
        }
        .unwrap_or_else(|error| panic!("Failed to serialize {:#?} with {:#?}", meta, error));
        debug!("Generated `{:#?}`", file.metadata());
        file
    }
}
