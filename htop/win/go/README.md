# htop-win-go

Windows 11 下使用 Go 实现的简洁版 htop。基于 tview+tcell 构建终端 TUI，使用 gopsutil 采集系统与进程信息。

## 功能

- 简洁系统概览：CPU 使用率、内存使用情况。
- 进程列表（默认按 CPU% 降序）：PID、Name、CPU%、RSS、Mem%、Start 时间。
- 操作：
  - q / Esc 退出。
  - C/M/P/N 切换排序：CPU%、Mem%、PID、Name。
  - 刷新间隔：1s。

## 快速开始

前置：
- Windows 11
- Go 1.21+（建议 1.22）
- Windows Terminal/PowerShell

步骤：

```powershell
# 安装依赖
cd d:\htop-win-go
go mod tidy

# 运行
go run ./cmd/htop-win-go
```

编译：

```powershell
go build -o bin\htop-win-go.exe ./cmd/htop-win-go
```

## 目录结构

```
htop-win-go/
├─ cmd/
│  └─ htop-win-go/
│     └─ main.go
├─ internal/
│  ├─ metrics/
│  │  ├─ system.go
│  │  └─ processes.go
│  └─ ui/
│     └─ ui.go
├─ changelog/
│  └─ 2025-09-25-init.md
└─ go.mod
```

## 实现要点

- CPU% 计算：通过进程 `Times`（user+system）与 wall clock 的时间差估算，除以 `runtime.NumCPU()` 得到总占比。
- 内存：显示 RSS 与内存占比（gopsutil 的 MemoryPercent）。
- 设计：KISS，仅保留最常用字段与交互。

## 注意事项

- 某些系统进程可能因权限或瞬态状态导致信息获取失败，程序会跳过这些进程并继续渲染。
- 首次刷新时 CPU% 可能为 0，随后每秒更新将显示趋势。

## Roadmap（可选）

- 进程排序切换（CPU/Mem/Name/PID）。
- 搜索/过滤。
- 进程终止（需确认并谨慎使用）。
- 每核 CPU 与历史折线小图。
- 自定义刷新间隔与最大显示行数。

## 致谢

- [gopsutil](https://github.com/shirou/gopsutil)
- [tview](https://github.com/rivo/tview) 与 [tcell](https://github.com/gdamore/tcell)
