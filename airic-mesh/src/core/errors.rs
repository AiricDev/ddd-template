use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Failed to (de)serialize entity: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Cryptography error from vodozemac: {0}")]
    Cryptography(#[from] vodozemac::olm::DecryptionError),
    #[error("Decode error from vodozemac: {0}")]
    Decode(#[from] vodozemac::DecodeError),
    #[error("Device with id {0} not found")]
    DeviceNotFound(String),
    #[error("Session for devices {0} and {1} not found")]
    SessionNotFound(String, String),
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, MeshError>;
