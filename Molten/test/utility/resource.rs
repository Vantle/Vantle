pub mod attributes {

    use component::graph::attribute::Attribute;
    use constructor::Constructor;
    use std::path::Path;

    pub fn attribute(resource: impl AsRef<Path>) -> Attribute<String> {
        crate::file::cursor(resource.as_ref())
            .attribute()
            .expect("Failed to parse into Attribute")
    }

    pub fn module(resource: impl AsRef<Path>) -> Attribute<String> {
        crate::file::cursor(resource.as_ref())
            .module()
            .expect("Failed to parse into Module")
    }

    pub fn context(resource: impl AsRef<Path>) -> Attribute<String> {
        crate::file::cursor(resource.as_ref())
            .context()
            .expect("Failed to parse into Context")
    }

    pub fn group(resource: impl AsRef<Path>) -> Attribute<String> {
        crate::file::cursor(resource.as_ref())
            .group()
            .expect("Failed to parse into Group")
    }

    pub fn partition(resource: impl AsRef<Path>) -> Attribute<String> {
        crate::file::cursor(resource.as_ref())
            .partition()
            .expect("Failed to parse into Partition")
    }
}

pub mod file {
    use std::borrow::Borrow;
    use std::fs::File;
    use std::io::Cursor;
    use std::io::Read;
    use std::path::{Path, PathBuf};

    use crate::path;

    #[derive(Debug, Clone, Default)]
    pub enum Format {
        #[default]
        JSON,
        ANSI,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Meta {
        path: PathBuf,
        format: Format,
    }

    impl Meta {
        pub fn new(path: impl AsRef<Path>, format: Format) -> Self {
            Self {
                path: path.as_ref().to_path_buf(),
                format,
            }
        }

        pub fn json(path: impl AsRef<Path>) -> Self {
            Self::new(path.as_ref(), Format::JSON)
        }

        pub fn ansi(path: impl AsRef<Path>) -> Self {
            Self::new(path.as_ref(), Format::ANSI)
        }

        pub fn path(&self) -> &PathBuf {
            self.path.borrow()
        }

        pub fn format(&self) -> Format {
            self.format.clone()
        }
    }

    pub fn read(path: impl AsRef<Path>) -> File {
        let path = path.as_ref();
        File::options()
            .read(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open file '{}'", path.display()))
    }

    pub fn write(path: impl AsRef<Path>) -> File {
        let path = path.as_ref();
        path.parent().map(path::create::directory);
        File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to write to file '{}'", path.display()))
    }

    pub fn stringify(path: impl AsRef<Path>) -> String {
        let mut file = read(path);
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .unwrap_or_else(|_| panic!("Failed to read '{:?}' to String buffer", file.metadata()));
        buffer
    }

    pub fn cursor(resource: impl AsRef<Path>) -> Cursor<Vec<u8>> {
        let resource = resource.as_ref();
        let buffer = stringify(resource);
        Cursor::new(buffer.as_bytes().into())
    }
}

pub mod path {
    use std::path::PathBuf;

    pub fn base() -> PathBuf {
        PathBuf::from("Molten/test/resource/")
    }

    pub fn component() -> PathBuf {
        base().join("component/")
    }

    pub fn system() -> PathBuf {
        base().join("system/")
    }

    pub mod create {
        use std::fs;
        use std::path::{Path, PathBuf};

        pub fn directory(path: impl AsRef<Path>) -> PathBuf {
            let path = path.as_ref();
            fs::create_dir_all(path)
                .unwrap_or_else(|_| panic!("Failed to create directory {}", path.display()));
            path.to_path_buf()
        }
    }
}
