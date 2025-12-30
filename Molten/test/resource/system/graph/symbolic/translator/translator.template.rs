use component::graph::symbolic::translator::Translation;
use component::graph::symbolic::translator::rule;
use std::io::{Cursor, Read};
use symbolic::translator::consume::Rules as ConsumeRules;
use symbolic::translator::view::Rules as ViewRules;

mod view {
    use super::{Cursor, Read, Translation, ViewRules, rule};

    pub fn quantity(input: String, limit: usize) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = utility::unwrap(Translation::rules().limiter(limit).view(cursor.by_ref()));
        viewed.characterize().parsed()
    }

    pub fn termination(input: String, terminator: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = utility::unwrap(
            Translation::rules()
                .terminator(rule::is(terminator as u8))
                .view(cursor.by_ref()),
        );
        viewed.characterize().parsed()
    }

    pub fn filter(input: String, target: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = utility::unwrap(
            Translation::rules()
                .filter(rule::is(target as u8))
                .view(cursor.by_ref()),
        );
        viewed.characterize().parsed()
    }

    pub fn space(input: String) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = utility::unwrap(
            Translation::rules()
                .terminator(rule::glyph())
                .view(cursor.by_ref()),
        );
        viewed.characterize().parsed()
    }
}

mod consume {
    use super::{ConsumeRules, Cursor, Read, Translation, rule};

    pub fn quantity(input: String, limit: usize) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed =
            utility::unwrap(Translation::rules().limiter(limit).consume(cursor.by_ref()));
        consumed.characterize().parsed()
    }

    pub fn termination(input: String, terminator: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = utility::unwrap(
            Translation::rules()
                .terminator(rule::is(terminator as u8))
                .consume(cursor.by_ref()),
        );
        consumed.characterize().parsed()
    }

    pub fn filter(input: String, target: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = utility::unwrap(
            Translation::rules()
                .filter(rule::is(target as u8))
                .consume(cursor.by_ref()),
        );
        consumed.characterize().parsed()
    }

    pub fn space(input: String) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = utility::unwrap(
            Translation::rules()
                .terminator(rule::glyph())
                .consume(cursor.by_ref()),
        );
        consumed.characterize().parsed()
    }
}
