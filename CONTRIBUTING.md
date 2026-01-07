# 贡献指南

感谢你对 Build-Your-Own-Utils 项目的关注！我们欢迎各种形式的贡献。

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)

## 行为准则

参与本项目即表示你同意遵守我们的 [行为准则](CODE_OF_CONDUCT.md)。请确保你的行为符合社区标准。

## 如何贡献

### 报告 Bug

1. 在 [Issues](https://github.com/user/Build-Your-Own-Utils/issues) 中搜索是否已有相同问题
2. 如果没有，创建新 Issue 并使用 Bug Report 模板
3. 提供详细的复现步骤和环境信息

### 提出新功能

1. 在 Issues 中搜索是否已有相同建议
2. 创建新 Issue 并使用 Feature Request 模板
3. 描述功能的使用场景和预期行为

### 提交代码

1. Fork 本仓库
2. 创建功能分支
3. 编写代码和测试
4. 提交 Pull Request

## 开发流程

### 环境准备

```bash
# 克隆你的 fork
git clone https://github.com/YOUR_USERNAME/Build-Your-Own-Utils.git
cd Build-Your-Own-Utils

# 添加上游仓库
git remote add upstream https://github.com/user/Build-Your-Own-Utils.git

# 安装依赖
# Rust: https://rustup.rs/
# Go: https://golang.org/dl/
```

### 创建分支

```bash
# 同步上游代码
git fetch upstream
git checkout main
git merge upstream/main

# 创建功能分支
git checkout -b feature/your-feature-name
```

### 本地测试

```bash
# Rust 项目
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all

# Go 项目
gofmt -w .
go vet ./...
go test ./...
```

## 代码规范

### Rust

- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行静态分析
- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 公共 API 必须有文档注释

```rust
/// 将 CRLF 换行符转换为 LF
///
/// # Arguments
///
/// * `input` - 输入字符串
///
/// # Returns
///
/// 转换后的字符串
pub fn convert_crlf_to_lf(input: &str) -> String {
    input.replace("\r\n", "\n")
}
```

### Go

- 使用 `gofmt` 格式化代码
- 使用 `go vet` 进行静态分析
- 遵循 [Effective Go](https://golang.org/doc/effective_go)
- 导出的函数必须有文档注释

```go
// ConvertCRLFToLF converts Windows line endings to Unix line endings.
func ConvertCRLFToLF(input string) string {
    return strings.ReplaceAll(input, "\r\n", "\n")
}
```

### 通用规范

- 使用 UTF-8 编码
- 使用 LF 换行符
- 文件末尾保留一个空行
- 删除行尾空白字符

## 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范。

### 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构（不是新功能或 Bug 修复）
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具相关

### Scope

- `dos2unix`: dos2unix 子项目
- `gzip`: gzip 子项目
- `htop`: htop 子项目
- `ci`: CI/CD 相关
- `docs`: 文档相关

### 示例

```
feat(dos2unix): add support for recursive directory processing

Add -r/--recursive flag to process all files in a directory tree.

Closes #123
```

```
fix(gzip): handle empty input files correctly

Previously, empty files would cause a panic. Now they are handled
gracefully with an appropriate error message.
```

## Pull Request 流程

### 提交前检查

- [ ] 代码通过所有测试
- [ ] 代码通过格式化检查
- [ ] 代码通过 lint 检查
- [ ] 更新了相关文档
- [ ] 添加了 changelog 条目（如适用）

### PR 描述

请在 PR 描述中包含：

1. 变更的目的和背景
2. 主要改动内容
3. 测试方法
4. 关联的 Issue（如有）

### 代码审查

- 所有 PR 需要至少一位维护者审查
- 请及时回复审查意见
- 必要时更新代码并推送

### 合并

- PR 通过审查后由维护者合并
- 使用 Squash and Merge 保持提交历史整洁

## 📝 Changelog

每次功能变更请在对应子项目的 `changelog/` 目录添加条目：

```markdown
# YYYY-MM-DD: 简短描述

## 变更内容

- 具体变更 1
- 具体变更 2

## 影响

描述此变更对用户的影响
```

## 🙏 感谢

感谢你花时间阅读贡献指南，期待你的贡献！
