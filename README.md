# claude-memo

Claude Code 会话记录管理工具。快速搜索、收藏、分享你的 AI 对话历史。

## 功能

- **快速搜索**: 全文检索 + 高级过滤（项目、时间、会话）
- **收藏管理**: 将重要的会话标记收藏，支持标签分类
- **分享导出**: 生成带样式的 HTML 页面，支持截图分享

## 安装

```bash
cargo build --release
cargo install --path .
```

## 使用

### 搜索会话

```bash
# 全文搜索
claude-memo search "关键词"

# 按项目搜索
claude-memo search --project /path/to/project

# 按时间过滤
claude-memo search --last 7d

# 组合查询
claude-memo search --project /path/to/project "关键词"
```

### 收藏会话

```bash
# 收藏指定会话
claude-memo favorite add <session-id>

# 取消收藏
claude-memo favorite remove <session-id>

# 列出收藏
claude-memo favorite list
```

### 分享会话

```bash
# 导出为 HTML
claude-memo export --session <session-id> --output session.html

# 生成长图（需要截图工具）
claude-memo export --session <session-id> --screenshot
```

## 数据存储

| 路径 | 说明 |
|------|------|
| `~/.claude/history.jsonl` | 官方会话记录（只读） |
| `~/.claude-memo/` | 应用数据（索引、收藏、标签） |

## 项目结构

```
claude-memo/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # 主入口，模块声明
│   ├── parser.rs        # 解析 history.jsonl
│   ├── indexer.rs       # 构建搜索索引
│   ├── storage.rs       # ~/.claude-memo/ 数据管理
│   ├── search.rs        # 全文搜索
│   ├── exporter.rs      # HTML 导出
│   └── cli.rs           # CLI 界面
└── docs/
    └── plans/
        └── 2026-01-29-design.md
```

## 开发

```bash
# 检查代码
cargo check

# 运行测试
cargo test

# 格式化
cargo fmt

# 代码检查
cargo clippy
```

## 贡献

欢迎提交 Issue 和 PR。

## 许可证

MIT
