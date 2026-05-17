---
title: gzip - 压缩工具
---

# gzip

多语言实现（Go + Rust）的 gzip 压缩/解压工具。

## 概览

本项目提供 gzip 标准工具的 Go 和 Rust 两种实现，便于对比：
- 不同流式压缩方法
- 两种语言的错误处理模式
- 并发模型（goroutine vs async）
- CLI 库设计（pflag vs clap）

## 特性

### 两种实现共有

- ✅ **流式压缩/解压**
- ✅ **多种压缩级别**（0-9）
- ✅ **保留源文件**（`-k` 参数）
- ✅ **强制覆盖**（`-f` 参数）
- ✅ **递归目录支持**（`-r` 参数）
- ✅ **stdin/stdout 支持**

### Go 实现

- 使用 goroutine 并行处理文件
- 标准库 `compress/gzip`
- 内置并发支持

### Rust 实现

- 库 crate 可嵌入其他项目
- `clap` 派生宏处理 CLI
- `flate2` 实现 DEFLATE 压缩

## 快速开始

```bash
# 构建 Go 版本
cd gzip/go && go build -o bin/gzip-go ./cmd/gzip-go

# 构建 Rust 版本
cargo build --release -p rgzip

# 压缩文件
./gzip/go/bin/gzip-go file.txt
./target/release/rgzip file.txt

# 解压
./gzip/go/bin/gzip-go -d file.txt.gz
./target/release/rgzip -d file.txt.gz

# 带参数
./target/release/rgzip -k -l 9 file.txt  # 保留源文件，最大压缩
```

## 学习主题

| 主题 | Go | Rust |
|------|:--:|:----:|
| 流式处理 | ✅ | ✅ |
| 并发 | ✅ goroutine | ✅ async |
| 错误处理 | ✅ error 类型 | ✅ anyhow/Result |
| CLI 框架 | ✅ pflag | ✅ clap |
| 库设计 | ✅ | ✅ |

## 对比要点

| 方面 | Go | Rust |
|------|:--:|:----:|
| 二进制大小 | ~4MB | ~2MB |
| 构建时间 | 快 | 中等 |
| 内存管理 | GC 管理 | 所有权系统 |
| 安全性 | 运行时检查 | 编译时检查 |

## 源代码

- [Go 实现](https://github.com/LessUp/build-your-own-tools/tree/master/gzip/go)
- [Rust 实现](https://github.com/LessUp/build-your-own-tools/tree/master/gzip/rust)
- [Go 变更日志](/gzip/go/changelog/CHANGELOG.md)
- [Rust 变更日志](/gzip/rust/changelog/CHANGELOG.md)

## 相关

- [dos2unix](/dos2unix/) - 换行符转换器
- [htop](/htop/) - 系统监控
