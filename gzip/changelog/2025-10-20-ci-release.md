# 2025-10-20 新增 CI 与发布工作流

## 变更
- 新增：`.github/workflows/ci.yml`
  - 触发：push / pull_request
  - Go（`go/`）：`go build` + `go vet`
  - Rust（`rust/`）：`cargo build --release --locked`
- 新增：`.github/workflows/release.yml`
  - 触发：push tag `v*`
  - 矩阵：`ubuntu-latest` / `macos-latest` / `windows-latest`
  - Go 打包：`gzip-go_${tag}_${GOOS}_${GOARCH}.tar.gz`（Windows 为 zip）
  - Rust 打包：`rgzip_${tag}_${os}_${arch}.tar.gz`（Windows 为 zip）
  - 生成 `SHA256SUMS` 并附加到 GitHub Release

## 使用说明
- 创建并推送标签以触发发布：
  ```bash
  git tag v0.1.0
  git push origin v0.1.0
  ```
- 工作流需要默认 `GITHUB_TOKEN`（已自动提供），无需额外密钥配置。

## 备注
- 当前为“原生架构”构建（与各运行器一致）。如需额外架构（如 Linux/arm64），后续可扩展交叉编译矩阵。
