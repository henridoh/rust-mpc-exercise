use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::circuit::GateOperation;

use crate::network::NetworkError;

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


