//! Indexer module for building FTS5 search indexes

use crate::parser::SessionRecord;
use crate::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// FTS5 Indexer for session records
#[derive(Debug, Clone)]
pub struct Indexer {
    /// Path to the SQLite database
    db_path: PathBuf,
}

impl Indexer {
    /// Create a new Indexer
    pub fn new() -> Result<Self> {
        let data_dir = crate::storage::Storage::new()?.data_dir().clone();
        let db_path = data_dir.join("index/sessions.db");

        // Create index directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(Self { db_path })
    }

    /// Create an Indexer with a custom database path (for testing)
    #[cfg(test)]
    pub fn with_db_path(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    /// Build or rebuild the FTS5 index from session records
    pub fn build_index(&self, records: &[SessionRecord]) -> Result<usize> {
        // Ensure parent directory exists before opening database
        if let Some(parent) = self.db_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let conn = Connection::open(&self.db_path)?;

        // Enable WAL mode for better performance
        conn.pragma_update(None, "journal_mode", "WAL")?;

        // Create tables and FTS5 virtual table
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                display TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                project TEXT NOT NULL,
                session_id TEXT NOT NULL UNIQUE
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS sessions_fts USING fts5(
                display,
                project,
                session_id,
                content='sessions',
                content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS sessions_ai AFTER INSERT ON sessions BEGIN
                INSERT INTO sessions_fts(rowid, display, project, session_id)
                VALUES (new.rowid, new.display, new.project, new.session_id);
            END;

            CREATE TRIGGER IF NOT EXISTS sessions_ad AFTER DELETE ON sessions BEGIN
                INSERT INTO sessions_fts(sessions_fts, rowid, display, project, session_id)
                VALUES ('delete', old.rowid, old.display, old.project, old.session_id);
            END;

            CREATE TRIGGER IF NOT EXISTS sessions_au AFTER UPDATE ON sessions BEGIN
                INSERT INTO sessions_fts(sessions_fts, rowid, display, project, session_id)
                VALUES ('delete', old.rowid, old.display, old.project, old.session_id);
                INSERT INTO sessions_fts(rowid, display, project, session_id)
                VALUES (new.rowid, new.display, new.project, new.session_id);
            END;
            "#,
        )?;

        // Clear existing data and rebuild
        conn.execute("DELETE FROM sessions", [])?;
        conn.execute("DELETE FROM sessions_fts", [])?;

        // Insert records
        let mut insert_stmt = conn.prepare(
            "INSERT OR REPLACE INTO sessions (display, timestamp, project, session_id) VALUES (?1, ?2, ?3, ?4)",
        )?;

        let mut count = 0;
        for record in records {
            insert_stmt.execute(params![
                record.display,
                record.timestamp,
                record.project,
                record.session_id
            ])?;
            count += 1;
        }

        // Optimize FTS5 index
        conn.execute(
            "INSERT INTO sessions_fts(sessions_fts) VALUES('optimize')",
            [],
        )?;

        Ok(count)
    }

    /// Get the number of indexed records
    pub fn get_count(&self) -> Result<usize> {
        let conn = Connection::open(&self.db_path)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Check if index exists
    pub fn index_exists(&self) -> bool {
        self.db_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_build_index() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let indexer = Indexer {
            db_path: db_path.clone(),
        };

        let records = vec![
            SessionRecord::new(
                "/model".to_string(),
                1766567616338,
                "/Users/yym".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/search test".to_string(),
                1766567617000,
                "/Users/yym/project".to_string(),
                "def456".to_string(),
            ),
        ];

        let count = indexer.build_index(&records).unwrap();
        assert_eq!(count, 2);

        // Verify index exists
        assert!(db_path.exists());
        assert!(indexer.index_exists());

        // Verify count
        assert_eq!(indexer.get_count().unwrap(), 2);
    }

    #[test]
    fn test_indexer_new_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("index/test.db");

        let indexer = Indexer::with_db_path(db_path.clone());

        // Build index should create directory
        let records = vec![SessionRecord::new(
            "/test".to_string(),
            1766567616338,
            "/Users/yym".to_string(),
            "test-id".to_string(),
        )];
        indexer.build_index(&records).unwrap();

        // Directory should now exist
        assert!(db_path.parent().unwrap().exists());
        assert!(db_path.exists());
    }
}
