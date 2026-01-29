# Claude Memo Constitution

## Core Principles

### I. Test-First (NON-NEGOTIABLE)

所有功能必须遵循 TDD 流程：
1. 先编写失败的单元测试
2. 获得用户对测试方案的确认
3. 实现功能使测试通过
4. 重构代码

核心模块（parser、indexer、search）必须有 80%+ 测试覆盖率。

### II. Incremental Implementation

采用渐进式实现策略：
- 第一阶段：核心原型（解析、搜索、收藏）+ 单元测试
- 第二阶段：高级功能（过滤、导出、标签）
- 第三阶段：外壳（CLI、UI）

每个阶段完成后才能进入下一阶段，禁止跳跃式开发。

### III. Data Isolation

数据存储严格分离：
- 官方数据 `~/.claude/history.jsonl` 为只读
- 应用数据存储在 `~/.claude-memo/`

### IV. Immutable Source Data (NON-NEGOTIABLE)

**绝对禁止修改用户原始数据目录**：
- `~/.claude/` 目录下的所有文件均为只读
- 禁止对该目录进行写入、删除、修改操作
- 禁止创建、修改或删除任何文件或子目录
- 只读操作包括：解析、索引、搜索

违反此原则的代码不得合并。

### V. CLI-First Interface

CLI 是主要交互方式：
- 支持交互式模式（fzf 风格）和非交互模式
- 输出格式：文本为主，支持 JSON 导出
- 错误信息必须清晰、 actionable

### VI. Minimalist Design (YAGNI)

避免过度设计：
- 只实现当前阶段必需的功能
- 不预留"未来可能需要"的抽象
- 优先简单方案，复杂方案需充分理由

## Additional Constraints

### 技术栈约束

- 语言：Rust（稳定版）
- 索引方案：SQLite FTS5 或 tantivy
- 配置格式：TOML
- CLI 框架：clap

### 代码质量

- 所有公开 API 必须有文档注释（`///`）
- 遵守 `cargo clippy` 和 `cargo fmt` 规范
- PR 必须通过所有 CI 检查才能合并

## Development Workflow

### 开发流程

1. 创建功能分支
2. 编写测试 → 实现功能 → 重构
3. 确保 `cargo clippy`、`cargo fmt`、`cargo test` 通过
4. 提交 PR，邀请代码审查
5. 合并后删除分支

### 质量门禁

- `cargo check` 必须通过
- `cargo clippy` 无警告
- `cargo fmt` 无需格式化
- 所有单元测试通过
- 集成测试覆盖核心功能

## Governance

本宪法凌驾于其他开发实践之上。

**修订规则**：
- 原则的添加/删除需文档化并获得确认
- 重大变更需要迁移计划
- 所有 PR 必须验证合规性

**版本策略**：
- MAJOR：不兼容的原则移除或重新定义
- MINOR：新增原则或重大扩展
- PATCH：澄清、措辞、typo 修复

**合规审查**：
- 每次代码审查需验证原则遵守
- 复杂度必须有充分理由

---

**Version**: 1.1.0 | **Ratified**: 2026-01-29 | **Last Amended**: 2026-01-29
