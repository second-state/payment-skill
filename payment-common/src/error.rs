use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Wallet already exists at {0}")]
    WalletExists(String),

    #[error("Wallet not found at {0}")]
    WalletNotFound(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Missing required configuration: {0}")]
    MissingConfig(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl Error {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::MissingConfig(_) => 10,
            Error::Config(_) => 11,
            Error::WalletNotFound(_) => 12,
            Error::InvalidArgument(_) => 20,
            _ => 1,
        }
    }
}
