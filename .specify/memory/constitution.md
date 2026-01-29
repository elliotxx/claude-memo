<!--
Sync Impact Report
==================
Version Change: 1.1.0 → 1.2.0

Modified Principles:
- I. Test-First (ENHANCED) → Now explicitly requires TDD workflow with detailed steps
- II. Incremental Implementation → Minor wording clarification
- IV. Immutable Source Data → Added explicit violation consequences

Added Sections:
- TDD Workflow Details (new subsection under Test-First)
- TDD Compliance Checklist (new)
- Feature Implementation Checklist (new)

Removed Sections: None

Templates Status:
- ✅ plan-template.md - Already has "Constitution Check" section, no update needed
- ✅ spec-template.md - Already has "User Scenarios & Testing" mandatory section
- ✅ tasks-template.md - Already emphasizes "Tests MUST be written and FAIL before implementation"
- ✅ .specify/templates/commands/plan.md - No update needed

Follow-up TODOs: None
-->

# Claude Memo Constitution

## Core Principles

### I. Test-First Development (NON-NEGOTIABLE)

所有功能必须严格遵循 TDD 流程：

**TDD 三步循环**：
1. **RED**: 编写失败的测试（测试必须先于实现）
2. **GREEN**: 编写最少代码使测试通过（不追求完美）
3. **REFACTOR**: 重构代码，消除重复，提升质量

**TDD 流程要求**：
- 每个功能必须有对应的单元测试
- 测试文件与实现文件同名，放在 `tests/` 目录
- 测试必须独立运行，不依赖其他测试的执行顺序
- 测试必须快速执行（单次运行 < 1秒）
- **禁止**：先实现功能后补测试

**测试覆盖率要求**：
- parser、indexer、search 模块：≥ 80%
- 其他核心模块：≥ 60%
- 新增功能必须同时添加测试

**TDD 合规检查清单**：
- [ ] 功能实现前，测试已编写且失败
- [ ] 测试覆盖主要分支（if/else、match）
- [ ] 测试覆盖边界条件
- [ ] 测试覆盖错误处理路径
- [ ] 所有测试通过后才能提交

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
- 只读操作包括：解析、索引、搜索

**违反此原则的代码不得合并**。

### V. CLI-First Interface

CLI 是主要交互方式：
- 支持交互式模式（fzf 风格）和非交互模式
- 输出格式：文本为主，支持 JSON 导出
- 错误信息必须清晰、actionable

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

### TDD 实施流程

1. **理解需求** → 编写失败的测试（RED）
2. **实现功能** → 最少代码使测试通过（GREEN）
3. **重构** → 提升代码质量（REFACTOR）
4. **验证** → 确保所有测试通过
5. **提交** → 进入下一功能

### 功能实现检查清单

- [ ] 符合 TDD 三步循环
- [ ] 测试覆盖率达标
- [ ] `cargo check` 通过
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 无需格式化
- [ ] 所有单元测试通过
- [ ] 集成测试覆盖核心功能
- [ ] 代码审查通过

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
- TDD 流程必须严格执行

---

**Version**: 1.2.0 | **Ratified**: 2026-01-29 | **Last Amended**: 2026-01-29
