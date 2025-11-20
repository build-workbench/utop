# htop (Windows/Unix)

跨平台 htop 学习/演示项目，包含 `unix/rust/` 与 `win/rust/`、`win/go/` 子模块。

## 特性
- 轻量、可学习
- Windows 与 Unix 双平台示例
- Rust 与 Go 实现对比

## 目录结构
```
htop/
  unix/
    rust/
  win/
    rust/
    go/
  changelog/
```

## 构建与运行
- Rust（Unix 或 Windows）
```bash
# 在对应子目录执行
cargo build --release
cargo run
```
- Go（Windows 示例）
```bash
# 在 win/go 目录执行
go build ./...
```

## 贡献
- 提交前请运行格式化与基本检查
```bash
cargo fmt && cargo clippy
# 或
gofmt -w . && go vet ./...
```
- 提交 PR 前附上动机与变更简述

## 许可
- 遵循仓库根目录 `LICENSE` 文件，采用 `MIT OR Apache-2.0` 双许可证。

## 变更日志
- 见 `changelog/CHANGELOG.md`
