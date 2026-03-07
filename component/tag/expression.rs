use std::io::{Cursor, Read, Seek};

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Expression {
    Name(String),
    Not(Box<Expression>),
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Any,
}

impl Expression {
    #[must_use]
    pub fn evaluate<S: AsRef<str>>(&self, names: &[S]) -> bool {
        match self {
            Self::Any => true,
            Self::Name(name) => names.iter().any(|n| n.as_ref() == name),
            Self::Not(inner) => !inner.evaluate(names),
            Self::And(terms) => terms.iter().all(|t| t.evaluate(names)),
            Self::Or(terms) => terms.iter().any(|t| t.evaluate(names)),
        }
    }

    #[must_use]
    pub fn combine(expressions: Vec<Self>) -> Self {
        let mut filtered: Vec<Self> = Vec::new();
        for expression in expressions {
            match expression {
                Self::Any => return Self::Any,
                other => filtered.push(other),
            }
        }
        match filtered.len() {
            0 => Self::Any,
            1 => filtered.into_iter().next().unwrap(),
            _ => Self::Or(filtered),
        }
    }
}

pub fn parse(input: &str) -> Result<Expression, Sourced> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Expression::Any);
    }
    let mut cursor = Cursor::new(trimmed.as_bytes().to_vec());
    let result = sum(&mut cursor).map_err(|error| sourced(input, error))?;
    let remaining = remaining(&mut cursor);
    if !remaining.is_empty() {
        let position = cursor.position().saturating_sub(remaining.len() as u64);
        return Err(sourced(
            input,
            Error::Token {
                expected: "end of expression".to_string(),
                span: offset(position, 1),
            },
        ));
    }
    Ok(result)
}

fn sourced(input: &str, error: Error) -> Sourced {
    Sourced {
        error,
        input: NamedSource::new("expression", input.to_string()),
    }
}

fn offset(position: u64, length: usize) -> SourceSpan {
    SourceSpan::new(usize::try_from(position).unwrap_or(0).into(), length)
}

fn remaining<Source: Read>(source: &mut Source) -> Vec<u8> {
    let mut buffer = Vec::new();
    let _ = source.read_to_end(&mut buffer);
    buffer
}

fn peek<Source: Read + Seek>(source: &mut Source) -> Option<u8> {
    whitespace(source);
    let mut byte = [0u8; 1];
    let position = source.stream_position().unwrap_or(0);
    match source.read(&mut byte) {
        Ok(1..) => {
            source.seek(std::io::SeekFrom::Start(position)).ok();
            Some(byte[0])
        }
        _ => None,
    }
}

fn advance<Source: Read + Seek>(source: &mut Source) {
    let mut byte = [0u8; 1];
    let _ = source.read(&mut byte);
}

fn whitespace<Source: Read + Seek>(source: &mut Source) {
    loop {
        let position = source.stream_position().unwrap_or(0);
        let mut byte = [0u8; 1];
        match source.read(&mut byte) {
            Ok(1) if byte[0].is_ascii_whitespace() => {}
            Ok(1) => {
                source.seek(std::io::SeekFrom::Start(position)).ok();
                break;
            }
            _ => break,
        }
    }
}

fn identifier<Source: Read + Seek>(source: &mut Source) -> Result<String, Error> {
    let mut name = String::new();
    loop {
        let position = source.stream_position().unwrap_or(0);
        let mut byte = [0u8; 1];
        match source.read(&mut byte) {
            Ok(1) if byte[0].is_ascii_alphanumeric() || byte[0] == b'_' => {
                name.push(byte[0] as char);
            }
            Ok(1) => {
                source.seek(std::io::SeekFrom::Start(position)).ok();
                break;
            }
            _ => break,
        }
    }
    if name.is_empty() {
        let position = source.stream_position().unwrap_or(0);
        return Err(Error::End {
            expected: "identifier".to_string(),
            span: offset(position, 0),
        });
    }
    Ok(name)
}

fn sum<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    let mut terms = vec![product(source)?];

    while let Some(b',') = peek(source) {
        advance(source);
        terms.push(product(source)?);
    }

    Ok(match terms.len() {
        1 => terms.into_iter().next().unwrap(),
        _ => Expression::Or(terms),
    })
}

fn product<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    let mut factors = vec![unary(source)?];

    while let Some(b'.') = peek(source) {
        advance(source);
        factors.push(unary(source)?);
    }

    Ok(match factors.len() {
        1 => factors.into_iter().next().unwrap(),
        _ => Expression::And(factors),
    })
}

fn unary<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    match peek(source) {
        Some(b'!') => {
            advance(source);
            let operand = unary(source)?;
            Ok(Expression::Not(Box::new(operand)))
        }
        Some(_) => atom(source),
        None => {
            let position = source.stream_position().unwrap_or(0);
            Err(Error::End {
                expected: "identifier or !".to_string(),
                span: offset(position, 0),
            })
        }
    }
}

fn atom<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    match peek(source) {
        Some(b'(') => {
            advance(source);
            let inner = sum(source)?;
            match peek(source) {
                Some(b')') => {
                    advance(source);
                    Ok(inner)
                }
                Some(_) => {
                    let position = source.stream_position().unwrap_or(0);
                    Err(Error::Token {
                        expected: ")".to_string(),
                        span: offset(position, 1),
                    })
                }
                None => {
                    let position = source.stream_position().unwrap_or(0);
                    Err(Error::End {
                        expected: ")".to_string(),
                        span: offset(position, 0),
                    })
                }
            }
        }
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {
            let name = identifier(source)?;
            Ok(Expression::Name(name))
        }
        Some(_) => {
            let position = source.stream_position().unwrap_or(0);
            Err(Error::Token {
                expected: "identifier or (".to_string(),
                span: offset(position, 1),
            })
        }
        None => {
            let position = source.stream_position().unwrap_or(0);
            Err(Error::End {
                expected: "identifier or (".to_string(),
                span: offset(position, 0),
            })
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("unexpected token in expression")]
    #[diagnostic(
        code(tag::expression::token),
        help("use . (and), , (or), ! (not), () (group) — e.g. performance.sorting, benchmark")
    )]
    Token {
        expected: String,
        #[label("expected {expected}")]
        span: SourceSpan,
    },

    #[error("unexpected end of expression")]
    #[diagnostic(
        code(tag::expression::end),
        help("expression appears incomplete — add the missing operand")
    )]
    End {
        expected: String,
        #[label("expected {expected} after this")]
        span: SourceSpan,
    },
}

#[derive(Error, Debug, Diagnostic)]
#[error("invalid tag filter expression")]
#[diagnostic(code(tag::expression::parse))]
pub struct Sourced {
    #[diagnostic_source]
    pub error: Error,
    #[source_code]
    pub input: NamedSource<String>,
}
