#[cfg(test)]
mod tests {

    mod index {
        use component::graph::arena::Valued;

        pub mod resources {
            use std::path::PathBuf;

            pub fn module() -> PathBuf {
                utility::resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn echo() {
            utility::file::test::equality(
                Valued::from(utility::resource::attributes::module(
                    resources::module().join("echo/symbolic/echo.lava"),
                )),
                utility::resource::file::Meta::json(
                    resources::module().join("echo/generated/echo.arena.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::module().join("echo/echo.arena.json"),
                ),
                utility::resource::file::Meta::ansi(
                    resources::module().join("echo/generated/echo.arena.json"),
                ),
            );
        }

        #[test]
        fn boolean() {
            utility::file::test::equality(
                Valued::from(utility::resource::attributes::module(
                    resources::module().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
                )),
                utility::resource::file::Meta::json(
                    resources::module()
                        .join("math/numeric/logic/boolean/generated/boolean.arena.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::module().join("math/numeric/logic/boolean/boolean.arena.json"),
                ),
                utility::resource::file::Meta::ansi(
                    resources::module().join("math/numeric/logic/boolean/generated/boolean.arena"),
                ),
            );
        }
    }
}
