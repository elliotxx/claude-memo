# claude-memo

Claude Code 会话记录管理工具。快速搜索、收藏你的 AI 对话历史。

## 功能

- **快速搜索**: SQLite FTS5 全文检索，支持 BM25  Relevance 排序
- **收藏管理**: 将重要的会话标记收藏
- **JSON 输出**: 支持 JSON 格式输出，便于脚本处理

## 安装

```bash
cargo build --release
cargo install --path .
```

## 使用

### 搜索会话

```bash
# 全文搜索（支持前缀匹配）
claude-memo search "关键词"

# 限制结果数量
claude-memo search "关键词" --limit 10

# JSON 格式输出
claude-memo search "关键词" --json
```

### 解析会话（调试用）

```bash
# 解析并显示历史记录
claude-memo parse

# 限制显示数量
claude-memo parse --limit 5

# JSON 格式输出
claude-memo parse --json
```

### 收藏会话

```bash
# 收藏指定会话
claude-memo favorite <session-id>

# 取消收藏
claude-memo unfavorite <session-id>

# 列出所有收藏
claude-memo favorites

# JSON 格式输出
claude-memo favorites --json
```

### 环境变量

| 变量 | 说明 |
|------|------|
| `CLAUDE_HISTORY` | 自定义历史文件路径 |

## 数据存储

| 路径 | 说明 |
|------|------|
| `~/.claude/history.jsonl` | 官方会话记录（只读） |
| `~/.claude-memo/index/sessions.db` | SQLite FTS5 搜索索引 |
| `~/.claude-memo/favorites/sessions.toml` | 收藏列表 |

## 项目结构

```
claude-memo/
├── Cargo.toml
├── README.md
├── rust-toolchain.toml
├── src/
│   ├── lib.rs           # 主入口，模块声明
│   ├── main.rs          # CLI 入口
│   ├── error.rs         # 错误类型定义
│   ├── parser.rs        # 解析 history.jsonl
│   ├── indexer.rs       # 构建 FTS5 搜索索引
│   ├── storage.rs       # ~/.claude-memo/ 收藏管理
│   ├── search.rs        # FTS5 全文搜索
│   ├── exporter.rs      # HTML 导出（预留）
│   └── cli.rs           # CLI 命令定义
├── tests/
│   └── cli_test.rs      # CLI 集成测试
└── specs/
    └── 001-core-prototype/
        ├── spec.md           # 功能规格
        ├── plan.md           # 实现计划
        ├── tasks.md          # 任务清单
        ├── data-model.md     # 数据模型
        ├── research.md       # 技术调研
        ├── contracts/        # 契约文档
        └── quickstart.md     # 快速入门
```

## 开发

```bash
# 检查代码
cargo check

# 运行测试
cargo test

# 运行所有测试（包括集成测试）
cargo test --all

# 格式化
cargo fmt

# 代码检查
cargo clippy

# 覆盖率报告
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 测试结果

- **31 个测试全部通过** (19 单元测试 + 12 集成测试)
- Parser、Storage、Search 模块覆盖率高

## 贡献

欢迎提交 Issue 和 PR。

## 许可证

MIT
