# 2025-10-20 标准化仓库结构

## 变更
- 新增：`/.gitignore`
  - 忽略系统与编辑器文件（`.DS_Store`、`.idea/`、`.vscode/` 等）
  - 忽略 Go 产物：`/go/bin/`
  - 忽略 Rust 产物：`/rust/target/`
  - 忽略常见二进制与库文件（`*.exe`、`*.dll`、`*.so`、`*.a`、`*.o` 等）
- 新增：`/.gitattributes`
  - 强制文本 `eol=lf`
  - 将归档/二进制（`*.gz`、`*.zip`、`*.tar`、`*.rlib` 等）标记为 `binary`
- 新增：`/.editorconfig`
  - 全局 UTF-8、LF、去除行尾空白、末行换行
  - `*.go` 使用 Tab、宽度 4；`*.rs` 使用空格、宽度 4；`Makefile` 使用 Tab
- 新增：`/README.md`
  - 仓库总览：`go/` 与 `rust/` 子项目简介与指引
  - 统一贡献建议与后续许可对齐提醒
- 新增：`/changelog/2025-10-20-standardize.md`（本文件）

## 说明
- 未改动 `go/README.md` 与 `rust/README.md` 的既有内容，待确认许可证策略后再对齐文案。
- Rust 子项目当前 `Cargo.toml` 采用 `MIT OR Apache-2.0`；建议根级确定后与 Go、Rust 一并对齐。

## 后续建议
- 选择许可证（`MIT`、`Apache-2.0` 或双许可证），在根级添加 `LICENSE`，并同步更新子项目文档与元数据。
- （可选）添加社区文件：`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`.github/ISSUE_TEMPLATE` 与 `PULL_REQUEST_TEMPLATE.md`。
