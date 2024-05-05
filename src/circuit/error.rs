use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::circuit::tokenizer::{Location, Token};

#[derive(Debug)]
pub enum ParserError {
    Token { expected: &'static str, actual: Token },
    Syntax { message: String, location: Location },
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Token {
                expected,
                actual: Token { location, value }
            } =>
                write!(f, "TokenError in line {}, column {}: Expected {} but got {:?}",
                       location.line,
                       location.column,
                       expected,
                       value),
            ParserError::Syntax { message, location } =>
                write!(f, "Syntax Error in line {}: {}", message, location.column),
        }
    }
}
