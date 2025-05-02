#[cfg(test)]
mod tests {
    mod attributes {

        pub mod resources {
            use std::path::PathBuf;

            pub fn component() -> PathBuf {
                resource::path::component().join("graph/attribute/")
            }
        }

        #[test]
        fn attribute() {
            file::test::equality(
                resource::attributes::attribute(
                    resources::component().join("attribute/attribute.lava"),
                ),
                resource::file::Meta::json(
                    resources::component()
                        .join("attribute/generated/attribute.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component().join("attribute/attribute.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component()
                        .join("attribute/generated/attribute.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn group() {
            file::test::equality(
                resource::attributes::attribute(resources::component().join("group/group.lava")),
                resource::file::Meta::json(
                    resources::component().join("group/generated/group.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component().join("group/group.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component().join("group/generated/group.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn partition() {
            file::test::equality(
                resource::attributes::attribute(
                    resources::component().join("partition/partition.lava"),
                ),
                resource::file::Meta::json(
                    resources::component()
                        .join("partition/generated/partition.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component().join("partition/partition.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component()
                        .join("partition/generated/partition.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn context() {
            file::test::equality(
                resource::attributes::context(resources::component().join("context/context.lava")),
                resource::file::Meta::json(
                    resources::component().join("context/generated/context.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component().join("context/context.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::component()
                        .join("context/generated/context.forge.artifact.difference"),
                ),
            );
        }
    }

    mod traversal {}

    mod modules {
        pub mod resources {
            use std::path::PathBuf;

            pub fn system() -> PathBuf {
                resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn breadth() {
            file::test::equality(
                resource::attributes::module(
                    resources::system().join("breadth/symbolic/breadth.lava"),
                ),
                resource::file::Meta::json(
                    resources::system().join("breadth/generated/breadth.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("breadth/breadth.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("breadth/generated/breadth.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn nested() {
            file::test::equality(
                resource::attributes::module(
                    resources::system().join("nested/symbolic/nested.lava"),
                ),
                resource::file::Meta::json(
                    resources::system().join("nested/generated/nested.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("nested/nested.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("nested/generated/nested.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn echo() {
            file::test::equality(
                resource::attributes::module(resources::system().join("echo/symbolic/echo.lava")),
                resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("echo/echo.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn boolean() {
            file::test::equality(
                resource::attributes::module(
                    resources::system().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
                ),
                resource::file::Meta::json(
                    resources::system()
                        .join("math/numeric/logic/boolean/generated/boolean.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system()
                        .join("math/numeric/logic/boolean/boolean.forge.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join(
                        "math/numeric/logic/boolean/symbolic/boolean.forge.artifact.difference",
                    ),
                ),
            );
        }
    }
}
