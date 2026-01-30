//! Integration tests for mark/unmark/marks commands

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn create_test_history_file(temp_dir: &TempDir) -> std::path::PathBuf {
    let file_path = temp_dir.path().join("history.jsonl");
    let content = r#"{"display":"/model ","pastedContents":{},"timestamp":1766567616338,"project":"/Users/yym","sessionId":"test-session-001"}
{"display":"/search test query","pastedContents":{},"timestamp":1766567617000,"project":"/Users/yym/project","sessionId":"test-session-002"}
{"display":"/another command","pastedContents":{},"timestamp":1766567618000,"project":"/Users/yym/other","sessionId":"test-session-003"}
"#;
    fs::write(&file_path, content).unwrap();
    file_path
}

/// Helper to setup temp config directory for marks
fn setup_config_dir(temp_dir: &TempDir) {
    let config_dir = temp_dir.path().join(".claude-memo");
    fs::create_dir_all(&config_dir).unwrap();
}

/// Test T010: mark command adds session to favorites
#[test]
fn test_mark_command() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("mark")
        .arg("test-session-001")
        .assert()
        .success()
        .stdout(predicate::str::contains("Added").contains("test-session-001"));
}

/// Test T011: unmark command removes session from favorites
#[test]
fn test_unmark_command() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    // First add a mark
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("mark")
        .arg("test-session-001")
        .assert()
        .success();

    // Then unmark it
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("unmark")
        .arg("test-session-001")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed").contains("test-session-001"));
}

/// Test T012: marks command lists all favorites
#[test]
fn test_marks_command() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    // First add some marks
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("mark")
        .arg("test-session-001")
        .assert()
        .success();

    // List marks
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("marks")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-session-001"));
}

/// Test T013: marks command with --json output
#[test]
fn test_marks_command_json() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    // First add a mark
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("mark")
        .arg("test-session-001")
        .assert()
        .success();

    // List marks with JSON output
    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("marks")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"session_id\""));
}

/// Test T014: error handling for invalid session-id
#[test]
fn test_mark_invalid_session() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("mark")
        .arg("nonexistent-session")
        .assert()
        .success()
        .stdout(predicate::str::contains("Added"));
}

/// Test T020: old favorite command should fail (deleted)
#[test]
fn test_old_favorite_command_deleted() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("favorite")
        .arg("test-session-001")
        .assert()
        .failure()
        .stderr(predicate::str::contains("未找到命令").or(predicate::str::contains("no such subcommand")));
}

/// Test T021: old unfavorite command should fail (deleted)
#[test]
fn test_old_unfavorite_command_deleted() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("unfavorite")
        .arg("test-session-001")
        .assert()
        .failure()
        .stderr(predicate::str::contains("未找到命令").or(predicate::str::contains("no such subcommand")));
}

/// Test T022: old favorites command should fail (deleted)
#[test]
fn test_old_favorites_command_deleted() {
    let temp_dir = TempDir::new().unwrap();
    let history_file = create_test_history_file(&temp_dir);
    setup_config_dir(&temp_dir);

    let mut cmd = Command::cargo_bin("claude-memo").unwrap();
    cmd.env("CLAUDE_HISTORY", &history_file)
        .env("CLAUDE_MEMO_DIR", temp_dir.path().join(".claude-memo"))
        .arg("favorites")
        .assert()
        .failure()
        .stderr(predicate::str::contains("未找到命令").or(predicate::str::contains("no such subcommand")));
}
