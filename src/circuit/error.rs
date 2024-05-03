use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::circuit::tokenizer::{Token};

#[derive(Debug)]
pub enum ParserError {
    Token { expected: &'static str, actual: Token },
    Syntax(String),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Token { expected, actual } =>
                write!(f, "TokenError in line {}, column {}: expected {} but got {:?}",
                       actual.location.line,
                       actual.location.char,
                       expected,
                       actual.value,
                ),
            ParserError::Syntax(s) =>
                write!(f, "Syntax Error: {s}"),
        }
    }
}
