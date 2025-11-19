# 2025-10-20 UI 增强：标题与进度条

- 新增：头部标题 `htop-win-go`。
- 新增：CPU、内存文本进度条（带颜色阈值：绿色<60%，黄色60-85%，红色≥85%），同时显示百分比与内存用量。
- 新增：右侧显示进程数 Procs。
- 重构：`internal/ui/ui.go` 头部由单 `TextView` 改为 `Flex(Row)`，包含“标题行 + 进度条行”。
- 实现：使用 `TextView` 渲染 ASCII 进度条，新增 `renderBar()` 与 `colorByPercent()` 辅助函数。
- 兼容性：避免依赖 tview Gauge 组件（该版本无内置 Gauge）。

影响文件：
- `internal/ui/ui.go`

验证：
- Windows 11 / Go 1.23
- `go build ./cmd/htop-win-go` 编译通过
- 运行后在顶部显示标题、CPU/Mem 条与 Procs 计数
