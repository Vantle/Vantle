#[cfg(test)]
mod tests {

    pub mod matrices {
        use arena::Valued;
        use std::borrow::Borrow;

        pub mod resources {

            use std::path::PathBuf;

            pub fn system() -> PathBuf {
                resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn echo() {
            let echo =
                resource::attributes::module(resources::system().join("echo/symbolic/echo.lava"));
            let index = Valued::from(echo.clone());
            file::test::equality(
                constructor::relate(echo.borrow(), index.borrow()),
                resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.relation.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system().join("echo/echo.forge.relation.artifact.json"),
                ),
                resource::file::Meta::json(
                    resources::system()
                        .join("echo/generated/echo.forge.relation.artifact.difference"),
                ),
            );
        }

        #[test]
        fn boolean() {
            let boolean = resource::attributes::module(
                resources::system().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
            );
            let _ = Valued::from(boolean.clone());
            // file::test::equality(
            //     constructor::relate(boolean.borrow(), index.borrow()),
            //     resource::file::Meta::json(
            //         resources::system().join("math/numeric/logic/boolean/generated/generated/boolean.forge.relation.artifact.json"),
            //     ),
            //     resource::file::Meta::json(
            //         resources::system().join("math/numeric/logic/boolean/boolean.forge.relation.artifact.json"),
            //     ),
            //     resource::file::Meta::json(
            //         resources::system()
            //             .join("math/numeric/logic/boolean/generated/boolean.forge.relation.artifact.difference"),
            //     ),
            // );
        }
    }
}
