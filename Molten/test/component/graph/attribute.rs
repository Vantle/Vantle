#[cfg(test)]
mod tests {

    pub mod resources {
        use std::path::PathBuf;

        pub fn module() -> PathBuf {
            resource::path::system().join("graph/module/")
        }
    }

    use attribute::Attribute;

    #[test]
    fn breath() {
        file::test::equality(
            resource::attributes::module(resources::module().join("breadth/symbolic/breadth.lava"))
                .breadth()
                .cloned()
                .collect::<Vec<Attribute<String>>>(),
            resource::file::Meta::json(
                resources::module().join("breadth/generated/breadth.breadth.json"),
            ),
            resource::file::Meta::json(resources::module().join("breadth/breadth.breadth.json")),
            resource::file::Meta::ansi("generated/breadth.difference.ansi"),
        );
    }
}
