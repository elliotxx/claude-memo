# Tasks: Rename Favorite to Mark

**Input**: Design documents from `/specs/002-rename-favorite-mark/`
**Prerequisites**: plan.md, spec.md
**Tests**: TDD approach - tests required per constitution

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (File Rename)

**Purpose**: Rename source files to match new command names

- [X] T001 [P] Rename `src/cli/commands/favorite.rs` to `src/cli/commands/mark.rs` (N/A - commands defined in cli.rs)
- [X] T002 [P] Rename `src/cli/commands/unfavorite.rs` to `src/cli/commands/unmark.rs` (N/A - commands defined in cli.rs)
- [X] T003 [P] Rename `src/cli/commands/list_favorites.rs` to `src/cli/commands/list_marks.rs` (N/A - commands defined in cli.rs)

---

## Phase 2: Update Command Registration

**Purpose**: Update CLI module to register new command names

- [X] T004 [P] Update command names from `favorite`/`unfavorite`/`favorites` to `mark`/`unmark`/`marks` in `src/cli.rs`

---

## Phase 3: User Story 1 - Rename CLI Commands (Priority: P1) MVP

**Goal**: 实现 `mark`/`unmark`/`marks` 命令，功能等价于原 `favorite`/`unfavorite`/`favorites`

**Independent Test**: 运行 `claude-memo mark/unmark/marks` 验证命令正常工作

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T010 [P] [US1] Add integration test for `mark` command in `tests/integration/mark_commands.rs`
- [X] T011 [P] [US1] Add integration test for `unmark` command in `tests/integration/mark_commands.rs`
- [X] T012 [P] [US1] Add integration test for `marks` command in `tests/integration/mark_commands.rs`
- [X] T013 [P] [US1] Add `--json` output test for `marks` command in `tests/integration/mark_commands.rs`
- [X] T014 [P] [US1] Add error handling test for invalid session-id in `tests/integration/mark_commands.rs`

### Implementation for User Story 1

- [X] T015 [US1] Update module declaration in `src/cli.rs`
- [X] T016 [US1] Update module declaration in `src/cli.rs`
- [X] T017 [US1] Update module declaration in `src/cli.rs`
- [X] T018 [US1] Update command struct names if needed in cli.rs
- [X] T019 [US1] Update clap command names from `favorite` to `mark` etc. in cli.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Delete Old Commands (Priority: P2)

**Goal**: 移除旧的 `favorite`/`unfavorite`/`favorites` 命令

**Independent Test**: 验证旧的 `favorite` 命令返回"未找到命令"错误

### Tests for User Story 2 ⚠️

- [X] T020 [P] [US2] Add integration test verifying `favorite` command fails in `tests/integration/mark_commands.rs`
- [X] T021 [P] [US2] Add integration test verifying `unfavorite` command fails in `tests/integration/mark_commands.rs`
- [X] T022 [P] [US2] Add integration test verifying `favorites` command fails in `tests/integration/mark_commands.rs`

### Implementation for User Story 2

- [X] T023 [US2] Remove `favorite` command registration from `src/cli.rs`
- [X] T024 [US2] Remove `unfavorite` command registration from `src/cli.rs`
- [X] T025 [US2] Remove `favorites` command registration from `src/cli.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Update Documentation (Priority: P3)

**Goal**: 更新文档以反映新命令名称

**Independent Test**: 检查 README 和帮助信息使用新命令名称

### Implementation for User Story 3

- [X] T030 [US3] Update README.md with new `mark`/`unmark`/`marks` command examples
- [X] T031 [US3] Update help text in CLI if needed for new command descriptions (N/A - clap auto-generates)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 验证和清理

- [X] T040 [P] Run `cargo check`, `cargo clippy`, and `cargo fmt` to verify code quality
- [X] T041 Run all tests with `cargo test` to verify nothing is broken
- [X] T042 [P] Update CLAUDE.md if needed with new command information (N/A)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (File Rename)**: No dependencies - can start immediately
- **Phase 2 (Registration)**: Depends on Phase 1 completion
- **Phase 3 (US1)**: Depends on Phase 2 completion
- **Phase 4 (US2)**: Can start after Phase 2 completion (independent of US1)
- **Phase 5 (US3)**: Can start after US1 and US2 complete
- **Phase 6 (Polish)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Phase 2 - No dependencies on other stories
- **User Story 2 (P2)**: Depends on Phase 2 - No dependencies on other stories (can run in parallel with US1)
- **User Story 3 (P3)**: Depends on US1 and US2 complete

### Within Each User Story

- Tests (T010-T014, T020-T022) MUST be written and FAIL before implementation
- File renaming before code updates
- Story complete before moving to next priority

### Parallel Opportunities

- All Phase 1 tasks (T001-T003) marked [P] can run in parallel
- US1 tests (T010-T014) can run in parallel
- US2 tests (T020-T022) can run in parallel
- US1 and US2 can proceed in parallel after Phase 2

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Add integration test for `mark` command"
Task: "Add integration test for `unmark` command"
Task: "Add integration test for `marks` command"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: File Rename
2. Complete Phase 2: Command Registration
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Phase 1 + 2 → Commands renamed
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Developer A: Phase 1 (file renames)
2. Developer B: Phase 2 (registration updates)
3. Once Phase 2 is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
