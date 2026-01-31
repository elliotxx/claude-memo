# Data Model: Core Prototype

**Date**: 2026-01-29
**Feature**: 001-core-prototype

## Entities

### SessionRecord

代表一条 Claude Code 会话记录。

| Field | Type | Description |
|-------|------|-------------|
| `display` | String | 命令/消息内容 |
| `timestamp` | i64 | Unix 时间戳（毫秒） |
| `project` | String | 项目路径 |
| `session_id` | String | 会话唯一标识 (UUID) |

### FavoriteSession

代表一条收藏的会话。

| Field | Type | Description |
|-------|------|-------------|
| `session_id` | String | 收藏的会话 ID |
| `favorited_at` | i64 | 收藏时间戳（毫秒） |

## Data Sources

### Input: `~/.claude/history.jsonl`

```json
{"display":"/model ","pastedContents":{},"timestamp":1766567616338,"project":"/Users/elliotxx","sessionId":"d55aaa1c-b149-4aa4-9809-7eab1dba8d4c"}
```

### Output: `~/.claude-memo/`

```
~/.claude-memo/
├── index/
│   └── sessions.db        # SQLite FTS5 索引
└── favorites/
    └── sessions.toml      # 收藏列表
```

## Validation Rules

| Rule | Description |
|------|-------------|
| `session_id` | 必须为有效的 UUID 格式 |
| `timestamp` | 必须为正数（未来时间戳被拒绝） |
| `display` | 非空字符串，允许空白字符 |

## File Formats

### `sessions.toml`

```toml
[sessions]
# session_id = timestamp (收藏时间)
"abc123-def456" = 1700000000000
"xyz789-abc123" = 1700000001000
```
