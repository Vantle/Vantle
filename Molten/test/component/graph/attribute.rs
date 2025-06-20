#[cfg(test)]
mod tests {

    pub mod resources {
        use std::path::PathBuf;

        pub fn module() -> PathBuf {
            utility::resource::path::system().join("graph/module/")
        }
    }

    use component::graph::attribute::Attribute;

    #[test]
    fn breath() {
        utility::file::test::equality(
            utility::resource::attributes::module(
                resources::module().join("breadth/symbolic/breadth.lava"),
            )
            .breadth()
            .cloned()
            .collect::<Vec<Attribute<String>>>(),
            utility::resource::file::Meta::json(
                resources::module().join("breadth/generated/breadth.breadth.json"),
            ),
            utility::resource::file::Meta::json(
                resources::module().join("breadth/breadth.breadth.json"),
            ),
            utility::resource::file::Meta::ansi("generated/breadth.difference.ansi"),
        );
    }
}
