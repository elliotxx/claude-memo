# CLI Commands Contract

**Date**: 2026-01-29
**Feature**: 001-core-prototype
**Status**: 已实现 ✓

## Command Structure

```
claude-memo <command> [options] [arguments]
```

## Commands

### `search`

全文搜索会话记录（使用 SQLite FTS5 索引）。

```bash
claude-memo search [OPTIONS] <KEYWORD>
```

**Options**:

| Flag | Description |
|------|-------------|
| `--json` | JSON 格式输出 |
| `--limit <N>` | 限制结果数量 (默认: 20) |

**Example**:

```bash
claude-memo search "model" --limit 10
claude-memo search "agent" --json
```

**Output (Text)**:

```
2026-01-29 10:30 /Users/elliotxx/workspace/project > /model
```

**Output (JSON)**:

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

### `parse`

解析并显示历史记录（调试用）。

```bash
claude-memo parse [OPTIONS]
```

**Options**:

| Flag | Description |
|------|-------------|
| `--json` | JSON 格式输出 |
| `--limit <N>`, `-n <N>` | 限制显示数量 |

**Example**:

```bash
claude-memo parse --limit 5 --json
```

### `favorite`

收藏指定会话。

```bash
claude-memo favorite <SESSION_ID>
```

**Example**:

```bash
claude-memo favorite abc123-def456-xxx
```

**Output**:

```
✅ Added abc123-def456-xxx to favorites
```

### `unfavorite`

取消收藏。

```bash
claude-memo unfavorite <SESSION_ID>
```

**Example**:

```bash
claude-memo unfavorite abc123-def456-xxx
```

**Output**:

```
✅ Removed abc123-def456-xxx from favorites
```

### `favorites`

列出所有收藏。

```bash
claude-memo favorites [OPTIONS]
```

**Options**:

| Flag | Description |
|------|-------------|
| `--json` | JSON 格式输出 |

**Example**:

```bash
claude-memo favorites --json
```

**Output (Text)**:

```
⭐ abc123-def456-xxx (2026-01-29 10:30)
```

**Output (JSON)**:

```json
[
  {
    "session_id": "abc123-def456-xxx",
    "favorited_at": 1700000000000
  }
]
```

### `help`

显示帮助信息。

```bash
claude-memo help
claude-memo <command> --help
```

### Global Options

| Flag | Description |
|------|-------------|
| `-h`, `--help` | 显示帮助 |
 `--version` || `-V`, 显示版本 |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CLAUDE_HISTORY` | 自定义历史文件路径 |

## Error Codes

| Code | Description |
|------|-------------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 参数错误 |
| 3 | 文件不存在 |
| 4 | 解析错误 |

## Exit Behavior

| Scenario | Exit Code |
|----------|-----------|
| 搜索有结果 | 0 |
| 搜索无结果 | 0 (输出 "No results") |
| 收藏成功 | 0 |
| 取消收藏成功 | 0 |
| 文件不存在 | 3 |
| 解析错误 | 4 |
