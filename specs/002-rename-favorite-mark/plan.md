# Implementation Plan: Rename Favorite to Mark

**Branch**: `002-rename-favorite-mark` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-rename-favorite-mark/spec.md`

## Summary

将 CLI 命令从 `favorite`/`unfavorite`/`favorites` 重命名为 `mark`/`unmark`/`marks`，直接删除旧命令。这是一个纯 CLI 重命名任务，不涉及数据模型变更。

## Technical Context

**Language/Version**: Rust 2024 Edition (1.75+) | **Primary Dependencies**: clap 4.4 (CLI), rusqlite 0.31 + FTS5, toml 0.8 | **Storage**: TOML (收藏配置) | **Testing**: cargo test, assert_cmd | **Target Platform**: macOS/Linux CLI | **Project Type**: 单 CLI 项目 | **Performance Goals**: N/A (CLI 命令重命名) | **Constraints**: 不保持向后兼容，直接删除旧命令 | **Scale/Scope**: 小型重构，约 10-15 处代码修改

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### TDD 合规性检查

| 要求 | 状态 | 说明 |
|------|------|------|
| 功能实现前测试已编写且失败 | ✅ | 先为新命令编写集成测试 |
| 单元测试覆盖主要分支 | ✅ | 新命令功能分支需要测试 |
| 集成测试覆盖所有 CLI 子命令 | ✅ | 需要添加新命令的 CLI 测试 |
| 集成测试覆盖错误场景 | ✅ | 旧命令删除后验证不存在测试 |
| `cargo check` 通过 | ⏳ | 需验证 |
| `cargo clippy` 无警告 | ⏳ | 需验证 |
| `cargo fmt` 无需格式化 | ⏳ | 需验证 |

### 关键原则检查

| 原则 | 状态 | 说明 |
|------|------|------|
| Test-First Development | ✅ | 遵循 TDD 流程 |
| Minimalist Design (YAGNI) | ✅ | 仅重命名命令，无过度设计 |
| Immutable Source Data | ✅ | 不涉及数据变更 |
| CLI-First Interface | ✅ | 纯 CLI 变更 |

## Project Structure

### Documentation (this feature)

```text
specs/002-rename-favorite-mark/
├── plan.md              # This file
├── research.md          # Phase 0 output (if needed)
├── data-model.md        # N/A - no data model change
├── quickstart.md        # N/A - no new usage patterns
└── contracts/           # N/A - no API changes
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI 入口
└── cli/
    ├── mod.rs           # CLI 模块
    ├── commands/
    │   ├── mod.rs       # 命令注册
    │   ├── mark.rs      # 新增：mark 命令 (从 favorite.rs 重命名)
    │   └── unmark.rs    # 新增：unmark 命令 (从 unfavorite.rs 重命名)
    └── list_marks.rs    # 新增：marks 命令 (从 list_favorites.rs 重命名)

tests/
├── cli_test.rs          # 现有 CLI 测试
└── integration/
    └── mark_commands.rs # 新增：mark/unmark/marks 命令测试
```

**Structure Decision**: 采用单项目结构，所有变更集中在 `src/cli/` 目录下。命令实现从 `favorite.rs`/`unfavorite.rs` 重命名为 `mark.rs`/`unmark.rs`，新增 `marks.rs` 命令文件。

## Phase 0: Research

本功能为简单重命名，无复杂技术决策需要研究。

**已确认**：
- clap 子命令重命名机制（使用 `rename` 属性）
- 直接删除旧命令，无需 deprecated 属性
- 无需数据迁移

## Phase 1: Design & Contracts

### 命令映射

| 原命令 | 新命令 | 处理方式 |
|--------|--------|----------|
| `favorite <id>` | `mark <id>` | 重命名实现 |
| `unfavorite <id>` | `unmark <id>` | 重命名实现 |
| `favorites` | `marks` | 重命名实现 |
| `favorite` (旧) | - | **删除** |
| `unfavorite` (旧) | - | **删除** |
| `favorites` (旧) | - | **删除** |

### 实现策略

1. **重命名命令**：将 `favorite.rs` → `mark.rs`、`unfavorite.rs` → `unmark.rs`、`list_favorites.rs` → `marks.rs`
2. **更新命令注册**：在 `commands/mod.rs` 中更新子命令名称
3. **测试覆盖**：添加新命令的集成测试

### 集成测试场景

```rust
// tests/integration/mark_commands.rs

// 新命令测试 - mark 添加收藏
assert_cmd::Command::cargo_bin("claude-memo")
    .arg("mark")
    .arg("test-session-id")
    .assert()
    .success();

// 新命令测试 - unmark 移除收藏
assert_cmd::Command::cargo_bin("claude-memo")
    .arg("unmark")
    .arg("test-session-id")
    .assert()
    .success();

// 新命令测试 - marks 列出收藏
assert_cmd::Command::cargo_bin("claude-memo")
    .arg("marks")
    .assert()
    .success()
    .stdout(predicate::str::contains("session_id"));

// 新命令测试 - --json 输出格式
assert_cmd::Command::cargo_bin("claude-memo")
    .arg("marks")
    .arg("--json")
    .assert()
    .success()
    .stdout(predicate::str::contains("session_id"));

// 旧命令已删除测试
assert_cmd::Command::cargo_bin("claude-memo")
    .arg("favorite")
    .arg("test-session-id")
    .assert()
    .failure()
    .stderr(predicate::str::contains("未找到命令"));
```

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | 本次为简单重命名，无违规 | - |

## Open Questions

1. ~~是否需要保留旧命令作为别名？~~ 已决定：直接删除旧命令
2. ~~测试数据目录是否需要单独配置？~~ 使用现有测试配置

## Estimated Effort

- **代码修改**：约 5-10 处（文件重命名和命令注册更新）
- **测试编写**：约 8-12 个测试用例
- **文档更新**：README.md 和帮助信息
- **总工作量**：约 1-2 小时
