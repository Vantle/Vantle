#[cfg(test)]
mod tests {
    mod attributes {

        pub mod resources {
            use std::path::PathBuf;

            pub fn component() -> PathBuf {
                utility::resource::path::component().join("graph/attribute/")
            }
        }

        #[test]
        fn attribute() {
            utility::file::test::equality(
                utility::resource::attributes::attribute(
                    resources::component().join("attribute/attribute.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::component()
                        .join("attribute/generated/attribute.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("attribute/attribute.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component()
                        .join("attribute/generated/attribute.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn group() {
            utility::file::test::equality(
                utility::resource::attributes::attribute(
                    resources::component().join("group/group.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("group/generated/group.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("group/group.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("group/generated/group.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn partition() {
            utility::file::test::equality(
                utility::resource::attributes::attribute(
                    resources::component().join("partition/partition.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::component()
                        .join("partition/generated/partition.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("partition/partition.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component()
                        .join("partition/generated/partition.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn context() {
            utility::file::test::equality(
                utility::resource::attributes::context(
                    resources::component().join("context/context.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("context/generated/context.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::component().join("context/context.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
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
                utility::resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn breadth() {
            utility::file::test::equality(
                utility::resource::attributes::module(
                    resources::system().join("breadth/symbolic/breadth.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("breadth/generated/breadth.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("breadth/breadth.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("breadth/generated/breadth.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn nested() {
            utility::file::test::equality(
                utility::resource::attributes::module(
                    resources::system().join("nested/symbolic/nested.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("nested/generated/nested.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("nested/nested.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("nested/generated/nested.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn echo() {
            utility::file::test::equality(
                utility::resource::attributes::module(
                    resources::system().join("echo/symbolic/echo.lava"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("echo/echo.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.artifact.difference"),
                ),
            );
        }

        #[test]
        fn boolean() {
            utility::file::test::equality(
                utility::resource::attributes::module(
                    resources::system().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
                ),
                utility::resource::file::Meta::json(
                    resources::system()
                        .join("math/numeric/logic/boolean/generated/boolean.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system()
                        .join("math/numeric/logic/boolean/boolean.forge.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join(
                        "math/numeric/logic/boolean/symbolic/boolean.forge.artifact.difference",
                    ),
                ),
            );
        }
    }
}
