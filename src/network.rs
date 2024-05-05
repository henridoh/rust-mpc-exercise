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

#[derive(Debug)]
pub enum NetworkPacket<T: 'static> {
    GMW(GMWPacket),
    MTP(T),
}

pub trait GMWConnection {
    type OtherPacket: 'static;
    fn exchange(&self, message: NetworkPacket<Self::OtherPacket>)
                -> Result<NetworkPacket<Self::OtherPacket>, NetworkError> {
        self.send(message)?;
        self.recv()
    }
    fn send(&self, message: NetworkPacket<Self::OtherPacket>) -> Result<(), NetworkError>;
    fn recv(&self) -> Result<NetworkPacket<Self::OtherPacket>, NetworkError>;
}

