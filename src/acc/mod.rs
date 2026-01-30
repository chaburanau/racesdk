pub mod client;
pub mod types;
pub use client::*;
pub use types::*;

use std::fmt;

#[derive(Debug)]
pub enum AssettoCorsaCompetizioneError {
    SharedMemoryNotFound(String),
    ConnectionFailed(String),
    InvalidData(String),
}

impl fmt::Display for AssettoCorsaCompetizioneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssettoCorsaCompetizioneError::SharedMemoryNotFound(msg) => write!(f, "Shared memory not found: {}", msg),
            AssettoCorsaCompetizioneError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            AssettoCorsaCompetizioneError::InvalidData(msg) => write!(f, "Invalid data format: {}", msg),
        }
    }
}

impl std::error::Error for AssettoCorsaCompetizioneError {}

pub type Result<T> = std::result::Result<T, AssettoCorsaCompetizioneError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connected,
}

pub struct AssettoCorsaCompetizione {
    client: AssettoCorsaCompetizioneClient,
}

impl AssettoCorsaCompetizione {
    pub fn new() -> Self {
        Self {
            client: AssettoCorsaCompetizioneClient::new(),
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

impl Default for AssettoCorsaCompetizione {
    fn default() -> Self {
        Self::new()
    }
}
