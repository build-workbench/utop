# Design Document: Project Standardization

## Overview

本设计文档描述如何将 Build-Your-Own-Utils 项目整理完善为一个规范的开源项目。设计遵循开源社区最佳实践，参考 GitHub 官方推荐的项目结构和文档标准。

## Architecture

### 项目结构设计

```
Build-Your-Own-Utils/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   ├── workflows/
│   │   ├── ci.yml              # 持续集成
│   │   └── release.yml         # 自动发布
│   ├── FUNDING.yml             # 赞助配置（可选）
│   └── pull_request_template.md
├── docs/
│   ├── ARCHITECTURE.md         # 架构说明
│   └── COMPARISON.md           # Rust vs Go 对比
├── dos2unix/                   # 子项目（保持现有结构）
├── gzip/                       # 子项目
├── htop/                       # 子项目
├── .editorconfig               # 编辑器配置
├── .gitattributes              # Git 属性
├── .gitignore                  # Git 忽略
├── CHANGELOG.md                # 变更日志
├── CODE_OF_CONDUCT.md          # 行为准则
├── CONTRIBUTING.md             # 贡献指南
├── Cargo.toml                  # Rust workspace
├── LICENSE                     # 许可证
├── Makefile                    # 构建脚本
├── README.md                   # 项目主文档
├── SECURITY.md                 # 安全政策
├── go.work                     # Go workspace
└── rustfmt.toml                # Rust 格式化配置
```

## Components and Interfaces

### 1. 文档组件

#### README.md 结构
```markdown
# 项目名称
[徽章区域]

## 简介
## 特性
## 快速开始
## 子项目
## 构建
## 贡献
## 许可证
```

#### CONTRIBUTING.md 结构
- 贡献流程
- 代码规范
- 提交信息格式
- PR 流程

#### CODE_OF_CONDUCT.md
- 采用 Contributor Covenant v2.1

### 2. CI/CD 组件

#### ci.yml 增强
- 多平台矩阵构建 (ubuntu, macos, windows)
- 代码覆盖率收集
- 缓存优化

#### release.yml 新增
- 基于 tag 触发
- 跨平台二进制构建
- 自动生成 release notes
- 上传 artifacts

### 3. 代码质量组件

#### .editorconfig
```ini
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.{yml,yaml,json,md}]
indent_size = 2

[Makefile]
indent_style = tab
```

#### rustfmt.toml
```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
```

### 4. Issue/PR 模板组件

#### Bug Report 模板
- 问题描述
- 复现步骤
- 期望行为
- 环境信息

#### Feature Request 模板
- 功能描述
- 使用场景
- 可能的实现方案

#### PR 模板
- 变更描述
- 关联 Issue
- 检查清单

## Data Models

### CHANGELOG 格式 (Keep a Changelog)

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [0.1.0] - YYYY-MM-DD
### Added
- Initial release
```

### 版本号规范 (SemVer)
- MAJOR: 不兼容的 API 变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修复



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

由于本项目主要涉及文档和配置文件的创建，大部分验收标准是关于文件存在性和内容结构的检查，属于示例测试而非属性测试。以下是可以作为属性测试的标准：

### Property 1: Sub-project Structure Consistency

*For any* sub-project in the repository, it SHALL contain:
- A README.md file with usage examples and build instructions
- A changelog directory
- Appropriate language-specific configuration files
- Test files demonstrating core functionality

**Validates: Requirements 1.5, 6.1, 6.2, 6.3, 6.4, 6.5**

### Property 2: File Naming Convention Consistency

*For any* file in the repository, it SHALL follow consistent naming conventions:
- Markdown files: UPPERCASE for root docs (README.md, CONTRIBUTING.md), lowercase for others
- Configuration files: lowercase with appropriate extensions
- Source files: follow language conventions (snake_case for Rust, camelCase for Go)

**Validates: Requirements 4.4**

## Error Handling

### 文档缺失处理
- CI 流水线应检查必要文档是否存在
- 缺失文档时应在 PR 检查中给出明确提示

### 构建失败处理
- 跨平台构建失败时应记录详细错误信息
- Release 构建失败不应影响 CI 状态

## Testing Strategy

### 验证方法

由于本项目主要是文档和配置的完善，测试策略以手动验证和 CI 检查为主：

1. **文件存在性检查** - 通过 CI 脚本验证必要文件存在
2. **文档结构检查** - 通过 markdown lint 验证文档格式
3. **配置有效性检查** - 通过实际构建验证配置正确性

### CI 验证清单
- [ ] 所有必要文档文件存在
- [ ] README.md 包含徽章
- [ ] CHANGELOG.md 格式正确
- [ ] 跨平台构建成功
- [ ] 代码格式化检查通过
- [ ] 代码 lint 检查通过
