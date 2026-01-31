# claude-memo

<div align="center">
  <img src="./assets/logo.svg" alt="claude-memo Logo" width="80" height="160">

  <h1 style="margin-top: 10px;">claude-memo</h1>

  Claude Code 会话记录管理工具。快速搜索、收藏你的 AI 对话历史。

  <a href="https://github.com/yym/claude-memo/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/yym/claude-memo"/></a>
  <a href="https://github.com/yym/claude-memo/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-green"/></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-2024 Edition-blue"/></a>

  <p>
    <a href="#功能">功能</a>
    | <a href="#安装">安装</a>
    | <a href="#使用">使用</a>
  </p>
</div>

---

## 功能

- **🔍 全文搜索** - SQLite FTS5 索引，BM25  relevance 排序
- **⭐ 收藏管理** - 标记重要会话，快速找回
- **📄 JSON 输出** - 便于脚本处理和数据集成
- **🔒 数据安全** - 官方数据只读，不修改原始记录
- **🚀 轻量快速** - Rust 开发，启动快、占用少

---

## 安装

```bash
git clone https://github.com/yym/claude-memo.git
cd claude-memo
cargo build --release
cargo install --path .
```

**前提条件**: Rust 1.75+ (2024 Edition)

---

## 使用

| 命令 | 说明 |
|------|------|
| `claude-memo search "关键词"` | 全文搜索会话 |
| `claude-memo search "关键词" -n 10` | 限制结果数量 |
| `claude-memo search "关键词" --json` | JSON 格式输出 |
| `claude-memo mark <session-id>` | 收藏会话 |
| `claude-memo marks` | 列出收藏 |
| `claude-memo parse` | 解析历史记录（调试用） |

**环境变量**

| 变量 | 说明 |
|------|------|
| `CLAUDE_HISTORY` | 自定义历史文件路径 |
| `CLAUDE_MEMO_DIR` | 自定义应用数据目录 |

---

## 架构

```
┌─────────────────────────────────────────────────────────┐
│                        CLI (clap)                        │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                      Parser                             │
│               parse_history_file()                       │
└──────────────────────┬──────────────────────────────────┘
                       │
          ┌────────────┴────────────┐
          ▼                         ▼
┌─────────────────┐      ┌─────────────────────────────┐
│   Indexer       │      │        Storage              │
│   FTS5 索引     │      │        TOML 持久化           │
└─────────────────┘      └─────────────────────────────┘
          │                         │
          ▼                         ▼
┌─────────────────┐      ┌─────────────────────────────┐
│   Search        │      │         Output              │
│   BM25 排序     │      │         文本 / JSON          │
└─────────────────┘      └─────────────────────────────┘
```

**数据存储**

| 路径 | 说明 |
|------|------|
| `~/.claude/history.jsonl` | 官方会话记录（只读） |
| `~/.claude-memo/index/sessions.db` | SQLite FTS5 索引 |
| `~/.claude-memo/favorites/sessions.toml` | 收藏列表 |

---

## 开发

```bash
cargo check        # 检查代码
cargo test         # 运行测试
cargo fmt          # 格式化
cargo clippy       # 代码检查
```

---

## License

MIT License - 详见 [LICENSE](LICENSE) 文件。

---

<div align="center">
  <strong>Built with ❤️ for elliotxx</strong>
</div>
