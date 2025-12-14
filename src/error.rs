use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniShellError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Cryptography error: {0}")]
    Crypto(String),

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

pub type Result<T> = std::result::Result<T, OmniShellError>;
