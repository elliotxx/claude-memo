# 开发者验证指南

本文档面向 claude-memo 项目的开发者，介绍如何验证代码质量和功能正确性。

## 快速开始

```bash
# 克隆项目
git clone https://github.com/elliotxx/claude-memo.git
cd claude-memo

# 安装依赖并构建
cargo build

# 运行所有验证（推荐）
make all
```

## 验证命令速查

| 命令 | 说明 |
|------|------|
| `make` / `make help` | 显示帮助信息 |
| `make test` | 运行所有测试 |
| `make test-unit` | 运行单元测试 |
| `make test-integration` | 运行集成测试 |
| `make check` | 编译检查 |
| `make clippy` | 代码质量检查 |
| `make fmt-check` | 格式检查 |
| `make all` | 完整验证（推荐提交前执行） |
| `make coverage` | 生成覆盖率报告 |

## 详细说明

### 1. 测试验证

#### 运行所有测试

```bash
make test
```

或手动执行：

```bash
cargo test --all
```

**预期输出**: 31 个测试全部通过

```
running 19 tests
...
test result: ok. 19 passed; 0 failed

running 12 tests
...
test result: ok. 12 passed; 0 failed
```

#### 运行单元测试

```bash
make test-unit
```

验证 parser、storage、indexer、search 模块的核心逻辑。

#### 运行集成测试

```bash
make test-integration
```

验证 CLI 命令的实际行为，包括：
- 解析历史文件
- 搜索功能
- 收藏管理
- 错误处理

### 2. 代码质量验证

#### 编译检查

```bash
make check
```

确保代码能够编译通过，无语法错误。

#### 代码质量检查 (Clippy)

```bash
make clippy
```

检查代码中的常见问题：
- 未使用的变量/导入
- 低效的模式
- 不安全的代码模式
- 代码风格建议

#### 格式检查

```bash
make fmt-check
```

检查代码是否符合 Rust 格式化规范。

如果格式有问题，修复方式：

```bash
make fmt  # 自动格式化
```

### 3. 完整验证

在提交代码前，运行完整验证：

```bash
make all
```

这将依次执行：
1. `cargo check` - 编译检查
2. `cargo fmt --check` - 格式检查
3. `cargo clippy` - 代码质量检查
4. `cargo test --all` - 运行所有测试

### 4. 覆盖率报告

生成测试覆盖率报告：

```bash
make coverage
```

需要先安装 cargo-tarpaulin：

```bash
cargo install cargo-tarpaulin
```

报告生成位置: `target/tarpaulin-report.html`

## CI/CD 验证

项目使用 GitHub Actions 进行持续集成。验证流程：

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Check
        run: make check
      - name: Clippy
        run: make clippy
      - name: Format
        run: make fmt-check
      - name: Test
        run: make test
```

## 本地开发工作流

### 日常开发

```bash
# 1. 开始新功能/修复
git checkout -b feature/xxx

# 2. 编写代码

# 3. 运行验证（频繁执行）
make check      # 快速检查编译
make fmt        # 保持格式正确

# 4. 编写测试
# 在对应的模块 tests/ 目录添加测试

# 5. 运行测试
make test-unit  # 单元测试
make test       # 所有测试
```

### 提交前检查

```bash
# 最终验证
make all

# 必须使用 /gitacp CALUDE SKILL 提交代码,示例：/gitacp
```

## 验证失败排查

### cargo check 失败

**常见原因**:
- 语法错误
- 类型不匹配
- 缺少依赖

**排查**:
```bash
cargo check 2>&1 | head -50
```

### clippy 失败

**常见原因**:
- 未使用的变量
- 低效代码模式
- 错误的借用检查

**排查**:
```bash
cargo clippy 2>&1 | grep -A 5 "error:"
```

### 测试失败

**常见原因**:
- 断言失败
- 异步代码问题
- 环境依赖

**排查**:
```bash
cargo test --all -- --nocapture
```

### fmt 检查失败

**解决**:
```bash
make fmt
```

## 测试覆盖要求

根据项目 Constitution 要求：
- Parser 模块: >= 80%
- Search 模块: >= 80%
- Storage 模块: >= 80%

查看覆盖率：

```bash
make coverage
# 打开 target/tarpaulin-report.html 查看详情
```

## 性能基准

搜索性能要求：
- 10,000 条记录 < 5 秒

验证方式：

```bash
# 生成测试数据（10k 条）
# 运行搜索测试
cargo test --lib --release -- --nocapture test_search_performance
```

## 相关链接

- [Rust 官方测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [cargo clippy](https://github.com/rust-lang/rust-clippy)
- [cargo tarpaulin](https://github.com/xd009642/tarpaulin)
