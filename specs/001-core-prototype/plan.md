# Implementation Plan: Core Prototype

**Branch**: `001-core-prototype` | **Date**: 2026-01-29 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-core-prototype/spec.md`

## Summary

实现 claude-memo 核心原型：一个 CLI 工具，用于解析 Claude Code 的 `history.jsonl` 文件，支持全文搜索和会话收藏功能。

**技术方案**：
- CLI 框架：clap 4.4（derive 宏，子命令结构）
- 全文搜索：SQLite FTS5 内置索引（BM25 排序）
- 收藏存储：TOML 文件（`~/.claude-memo/favorites/sessions.toml`）
- 测试框架：cargo test + assert_cmd（集成测试）
- 错误处理：thiserror + anyhow

## Technical Context

**Language/Version**: Rust 2021 Edition (1.75+)
**Primary Dependencies**:
- `clap 4.4`: CLI 解析和子命令
- `rusqlite`: SQLite FTS5 全文索引
- `toml`: 收藏数据序列化
- `chrono`: 时间处理
- `thiserror`: 错误类型定义
- `anyhow`: 错误上下文
**Storage**:
- `~/.claude/history.jsonl`: 只读输入源
- `~/.claude-memo/index/sessions.db`: SQLite FTS5 索引
- `~/.claude-memo/favorites/sessions.toml`: 收藏列表
- `~/.claude-memo/config.toml`: 用户配置（待实现）
**Testing**: cargo test, assert_cmd (CLI integration), tarpaulin (coverage)
**Target Platform**: macOS/Linux (CLI tool)
**Project Type**: Single Rust binary
**Performance Goals**: 10,000 条记录搜索 < 5 秒
**Constraints**: 只读访问 ~/.claude/ 目录
**Scale/Scope**: 单用户本地工具，支持 10 万+ 记录

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Check | Status | Notes |
|-------|--------|-------|
| TDD 三步循环 | ✅ | RED→GREEN→REFACTOR 流程已定义 |
| 单元测试覆盖率 ≥ 80% | ✅ | parser、search 模块需达标 |
| 集成测试覆盖所有子命令 | ✅ | parse、search、favorite、unfavorite、favorites |
| 数据隔离 | ✅ | ~/.claude/ 只读，数据存储在 ~/.claude-memo/ |
| 测试先于实现 | ⚠️ | **技术债务**: 核心功能已先于测试实现，需补充验收测试 |
| cargo check/clippy/fmt | ✅ | 质量门禁已定义 |

**Technical Debt Note**: 由于项目采用"先实现后补测试"的路径，Constitution I (Test-First) 原则已被绕过。补救措施：
- 所有核心功能已有测试覆盖（68 tests passing）
- 新功能开发必须严格遵循 TDD 流程
- 集成测试已补充 session_id 显示验证

**Post-Design Re-check**:
- 单元测试: 37 passed ✅
- 集成测试: 31 passed ✅
- 总计: 68 tests passing ✅

## Project Structure

### Documentation (this feature)

```text
specs/001-core-prototype/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output - 技术选型已确定
├── data-model.md        # Phase 1 output - 实体模型已定义
├── quickstart.md        # Phase 1 output - 开发环境设置
├── contracts/           # Phase 1 output (not needed - CLI tool)
├── spec.md              # 功能规格
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # 库入口，导出公共 API
├── main.rs              # CLI 入口点
├── error.rs             # 错误类型定义 (thiserror)
├── parser.rs            # JSONL 解析模块
├── indexer.rs           # FTS5 索引构建
├── search.rs            # 搜索逻辑（FTS5 + BM25）
└── storage.rs           # 收藏存储（TOML）

tests/
├── cli_test.rs          # CLI 集成测试 (assert_cmd)
```

**Structure Decision**: 单项目结构，模块按功能划分（解析、索引、搜索、存储）

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| FTS5 全文索引 | 10,000+ 记录需要 <5s 搜索 | 简单遍历 O(n) 无法满足性能 |
| SQLite 而非内存 | 索引持久化，避免重复构建 | 首次加载可接受，未来支持增量 |

## Phase 0: Research Summary

**Completed** ✅

| Topic | Decision | Status |
|-------|----------|--------|
| CLI Framework | clap 4.4 | Implemented |
| Search Index | SQLite FTS5 | Implemented |
| Favorite Storage | TOML | Implemented |
| Testing | cargo test + assert_cmd | Partial |
| Error Handling | thiserror + anyhow | Implemented |

## Phase 1: Design Summary

**Completed** ✅

**Entities**:
- `SessionRecord`: display, timestamp, project, session_id
- `FavoriteSession`: session_id, favorited_at

**Data Flow**:
```
history.jsonl → parser → SessionRecord → indexer → FTS5
                                          ↓
                                     search → results
                                          ↓
storage (TOML) ← favorite ← session_id
```

**CLI Commands**:
- `parse [--json] [--limit N]`: 解析并显示记录
- `search KEYWORD [--json] [--limit N]`: 全文搜索
- `favorite SESSION_ID`: 收藏会话（add 操作）
- `unfavorite SESSION_ID`: 取消收藏（remove 操作）
- `favorites [--json]`: 列出收藏

## Next Steps

1. **补充缺失测试**（已修复）：
   - `test_search_output_includes_session_id` - 已添加 ✅
   - 测试路径修正：`tests/integration/cli_test.rs` → `tests/cli_test.rs` ✅
   - 验证所有验收标准可测试

2. **新增功能任务**（FR-011 配置存储）：
   - T049 [US5] Create Config struct in src/config.rs
   - T050 [US5] Implement load_config function
   - T051 [US5] Implement save_config function
   - T052 [US5] Integrate config into CLI

3. **性能测试任务**（SC-001/SC-002）：
   - T053 [US6] 搜索性能测试（10k 记录 <5s）
   - T054 [US6] 搜索延迟测试（100 记录 <1s）
   - T055 [US6] 收藏操作测试（<1s）
   - T056 [US6] storage 覆盖率测试（>=60%）

4. **运行质量检查**：
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **生成更新后的任务列表**：
   ```bash
   /speckit.tasks
   ```

---

**Generated**: 2026-01-29 | **Version**: 1.1 ( Updated)
