# Research: Core Prototype

**Date**: 2026-01-29
**Feature**: 001-core-prototype

## CLI Framework: clap 4.4

**Decision**: 使用 clap 4.4

**Rationale**:
- Rust 生态系统最成熟的 CLI 解析库
- 支持 derive 宏，代码简洁
- 子命令支持符合规范要求的 `search`、`favorite` 等命令
- 良好的错误信息和帮助文档

**Alternatives Considered**:
- `structopt`: 已合并到 clap，不再独立维护
- `clap v5`: 特性类似，v4 稳定且已验证

## Search Index: SQLite FTS5

**Decision**: 使用 SQLite FTS5 内置全文索引

**Rationale**:
- rusqlite 已包含 FTS5 支持（`features = ["bundled"]`）
- 无需额外依赖
- 查询性能 O(log n)，满足 10,000 条 < 5 秒要求
- 内置支持中文分词（通过 tokenizer）

**Alternatives Considered**:
- `tantivy`: 专业搜索引擎，功能全但引入新依赖
- 简单遍历: O(n) 复杂度，无法满足性能要求
- `regex`: 正则匹配，功能强但无索引支持

## Favorite Storage: TOML

**Decision**: 使用 TOML 文件存储收藏

**Rationale**:
- `toml` crate 已作为项目依赖
- 手动可编辑，用户可直接修改
- 格式简单，易于解析
- 符合项目技术栈

**Alternatives Considered**:
- SQLite: 过度设计，收藏数据量小
- JSON: 虽通用但 TOML 更符合 Rust 惯例

## Testing: cargo test + tarpaulin

**Decision**: 使用 tarpaulin 进行代码覆盖率测试

**Rationale**:
- Rust 官方推荐的覆盖率工具
- 与 cargo test 集成良好
- 支持生成 HTML 报告

**Usage**:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Error Handling

**Approach**: 使用 `anyhow` + `thiserror`

- `thiserror` 定义具体错误类型
- `anyhow` 提供上下文友好的错误链
- 错误信息对用户友好，actionable

## Code Structure

### 模块划分

```
src/
├── lib.rs           # 主入口
├── parser.rs        # 解析 history.jsonl
├── indexer.rs       # 构建 FTS5 索引
├── storage.rs       # 收藏存储
├── search.rs        # 搜索逻辑
└── cli.rs           # CLI 命令定义
```

### 测试结构

```
tests/
├── unit/
│   ├── parser_test.rs
│   ├── search_test.rs
│   └── storage_test.rs
└── integration/
    └── cli_test.rs
```

## Performance Considerations

- **索引构建**: 首次加载时构建 FTS5 索引，后续增量更新
- **内存占用**: SQLite 内存模式用于搜索，持久化到磁盘
- **搜索延迟**: FTS5 查询 < 100ms（10,000 条记录）
