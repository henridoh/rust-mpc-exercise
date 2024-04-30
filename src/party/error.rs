use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError};
use crate::circuit::Gate;

#[derive(Debug)]
pub enum GMWError<'a> {
    InvalidGate(Gate),
    ProtocolError,
    NetworkError(Box<dyn Error + 'a>),
    InputLengthMismatch { actual: usize, expected: usize },
}

impl<'a> Error for GMWError<'a> {}

impl<'a> Display for GMWError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GMWError::InvalidGate(gate) =>
                write!(f, "Expected AND, INV, XOR gate, but got {:?}.", gate),
            GMWError::NetworkError(err) =>
                write!(f, "Network Error {}", *err),
            GMWError::InputLengthMismatch { actual, expected } =>
                write!(f, "Input provided of length {actual}, but expected length {expected}"),
            GMWError::ProtocolError =>
                write!(f, "ProtocolError!"),
        }
    }
}

impl<'a, T: 'a> From<SendError<T>> for GMWError<'a> {
    fn from(value: SendError<T>) -> Self {
        Self::NetworkError(Box::new(value))
    }
}

impl<'a> From<RecvError> for GMWError<'a> {
    fn from(value: RecvError) -> Self {
        Self::NetworkError(Box::new(value))
    }
}