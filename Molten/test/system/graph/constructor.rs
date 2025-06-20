#[cfg(test)]
mod tests {

    pub mod matrices {
        use component::graph::arena::Valued;
        use component::graph::traits::attribute::Contextualized;
        use std::borrow::Borrow;

        pub mod resources {

            use std::path::PathBuf;

            pub fn system() -> PathBuf {
                utility::resource::path::system().join("graph/module/")
            }
        }

        #[test]
        fn echo() {
            let echo = utility::resource::attributes::module(
                resources::system().join("echo/symbolic/echo.lava"),
            );
            let index = Valued::from(echo.clone());
            utility::file::test::equality(
                system::graph::constructor::relate(echo.context(), index.borrow()),
                utility::resource::file::Meta::json(
                    resources::system().join("echo/generated/echo.forge.relation.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("echo/echo.forge.relation.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system()
                        .join("echo/generated/echo.forge.relation.artifact.difference"),
                ),
            );
        }

        #[test]
        fn boolean() {
            let boolean = utility::resource::attributes::module(
                resources::system().join("math/numeric/logic/boolean/symbolic/boolean.magma"),
            );
            let index = Valued::from(boolean.clone());
            utility::file::test::equality(
                system::graph::constructor::relate(boolean.context(), index.borrow()),
                utility::resource::file::Meta::json(
                    resources::system().join("math/numeric/logic/boolean/generated/generated/boolean.forge.relation.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system().join("math/numeric/logic/boolean/boolean.forge.relation.artifact.json"),
                ),
                utility::resource::file::Meta::json(
                    resources::system()
                        .join("math/numeric/logic/boolean/generated/boolean.forge.relation.artifact.difference"),
                ),
            );
        }
    }
}
