.PHONY: help test test-unit test-integration check clippy fmt fmt-check build build-release coverage clean all

# 默认目标：显示帮助
.DEFAULT_GOAL := help

# 彩色输出
GREEN  := \033[0;32m
YELLOW := \033[0;33m
BLUE   := \033[0;34m
RESET  := \033[0m

# 打印带颜色的信息
info = @printf "$(GREEN)📦 %s$(RESET)\n" "$1"
warn = @printf "$(YELLOW)⚠️  %s$(RESET)\n" "$1"
done = @printf "$(BLUE)✅ %s$(RESET)\n" "$1"

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
	@printf "  $(GREEN)make all$(RESET)            运行所有验证（check + clippy + fmt + test）\n"
	@echo ""
	@echo "构建:"
	@printf "  $(GREEN)make build$(RESET)          开发构建\n"
	@printf "  $(GREEN)make build-release$(RESET)  发布构建\n"
	@printf "  $(GREEN)make clean$(RESET)          清理构建产物\n"
	@echo ""
	@printf "  $(GREEN)make coverage$(RESET)       生成测试覆盖率报告\n"
	@echo ""
	@echo "使用:"
	@printf "  $(GREEN)make help$(RESET)           显示此帮助信息\n"
	@echo ""

## ============================================================================
## 测试命令
## ============================================================================

test: ## 运行所有测试（单元测试 + 集成测试）
	$(info 运行所有测试...)
	cargo test --all
	$(done "所有测试通过 ✅")

test-unit: ## 运行单元测试
	$(info 运行单元测试...)
	cargo test --lib
	$(done "单元测试通过 ✅")

test-integration: ## 运行集成测试
	$(info 运行集成测试...)
	cargo test --test cli_test
	$(done "集成测试通过 ✅")

## ============================================================================
## 代码质量命令
## ============================================================================

check: ## 代码编译检查
	$(info 运行 cargo check...)
	cargo check
	$(done "编译检查通过 ✅")

clippy: ## 代码质量检查
	$(info 运行 cargo clippy...)
	cargo clippy -- -D warnings
	$(done "代码质量检查通过 ✅")

fmt: ## 代码格式化
	$(info 格式化代码...)
	cargo fmt
	$(done "代码格式化完成 ✅")

fmt-check: ## 代码格式检查
	$(info 检查代码格式...)
	@cargo fmt --check -- --color=never
	@$(done "代码格式正确 ✅")

## ============================================================================
## 构建命令
## ============================================================================

build: ## 开发构建
	$(info 开发构建中...)
	cargo build
	$(done "开发构建完成 ✅")

build-release: ## 发布构建
	$(info 发布构建中...)
	cargo build --release
	$(done "发布构建完成 ✅")

clean: ## 清理构建产物
	$(info 清理构建产物...)
	cargo clean
	$(done "清理完成 ✅")

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
## 完整验证
## ============================================================================

all: check fmt clippy test ## 运行所有验证（推荐在提交前执行）
	$(info)
	$(done "所有验证通过，可以提交代码 🎉")
