use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NetworkError{
    ConnectionError(Box<dyn Error>),
    Other(&'static str), //TODO
}

impl Error for NetworkError {}
impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::ConnectionError(err) => {
                write!(f, "NetworkError: {}", err)
            }
            &NetworkError::Other(msg) => {
                write!(f, "NetworkError: {}", msg)
            }
        }
    }
}

impl From<&'static str> for NetworkError {
    fn from(value: &'static str) -> Self {
        NetworkError::Other(value)
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(value: std::io::Error) -> Self {
        NetworkError::ConnectionError(Box::new(value))
    }
}