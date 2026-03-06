use std::io::{Cursor, Read, Seek};

use miette::{NamedSource, SourceSpan};

use expression::{Error, Expression, Sourced};

pub fn parse(input: &str) -> Result<Expression, Sourced> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Expression::Any);
    }
    let mut source = Cursor::new(trimmed.as_bytes().to_vec());
    let result = sum(&mut source).map_err(|error| sourced(input, error))?;
    let peeked = view::next(&mut source).map_err(|error| sourced(input, bridge(error)))?;
    if !peeked.is_empty() {
        return Err(sourced(
            input,
            Error::Token {
                expected: "end of expression".to_string(),
                span: offset(peeked.initial(), 1),
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

fn bridge(_error: error::Error) -> Error {
    Error::Token {
        expected: "valid input".to_string(),
        span: SourceSpan::new(0.into(), 0),
    }
}

fn peek<Source: Read + Seek>(source: &mut Source) -> Result<Option<u8>, Error> {
    consume::space(source.by_ref()).map_err(bridge)?;
    let peeked = view::next(source.by_ref()).map_err(bridge)?;
    if peeked.is_empty() {
        Ok(None)
    } else {
        Ok(Some(peeked.elements()[0]))
    }
}

fn advance<Source: Read + Seek>(source: &mut Source) -> Result<(), Error> {
    consume::next(source.by_ref()).map_err(bridge)?;
    Ok(())
}

fn identifier<Source: Read + Seek>(source: &mut Source) -> Result<String, Error> {
    consume::name(source.by_ref()).map_err(bridge)
}

fn sum<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    let mut terms = vec![product(source)?];

    while let Some(b',') = peek(source)? {
        advance(source)?;
        terms.push(product(source)?);
    }

    Ok(match terms.len() {
        1 => terms.into_iter().next().unwrap(),
        _ => Expression::Or(terms),
    })
}

fn product<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    let mut factors = vec![unary(source)?];

    while let Some(b'.') = peek(source)? {
        advance(source)?;
        factors.push(unary(source)?);
    }

    Ok(match factors.len() {
        1 => factors.into_iter().next().unwrap(),
        _ => Expression::And(factors),
    })
}

fn unary<Source: Read + Seek>(source: &mut Source) -> Result<Expression, Error> {
    match peek(source)? {
        Some(b'!') => {
            advance(source)?;
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
    match peek(source)? {
        Some(b'(') => {
            advance(source)?;
            let inner = sum(source)?;
            match peek(source)? {
                Some(b')') => {
                    advance(source)?;
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
        Some(b) if b.is_ascii_alphabetic() => {
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
