use syntax::Syntax;

mod from {
    use super::Syntax;

    pub fn u8(value: u8) -> String {
        format!("{:?}", Syntax::from(value))
    }
}
