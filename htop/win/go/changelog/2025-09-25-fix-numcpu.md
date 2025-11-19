# 2025-09-25 修正 CPU 核心数获取方法

- 将 `internal/metrics/processes.go` 中的 `getNumCPU()` 改为使用 `runtime.NumCPU()`。
- 原因：`gopsutil/process` 不提供 `NumCPU()`；使用标准库更可靠且跨平台。
