use std::io::{Cursor, Read};
use symbolic::translator::{rule, Translation};

mod view {
    use super::*;

    pub fn quantity(input: String, limit: usize) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = Translation::rules()
            .limiter(limit)
            .view(cursor.by_ref())
            .expect("Failed to get view");
        viewed.characterize().parsed()
    }

    pub fn termination(input: String, terminator: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = Translation::rules()
            .terminator(rule::is(terminator as u8))
            .view(cursor.by_ref())
            .expect("Failed to get view");
        viewed.characterize().parsed()
    }

    pub fn filter(input: String, target: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = Translation::rules()
            .filter(rule::is(target as u8))
            .view(cursor.by_ref())
            .expect("Failed to get view");
        viewed.characterize().parsed()
    }

    pub fn space(input: String) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let viewed = Translation::rules()
            .terminator(rule::glyph())
            .view(cursor.by_ref())
            .expect("Failed to get view");
        viewed.characterize().parsed()
    }
}

mod consume {
    use super::*;

    pub fn quantity(input: String, limit: usize) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = Translation::rules()
            .limiter(limit)
            .consume(cursor.by_ref())
            .expect("Failed to consume");
        consumed.characterize().parsed()
    }

    pub fn termination(input: String, terminator: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = Translation::rules()
            .terminator(rule::is(terminator as u8))
            .consume(cursor.by_ref())
            .expect("Failed to consume");
        consumed.characterize().parsed()
    }

    pub fn filter(input: String, target: char) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = Translation::rules()
            .filter(rule::is(target as u8))
            .consume(cursor.by_ref())
            .expect("Failed to consume");
        consumed.characterize().parsed()
    }

    pub fn space(input: String) -> String {
        let mut cursor = Cursor::new(input.as_bytes());
        let consumed = Translation::rules()
            .terminator(rule::glyph())
            .consume(cursor.by_ref())
            .expect("Failed to consume");
        consumed.characterize().parsed()
    }
}
