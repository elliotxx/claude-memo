<div align="center">

  <div>
    <img src="./assets/logo.svg" alt="claude-memo Logo" width="80" height="160">
  </div>

  <h1 style="margin-top: 10px;">claude-memo</h1>

  Claude Code 会话记录管理工具。快速搜索、收藏你的 AI 对话历史。

  <div align="center">
    <a href="https://github.com/elliotxx/claude-memo/actions"><img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/elliotxx/claude-memo?logo=github"/></a>
    <a href="https://github.com/elliotxx/claude-memo/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-green"/></a>
    <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.83+-orange.svg"/></a>
  </div>

  <p>
    <a href="#why-claude-memo">Why?</a>
    | <a href="#quick-start">Quick Start</a>
    | <a href="#features">Features</a>
    | <a href="#installation">Installation</a>
    | <a href="#architecture">Architecture</a>
  </p>
</div>

---

## Why claude-memo?

高效管理你的 Claude Code 对话历史，不丢失任何有价值的会话。

- **🔍 全文搜索** - SQLite FTS5 全文检索，BM25 relevance 排序
- **⭐ 收藏管理** - 标记重要会话，快速找回
- **📄 JSON 输出** - 便于脚本处理和数据集成
- **🔒 数据安全** - 官方数据只读，不修改原始记录
- **🚀 轻量快速** - Rust 开发，启动快、占用少

---

## Quick Start

```bash
# 从源码安装
git clone https://github.com/elliotxx/claude-memo.git
cd claude-memo
cargo build --release
cargo install --path .

# 基本使用
claude-memo search "关键词"       # 全文搜索
claude-memo mark <session-id>     # 收藏会话
claude-memo marks                 # 列出收藏
```

**Need more details?** See [Installation](#installation) below for all options.

---

## Features

### 搜索功能

```bash
claude-memo search "关键词"       # 全文搜索
claude-memo search "关键词" -n 10 # 限制结果数量
claude-memo search "关键词" --json # JSON 格式输出
```

### 收藏管理

```bash
claude-memo mark <session-id>     # 收藏会话
claude-memo unmark <session-id>   # 取消收藏
claude-memo marks                 # 列出收藏
claude-memo marks --json          # JSON 格式输出
```

### 调试工具

```bash
claude-memo parse                 # 解析历史记录
claude-memo parse -n 5            # 限制显示数量
claude-memo parse --json          # JSON 格式输出
```

### 环境变量

| 变量 | 说明 |
|------|------|
| `CLAUDE_HISTORY` | 自定义历史文件路径 |
| `CLAUDE_MEMO_DIR` | 自定义应用数据目录 |

---

## Demo

### 搜索演示

```
$ claude-memo search "authentication"

[2024-03-15 14:30] /Users/elliotxx/project-a
[🔍 Score: 5.2] Implement user authentication feature
Session ID: abc123-def456-789

[2024-03-14 09:15] /Users/elliotxx/project-b
[🔍 Score: 3.1] Add OAuth2 authentication
Session ID: xyz789-abc123-def
```

### 收藏演示

```
$ claude-memo mark abc123-def456-789
✅ Added abc123-def456-789 to marks

$ claude-memo marks

⭐ [2024-03-15 14:30] Implement user authentication feature
   /Users/elliotxx/project-a | Session: abc123-def456-789
```

### JSON 输出

```json
{
  "version": "0.1.0",
  "results": [
    {
      "display": "Implement user authentication feature",
      "timestamp": "2024-03-15 14:30:00",
      "project": "/Users/elliotxx/project-a",
      "session_id": "abc123-def456-789",
      "score": 5.2
    }
  ]
}
```

---

## Installation

### 从源码安装

```bash
git clone https://github.com/elliotxx/claude-memo.git
cd claude-memo
cargo build --release
cargo install --path .
```

### 环境要求

- Rust 1.83+ (2024 Edition)
- macOS / Linux

---

## Architecture

### Component Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     CLI (clap)                          │
│                   main.rs / cli.rs                      │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                     Parser                              │
│  • parse_history_file() - 解析 history.jsonl           │
│  • Record 结构体 - 会话记录模型                          │
└──────────────────────┬──────────────────────────────────┘
                       │
          ┌────────────┴────────────┐
          ▼                         ▼
┌─────────────────┐      ┌─────────────────────────────┐
│   Indexer       │      │        Storage              │
│   FTS5 索引构建 │      │        TOML 持久化           │
│   • 增量更新    │      │   • add_favorite()          │
└─────────────────┘      │   • remove_favorite()       │
                         │   • list_favorites()        │
                         └─────────────────────────────┘
          ┌────────────┴────────────┐
          ▼                         ▼
┌─────────────────┐      ┌─────────────────────────────┐
│     Search      │      │         Output              │
│   • BM25 排序   │      │         文本 / JSON          │
│   • FTS5 查询   │      │                              │
└─────────────────┘      └─────────────────────────────┘
```

### 数据存储

| 路径 | 说明 |
|------|------|
| `~/.claude/history.jsonl` | 官方会话记录（只读） |
| `~/.claude-memo/index/sessions.db` | SQLite FTS5 搜索索引 |
| `~/.claude-memo/favorites/sessions.toml` | 收藏列表 |

### 技术栈

- **语言**: Rust 2024 Edition
- **CLI**: clap 4.4
- **数据库**: SQLite (FTS5 全文检索)
- **配置**: TOML
- **测试**: assert_cmd + predicates

---

## Development

### 快速开始

```bash
# 运行所有检查
cargo fmt && cargo clippy && cargo test

# 运行测试
cargo test

# 集成测试
cargo test --test cli_test
```

### 构建

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 检查格式
cargo fmt --check

# 运行 clippy
cargo clippy --all-features -- -D warnings
```

### 测试

```bash
# 运行所有测试
cargo test --all-features

# 详细输出
cargo test --all-features --verbose

# 运行特定测试
cargo test test_name
```

---

## Contributing

欢迎提交 Issue 和 PR！

### 贡献方式

- **功能开发**: 添加新功能和特性
- **Bug 修复**: 修复问题和改进稳定性
- **文档改进**: 完善文档、示例和教程
- **测试添加**: 增加测试用例，提高覆盖率

### 快速开始

```bash
# Fork 仓库后克隆
git clone https://github.com/elliotxx/claude-memo.git
cd claude-memo

# 安装依赖（见上方 Installation）

# 创建功能分支
git checkout -b feature/your-feature-name

# 修改、测试、提交并推送
git add .
git commit -m "feat: description"
git push origin feature/your-feature-name
```

---

## License

本项目采用 **MIT License** - 详见 [LICENSE](LICENSE) 文件。

---

## Acknowledgments

- [Claude Code](https://claude.com/claude-code) - 项目灵感来源
- [clap](https://github.com/clap-rs/clap) - CLI 参数解析
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite 绑定

---

<div align="center">
  <p>
    <strong>Built with ❤️ for elliotxx</strong><br>
    <sub>高效管理你的 AI 对话历史</sub>
  </p>
</div>
