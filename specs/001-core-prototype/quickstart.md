# Quickstart: Core Prototype

**Date**: 2026-01-29
**Feature**: 001-core-prototype

## Prerequisites

- Rust 2024 Edition
- `~/.claude/history.jsonl` 文件存在

## Build

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 安装到 PATH
cargo install --path .
```

## Usage

### 搜索会话

```bash
# 全文搜索
claude-memo search "关键词"

# 显示帮助
claude-memo search --help
```

### 收藏管理

```bash
# 收藏会话
claude-memo favorite add <session-id>

# 取消收藏
claude-memo favorite remove <session-id>

# 列出收藏
claude-memo favorite list
```

### 解析测试

```bash
# 解析并显示历史记录（用于测试）
claude-memo parse
```

## Data Locations

| Data | Location |
|------|----------|
| 输入 | `~/.claude/history.jsonl` |
| 索引 | `~/.claude-memo/index/sessions.db` |
| 收藏 | `~/.claude-memo/favorites/sessions.toml` |

## Running Tests

```bash
# 运行所有测试
cargo test

# 运行覆盖率
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# 检查代码质量
cargo check
cargo fmt
cargo clippy
```

## Expected Output

### 搜索结果

```
2026-01-29 10:30 /Users/yym/workspace/project > /model
2026-01-29 10:31 /Users/yym/workspace/project > /agents
2026-01-29 10:32 /Users/yym/workspace/project > /glm-plan-usage:usage-query
```

### 收藏列表

```
⭐ abc123-def456-xxx (2026-01-29 10:30)
⭐ xyz789-abc123-yyy (2026-01-28 15:00)
```
