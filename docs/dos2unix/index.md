---
title: dos2unix - CRLF 转 LF 转换器
---

# dos2unix

Rust 实现的 CRLF（Windows）到 LF（Unix）换行符转换器。

## 概览

本工具将 Windows 风格换行符（CRLF，`\r\n`）转换为 Unix 风格换行符（LF，`\n`）。适用于：
- 跨操作系统文件标准化
- 为 Unix/Linux 环境准备文件
- 学习 Rust 中的文件 I/O 和流式处理

## 特性

- ✅ **流式处理** - 8KB 缓冲区，内存高效处理大文件
- ✅ **原地转换** - 直接修改文件
- ✅ **stdin/stdout 支持** - 支持管道操作
- ✅ **检查模式** - 检测 CRLF 而不修改
- ✅ **静默模式** - 最小输出，适合脚本

## 快速开始

```bash
# 构建
cargo build --release -p dos2unix-rust

# 转换文件
./target/release/dos2unix-rust file.txt

# 检查 CRLF（发现则返回退出码 2）
./target/release/dos2unix-rust --check file.txt

# 管道用法
echo "Hello\r\nWorld" | ./target/release/dos2unix-rust
```

## 学习主题

| 主题 | 描述 |
|------|------|
| 文件 I/O | 使用缓冲 I/O 进行流式读写 |
| 错误处理 | 使用 `anyhow` 进行符合人体工程学的错误传播 |
| CLI 设计 | 手动参数解析 |
| 跨平台 | 处理不同换行符 |

## 技术细节

- **缓冲区大小**：8KB，优化流式性能
- **内存使用**：O(1) - 无论文件大小，内存恒定
- **性能**：大文件约 120 MB/s

## 源代码

- [主实现](https://github.com/LessUp/build-your-own-tools/tree/master/dos2unix/src)
- [变更日志](/dos2unix/changelog/CHANGELOG.md)

## 相关

- [gzip](/gzip/) - 压缩工具
- [htop](/htop/) - 系统监控
