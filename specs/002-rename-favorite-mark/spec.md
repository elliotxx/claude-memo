# Feature Specification: Rename Favorite to Mark

**Feature Branch**: `002-rename-favorite-mark`
**Created**: 2026-01-29
**Status**: Draft
**Input**: User description: "将 收藏功能的 favorite 全部改成 mark"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rename CLI Commands (Priority: P1)

作为用户，我希望使用 `mark`/`unmark`/`marks` 命令来管理收藏，以便更简洁地访问会话记录。

**Why this priority**: 这是核心功能的简单重命名，不影响现有数据，但会改变用户交互方式。

**Independent Test**: 通过 CLI 命令 `claude-memo mark/add/remove/list` 验证命令正常工作。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-memo mark <session-id>`，**Then** 系统将会话添加到收藏。
2. **Given** 用户运行 `claude-memo unmark <session-id>`，**Then** 系统从收藏中移除会话。
3. **Given** 用户运行 `claude-memo marks`，**Then** 系统列出所有收藏的会话。
4. **Given** 用户运行 `claude-memo --help`，**Then** 帮助信息显示新的命令名称。
5. **Given** 用户运行旧的 `favorite` 命令，**Then** 系统提示使用新的 `mark` 命令。

---

### User Story 2 - Backward Compatibility (Priority: P2)

作为从旧版本升级的用户，我希望系统提示旧命令已废弃，而不是直接失败。

**Why this priority**: 提升升级用户体验，给用户缓冲期适应新命令。

**Independent Test**: 运行旧命令并验证系统返回友好的废弃提示。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-memo favorite <id>`，**Then** 系统显示"命令已废弃，请使用 mark"提示。
2. **Given** 用户运行 `claude-memo favorites`，**Then** 系统显示"命令已废弃，请使用 marks"提示。
3. **Given** 用户运行 `claude-memo unfavorite <id>`，**Then** 系统显示"命令已废弃，请使用 unmark"提示。

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
- 是否保留旧的命令作为别名？采用废弃提示策略

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统 MUST 支持 `mark` 子命令添加收藏。
- **FR-002**: 系统 MUST 支持 `unmark` 子命令移除收藏。
- **FR-003**: 系统 MUST 支持 `marks` 子命令列出收藏。
- **FR-004**: 系统 MUST 对旧的 `favorite`/`unfavorite`/`favorites` 命令显示废弃提示。
- **FR-005**: 废弃提示 MUST 包含新命令的使用说明。

### Key Entities *(include if feature involves data)*

无新增数据实体。现有 `FavoriteSession` 和 `Storage` 保持不变，仅修改 CLI 命令名称。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 所有新命令 `mark`/`unmark`/`marks` 正常工作（功能等价于原 favorite/unfavorite/favorites）。
- **SC-002**: 旧命令显示清晰的废弃提示，不产生错误。
- **SC-003**: 现有收藏数据无需迁移即可继续使用。
- **SC-004**: 所有现有测试通过，更新测试覆盖新命令名称。
