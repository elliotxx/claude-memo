# Feature Specification: Core Prototype

**Feature Branch**: `001-core-prototype`
**Created**: 2026-01-29
**Status**: Draft
**Input**: User description: "实现 claude-memo 核心原型：解析 history.jsonl、基础搜索、收藏功能和单元测试验证"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 解析会话记录 (Priority: P1)

作为用户，我希望能够读取 Claude Code 的历史记录文件，以便访问我的会话数据。

**Why this priority**: 这是所有功能的基础，没有数据解析就无法进行搜索或收藏。

**Independent Test**: 通过 CLI 命令读取 history.jsonl 文件并输出解析结果，验证数据格式正确性。

**Acceptance Scenarios**:

1. **Given** 存在 `~/.claude/history.jsonl` 文件，**When** 用户执行解析命令，**Then** 系统返回结构化的会话记录列表。
2. **Given** 解析后的会话记录，**When** 检查每条记录，**Then** 包含 display、timestamp、project、session_id 四个字段。
3. **Given** history.jsonl 文件为空，**When** 执行解析命令，**Then** 返回空列表，不报错。
4. **Given** history.jsonl 文件格式错误，**When** 执行解析命令，**Then** 返回友好的错误信息。

---

### User Story 2 - 全文搜索 (Priority: P1)

作为用户，我希望能够搜索会话记录中的文本内容，以便快速找到相关的对话历史。

**Why this priority**: 快速搜索是 claude-memo 的核心价值主张，用户需要高效找到历史记录。

**Independent Test**: 通过 CLI 命令搜索关键词，验证返回结果包含匹配关键词的记录。

**Acceptance Scenarios**:

1. **Given** 存在多条会话记录，**When** 用户搜索 "keyword"，**Then** 返回所有包含 "keyword" 的记录，按时间倒序排列（最新的在前）。
2. **Given** 搜索无结果，**When** 执行搜索命令，**Then** 返回空列表并提示无匹配结果。
3. **Given** 搜索结果包含多条记录，**When** 显示结果，**Then** 每条记录显示时间、项目路径、内容摘要和 session_id。
4. **Given** 搜索关键词为空，**When** 执行搜索命令，**Then** 返回所有记录。
5. **Given** 搜索结果包含 session_id，**When** 用户查看输出，**Then** 用户能够从输出中复制 session_id（便于收藏使用）。

**可发现性验证**：
- [ ] search 输出显示每条记录的 session_id
- [ ] session_id 格式易于复制（如用 `[]` 包裹或明确标注）
- [ ] 集成测试验证 session_id 出现在输出中

**工作流验证**：
- [ ] 用户能够从 search 输出复制 session_id
- [ ] 用户能够使用复制的 session_id 执行 `favorite` 命令
- [ ] 完整搜索→复制→收藏工作流可正常运行

---

### User Story 3 - 收藏管理 (Priority: P2)

作为用户，我希望能够收藏重要的会话记录，以便快速访问和回顾关键对话。

**Why this priority**: 收藏功能让用户可以标记重要内容，是搜索的重要补充。

**Independent Test**: 通过 CLI 命令收藏/取消收藏会话，验证收藏状态正确持久化。

**Acceptance Scenarios**:

1. **Given** 存在会话记录，**When** 用户执行收藏命令（提供 session_id），**Then** 该会话被标记为收藏。
2. **Given** 存在已收藏的会话，**When** 用户执行取消收藏命令，**Then** 该会话取消收藏状态。
3. **Given** 用户执行收藏列表命令，**When** 系统返回，**Then** 显示所有已收藏的会话。
4. **Given** 应用重启后，**When** 查询收藏状态，**Then** 收藏状态保持不变。
5. **Given** 用户从 search 输出复制了 session_id，**When** 用户执行 `favorite <session_id>`，**Then** 该会话成功被收藏。

**可发现性验证**：
- [ ] favorite 命令正确接受 session_id 作为参数
- [ ] 收藏成功/失败的反馈信息清晰明确
- [ ] favorites 列表显示收藏时间和 session_id

**工作流验证**：
- [ ] 用户能够从 search 输出复制 session_id
- [ ] 用户能够将复制的 session_id 作为参数传递给 favorite 命令
- [ ] 收藏后可在 favorites 列表中看到该会话

---

### User Story 4 - 单元测试验证 (Priority: P1)

作为开发者，我希望核心模块有单元测试覆盖，以确保代码质量和功能正确性。

**Why this priority**: 根据项目宪法，核心模块需要 80%+ 测试覆盖率，这是交付标准。

**Independent Test**: 运行 `cargo test` 命令，验证所有测试通过且覆盖率达标。

**Acceptance Scenarios**:

1. **Given** 项目代码完整，**When** 运行单元测试，**Then** 所有测试通过。
2. **Given** parser 模块代码，**When** 检查测试覆盖率，**Then** 达到 80% 以上。
3. **Given** search 模块代码，**When** 检查测试覆盖率，**Then** 达到 80% 以上。

---

### Edge Cases

- 空或损坏的 history.jsonl 文件如何处理
- 收藏不存在的 session_id 时如何反馈
- 大量记录（10万+）时的搜索性能
- 收藏数据存储失败时的错误处理

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统 MUST 能够读取 `~/.claude/history.jsonl` 文件。
- **FR-002**: 系统 MUST 正确解析 JSONL 格式的每行记录。
- **FR-003**: 系统 MUST 提取每条记录的 display、timestamp、project、session_id 字段。
- **FR-004**: 系统 MUST 支持全文搜索，匹配 display 字段内容。
- **FR-005**: 系统 MUST 支持收藏指定的 session_id。
- **FR-006**: 系统 MUST 支持取消收藏指定的 session_id。
- **FR-007**: 系统 MUST 列出所有已收藏的会话。
- **FR-008**: 系统 MUST 将收藏数据持久化到 `~/.claude-memo/` 目录。
- **FR-009**: 系统 MUST 通过 `cargo test` 运行所有单元测试并通过。
- **FR-010**: 系统 MUST 核心模块（parser、search）测试覆盖率 >= 80%。
- **FR-011**: 系统 MUST 支持通过 `~/.claude-memo/config.toml` 存储用户配置。

### Key Entities *(include if feature involves data)*

- **SessionRecord**: 代表一条会话记录，包含 display（内容）、timestamp（时间戳）、project（项目路径）、session_id（会话标识，同一会话的多条记录共享同一 session_id）。
- **FavoriteSession**: 代表收藏的会话，只需存储 session_id 和收藏时间。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户可以在 5 秒内完成对 10,000 条历史记录的全文搜索。
- **SC-002**: 搜索结果在 1 秒内返回并显示给用户。
- **SC-003**: 收藏操作在 1 秒内完成并持久化。
- **SC-004**: 所有单元测试在 30 秒内运行完成并通过。
- **SC-005**: parser 模块代码行测试覆盖率 >= 80%。
- **SC-006**: search 模块代码行测试覆盖率 >= 80%。
- **SC-007**: storage 模块代码行测试覆盖率 >= 60%（Constitution 要求）。

## Clarifications

### Session 2026-01-29

- Q: CLI 命令接口定义 → A: 使用子命令结构，如 `claude-memo search "keyword"`、`claude-memo favorite add <id>`
- Q: 搜索实现方式 → A: 使用 SQLite FTS5 内置全文索引，平衡性能和复杂度
- Q: 搜索结果输出格式 → A: 纯文本格式：`时间 项目 > 内容 [session_id]`，每行一条，session_id 用方括号包裹便于复制
- Q: 收藏数据存储格式 → A: TOML 文件：~/.claude-memo/favorites/sessions.toml，手动可编辑
- Q: 用户配置存储方式 → A: Local TOML config file (`~/.claude-memo/config.toml`)
- Q: session_id 唯一性规则 → A: session_id groups multiple records (1:N mapping)，一次会话产生多条记录共享同一 session_id
- Q: 术语统一 → A: 使用 session_id (snake_case)，与 Rust 惯例和代码实现保持一致
- Q: 搜索结果排序 → A: timestamp descending (newest first)，用户优先看到最近的对话
- Q: 错误输出格式 → A: stderr with exit code，符合 Unix 惯例，便于程序化处理
