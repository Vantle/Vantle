use std::sync::Arc;

use channel::Channel;
use miette::{Diagnostic, NamedSource, SourceSpan};
use stream::Predicate;
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
    pub fn evaluate(&self, channels: &[Channel]) -> bool {
        match self {
            Self::Any => true,
            Self::Name(name) => channels.iter().any(|c| c.name == *name),
            Self::Not(inner) => !inner.evaluate(channels),
            Self::And(terms) => terms.iter().all(|t| t.evaluate(channels)),
            Self::Or(terms) => terms.iter().any(|t| t.evaluate(channels)),
        }
    }

    #[must_use]
    pub fn predicate(self) -> Predicate {
        Arc::new(move |channels| self.evaluate(channels))
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

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("unexpected token in channel expression")]
    #[diagnostic(
        code(observation::expression::token),
        help("use . (and), , (or), ! (not), () (group) — e.g. core,http.!debug")
    )]
    Token {
        expected: String,
        #[label("expected {expected}")]
        span: SourceSpan,
    },

    #[error("unexpected end of channel expression")]
    #[diagnostic(
        code(observation::expression::end),
        help("expression appears incomplete — add the missing operand")
    )]
    End {
        expected: String,
        #[label("expected {expected} after this")]
        span: SourceSpan,
    },
}

#[derive(Error, Debug, Diagnostic)]
#[error("invalid channel filter expression")]
#[diagnostic(code(observation::expression::parse))]
pub struct Sourced {
    #[diagnostic_source]
    pub error: Error,
    #[source_code]
    pub input: NamedSource<String>,
}
