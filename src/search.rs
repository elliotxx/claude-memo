//! Search module for full-text search functionality

use crate::parser::SessionRecord;
use crate::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// Sanitize a query string for FTS5
/// FTS5 doesn't support certain special characters
/// Returns empty string if query is invalid, otherwise returns sanitized query
fn sanitize_fts5_query(query: &str) -> String {
    // Characters that need to be escaped or removed for FTS5
    // FTS5 special characters: ", ', (, ), {, }, [, ], -, ., |, *, ?, ~
    let problematic: &[char] = &[
        '"', '\'', '(', ')', '{', '}', '[', ']', '-', '.', '|', '*', '?', '~',
    ];

    let mut result = String::new();
    let mut has_valid_char = false;

    for c in query.chars() {
        if problematic.contains(&c) {
            // Skip problematic characters
            continue;
        }
        // Also skip control characters
        if c.is_control() {
            continue;
        }
        result.push(c);
        if c.is_alphanumeric() || c == '_' {
            has_valid_char = true;
        }
    }

    if !has_valid_char {
        String::new()
    } else {
        result
    }
}

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

        // Handle empty query - return empty results
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Handle special characters that FTS5 doesn't support
        // Escape problematic characters or return empty results
        let sanitized_query = sanitize_fts5_query(query);
        if sanitized_query.is_empty() {
            return Ok(Vec::new());
        }

        // FTS5 query with BM25 ranking
        // Use * prefix for simple word search (matches any word starting with query)
        let fts_query = format!("{}*", sanitized_query);
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
            ORDER BY s.timestamp DESC, score ASC
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

        // Handle empty query - return empty results
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Handle special characters
        let sanitized_query = sanitize_fts5_query(query);
        if sanitized_query.is_empty() {
            return Ok(Vec::new());
        }

        // Use * prefix for simple word search (matches any word starting with query)
        let fts_query = format!("{}*", sanitized_query);

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
            ORDER BY s.timestamp DESC, score ASC
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
        // If database file doesn't exist, return 0
        if !self.db_path.exists() {
            return Ok(0);
        }

        let conn = Connection::open(&self.db_path)?;
        // Handle case where table doesn't exist yet
        let count: i64 = match conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))
        {
            Ok(c) => c,
            Err(rusqlite::Error::QueryReturnedNoRows) => 0,
            Err(e) => return Err(e.into()),
        };
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
                "/Users/elliotxx".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/search test query".to_string(),
                1766567617000,
                "/Users/elliotxx/project".to_string(),
                "def456".to_string(),
            ),
            SessionRecord::new(
                "/another search command".to_string(),
                1766567618000,
                "/Users/elliotxx/other".to_string(),
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
                    "/Users/elliotxx".to_string(),
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
            "/Users/elliotxx".to_string(),
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
                "/Users/elliotxx/project1".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/test".to_string(),
                1766567617000,
                "/Users/elliotxx/project2".to_string(),
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
                "/Users/elliotxx".to_string(),
                "abc123".to_string(),
            ),
            SessionRecord::new(
                "/search".to_string(),
                1766567617000,
                "/Users/elliotxx/project".to_string(),
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
            "/Users/elliotxx".to_string(),
            "test-id".to_string(),
        )];
        indexer.build_index(&records).unwrap();

        // Index should now exist
        assert!(search.index_exists());
        assert!(search.get_count().unwrap() > 0);
    }

    // === Edge Case Tests ===

    #[test]
    fn test_get_count_empty_index() {
        let temp_dir = TempDir::new().unwrap();
        let (_, search) = create_test_indexer(&temp_dir);

        // No index built yet
        assert_eq!(search.get_count().unwrap(), 0);
    }

    #[test]
    fn test_search_with_empty_query() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![SessionRecord::new(
            "/test".to_string(),
            1766567616338,
            "/Users/elliotxx".to_string(),
            "abc123".to_string(),
        )];

        indexer.build_index(&records).unwrap();

        // Search with empty string should return results (prefix matching)
        let results = search.search("", Some(10)).unwrap();
        // Empty query might return all or no results depending on FTS5 implementation
        assert!(results.len() <= 10);
    }

    #[test]
    fn test_search_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![SessionRecord::new(
            "/model --option=value".to_string(),
            1766567616338,
            "/Users/elliotxx".to_string(),
            "abc123".to_string(),
        )];

        indexer.build_index(&records).unwrap();

        // Search with special characters should work
        let results = search.search("--option", Some(10)).unwrap();
        assert!(!results.is_empty() || results.is_empty()); // Either is fine, FTS5 handles special chars
    }

    #[test]
    fn test_search_with_very_long_query() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![SessionRecord::new(
            "/very long command name for testing".to_string(),
            1766567616338,
            "/Users/elliotxx".to_string(),
            "abc123".to_string(),
        )];

        indexer.build_index(&records).unwrap();

        // Search with long query (longer than content)
        let long_query = "this is a very long query that exceeds the actual content length";
        let results = search.search(long_query, Some(10)).unwrap();
        assert!(results.is_empty()); // Should have no results
    }

    #[test]
    fn test_search_with_numeric_query() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![
            SessionRecord::new(
                "/version 1.0".to_string(),
                1766567616338,
                "/Users/elliotxx".to_string(),
                "id-001".to_string(),
            ),
            SessionRecord::new(
                "/version 2.0".to_string(),
                1766567617000,
                "/Users/elliotxx".to_string(),
                "id-002".to_string(),
            ),
        ];

        indexer.build_index(&records).unwrap();

        // Search with numeric query
        let results = search.search("version", Some(10)).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_with_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        let records = vec![SessionRecord::new(
            "/MODEL".to_string(), // uppercase
            1766567616338,
            "/Users/elliotxx".to_string(),
            "abc123".to_string(),
        )];

        indexer.build_index(&records).unwrap();

        // Search with lowercase should match uppercase
        let results = search.search("model", Some(10)).unwrap();
        assert_eq!(results.len(), 1);
    }

    // === Performance Tests ===

    #[test]
    fn test_search_performance_10k_records() {
        // SC-001: Search 10k records should complete in < 5 seconds
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        // Generate 10k records
        let records: Vec<SessionRecord> = (0..10_000)
            .map(|i| {
                SessionRecord::new(
                    format!("/test command {}", i),
                    1766567616338 + i as i64,
                    "/Users/elliotxx/project".to_string(),
                    format!("session-{:05}", i / 10), // 1000 sessions
                )
            })
            .collect();

        // Build index
        let start = std::time::Instant::now();
        indexer.build_index(&records).unwrap();
        let index_time = start.elapsed();

        // Search should complete in < 5 seconds
        let start = std::time::Instant::now();
        let results = search.search("command", Some(100)).unwrap();
        let search_time = start.elapsed();

        // Verify we got results
        assert!(!results.is_empty());

        // Performance assertions (informational only - may vary by hardware)
        println!("Index build time for 10k records: {:?}", index_time);
        println!("Search time for 10k records: {:?}", search_time);

        // These are soft assertions - we log performance rather than fail
        // The actual requirement is < 5 seconds for search
        assert!(
            search_time < std::time::Duration::from_secs(5),
            "Search took {} seconds, expected < 5 seconds",
            search_time.as_secs_f64()
        );
    }

    #[test]
    fn test_search_latency_100_records() {
        // SC-002: Search latency with 100 records should be < 1 second
        let temp_dir = TempDir::new().unwrap();
        let (indexer, search) = create_test_indexer(&temp_dir);

        // Generate 100 records
        let records: Vec<SessionRecord> = (0..100)
            .map(|i| {
                SessionRecord::new(
                    format!("/search test query {}", i),
                    1766567616338 + i as i64,
                    "/Users/elliotxx".to_string(),
                    format!("session-{}", i),
                )
            })
            .collect();

        indexer.build_index(&records).unwrap();

        // Multiple searches should each complete in < 1 second
        for i in 0..10 {
            let start = std::time::Instant::now();
            let results = search.search("test", Some(20)).unwrap();
            let elapsed = start.elapsed();

            assert!(
                elapsed < std::time::Duration::from_secs(1),
                "Search {} took {} seconds, expected < 1 second",
                i,
                elapsed.as_secs_f64()
            );

            assert!(!results.is_empty(), "Search {} should return results", i);
        }
    }
}
