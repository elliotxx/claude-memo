//! Parser module for processing history.jsonl files

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::io::BufRead;

/// Represents a single Claude Code session record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    /// Command/message content
    pub display: String,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
    /// Project path
    pub project: String,
    /// Unique session identifier (UUID)
    pub session_id: String,
}

impl SessionRecord {
    /// Create a new SessionRecord
    #[inline]
    pub fn new(display: String, timestamp: i64, project: String, session_id: String) -> Self {
        Self {
            display,
            timestamp,
            project,
            session_id,
        }
    }

    /// Validate the session record
    /// Returns error if validation fails
    pub fn validate(&self) -> Result<(), crate::error::Error> {
        // Validate session_id is a valid UUID format (basic check)
        if self.session_id.is_empty() {
            return Err(crate::error::Error::InvalidSessionId(
                "session_id cannot be empty".to_string(),
            ));
        }

        // Validate timestamp is positive
        if self.timestamp <= 0 {
            return Err(crate::error::Error::InvalidTimestamp(self.timestamp));
        }

        Ok(())
    }
}

impl std::fmt::Display for SessionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime: DateTime<Utc> = Utc
            .timestamp_millis_opt(self.timestamp)
            .single()
            .unwrap_or(Utc::now());
        write!(
            f,
            "{} {} > {}  [{}]",
            datetime.format("%Y-%m-%d %H:%M"),
            self.project,
            self.display,
            self.session_id
        )
    }
}

/// Raw JSON structure from history.jsonl
#[derive(Debug, Deserialize)]
struct RawSessionRecord {
    display: String,
    #[serde(default)]
    _pasted_contents: serde_json::Value,
    timestamp: i64,
    project: String,
    #[serde(alias = "sessionId")]
    session_id: String,
}

impl TryFrom<RawSessionRecord> for SessionRecord {
    type Error = crate::error::Error;

    fn try_from(raw: RawSessionRecord) -> Result<Self, Self::Error> {
        let record = SessionRecord::new(raw.display, raw.timestamp, raw.project, raw.session_id);
        record.validate()?;
        Ok(record)
    }
}

/// Parse a single JSONL line into a SessionRecord
pub fn parse_line(line: &str) -> Result<Option<SessionRecord>, crate::error::Error> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(None);
    }

    let raw: RawSessionRecord = serde_json::from_str(line)?;
    let record = SessionRecord::try_from(raw)?;
    Ok(Some(record))
}

/// Parse history.jsonl file and return all session records
pub fn parse_history_file(
    path: &std::path::Path,
) -> Result<Vec<SessionRecord>, crate::error::Error> {
    let file = std::fs::File::open(path)
        .map_err(|_| crate::error::Error::NotFound(path.to_string_lossy().to_string()))?;
    let reader = std::io::BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Skip invalid lines instead of failing (graceful handling)
        match parse_line(&line) {
            Ok(Some(record)) => records.push(record),
            Ok(None) => continue, // Empty/whitespace line
            Err(_) => continue,   // Invalid line, skip it
        }
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_line() {
        let json = r#"{"display":"/model ","pastedContents":{},"timestamp":1766567616338,"project":"/Users/elliotxx","sessionId":"d55aaa1c-b149-4aa4-9809-7eab1dba8d4c"}"#;
        let result = parse_line(json);
        assert!(result.is_ok());
        let record = result.unwrap().unwrap();
        assert_eq!(record.display, "/model ");
        assert_eq!(record.timestamp, 1766567616338);
        assert_eq!(record.project, "/Users/elliotxx");
        assert_eq!(record.session_id, "d55aaa1c-b149-4aa4-9809-7eab1dba8d4c");
    }

    #[test]
    fn test_parse_empty_line() {
        let result = parse_line("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_whitespace_line() {
        let result = parse_line("   ");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_malformed_json() {
        let result = parse_line("{invalid json}");
        assert!(result.is_err());
    }

    #[test]
    fn test_session_record_display() {
        let record = SessionRecord::new(
            "/model ".to_string(),
            1766567616338,
            "/Users/elliotxx".to_string(),
            "d55aaa1c-b149-4aa4-9809-7eab1dba8d4c".to_string(),
        );
        let display = record.to_string();
        assert!(display.contains("/model"));
        assert!(display.contains("/Users/elliotxx"));
    }

    // === Edge Case Tests ===

    #[test]
    fn test_parse_empty_session_id_fails() {
        // session_id is empty string, should fail validation
        let json = r#"{"display":"/model","timestamp":1766567616338,"project":"/Users/elliotxx","sessionId":""}"#;
        let result = parse_line(json);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("session_id cannot be empty"));
    }

    #[test]
    fn test_parse_zero_timestamp_fails() {
        // timestamp is zero, should fail validation
        let json = r#"{"display":"/model","timestamp":0,"project":"/Users/elliotxx","sessionId":"abc123-def456"}"#;
        let result = parse_line(json);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp"));
    }

    #[test]
    fn test_parse_negative_timestamp_fails() {
        // timestamp is negative, should fail validation
        let json = r#"{"display":"/model","timestamp":-1000,"project":"/Users/elliotxx","sessionId":"abc123-def456"}"#;
        let result = parse_line(json);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp"));
    }

    #[test]
    fn test_parse_multiple_lines() {
        // Test parsing multiple lines in one string
        let jsonl = r#"{"display":"/model","timestamp":1766567616338,"project":"/Users/elliotxx","sessionId":"id-001"}
{"display":"/search","timestamp":1766567617000,"project":"/Users/elliotxx/project","sessionId":"id-002"}
{"display":"/test","timestamp":1766567618000,"project":"/Users/elliotxx/other","sessionId":"id-003"}"#;

        let lines: Vec<&str> = jsonl.lines().collect();
        assert_eq!(lines.len(), 3);

        for line in lines {
            let result = parse_line(line);
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }
    }

    #[test]
    fn test_parse_with_special_characters_in_display() {
        // Display with special characters should be preserved
        let json = r#"{"display":"/model --option=value","timestamp":1766567616338,"project":"/Users/elliotxx/project","sessionId":"abc123-def456"}"#;
        let result = parse_line(json);
        assert!(result.is_ok());
        let record = result.unwrap().unwrap();
        assert!(record.display.contains("--option=value"));
    }

    #[test]
    fn test_parse_with_nested_project_path() {
        // Deeply nested project path
        let json = r#"{"display":"/test","timestamp":1766567616338,"project":"/Users/elliotxx/workspace/my-project/src/components","sessionId":"abc123-def456"}"#;
        let result = parse_line(json);
        assert!(result.is_ok());
        let record = result.unwrap().unwrap();
        assert!(record.project.contains("workspace/my-project"));
    }

    #[test]
    fn test_session_record_display_includes_session_id() {
        // Verify Display format includes session_id for favorite workflow
        let record = SessionRecord::new(
            "/model".to_string(),
            1766567616338,
            "/Users/elliotxx".to_string(),
            "abc123-def456-789".to_string(),
        );
        let display = record.to_string();
        assert!(display.contains("abc123-def456-789")); // session_id should be visible
        assert!(display.contains("[")); // session_id wrapped in brackets
        assert!(display.contains("]"));
    }
}
