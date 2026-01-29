/// Error types for claude-memo
use thiserror::Error;

/// Main error enum for claude-memo operations
#[derive(Debug, Error)]
pub enum Error {
    /// IO error for file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// TOML parsing error
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSerialize(toml::ser::Error),

    /// SQLite error
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// File not found error (exit code 3)
    #[error("File not found: {0}")]
    NotFound(String),

    /// Invalid session ID format
    #[error("Invalid session ID: {0}")]
    InvalidSessionId(String),

    /// Session not found in favorites
    #[error("Session not found in favorites: {0}")]
    SessionNotFound(String),

    /// Invalid timestamp
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(i64),

    /// Home directory not found
    #[error("Home directory not found")]
    HomeDirNotFound,
}
