# Tasks: Core Prototype

**Input**: Design documents from `/specs/001-core-prototype/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are included for this feature to meet the Constitution's 80%+ coverage requirement.

**TDD Workflow (Constitution I)**: For ALL user stories, tests MUST be written and FAIL before implementation.
1. Write failing test
2. Get approval on test approach
3. Implement to make test pass
4. Refactor

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan in src/
- [ ] T002 Configure Cargo.toml dependencies (clap, rusqlite, toml, chrono, anyhow, thiserror)
- [ ] T003 Configure rust-toolchain.toml for Rust 2024 Edition
- [ ] T004 [P] Create .gitignore with ~/.claude-memo/ exclusion

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Create error types module in src/error.rs using thiserror
- [ ] T006 [P] Implement Result type alias with anyhow::Error
- [ ] T007 Create SessionRecord struct in src/parser.rs (display, timestamp, project, session_id)
- [ ] T008 Create FavoriteSession struct in src/storage.rs (session_id, favorited_at)
- [ ] T009 [P] Create data directory ~/.claude-memo/ if not exists

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 解析会话记录 (Priority: P1) 🎯 MVP

**Goal**: 用户能够读取 Claude Code 的历史记录文件

**Independent Test**: 通过 `claude-memo parse` 命令读取 history.jsonl 并输出解析结果

### Tests for User Story 1

> **TDD: Write tests FIRST, ensure they FAIL before implementation**

- [ ] T010 [P] [US1] Unit test for valid JSONL line parsing in tests/unit/parser_test.rs
- [ ] T011 [P] [US1] Unit test for empty file handling in tests/unit/parser_test.rs
- [ ] T012 [P] [US1] Unit test for malformed JSON handling in tests/unit/parser_test.rs
- [ ] T013 [P] [US1] Unit test for missing history.jsonl file (error code 3)

### Implementation for User Story 1

- [ ] T014 [US1] Implement parse_history_file function in src/parser.rs
- [ ] T015 [US1] Implement parse_line function in src/parser.rs (returns SessionRecord)
- [ ] T016 [US1] Create parse CLI subcommand in src/cli.rs for `claude-memo parse`

**Checkpoint**: User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - 全文搜索 (Priority: P1)

**Goal**: 用户能够搜索会话记录中的文本内容

**Independent Test**: 通过 `claude-memo search "keyword"` 验证返回匹配结果

**Note**: US2 uses same data format as US1, but works independently

### Tests for User Story 2

> **TDD: Write tests FIRST, ensure they FAIL before implementation**

- [ ] T017 [P] [US2] Unit test for FTS5 index creation in tests/unit/search_test.rs
- [ ] T018 [P] [US2] Unit test for keyword search matching in tests/unit/search_test.rs
- [ ] T019 [P] [US2] Unit test for empty search results in tests/unit/search_test.rs
- [ ] T020 [P] [US2] Unit test for limit parameter in tests/unit/search_test.rs
- [ ] T021 [P] [US2] Unit test for search performance with 10k records (<5 seconds)

### Implementation for User Story 2

- [ ] T022 [US2] Create Indexer struct in src/indexer.rs
- [ ] T023 [US2] Implement build_index function in src/indexer.rs (FTS5 virtual table)
- [ ] T024 [US2] Create Search struct in src/search.rs
- [ ] T025 [US2] Implement search function in src/search.rs (FTS5 query)
- [ ] T026 [US2] Create search CLI subcommand in src/cli.rs for `claude-memo search`

**Checkpoint**: User Story 2 should be fully functional and testable independently

---

## Phase 5: User Story 3 - 收藏管理 (Priority: P2)

**Goal**: 用户能够收藏重要的会话记录

**Independent Test**: 通过 `claude-memo favorite add/remove/list` 验证收藏状态

### Tests for User Story 3

> **TDD: Write tests FIRST, ensure they FAIL before implementation**

- [ ] T027 [P] [US3] Unit test for adding favorite in tests/unit/storage_test.rs
- [ ] T028 [P] [US3] Unit test for removing favorite in tests/unit/storage_test.rs
- [ ] T029 [P] [US3] Unit test for listing favorites in tests/unit/storage_test.rs
- [ ] T030 [P] [US3] Unit test for persisting favorites to TOML in tests/unit/storage_test.rs
- [ ] T031 [P] [US3] Unit test for adding non-existent sessionId (error handling)

### Implementation for User Story 3

- [ ] T032 [US3] Create Storage struct in src/storage.rs
- [ ] T033 [US3] Implement add_favorite function in src/storage.rs (TOML write)
- [ ] T034 [US3] Implement remove_favorite function in src/storage.rs (TOML update)
- [ ] T035 [US3] Implement list_favorites function in src/storage.rs (TOML read)
- [ ] T036 [US3] Create favorite CLI subcommand in src/cli.rs for `claude-memo favorite add/remove/list`

**Checkpoint**: User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - 单元测试验证 (Priority: P1)

**Goal**: 核心模块达到 80%+ 测试覆盖率

**Independent Test**: 运行 `cargo test` 和 `cargo tarpaulin` 验证覆盖率

### Integration Tests

- [ ] T037 [US4] Create CLI integration test in tests/integration/cli_test.rs
- [ ] T038 [US4] Run cargo test to verify all tests pass
- [ ] T039 [US4] Run cargo tarpaulin to verify parser coverage >= 80%
- [ ] T040 [US4] Run cargo tarpaulin to verify search coverage >= 80%
- [ ] T041 [US4] Run cargo tarpaulin to verify storage coverage >= 80%

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T042 [P] Add /// docs to SessionRecord, FavoriteSession, Search, Storage structs
- [ ] T043 [P] Add /// docs to parse_history_file, search, add_favorite functions
- [ ] T044 Run cargo check in src/ and ensure no errors
- [ ] T045 Run cargo clippy in src/ and fix all warnings
- [ ] T046 Run cargo fmt in src/ and tests/ to ensure consistent formatting
- [ ] T047 [P] Update README.md with CLI usage examples
- [ ] T048 Create integration test for end-to-end search flow

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P1)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P1)**: Must start after User Stories 1, 2, 3 complete (needs code to test)

### Within Each User Story

- Tests (TDD) MUST be written and FAIL before implementation
- Models/structs before services
- Services before CLI integration
- Core implementation before CLI
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel
- All tests for a user story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Demo MVP

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo (MVP!)
3. Add User Story 2 → Test independently → Demo
4. Add User Story 3 → Test independently → Demo
5. Add User Story 4 → Coverage check → Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for valid JSONL line parsing in tests/unit/parser_test.rs"
Task: "Unit test for empty file handling in tests/unit/parser_test.rs"
Task: "Unit test for malformed JSON handling in tests/unit/parser_test.rs"

# Launch all implementation for User Story 1 together:
Task: "Implement parse_history_file function in src/parser.rs"
Task: "Implement parse_line function in src/parser.rs"
Task: "Create parse CLI subcommand in src/cli.rs"
```

---

## Summary

| Metric | Value |
|--------|-------|
| Total Tasks | 48 |
| Setup Phase | 4 |
| Foundational Phase | 5 |
| User Story 1 | 7 (4 tests + 3 impl) |
| User Story 2 | 7 (5 tests + 2 impl) |
| User Story 3 | 7 (5 tests + 3 impl) |
| User Story 4 | 5 |
| Polish Phase | 7 |
| Parallelizable Tasks | 25 (marked with [P]) |

---

## Notes

- **[P] tasks** = different files, no dependencies
- **[Story] label** maps task to specific user story for traceability
- **TDD required** for all user stories: tests must fail before implementation
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD approach)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- **Constitution I (Test-First)**: All user story tests must be written FIRST
- **Constitution IV (Immutable Source)**: Only read ~/.claude/, never write
