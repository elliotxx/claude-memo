# Implementation Plan: Core Prototype

**Branch**: `001-core-prototype` | **Date**: 2026-01-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-core-prototype/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

实现 claude-memo 核心原型，包括：解析 `history.jsonl` 文件、基于 SQLite FTS5 的全文搜索、TOML 格式的收藏管理，以及满足 80%+ 测试覆盖率的单元测试。

## Technical Context

**Language/Version**: Rust 2024 Edition
**Primary Dependencies**: clap 4.4 (CLI), rusqlite 0.31 + FTS5 (搜索索引), toml 0.8 (配置)
**Storage**: `~/.claude-memo/` - SQLite FTS5 索引 + TOML 收藏文件
**Testing**: cargo test + tarpaulin (代码覆盖率)
**Target Platform**: macOS / Linux (跨平台 CLI 工具)
**Project Type**: single (Rust 二进制应用)
**Performance Goals**: 10,000 条记录搜索 < 5 秒，结果返回 < 1 秒
**Constraints**: 遵守宪法 IV (Immutable Source Data)，禁止修改 `~/.claude/`
**Scale/Scope**: 单用户本地工具，预计支持 10,000-100,000 条历史记录

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Test-First (NON-NEGOTIABLE) | PASS | 核心模块 80%+ 覆盖率要求已纳入 SC |
| II. Incremental Implementation | PASS | 三阶段规划符合要求 |
| III. Data Isolation | PASS | 只读 `~/.claude/`，数据存储在 `~/.claude-memo/` |
| IV. Immutable Source Data | PASS | 禁止修改原始数据目录是核心约束 |
| V. CLI-First Interface | PASS | 子命令结构已定义 |
| VI. Minimalist Design (YAGNI) | PASS | 聚焦核心原型实现 |

## Project Structure

### Documentation (this feature)

```text
specs/001-core-prototype/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── cli-commands.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # 主入口，模块声明
├── parser.rs            # 解析 history.jsonl
├── indexer.rs           # 构建搜索索引
├── storage.rs           # ~/.claude-memo/ 数据管理
├── search.rs            # 全文搜索
├── exporter.rs          # HTML 导出 (预留)
└── cli.rs               # CLI 界面

tests/
├── unit/
│   ├── parser_test.rs
│   ├── search_test.rs
│   └── storage_test.rs
└── integration/
    └── cli_test.rs
```

**Structure Decision**: Single Rust binary project，模块按功能划分，测试在 `tests/` 目录。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| SQLite FTS5 | 内置全文索引，性能好 | 简单遍历 O(n) 复杂度，无法满足 5 秒搜索要求 |
| TOML 收藏存储 | 手动可编辑，结构清晰 | JSON 虽通用但 TOML 符合项目技术栈 |

## Phase 0: Research

### Decisions

- **CLI 框架**: clap 4.4 - Rust 生态系统最成熟的 CLI 库
- **搜索索引**: SQLite FTS5 - 内置、性能好、零额外依赖
- **收藏存储**: TOML 文件 - 手动可编辑、简单
- **测试覆盖**: tarpaulin - Rust 官方推荐的覆盖率工具

## Phase 1: Design Artifacts

Generated artifacts:
- `research.md` - 技术决策文档
- `data-model.md` - 数据模型定义
- `quickstart.md` - 快速开始指南
- `contracts/cli-commands.md` - CLI 命令规范
