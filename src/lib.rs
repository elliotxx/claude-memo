//! claude-memo: Claude Code session record management tool
//!
//! # Modules
//!
//! - `config`: User configuration management
//! - `parser`: Parse history.jsonl files
//! - `indexer`: Build search indexes
//! - `storage`: Manage ~/.claude-memo/ data
//! - `search`: Full-text search functionality
//! - `exporter`: HTML export and screenshot
//! - `cli`: Command-line interface
//! - `error`: Error types

pub mod cli;
pub mod config;
pub mod error;
pub mod exporter;
pub mod indexer;
pub mod parser;
pub mod search;
pub mod storage;

/// Result type alias using anyhow::Error
pub type Result<T> = std::result::Result<T, anyhow::Error>;
