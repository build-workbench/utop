# htop-win-rust

面向 Windows 11 的 Rust 版简化 htop（TUI 任务管理器）。

- 基于 ratatui + crossterm 构建终端 UI。
- 基于 sysinfo 采集系统信息（CPU、内存、进程列表）。
- 目标：在 Windows 终端中获得类似 htop 的体验，保持实现简单（KISS）。

## 功能

- 顶部资源面板：
  - CPU 使用率仪表
  - 内存使用率仪表
  - CPU/内存历史曲线（Sparkline）
- 进程表格：PID、名称、CPU%、内存(MiB)
- 排序与选择：
  - s 切换排序键（CPU → 内存 → PID → 名称）
  - r 反转排序（升/降序）
  - ↑/↓ 选择行；PgUp/PgDn 翻页；Home/End 跳转首尾
- 过滤/搜索：
  - / 进入输入模式，输入名称或 PID 片段，Enter 应用，Esc 取消
  - 底部状态栏实时显示过滤状态
- 刷新控制：
  - p 暂停/继续刷新
  - + 或 = 加快刷新（最小 200ms），- 放慢刷新（最大 5000ms）
  - F5 强制立即刷新
- 进程操作：
  - k 结束选中进程（可能需要以管理员身份运行）
- 退出：
  - q
  - Esc：关闭弹窗/取消输入

## 快速开始（Windows 11）

1. 安装 Rust（MSVC 工具链）：https://rustup.rs/
2. 在 PowerShell 或 Windows Terminal 中执行：

```powershell
# 首次构建会下载依赖
cargo run --release
```

如果你使用的是 VS Code，推荐在 Windows Terminal 或 PowerShell 中运行，以获得更好的终端体验。

提示：若你已运行过本程序，请先在 TUI 内按 q 退出后再重新构建，否则 Windows 可能因可执行文件被占用而拒绝覆盖（os error 5）。

## 已知注意点

- 结束进程可能需要管理员权限；若失败，请以管理员身份重新运行终端。
- CPU% 的精确度依赖周期性刷新；首次打开时需等待 1~2 秒以稳定。
- 某些系统进程不可终止或不可获取完整信息（受权限限制）。

## 按键速查

- q：退出
- Esc：关闭弹窗/取消输入
- i：打开/关闭详情弹窗（弹窗内也可按 Enter/Esc 关闭）
- ↑ / ↓：选择上一/下一行
- PgUp / PgDn：翻页
- Home / End：跳转首尾
- s：切换排序键（CPU → 内存 → PID → 名称）
- r：反转排序（升/降序）
- /：进入过滤输入（Enter 应用，Esc 取消）
- p：暂停/继续刷新
- + 或 =：加快刷新（最小 200ms）
- -：放慢刷新（最大 5000ms）
- F5：强制立即刷新
- k：结束选中进程

## 目录结构

```
htop-win-rust/
├─ Cargo.toml
├─ README.md
├─ changelog/
│  └─ 2025-09-25.md
└─ src/
   └─ main.rs
```

## 变更记录

所有修改均记录在 `changelog/` 目录，按日期创建文件。

## Roadmap（可选）

- 进程树视图
- 每核 CPU 使用率、磁盘/网络/GPU 等更多指标
- 主题/高亮风格定制
- 更贴近 htop 的 F 键映射

如需这些功能，请告诉我你的优先级，我会按你的需求安排实现顺序。

## 许可
MIT OR Apache-2.0（遵循仓库根目录 LICENSE）。
