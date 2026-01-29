//! claude-memo: Claude Code session record management tool
//!
//! # Modules
//!
//! - `parser`: Parse history.jsonl files
//! - `indexer`: Build search indexes
//! - `storage`: Manage ~/.claude-memo/ data
//! - `search`: Full-text search functionality
//! - `exporter`: HTML export and screenshot
//! - `cli`: Command-line interface

pub mod parser;
pub mod indexer;
pub mod storage;
pub mod search;
pub mod exporter;
pub mod cli;
