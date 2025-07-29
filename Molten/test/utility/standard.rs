pub mod test {
    use autolog::debug;
    pub use pretty_assertions::assert_eq;
    use std::io::Write;
    use std::panic;

    pub fn equality<Type: std::cmp::PartialEq + std::fmt::Debug + std::panic::RefUnwindSafe>(
        result: Type,
        expected: Type,
        difference: resource::file::Meta,
    ) {
        let equivalence = panic::catch_unwind(|| assert_eq!(result, expected));

        match equivalence {
            Err(panic) => {
                if let Ok(message) = panic.downcast::<String>() {
                    resource::file::write(difference.path().with_extension("difference.ansi"))
                        .write_all(message.as_bytes())
                        .expect("Failed to write all message bytes to open file");
                }
                panic!("{:#?} does not equal expected {:#?}", result, expected);
            }
            Ok(()) => debug!("Equality"),
        }
    }
}
