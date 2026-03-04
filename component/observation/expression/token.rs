use miette::SourceSpan;

use expression::Error;

#[derive(Debug)]
pub enum Token {
    Name(String),
    Dot,
    Comma,
    Not,
    Open,
    Close,
}

#[derive(Debug)]
pub struct Located {
    pub token: Token,
    pub span: SourceSpan,
}

pub fn tokenize(input: &str) -> Result<Vec<Located>, Error> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(position, c)) = chars.peek() {
        match c {
            '.' => {
                tokens.push(Located {
                    token: Token::Dot,
                    span: SourceSpan::new(position.into(), 1),
                });
                chars.next();
            }
            ',' => {
                tokens.push(Located {
                    token: Token::Comma,
                    span: SourceSpan::new(position.into(), 1),
                });
                chars.next();
            }
            '!' => {
                tokens.push(Located {
                    token: Token::Not,
                    span: SourceSpan::new(position.into(), 1),
                });
                chars.next();
            }
            '(' => {
                tokens.push(Located {
                    token: Token::Open,
                    span: SourceSpan::new(position.into(), 1),
                });
                chars.next();
            }
            ')' => {
                tokens.push(Located {
                    token: Token::Close,
                    span: SourceSpan::new(position.into(), 1),
                });
                chars.next();
            }
            c if c.is_ascii_alphabetic() => {
                let start = position;
                let mut name = String::new();
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_ascii_alphanumeric() {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let len = name.len();
                tokens.push(Located {
                    token: Token::Name(name),
                    span: SourceSpan::new(start.into(), len),
                });
            }
            ' ' => {
                chars.next();
            }
            _ => {
                return Err(Error::Token {
                    expected: "identifier, operator, or parenthesis".to_string(),
                    span: SourceSpan::new(position.into(), 1),
                });
            }
        }
    }

    Ok(tokens)
}
