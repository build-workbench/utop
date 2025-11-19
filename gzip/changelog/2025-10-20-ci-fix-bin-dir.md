# 2025-10-20 修复 CI 中 Go 构建步骤

## 变更
- 更新：`.github/workflows/ci.yml`
  - 在执行 `go build -o bin/gzip-go` 前新增 `mkdir -p bin`，避免 `bin/` 不存在导致构建失败。

## 影响
- 仅影响 CI 行为，本地构建流程不变。
