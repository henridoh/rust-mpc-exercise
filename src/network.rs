mod error;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
pub use error::NetworkError;

#[derive(Debug, Serialize, Deserialize)]
pub enum GMWPacket {
    ParameterShares(Vec<bool>),
    And { d: bool, e: bool },
    Result(Vec<bool>),
}

pub struct ChannelPair<W: Write, R: Read> {
    sender: W,
    receiver: R,
}

impl<W: Write, R: Read> ChannelPair<W, R> {
    pub fn new(sender: W, receiver: R) -> Self {
        Self { sender, receiver }
    }
}

pub trait GMWConnection {
    fn exchange(&mut self, message: GMWPacket) -> Result<GMWPacket, NetworkError> {
        self.send(message)?;
        self.recv()
    }
    fn send(&mut self, message: GMWPacket) -> Result<(), NetworkError>;
    fn recv(&mut self) -> Result<GMWPacket, NetworkError>;
}


impl<W: Write, R: Read> Write for ChannelPair<W, R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sender.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.sender.flush()
    }
}

impl<W: Write, R: Read> Read for ChannelPair<W, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.receiver.read(buf)
    }
}

impl<T: Write + Read> GMWConnection for T {
    fn send(&mut self, message: GMWPacket) -> Result<(), NetworkError> {
        let bytes = bincode::serialize(&message).unwrap();
        self.write(&bytes)?;
        Ok(())
    }

    fn recv(&mut self) -> Result<GMWPacket, NetworkError> {
        let Ok(res) = bincode::deserialize_from(self) else {
            return Err(NetworkError::Other("Could not deserialize packet"))
        };
        Ok(res)
    }
}