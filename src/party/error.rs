use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError};
use crate::circuit::GateOperation;

#[derive(Debug)]
pub enum GMWError {
    InvalidGate(GateOperation),
    ProtocolError,
    InputLengthMismatch { actual: usize, expected: usize },
    NetworkError(NetworkError),
}

impl Error for GMWError {}

impl Display for GMWError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GMWError::InvalidGate(gate) =>
                write!(f, "Expected AND, INV, XOR gate, but got {:?}.", gate),
            GMWError::NetworkError(err) =>
                write!(f, "{}", *err),
            GMWError::InputLengthMismatch { actual, expected } =>
                write!(f, "Input provided of length {actual}, but expected length {expected}"),
            GMWError::ProtocolError =>
                write!(f, "ProtocolError!"),
        }
    }
}

impl From<NetworkError> for GMWError {
    fn from(value: NetworkError) -> Self {
        Self::NetworkError(value)
    }
}


#[derive(Debug)]
pub struct NetworkError(Box<dyn Error>);

impl Error for NetworkError {}
impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetworkError: {}", self.0)
    }
}

impl<T: 'static> From<SendError<T>> for NetworkError {
    fn from(value: SendError<T>) -> Self { 
        Self(Box::new(value)) 
    }
}

impl From<RecvError> for NetworkError {
    fn from(value: RecvError) -> Self {
        Self(Box::new(value))
    }
}
