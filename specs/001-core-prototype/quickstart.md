# Quickstart: Core Prototype

**Date**: 2026-01-29
**Feature**: 001-core-prototype
**Status**: 已实现 ✓

## Prerequisites

- Rust 2021 Edition
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
# 全文搜索（支持前缀匹配）
claude-memo search "关键词"

# 限制结果数量
claude-memo search "关键词" --limit 10

# JSON 格式输出
claude-memo search "关键词" --json

# 显示帮助
claude-memo search --help
```

### 收藏管理

```bash
# 收藏会话
claude-memo favorite <session-id>

# 取消收藏
claude-memo unfavorite <session-id>

# 列出所有收藏
claude-memo favorites

# JSON 格式输出
claude-memo favorites --json
```

### 解析测试

```bash
# 解析并显示历史记录（用于测试）
claude-memo parse

# 限制显示数量
claude-memo parse --limit 5

# JSON 格式输出
claude-memo parse --json
```

### 环境变量

```bash
# 自定义历史文件路径
CLAUDE_HISTORY=/path/to/history.jsonl claude-memo search "关键词"
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

# 运行所有测试（包括集成测试）
cargo test --all

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
2026-01-29 10:30 /Users/elliotxx/workspace/project > /model
2026-01-29 10:31 /Users/elliotxx/workspace/project > /agents
2026-01-29 10:32 /Users/elliotxx/workspace/project > /glm-plan-usage:usage-query
```

### 搜索结果 (JSON)

```json
[
  {
    "display": "/model",
    "timestamp": 1766567616338,
    "project": "/Users/elliotxx/workspace/project",
    "session_id": "abc123-def456-789",
    "score": -0.0
  }
]
```

### 收藏列表

```
⭐ abc123-def456-xxx (2026-01-29 10:30)
⭐ xyz789-abc123-yyy (2026-01-28 15:00)
```

### 收藏列表 (JSON)

```json
[
  {
    "session_id": "abc123-def456-xxx",
    "favorited_at": 1700000000000
  }
]
```

## Test Results

- **31 个测试全部通过** (19 单元测试 + 12 集成测试)
- Parser、Storage、Search 模块覆盖率高
- CLI 命令验证通过
