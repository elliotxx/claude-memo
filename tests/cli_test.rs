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
