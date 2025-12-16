use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniShellError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Bincode error: {0}")]
    Bincode(String),

    #[error("Dialoguer error: {0}")]
    Dialoguer(String),

    #[error("Hex decoding error: {0}")]
    Hex(String),

    #[error("Cryptography error: {0}")]
    Crypto(String),

    #[allow(dead_code)]
    #[error("Network error: {0}")]
    Network(String),

    #[error("Contact not found: {0}")]
    ContactNotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Not initialized. Run 'omnishell init' first")]
    NotInitialized,

    #[error("Already initialized. Use --force to re-initialize")]
    AlreadyInitialized,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("{0}")]
    Other(String),
}

// Implement From for bincode errors
impl From<Box<bincode::ErrorKind>> for OmniShellError {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        OmniShellError::Bincode(err.to_string())
    }
}

// Implement From for dialoguer errors
impl From<dialoguer::Error> for OmniShellError {
    fn from(err: dialoguer::Error) -> Self {
        OmniShellError::Dialoguer(err.to_string())
    }
}

// Implement From for hex errors
impl From<hex::FromHexError> for OmniShellError {
    fn from(err: hex::FromHexError) -> Self {
        OmniShellError::Hex(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, OmniShellError>;
