mod error;
mod in_memory;

pub use error::NetworkError;
pub use in_memory::MemChannelConnection;

#[derive(Debug)]
pub enum GMWPacket {
    ParameterShares(Vec<bool>),
    And { d: bool, e: bool },
    Result(Vec<bool>),
}


pub trait GMWConnection {
    fn exchange(&self, message: GMWPacket) -> Result<GMWPacket, NetworkError> {
        self.send(message)?;
        self.recv()
    }
    fn send(&self, message: GMWPacket) -> Result<(), NetworkError>;
    fn recv(&self) -> Result<GMWPacket, NetworkError>;
}

pub trait MTPConnection {
    type PacketType;
    
    fn exchange_mtp(&self, message: Self::PacketType) -> Result<GMWPacket, NetworkError> {
        self.send_mtp(message)?;
        self.recv_mtp()
    }
    
    fn send_mtp(&self, message: Self::PacketType) -> Result<(), NetworkError>;
    fn recv_mtp(&self) -> Result<GMWPacket, NetworkError>;
}
