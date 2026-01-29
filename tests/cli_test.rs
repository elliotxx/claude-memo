//! Integration tests for CLI commands

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn create_test_history_file(temp_dir: &TempDir) -> std::path::PathBuf {
    let file_path = temp_dir.path().join("history.jsonl");
    let content = r#"{"display":"/model ","pastedContents":{},"timestamp":1766567616338,"project":"/Users/yym","sessionId":"d55aaa1c-b149-4aa4-9809-7eab1dba8d4c"}
{"display":"/search test query","pastedContents":{},"timestamp":1766567617000,"project":"/Users/yym/project","sessionId":"abc123-def456-789"}
{"display":"/another command","pastedContents":{},"timestamp":1766567618000,"project":"/Users/yym/other","sessionId":"xyz789-abc123-def"}
"#;
    fs::write(&file_path, content).unwrap();
    file_path
}

#[test]
fn test_parse_command() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("parse")
        .arg("--limit")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("/search test query"));
}

#[test]
fn test_parse_command_json() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("parse")
        .arg("--json")
        .arg("--limit")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"display\""));
}

#[test]
fn test_parse_nonexistent_file() {
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", "/nonexistent/path/history.jsonl")
        .arg("parse")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_search_command() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("search")
        .assert()
        .success()
        .stdout(predicate::str::contains("/search test query"));
}

#[test]
fn test_search_command_json() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("model")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"session_id\""));
}

#[test]
fn test_search_no_results() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("nonexistentkeyword123")
        .assert()
        .success()
        .stdout(predicate::str::contains("No results found"));
}

#[test]
fn test_search_limit() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("model")
        .arg("--limit")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("/model").count(1));
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Claude Code 会话记录管理工具"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("claude-memo"));
}

#[test]
fn test_favorite_add() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // Use a custom data directory
    let data_dir = temp_dir.path().join(".claude-memo");

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("test-session-123")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Added test-session-123 to favorites",
        ));
}

#[test]
fn test_favorites_list() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // First add a favorite
    let data_dir = temp_dir.path().join(".claude-memo");

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("test-session-456")
        .assert()
        .success();

    // Then list favorites
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-session-456"));
}

#[test]
fn test_unfavorite() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // First add a favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("to-remove-session")
        .assert()
        .success();

    // Then remove it
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("unfavorite")
        .arg("to-remove-session")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Removed to-remove-session from favorites",
        ));
}

#[test]
fn test_favorite_already_exists() {
    // Adding the same session twice should be idempotent
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("duplicate-session")
        .assert()
        .success();

    // Adding again should still succeed (idempotent)
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("duplicate-session")
        .assert()
        .success();
}

#[test]
fn test_favorite_with_special_chars() {
    // Session IDs with special characters should work
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("abc123-def456_789.012")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Added abc123-def456_789.012 to favorites",
        ));
}

#[test]
fn test_favorite_multiple_sessions() {
    // Add multiple different sessions and verify all are stored
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let sessions = ["session-001", "session-002", "session-003"];

    for session in &sessions {
        let mut cmd = Command::cargo_bin("claude-memo").unwrap();
        cmd.env("CLAUDE_HISTORY", &history_file)
            .env("HOME", temp_dir.path())
            .arg("favorite")
            .arg(session)
            .assert()
            .success();
    }

    // Verify all are listed
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains("session-001"))
        .stdout(predicate::str::contains("session-002"))
        .stdout(predicate::str::contains("session-003"));
}

#[test]
fn test_unfavorite_then_add_again() {
    // Remove a session then add it back
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // Add
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("recyclable-session")
        .assert()
        .success();

    // Remove
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("unfavorite")
        .arg("recyclable-session")
        .assert()
        .success();

    // Add again
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("recyclable-session")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Added recyclable-session to favorites",
        ));
}

// === Edge Case Tests ===

#[test]
fn test_favorite_empty_session_id() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid session ID"));
}

#[test]
fn test_unfavorite_nonexistent_session() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("unfavorite")
        .arg("nonexistent-session-id")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Session not found"));
}

#[test]
fn test_search_with_empty_keyword() {
    // Empty keyword should still work (returns results or gracefully handles)
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("") // Empty keyword
        .assert()
        .success(); // Should not crash
}

#[test]
fn test_parse_with_invalid_jsonl_line() {
    // Test that parse handles invalid lines gracefully (skips them)
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("mixed_history.jsonl");
    let content = r#"{"display":"/valid","timestamp":1766567616338,"project":"/Users/yym","sessionId":"valid-001"}
{invalid line here}
{"display":"/also-valid","timestamp":1766567617000,"project":"/Users/yym","sessionId":"valid-002"}
"#;
    fs::write(&file_path, content).unwrap();

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &file_path)
        .arg("parse")
        .arg("--limit")
        .arg("10")
        .assert()
        .success();

    // Should parse valid lines (at least 2)
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("/valid") || stdout.contains("/also-valid"));
}

#[test]
fn test_search_json_format() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("model")
        .arg("--json")
        .arg("--limit")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"display\""))
        .stdout(predicate::str::contains("\"session_id\""));
}

#[test]
fn test_favorites_json_format() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // First add a favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("json-test-session")
        .assert()
        .success();

    // Then list in JSON format
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"session_id\""));
}

#[test]
fn test_subcommand_help() {
    // Test help for specific subcommand
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("search")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("KEYWORD"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--limit"));
}

// === Missing Boundary Case Tests ===

#[test]
fn test_favorites_empty_list() {
    // Test favorites when list is empty
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // No favorites added - list should be empty
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains("No favorites")); // Should indicate empty
}

#[test]
fn test_parse_subcommand_help() {
    // Test parse --help subcommand
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("parse")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("-n"));
}

#[test]
fn test_favorite_subcommand_help() {
    // Test favorite --help subcommand
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("favorite")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("SESSION_ID")) // Uppercase
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_unfavorite_subcommand_help() {
    // Test unfavorite --help subcommand
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("unfavorite")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("SESSION_ID")) // Uppercase
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_favorites_subcommand_help() {
    // Test favorites --help subcommand
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.arg("favorites")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_unfavorite_from_empty_list() {
    // Test unfavorite when list is empty
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // Try to unfavorite from empty list
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("unfavorite")
        .arg("nonexistent-session")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Session not found"));
}

#[test]
fn test_search_with_very_long_keyword() {
    // Test search with keyword > 100 characters
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let long_keyword = "this_is_a_very_long_search_keyword_that_exceeds_normal_length_requirements_and_should_not_crash_the_application_at_all";

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg(&long_keyword)
        .assert()
        .success(); // Should not crash
}

#[test]
fn test_search_output_includes_session_id() {
    // Verify search output includes session_id for favorite workflow
    // Users need to see session_id to copy it for `claude-memo favorite <session-id>`
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("search")
        .arg("model")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "d55aaa1c-b149-4aa4-9809-7eab1dba8d4c",
        )); // session_id from test data
}

// === Missing Acceptance Scenario Tests ===

#[test]
fn test_parse_empty_file() {
    // US1 场景3: 空文件返回空列表，不报错
    let temp_dir = TempDir::new().unwrap();
    let history_file = temp_dir.path().join("empty_history.jsonl");
    fs::write(&history_file, "").unwrap(); // Create empty file

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .arg("parse")
        .assert()
        .success()
        .stdout(predicate::str::is_empty()); // Should output nothing
}

#[test]
fn test_search_results_ordered_by_timestamp() {
    // US2 场景1: 搜索结果按时间倒序排列（最新的在前）
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("ordered_history.jsonl");
    // Create records with different timestamps
    let content = r#"{"display":"/older command","timestamp":1766567616000,"project":"/Users/yym","sessionId":"session-001"}
{"display":"/newer command","timestamp":1766567619000,"project":"/Users/yym","sessionId":"session-002"}
{"display":"/middle command","timestamp":1766567617500,"project":"/Users/yym","sessionId":"session-003"}
"#;
    fs::write(&file_path, content).unwrap();

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &file_path)
        .arg("search")
        .arg("command")
        .assert()
        .success();

    // Verify order: newer should appear before older
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let newer_pos = stdout.find("/newer command").unwrap();
    let middle_pos = stdout.find("/middle command").unwrap();
    let older_pos = stdout.find("/older command").unwrap();

    assert!(
        newer_pos < middle_pos && middle_pos < older_pos,
        "Results should be ordered by timestamp descending (newest first). Got: {}",
        stdout
    );
}

#[test]
fn test_favorites_persist_after_restart() {
    // US3 场景4: 应用重启后，收藏状态保持不变
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);

    // First: add a favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("persist-test-session")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Added persist-test-session to favorites",
        ));

    // Second: simulate restart by creating new Storage instance (same data dir)
    // In a real scenario, this would be a new process
    // Here we verify the file was written and can be read back
    let favorites_file = temp_dir.path().join(".claude-memo/favorites/sessions.toml");
    assert!(
        favorites_file.exists(),
        "Favorites file should exist after adding"
    );

    // Third: list favorites should show the persisted favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains("persist-test-session"));
}

// === Enhanced Favorites Display Tests ===

#[test]
fn test_favorites_show_display_content() {
    // Favorites display should show display/content field, not just session_id
    // This provides useful context about what the session contains
    let temp_dir = TempDir::new().unwrap();
    let history_file = temp_dir.path().join("history.jsonl");
    let content = r#"{"display":"/implement user authentication feature","timestamp":1766567616338,"project":"/Users/yym/project-a","sessionId":"session-with-content"}
"#;
    fs::write(&history_file, content).unwrap();

    // Add favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("session-with-content")
        .assert()
        .success();

    // List favorites should show display content
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "/implement user authentication feature",
        )); // display content
}

#[test]
fn test_favorites_show_project_info() {
    // Favorites display should show project path for context
    let temp_dir = TempDir::new().unwrap();
    let history_file = temp_dir.path().join("history.jsonl");
    let content = r#"{"display":"/test command","timestamp":1766567616338,"project":"/Users/yym/my-awesome-project","sessionId":"project-session"}
"#;
    fs::write(&history_file, content).unwrap();

    // Add favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("project-session")
        .assert()
        .success();

    // List favorites should show project info
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .assert()
        .success()
        .stdout(predicate::str::contains("my-awesome-project")); // project path
}

#[test]
fn test_favorites_json_includes_session_details() {
    // JSON output should include display, project, timestamp from history
    let temp_dir = TempDir::new().unwrap();
    let history_file = temp_dir.path().join("history.jsonl");
    let content = r#"{"display":"/search database query","timestamp":1766567616338,"project":"/Users/yym/backend","sessionId":"json-detail-session"}
"#;
    fs::write(&history_file, content).unwrap();

    // Add favorite
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorite")
        .arg("json-detail-session")
        .assert()
        .success();

    // JSON output should include full session details
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("HOME", temp_dir.path())
        .arg("favorites")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"/search database query\"")) // display
        .stdout(predicate::str::contains("\"display\"")) // display field name
        .stdout(predicate::str::contains("\"project\"")) // project field
        .stdout(predicate::str::contains("\"timestamp\"")); // timestamp field
}
