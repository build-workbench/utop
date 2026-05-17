---
title: htop - 系统监控
---

# htop

跨平台 TUI（终端用户界面）系统监控，多种实现版本。

## 概览

本项目实现类似流行 `htop` 工具的交互式系统监控，支持：
- **Unix/Linux**（Rust + ratatui）
- **Windows**（Rust + ratatui）
- **Windows**（Go + tview）

适合学习：
- TUI 开发
- 系统编程
- 进程管理
- 跨平台开发

## 特性

### 核心功能

- ✅ **实时 CPU/内存监控**
- ✅ **进程列表排序**（CPU、内存、PID、名称）
- ✅ **进程搜索/过滤**
- ✅ **进程终止功能**
- ✅ **可调刷新间隔**
- ✅ **颜色编码使用率指示器**

### 平台特定功能

| 功能 | Unix Rust | Win Rust | Win Go |
|------|:---------:|:--------:|:------:|
| 进程列表 | ✅ | ✅ | ✅ |
| CPU/内存 | ✅ | ✅ | ✅ |
| 进程终止 | ✅ | ✅ | ✅ |
| Sparkline 历史 | - | ✅ | - |
| 网络统计 | 计划中 | 计划中 | 计划中 |

## 快速开始

```bash
# 构建 Unix 版本（Linux/macOS）
cargo build --release -p htop-rust

# 运行
./target/release/htop-unix-rust

# 构建 Windows Rust 版本
cargo build --release -p htop-win-rust

# 构建 Windows Go 版本
cd htop/win/go && go build -o bin/htop-win-go ./cmd/htop-win-go
```

## 键盘快捷键

| 按键 | 操作 |
|------|------|
| `q` | 退出 |
| `k` | 终止选中进程 |
| `/` | 搜索/过滤进程 |
| `s` | 切换排序列 |
| `+`/`-` | 调整刷新间隔 |
| `↑`/`↓` | 导航进程列表 |

## 学习主题

| 主题 | 描述 |
|------|------|
| TUI 开发 | ratatui (Rust) / tview (Go) |
| 系统 API | 进程信息、CPU、内存统计 |
| 事件处理 | 键盘输入、异步刷新 |
| 跨平台 | Unix vs Windows 差异 |
| 并发 | 异步刷新循环 |

## 架构

```
htop/
├── shared/          # 共享 Rust 库
├── unix/rust/       # Unix 实现
└── win/
    ├── rust/        # Windows Rust 实现
    └── go/          # Windows Go 实现
```

## 源代码

- [共享库](https://github.com/LessUp/build-your-own-tools/tree/master/htop/shared)
- [Unix Rust](https://github.com/LessUp/build-your-own-tools/tree/master/htop/unix/rust)
- [Windows Rust](https://github.com/LessUp/build-your-own-tools/tree/master/htop/win/rust)
- [Windows Go](https://github.com/LessUp/build-your-own-tools/tree/master/htop/win/go)
- [变更日志](/htop/changelog/CHANGELOG.md)

## 相关

- [dos2unix](/dos2unix/) - 换行符转换器
- [gzip](/gzip/) - 压缩工具
