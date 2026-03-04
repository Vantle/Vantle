use miette::{NamedSource, SourceSpan};

use expression::{Error, Expression, Sourced};
use token::{Located, Token, tokenize};

pub fn parse(input: &str) -> Result<Expression, Sourced> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Expression::Any);
    }
    let tokens = tokenize(trimmed).map_err(|error| Sourced {
        error,
        input: NamedSource::new("expression", input.to_string()),
    })?;
    let mut cursor = 0;
    let result = sum(&tokens, &mut cursor).map_err(|error| Sourced {
        error,
        input: NamedSource::new("expression", input.to_string()),
    })?;
    if cursor < tokens.len() {
        return Err(Sourced {
            error: Error::Token {
                expected: "end of expression".to_string(),
                span: tokens[cursor].span,
            },
            input: NamedSource::new("expression", input.to_string()),
        });
    }
    Ok(result)
}

fn sum(tokens: &[Located], cursor: &mut usize) -> Result<Expression, Error> {
    let mut terms = vec![product(tokens, cursor)?];

    while *cursor < tokens.len() {
        if matches!(tokens[*cursor].token, Token::Comma) {
            *cursor += 1;
            terms.push(product(tokens, cursor)?);
        } else {
            break;
        }
    }

    Ok(match terms.len() {
        1 => terms.into_iter().next().unwrap(),
        _ => Expression::Or(terms),
    })
}

fn product(tokens: &[Located], cursor: &mut usize) -> Result<Expression, Error> {
    let mut factors = vec![unary(tokens, cursor)?];

    while *cursor < tokens.len() {
        if matches!(tokens[*cursor].token, Token::Dot) {
            *cursor += 1;
            factors.push(unary(tokens, cursor)?);
        } else {
            break;
        }
    }

    Ok(match factors.len() {
        1 => factors.into_iter().next().unwrap(),
        _ => Expression::And(factors),
    })
}

fn unary(tokens: &[Located], cursor: &mut usize) -> Result<Expression, Error> {
    if *cursor >= tokens.len() {
        let span = tokens
            .last()
            .map_or(SourceSpan::new(0.into(), 0), |t| t.span);
        return Err(Error::End {
            expected: "identifier or !".to_string(),
            span,
        });
    }

    if matches!(tokens[*cursor].token, Token::Not) {
        *cursor += 1;
        let operand = unary(tokens, cursor)?;
        return Ok(Expression::Not(Box::new(operand)));
    }

    atom(tokens, cursor)
}

fn atom(tokens: &[Located], cursor: &mut usize) -> Result<Expression, Error> {
    if *cursor >= tokens.len() {
        let span = tokens
            .last()
            .map_or(SourceSpan::new(0.into(), 0), |t| t.span);
        return Err(Error::End {
            expected: "identifier or (".to_string(),
            span,
        });
    }

    match &tokens[*cursor].token {
        Token::Name(name) => {
            let expression = Expression::Name(name.clone());
            *cursor += 1;
            Ok(expression)
        }
        Token::Open => {
            *cursor += 1;
            let expression = sum(tokens, cursor)?;
            if *cursor >= tokens.len() {
                let span = tokens
                    .last()
                    .map_or(SourceSpan::new(0.into(), 0), |t| t.span);
                return Err(Error::End {
                    expected: ")".to_string(),
                    span,
                });
            }
            if !matches!(tokens[*cursor].token, Token::Close) {
                return Err(Error::Token {
                    expected: ")".to_string(),
                    span: tokens[*cursor].span,
                });
            }
            *cursor += 1;
            Ok(expression)
        }
        _ => Err(Error::Token {
            expected: "identifier or (".to_string(),
            span: tokens[*cursor].span,
        }),
    }
}
