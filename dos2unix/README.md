# dos2unix-rust

一个用 Rust 实现的轻量 dos2unix 工具：将 Windows 的 CRLF 换行转换为 Unix 的 LF。

## 功能

- 将 CRLF 转换为 LF（就地转换文件）。
- 支持从 `stdin` 读取、向 `stdout` 写出（当无文件参数或使用 `-` 时）。
- 检测模式 `--check/-c`：报告哪些目标仍含 CRLF，退出码区分结果。
- 选项：`-h/--help`、`-v/--version`、`-q/--quiet`。

## 构建

需要安装 Rust（建议 1.70+）。

```bash
cargo build --release
```

可执行文件位于：`target/release/dos2unix-rust`（Windows 为 `dos2unix-rust.exe`）。

## 用法

```text
dos2unix-rust [OPTIONS] [FILES...]
```

- 无文件参数时：从标准输入读取并输出到标准输出。
- 指定 `-` 作为文件名时：从标准输入读取一次并将结果输出到标准输出（可与文件名混用）。

### 选项

- `-h, --help` 显示帮助
- `-v, --version` 显示版本
- `-c, --check` 检测模式，仅报告含 CRLF 的目标
- `-q, --quiet` 静默模式，减少日志输出

### 示例

- 就地转换文件：

```bash
dos2unix-rust README.md
```

- 从标准输入读取，输出到标准输出：

```bash
# Windows PowerShell
type README.md | dos2unix-rust > README.unix.md
```

- 只检测是否含 CRLF（含 CRLF 时退出码为 2）：

```bash
dos2unix-rust --check file1.txt file2.txt
```

- 混合使用标准流与文件：

```bash
dos2unix-rust - file1.txt file2.txt
```

## 注意

- `--check` 模式下：
  - 含 CRLF 的文件将逐一打印路径；若目标是 `-`（标准输入），则输出 `-`。
  - 退出码：0 表示所有目标均无 CRLF，2 表示至少存在一个含 CRLF 的目标，1 表示发生错误。
- 该工具以简单、可读为目标，默认不进行二进制文件检测，请谨慎对二进制文件使用（建议先做备份）。
- 目前仅将 `\r\n` 转换为 `\n`，不会处理仅包含 `\r` 的老式 Mac 换行。

## 许可证

MIT
