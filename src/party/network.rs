use std::sync::mpsc::{Receiver, Sender};
use crate::party::error::NetworkError;

#[derive(Debug)]
pub enum MessageType {
    ParameterShares(Vec<bool>),
    And { d: bool, e: bool, },
    Result(Vec<bool>),
}


pub struct MemChannelConnection {
    pub sender: Sender<MessageType>,
    pub receiver: Receiver<MessageType>,
}

impl GMWConnection for MemChannelConnection {
    fn exchange(&self, message: MessageType) -> Result<MessageType, NetworkError> {
        self.sender.send(message)?;
        Ok(self.receiver.recv()?)
    }
}

pub trait GMWConnection {
    fn exchange(&self, message: MessageType) -> Result<MessageType, NetworkError>;
    // fn send(&self, message: MessageType) -> Result<(), ()>;
    // fn recv(&self) -> Result<MessageType, ()>;
}