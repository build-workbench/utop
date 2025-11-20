# gzip

多语言（Go 与 Rust）实现的极简 gzip 命令行工具集合，遵循 KISS 原则：简单、可维护、易分发，默认聚焦 Linux 使用场景。

## 子项目
- **Go 版**：`go/`，可执行文件名为 `gzip-go`，入口位于 `go/cmd/gzip-go/main.go`。
  - 详细说明见 `go/README.md`
- **Rust 版**：`rust/`，可执行文件名为 `rgzip`，核心逻辑在 `rust/src/lib.rs`。
  - 详细说明见 `rust/README.md`

## 目录结构
```text
./
├── go/
│   ├── cmd/gzip-go/main.go
│   ├── go.mod
│   ├── Makefile
│   ├── README.md
│   └── changelog/
├── rust/
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/{main.rs, lib.rs}
│   └── changelog/
├── .gitignore
├── .gitattributes
├── .editorconfig
└── changelog/
```

## 快速开始
- **构建 Go 版**
  ```bash
  cd go
  make build      # 生成 go/bin/gzip-go
  # 或
  go build -o bin/gzip-go ./cmd/gzip-go
  ```
- **构建 Rust 版**
  ```bash
  cd rust
  cargo build --release   # 生成 target/release/rgzip
  ```

## 使用
- 参见子项目文档：`go/README.md` 与 `rust/README.md`。

## 开源许可
- 本项目遵循仓库根目录 `LICENSE` 文件约定，采用 `MIT OR Apache-2.0` 双许可证。
- Rust 与 Go 实现均可在上述许可证下使用和分发。

## 贡献
- 欢迎 PR。请遵循：
  - 保持 KISS 原则，尽量减少依赖与复杂度。
  - 每次修改请在对应子项目的 `changelog/` 或仓库根级 `changelog/` 新增记录。
  - 统一代码风格：见 `.editorconfig`；换行符 LF，文本自动归一见 `.gitattributes`。
