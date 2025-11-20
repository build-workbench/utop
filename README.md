# Build-Your-Own-Utils

一个用 Rust / Go 手写常用命令行工具的学习仓库，用于练习底层实现、命令行设计与跨语言对比。目前包含三个子项目：

- `dos2unix/`：Rust 实现的轻量 dos2unix 工具
- `gzip/`：Go 与 Rust 实现的极简 gzip 命令行
- `htop/`：Unix / Windows 平台的 htop 学习/演示项目（Rust 与 Go 多版本）

## 目录结构

```text
./
├── dos2unix/        # Rust 实现的 dos2unix
├── gzip/            # Go & Rust 实现的 gzip
└── htop/            # 跨平台 htop 示例（Unix/Win, Go/Rust）
```

每个子目录内都有单独的 `README.md`，包含更详细的设计说明、构建方式与使用示例。

## 前置依赖

- Rust 工具链（建议 1.70+）
- Go 工具链（建议 1.20+）
- Linux / macOS / WSL 等类 Unix 环境（主要目标平台）

## 快速开始

以下仅为示例，实际以各子项目 README 为准：

```bash
# 进入 dos2unix 并构建
cd dos2unix
cargo build --release

# 进入 gzip/Go 版
cd gzip/go
make build          # 或：go build -o bin/gzip-go ./cmd/gzip-go

# 进入 htop（以 Unix/Rust 版为例）
cd htop/unix/rust
cargo run
```

更多选项、用法示例、退出码约定等，请查看各子项目的 `README.md`。

## 开发约定

- 保持 KISS 原则：实现尽量简单、可读、易维护
- 优先使用 LF 换行，文本归一与换行相关设置见各子项目配置
- 修改功能时，请同步更新相应子目录的 `changelog/`
- 提交前尽量运行基本检查：
  - Rust：`cargo fmt && cargo clippy`
  - Go：`gofmt -w . && go vet ./...`

## 许可证

本仓库在根目录提供统一的 `LICENSE` 文件，采用 `MIT OR Apache-2.0` 双许可证。
各子项目（`dos2unix/`、`gzip/`、`htop/` 等）默认跟随根目录许可证策略；如需在具体分发中选择其一，可在相应发布说明中注明。
