# Feature Specification: Rename Favorite to Mark

**Feature Branch**: `002-rename-favorite-mark`
**Created**: 2026-01-29
**Status**: Draft
**Input**: User description: "将 收藏功能的 favorite 全部改成 mark"

## Clarifications

### Session 2026-01-30

- Q: Independent Test 中命令格式应该是 `mark/add/remove/list` 还是 `mark/unmark/marks`？ → A: 修正为 `mark/unmark/marks`，与 Acceptance Scenarios 保持一致
- Q: 是否需要保留旧命令作为别名或显示废弃提示？ → A: 直接删除旧命令，不保留废弃兼容

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rename CLI Commands (Priority: P1)

作为用户，我希望使用 `mark`/`unmark`/`marks` 命令来管理收藏，以便更简洁地访问会话记录。

**Why this priority**: 这是核心功能的简单重命名，不影响现有数据，但会改变用户交互方式。

**Independent Test**: 通过 CLI 命令 `claude-memo mark/unmark/marks` 验证命令正常工作。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-memo mark <session-id>`，**Then** 系统将会话添加到收藏。
2. **Given** 用户运行 `claude-memo unmark <session-id>`，**Then** 系统从收藏中移除会话。
3. **Given** 用户运行 `claude-memo marks`，**Then** 系统列出所有收藏的会话。
4. **Given** 用户运行 `claude-memo --help`，**Then** 帮助信息显示新的命令名称。

---

### User Story 2 - Backward Compatibility (Priority: P2)

作为用户，我希望系统直接移除旧的收藏命令，以便保持 CLI 简洁。

**Why this priority**: 简化代码维护，无需维护废弃命令。

**Independent Test**: 验证旧的 `favorite` 命令不再存在。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-memo favorite <id>`，**Then** 系统显示"未找到命令"错误。
2. **Given** 用户运行 `claude-memo favorites`，**Then** 系统显示"未找到命令"错误。
3. **Given** 用户运行 `claude-memo unfavorite <id>`，**Then** 系统显示"未找到命令"错误。

---

### User Story 3 - Update Documentation (Priority: P3)

作为文档维护者，我希望更新所有文档以反映新命令名称。

**Why this priority**: 确保文档与代码保持一致。

**Independent Test**: 检查 README 和帮助信息是否使用新命令名称。

**Acceptance Scenarios**:

1. **Given** 用户查看 README.md，**Then** 文档显示 `mark`/`unmark`/`marks` 命令。
2. **Given** 用户运行 `claude-memo <command> --help`，**Then** 帮助信息使用新术语。

---

### Edge Cases

- 现有收藏数据是否需要迁移？不需要，TOML 存储的文件名不变
- 环境变量和配置文件是否需要更新？不需要
- 是否保留旧的命令作为别名？不保留，直接删除旧命令

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统 MUST 支持 `mark` 子命令添加收藏。
- **FR-002**: 系统 MUST 支持 `unmark` 子命令移除收藏。
- **FR-003**: 系统 MUST 支持 `marks` 子命令列出收藏。

### Key Entities *(include if feature involves data)*

无新增数据实体。现有 `FavoriteSession` 和 `Storage` 保持不变，仅修改 CLI 命令名称。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 所有新命令 `mark`/`unmark`/`marks` 正常工作（功能等价于原 favorite/unfavorite/favorites）。
- **SC-002**: 旧的 `favorite`/`unfavorite`/`favorites` 命令被移除，不再可用。
- **SC-003**: 现有收藏数据无需迁移即可继续使用。
- **SC-004**: 所有现有测试通过，更新测试覆盖新命令名称。
