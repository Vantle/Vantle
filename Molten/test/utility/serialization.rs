pub mod file {
    use serde;
    use serde::de::DeserializeOwned;
    use serde_json;

    use autolog::debug;
    use resource::file::{Format, Meta};
    use std::borrow::Borrow;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    pub fn read<Value: DeserializeOwned>(meta: Meta) -> Value {
        let file = resource::file::read(meta.path());
        match meta.format() {
            Format::JSON => serde_json::from_reader(BufReader::new(file))
                .map_err(std::io::Error::other)
                .unwrap_or_else(|error| {
                    panic!("Failed to deserialize {:#?} with {:#?}", meta, error)
                }),
            format => panic!("{:#?} reading unsupported", format),
        }
    }

    pub fn write<Value: serde::Serialize>(meta: Meta, value: &Value) -> File {
        let file = resource::file::write(meta.path());
        match meta.format() {
            Format::JSON => serde_json::to_writer_pretty(BufWriter::new(file.borrow()), value)
                .map_err(std::io::Error::other)
                .unwrap_or_else(|error| {
                    panic!("Failed to serialize {:#?} with {:#?}", meta, error)
                }),
            format => panic!("{:#?} writing unsupported", format),
        }
        debug!("Generated `{:#?}`", file.metadata());
        file
    }
}
