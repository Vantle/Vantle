pub mod test {
    use resource::file::Meta;
    use std::borrow::Borrow;
    use std::fs::File;

    pub fn equality<Type>(value: Type, artifact: Meta, expected: Meta, difference: Meta) -> File
    where
        Type: serde::Serialize
            + for<'view> serde::Deserialize<'view>
            + std::cmp::PartialEq
            + std::fmt::Debug
            + std::panic::RefUnwindSafe,
    {
        let file = serialization::file::write(artifact, value.borrow());
        standard::test::equality(value, serialization::file::read(expected), difference);
        file
    }
}
