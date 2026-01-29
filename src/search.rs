//! Search module for full-text search functionality

use crate::parser::SessionRecord;
use crate::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// Search result containing a session record and its relevance info
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    /// The session record
    pub record: SessionRecord,
    /// BM25 relevance score (lower is more relevant)
    pub score: f64,
}

impl SearchResult {
    /// Create a new SearchResult
    pub fn new(record: SessionRecord, score: f64) -> Self {
        Self { record, score }
    }
}

/// FTS5 Search engine for session records
#[derive(Debug, Clone)]
pub struct Search {
    /// Path to the SQLite database
    db_path: PathBuf,
}

impl Search {
    /// Create a new Search instance
    pub fn new() -> Result<Self> {
        let data_dir = crate::storage::Storage::new()?.data_dir().clone();
        let db_path = data_dir.join("index/sessions.db");
        Ok(Self { db_path })
    }

    /// Create a Search with a custom database path (for testing)
    #[cfg(test)]
    pub fn with_db_path(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    /// Search for records matching the query
    ///
    /// Uses FTS5 BM25 ranking for relevance scoring
    /// FTS5 requires special query syntax: prefix matching with *
    pub fn search(&self, query: &str, limit: Option<usize>) -> Result<Vec<SearchResult>> {
        let conn = Connection::open(&self.db_path)?;

        // FTS5 query with BM25 ranking
        // Use * prefix for simple word search (matches any word starting with query)
        let fts_query = format!("{}*", query);
        let sql = r#"
            SELECT
                s.display,
                s.timestamp,
                s.project,
                s.session_id,
                bm25(sessions_fts, 0, 100, 0, 0) as score
            FROM sessions_fts
            JOIN sessions s ON s.rowid = sessions_fts.rowid
            WHERE sessions_fts MATCH ?1
            ORDER BY score ASC
            LIMIT ?2
        "#;

        let limit = limit.unwrap_or(20);

        let mut stmt = conn.prepare(sql)?;
        let results = stmt.query_map(params![fts_query, limit as i64], |row| {
            Ok(SearchResult::new(
                SessionRecord::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?),
                row.get(4)?,
            ))
        })?;

        let mut search_results = Vec::new();
        for result in results {
            search_results.push(result?);
        }

        Ok(search_results)
    }

    /// Search with project filter
    pub fn search_with_project(
        &self,
        query: &str,
        project: &str,
        limit: Option<usize>,
    ) -> Result<Vec<SearchResult>> {
        let conn = Connection::open(&self.db_path)?;

        // Use * prefix for simple word search (matches any word starting with query)
        let fts_query = format!("{}*", query);

        let sql = r#"
            SELECT
                s.display,
                s.timestamp,
                s.project,
                s.session_id,
                bm25(sessions_fts, 0, 100, 0, 0) as score
            FROM sessions_fts
            JOIN sessions s ON s.rowid = sessions_fts.rowid
            WHERE sessions_fts MATCH ?1 AND s.project LIKE ?2
            ORDER BY score ASC
            LIMIT ?3
        "#;

        let limit = limit.unwrap_or(20);
        let project_pattern = format!("%{}%", project);

        let mut stmt = conn.prepare(sql)?;
        let results = stmt.query_map(params![fts_query, project_pattern, limit as i64], |row| {
            Ok(SearchResult::new(
                SessionRecord::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?),
                row.get(4)?,
            ))
        })?;

        let mut search_results = Vec::new();
        for result in results {
            search_results.push(result?);
        }

        Ok(search_results)
    }

    /// Simple text search (fallback without FTS5)
    pub fn simple_search(&self, keyword: &str, limit: Option<usize>) -> Result<Vec<SessionRecord>> {
        let conn = Connection::open(&self.db_path)?;

        let sql = r#"
            SELECT display, timestamp, project, session_id
            FROM sessions
            WHERE display LIKE ?1 OR project LIKE ?1
            ORDER BY timestamp DESC
            LIMIT ?2
        "#;

        let limit = limit.unwrap_or(20);
        let pattern = format!("%{}%", keyword);

        let mut stmt = conn.prepare(sql)?;
        let results = stmt.query_map(params![pattern, limit as i64], |row| {
            Ok(SessionRecord::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut records = Vec::new();
        for result in results {
            records.push(result?);
        }

        Ok(records)
    }

    /// Check if search index exists
    pub fn index_exists(&self) -> bool {
        self.db_path.exists()
    }

    /// Get the number of indexed records
    pub fn get_count(&self) -> Result<usize> {
        let conn = Connection::open(&self.db_path)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indexer::Indexer;
    use tempfile::TempDir;

    fn create_test_indexer(temp_dir: &tempfile::TempDir) -> (Indexer, Search) {
        let db_path = temp_dir.path().join("test.db");
        let indexer = Indexer::with_db_path(db_path.clone());
        let search = Search::with_db_path(db_path);
        (indexer, search)
    }

    #[test]
    fn test_search_basic() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![
            SessionRecord::new(
                "/model".to_string(),
                1766567616338,
                "/Users/yym".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/search test query".to_string(),
                1766567617000,
                "/Users/yym/project".to_string(),
                "def456".to_string(),
            ),
            SessionRecord::new(
                "/another search command".to_string(),
                1766567618000,
                "/Users/yym/other".to_string(),
                "ghi789".to_string(),
            ),
        ];

        indexer.build_index(&records).unwrap();

        // Search for "search"
        let results = search.search("search", Some(10)).unwrap();
        assert_eq!(results.len(), 2);

        // Results should be ordered by relevance (BM25 score)
        for result in &results {
            assert!(result.record.display.to_lowercase().contains("search"));
        }
    }

    #[test]
    fn test_search_limit() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = (0..20)
            .map(|i| {
                SessionRecord::new(
                    format!("/command {}", i),
                    1766567616000 + i as i64,
                    "/Users/yym".to_string(),
                    format!("id-{}", i),
                )
            })
            .collect::<Vec<_>>();

        indexer.build_index(&records).unwrap();

        // Search with limit
        let results = search.search("command", Some(5)).unwrap();
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_search_no_results() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![SessionRecord::new(
            "/test".to_string(),
            1766567616338,
            "/Users/yym".to_string(),
            "abc123".to_string(),
        )];

        indexer.build_index(&records).unwrap();

        // Search for something that doesn't exist
        let results = search.search("nonexistent", Some(10)).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_with_project_filter() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![
            SessionRecord::new(
                "/test".to_string(),
                1766567616338,
                "/Users/yym/project1".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/test".to_string(),
                1766567617000,
                "/Users/yym/project2".to_string(),
                "def456".to_string(),
            ),
        ];

        indexer.build_index(&records).unwrap();

        // Search with project filter
        let results = search
            .search_with_project("test", "project1", Some(10))
            .unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].record.project.contains("project1"));
    }

    #[test]
    fn test_simple_search() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![
            SessionRecord::new(
                "/model".to_string(),
                1766567616338,
                "/Users/yym".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/search".to_string(),
                1766567617000,
                "/Users/yym/project".to_string(),
                "def456".to_string(),
            ),
        ];

        indexer.build_index(&records).unwrap();

        let results = search.simple_search("model", Some(10)).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].display.contains("model"));
    }

    #[test]
    fn test_search_index_exists() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        // Index doesn't exist yet
        assert!(!search.index_exists());

        // Build index
        let records = vec![SessionRecord::new(
            "/test".to_string(),
            1766567616338,
            "/Users/yym".to_string(),
            "test-id".to_string(),
        )];
        indexer.build_index(&records).unwrap();

        // Index should now exist
        assert!(search.index_exists());
        assert!(search.get_count().unwrap() > 0);
    }
}
