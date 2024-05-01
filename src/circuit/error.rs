use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParserError {
    TokenError(Box<dyn Error>),
    SyntaxError(String),
    EndOfLineError,
    EndOfFileError,
}

trait ParserErrorTrait {}
impl ParserErrorTrait for ParserError {}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO schöner machen
            ParserError::TokenError(err) =>
                write!(f, "ParserError: {}", *err),
            ParserError::EndOfLineError =>
                write!(f, "EOL while parsing."),
            ParserError::EndOfFileError =>
                write!(f, "EOF while parsing."),
            ParserError::SyntaxError(s) => 
                write!(f, "Syntax Error: {s}"),
        }
    }
}

impl From<ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        ParserError::TokenError(Box::new(value))
    }
}
