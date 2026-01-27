pub mod client;
pub mod types;
pub use client::*;
pub use types::*;

use std::fmt;

#[derive(Debug)]
pub enum AssettoCorsaError {
    SharedMemoryNotFound(String),
    ConnectionFailed(String),
    InvalidData(String),
}

impl fmt::Display for AssettoCorsaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssettoCorsaError::SharedMemoryNotFound(msg) => write!(f, "Shared memory not found: {}", msg),
            AssettoCorsaError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            AssettoCorsaError::InvalidData(msg) => write!(f, "Invalid data format: {}", msg),
        }
    }
}

impl std::error::Error for AssettoCorsaError {}

pub type Result<T> = std::result::Result<T, AssettoCorsaError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connected,
}

pub struct AssettoCorsa {
    client: AssettoCorsaClient,
}

impl AssettoCorsa {
    pub fn new() -> Self {
        Self {
            client: AssettoCorsaClient::new(),
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.client.connect()
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub fn status(&self) -> ConnectionStatus {
        if self.is_connected() {
            ConnectionStatus::Connected
        } else {
            ConnectionStatus::Disconnected
        }
    }

    pub fn get_physics(&self) -> Option<Physics> {
        self.client.get_physics()
    }

    pub fn get_graphics(&self) -> Option<Graphics> {
        self.client.get_graphics()
    }

    pub fn get_static_info(&self) -> Option<StaticInfo> {
        self.client.get_static_info()
    }
}

impl Default for AssettoCorsa {
    fn default() -> Self {
        Self::new()
    }
}
