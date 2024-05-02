use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParserError {
    Token(Box<dyn Error>),
    Syntax(String),
    EndOfLine,
    EndOfFile,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO schöner machen
            ParserError::Token(err) =>
                write!(f, "ParserError: {}", *err),
            ParserError::EndOfLine =>
                write!(f, "EOL while parsing."),
            ParserError::EndOfFile =>
                write!(f, "EOF while parsing."),
            ParserError::Syntax(s) =>
                write!(f, "Syntax Error: {s}"),
        }
    }
}

impl From<ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        ParserError::Token(Box::new(value))
    }
}
