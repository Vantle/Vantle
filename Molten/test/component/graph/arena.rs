#[cfg(test)]
mod tests {

    mod index {
        use component::graph::arena::Valued;

        pub mod resources {
            use std::path::PathBuf;

            pub fn module() -> PathBuf {
                resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn echo() {
            file::test::equality(
                Valued::from(resource::attributes::module(
                    resources::module().join("echo/symbolic/echo.lava"),
                )),
                resource::file::Meta::json(
                    resources::module().join("echo/generated/echo.arena.json"),
                ),
                resource::file::Meta::json(resources::module().join("echo/echo.arena.json")),
                resource::file::Meta::ansi(
                    resources::module().join("echo/generated/echo.arena.json"),
                ),
            );
        }

        #[test]
        fn boolean() {
            file::test::equality(
                Valued::from(resource::attributes::module(
                    resources::module().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
                )),
                resource::file::Meta::json(
                    resources::module()
                        .join("math/numeric/logic/boolean/generated/boolean.arena.json"),
                ),
                resource::file::Meta::json(
                    resources::module().join("math/numeric/logic/boolean/boolean.arena.json"),
                ),
                resource::file::Meta::ansi(
                    resources::module().join("math/numeric/logic/boolean/generated/boolean.arena"),
                ),
            );
        }
    }
}
