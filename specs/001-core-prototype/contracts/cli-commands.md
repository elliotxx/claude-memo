# CLI Commands Contract

**Date**: 2026-01-29
**Feature**: 001-core-prototype

## Command Structure

```
claude-memo <command> [options] [arguments]
```

## Commands

### `search`

全文搜索会话记录。

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
<timestamp> <project> > <display>
```

**Output (JSON)**:

```json
[
  {
    "display": "/model ",
    "timestamp": 1766567616338,
    "project": "/Users/yym",
    "session_id": "abc123-def456"
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
| `--limit <N>` | 限制显示数量 |

### `favorite add`

收藏指定会话。

```bash
claude-memo favorite add <SESSION_ID>
```

**Example**:

```bash
claude-memo favorite add abc123-def456-xxx
```

**Output**:

```
✅ Added abc123-def456-xxx to favorites
```

### `favorite remove`

取消收藏。

```bash
claude-memo favorite remove <SESSION_ID>
```

**Example**:

```bash
claude-memo favorite remove abc123-def456-xxx
```

**Output**:

```
✅ Removed abc123-def456-xxx from favorites
```

### `favorite list`

列出所有收藏。

```bash
claude-memo favorite list [OPTIONS]
```

**Options**:

| Flag | Description |
|------|-------------|
| `--json` | JSON 格式输出 |

**Output (Text)**:

```
⭐ <session_id> (<timestamp>)
  Project: <project>
```

### `help`

显示帮助信息。

```bash
claude-memo help
claude-memo <command> --help
```

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
| 文件不存在 | 3 |
| 解析错误 | 4 |
