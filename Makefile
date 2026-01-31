.PHONY: help test test-unit test-integration check check-fix lint clippy fmt fmt-check build build-release coverage clean all run run-test

# 默认目标：显示帮助
.DEFAULT_GOAL := help

# 彩色输出
GREEN  := \033[0;32m
YELLOW := \033[0;33m
BLUE   := \033[0;34m
CYAN   := \033[0;36m
RESET  := \033[0m

# 打印带颜色的信息
info = @printf "$(GREEN)📦 %s$(RESET)\n" "$1"
done = @printf "$(BLUE)✅ %s$(RESET)\n" "$1"
run-echo = @printf "$(CYAN)▶️  %s$(RESET)\n" "$1"

# Mock 数据目录
MOCK_DIR := tests/mock
MOCK_HISTORY := $(MOCK_DIR)/history.jsonl

## ============================================================================
## 开发常用命令
## ============================================================================

help: ## 显示帮助信息
	@echo ""
	@echo "claude-memo 开发命令"
	@echo "===================="
	@echo ""
	@echo "验证:"
	@printf "  $(GREEN)make test$(RESET)           运行所有测试（单元+集成）\n"
	@printf "  $(GREEN)make test-unit$(RESET)      运行单元测试\n"
	@printf "  $(GREEN)make test-integration$(RESET) 运行集成测试\n"
	@printf "  $(GREEN)make check$(RESET)          代码编译检查\n"
	@printf "  $(GREEN)make clippy$(RESET)         代码质量检查\n"
	@printf "  $(GREEN)make fmt-check$(RESET)      代码格式检查\n"
	@printf "  $(GREEN)make all$(RESET)            运行所有验证\n"
	@printf "  $(GREEN)make check$(RESET)          运行所有检查 (fmt + clippy + test)\n"
	@printf "  $(GREEN)make check-fix$(RESET)      修复所有检查问题并运行测试\n"
	@echo ""
	@echo "构建:"
	@printf "  $(GREEN)make build$(RESET)          开发构建\n"
	@printf "  $(GREEN)make build-release$(RESET)  发布构建\n"
	@printf "  $(GREEN)make clean$(RESET)          清理构建产物\n"
	@echo ""
	@printf "  $(GREEN)make coverage$(RESET)       生成测试覆盖率报告\n"
	@echo ""
	@echo "运行:"
	@printf "  $(GREEN)make run$(RESET)            运行（加载 ~/.claude/history.jsonl）\n"
	@printf "  $(GREEN)make run-test$(RESET)       运行测试（加载 tests/mock/history.jsonl）\n"
	@echo ""
	@printf "  $(CYAN)示例: make run search \"model\"$(RESET)\n"
	@printf "  $(CYAN)示例: make run-test parse --limit 3$(RESET)\n"
	@echo ""

## ============================================================================
## 测试命令
## ============================================================================

test: ## 运行所有测试（单元测试 + 集成测试）
	$(info 运行所有测试...)
	cargo test --all
	$(done "所有测试通过")

test-unit: ## 运行单元测试
	$(info 运行单元测试...)
	cargo test --lib
	$(done "单元测试通过")

test-integration: ## 运行集成测试
	$(info 运行集成测试...)
	cargo test --test cli_test
	$(done "集成测试通过")

## ============================================================================
## 代码质量命令
## ============================================================================

check: lint test ## 运行所有检查 (fmt + clippy + test)
	$(done "所有检查通过")

check-fix: fmt-fix ## 修复所有检查问题并运行测试
	$(info 运行 clippy fix...)
	@cargo clippy --all-features --fix --allow-staged --allow-dirty 2>/dev/null || true
	$(info 运行测试...)
	cargo test --all-features --verbose
	$(done "所有检查已修复")

lint: fmt clippy ## 代码检查 (fmt + clippy)

clippy: ## 代码质量检查
	$(info 运行 cargo clippy...)
	cargo clippy -- -D warnings
	$(done "代码质量检查通过")

fmt: ## 代码格式化
	$(info 格式化代码...)
	cargo fmt
	$(done "代码格式化完成")

fmt-check: ## 代码格式检查
	$(info 检查代码格式...)
	@cargo fmt --check -- --color=never
	@$(done "代码格式正确")

## ============================================================================
## 构建命令
## ============================================================================

build: ## 开发构建
	$(info 开发构建中...)
	cargo build
	$(done "开发构建完成")

build-release: ## 发布构建
	$(info 发布构建中...)
	cargo build --release
	$(done "发布构建完成")

clean: ## 清理构建产物
	$(info 清理构建产物...)
	cargo clean
	$(done "清理完成")

## ============================================================================
## 覆盖率命令
## ============================================================================

coverage: ## 生成测试覆盖率报告
	$(info 生成测试覆盖率报告...)
	@if ! command -v cargo-tarpaulin > /dev/null 2>&1; then \
		$(warn "未安装 cargo-tarpaulin，正在安装..."); \
		cargo install cargo-tarpaulin; \
	fi
	cargo tarpaulin --out Html
	$(done "覆盖率报告已生成：target/tarpaulin-report.html")

## ============================================================================
## 运行命令
## ============================================================================

# 确保 mock 数据存在
$(MOCK_HISTORY):
	@mkdir -p $(MOCK_DIR)
	@echo '{"display":"/model ","pastedContents":{},"timestamp":1766567616338,"project":"/Users/elliotxx","sessionId":"mock-001"}' > $@
	@echo '{"display":"/search test query","pastedContents":{},"timestamp":1766567617000,"project":"/Users/elliotxx/project","sessionId":"mock-002"}' >> $@
	@echo '{"display":"/another command","pastedContents":{},"timestamp":1766567618000,"project":"/Users/elliotxx/other","sessionId":"mock-003"}' >> $@
	$(done "Mock 数据已生成")

run: ## 运行（加载 ~/.claude/history.jsonl）
	$(run-echo "运行 claude-memo...")
	@sh -c 'cargo run -- $$*' sh $(filter-out run,$(MAKECMDGOALS))

run-test: $(MOCK_HISTORY) ## 运行测试（加载 tests/mock/history.jsonl）
	$(run-echo "运行 claude-memo（测试数据）...")
	@sh -c 'CLAUDE_HISTORY=$(MOCK_HISTORY) cargo run -- $$*' sh $(filter-out run-test,$(MAKECMDGOALS))

## ============================================================================
## 完整验证
## ============================================================================

all: check ## 运行所有验证
	$(done "所有验证通过，可以提交代码")
