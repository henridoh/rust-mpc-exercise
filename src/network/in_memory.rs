use std::sync::mpsc::{Receiver, RecvError, Sender, SendError};
use crate::network::{GMWConnection, GMWPacket, NetworkError};


pub enum MemChannelPacket<T: 'static> {
    Gmw(GMWPacket),
    Mtp(T)
}

pub struct MemChannelConnection<T: 'static> {
    pub sender: Sender<MemChannelPacket<T>>,
    pub receiver: Receiver<MemChannelPacket<T>>,
}



impl<T> GMWConnection for MemChannelConnection<T> {
    fn send(&self, message: GMWPacket) -> Result<(), NetworkError> {
        self.sender.send(MemChannelPacket::Gmw(message))?;
        Ok(())
    }

    fn recv(&self) -> Result<GMWPacket, NetworkError> {
        let MemChannelPacket::Gmw(packet) = self.receiver.recv()? else {
            return Err("Protocol Error: Expected GMW packet but got MTP packet.".into());
        };
        Ok(packet)
    }
}

impl<T: 'static> From<SendError<MemChannelPacket<T>>> for NetworkError {
    fn from(value: SendError<MemChannelPacket<T>>) -> Self {
        NetworkError::ConnectionError(Box::new(value))
    }
}

impl From<RecvError> for NetworkError {
    fn from(value: RecvError) -> Self{
        NetworkError::ConnectionError(Box::new(value))
    }
}

