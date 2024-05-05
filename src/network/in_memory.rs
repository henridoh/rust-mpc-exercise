use std::sync::mpsc::{Receiver, RecvError, Sender, SendError};
use crate::network::{GMWConnection, NetworkError, NetworkPacket};

pub struct MemChannelConnection<T: 'static> {
    pub sender: Sender<NetworkPacket<T>>,
    pub receiver: Receiver<NetworkPacket<T>>,
}

impl<T: 'static> GMWConnection for MemChannelConnection<T> {
    type OtherPacket = T;

    fn send(&self, message: NetworkPacket<T>) -> Result<(), NetworkError> {
        self.sender.send(message)?;
        Ok(())
    }

    fn recv(&self) -> Result<NetworkPacket<T>, NetworkError> {
        Ok(self.receiver.recv()?)
    }
}

impl<T> From<SendError<NetworkPacket<T>>> for NetworkError {
    fn from(value: SendError<NetworkPacket<T>>) -> Self {
        NetworkError(Box::new(value))
    }
}

impl From<RecvError> for NetworkError {
    fn from(value: RecvError) -> Self{
        NetworkError(Box::new(value))
    }
}

