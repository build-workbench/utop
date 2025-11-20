# gzip-go

一个简洁的 Go 语言 gzip 命令行工具，仅面向 Linux 使用场景（跨平台构建亦可）。遵循 KISS 原则：参数少、行为直观、易于脚本集成。

## 特性
- 压缩（默认）与解压（-d）
- 递归处理目录（-r）
- 并行处理多个文件（-p，默认 CPU 核心数）
- 标准输入/输出流支持（无参或 `-` 表示 stdin；`-stdout` 表示输出到 stdout）
- 覆盖已存在目标（-f）
- 压缩级别（-l 0..9，默认 -1）

## 安装与构建（Linux）
- 直接构建
  ```bash
  mkdir -p bin
  go build -o bin/gzip-go ./cmd/gzip-go
  ```
- 使用 Makefile
  ```bash
  make build
  sudo make install    # 安装到 /usr/local/bin/gzip-go
  ```
- 交叉编译（在任意主机为 Linux amd64 构建）
  ```bash
  make build-linux
  # 或者
  GOOS=linux GOARCH=amd64 go build -o bin/gzip-go ./cmd/gzip-go
  ```

## 用法
```
usage: gzip-go [flags] [files or dirs]

flags:
  -d           解压模式 (gunzip)
  -r           递归处理目录
  -stdout      输出到标准输出（仅允许单一输入或使用 - 表示 stdin）
  -f           覆盖已存在的目标文件
  -l int       压缩级别 0..9 (默认 -1)
  -p int       并行 worker 数量 (默认为 CPU 核心数)
```

- 无参数时，默认读取标准输入并将压缩结果写到标准输出：
  ```bash
  echo "hello" | gzip-go > hello.txt.gz
  ```
- 解压到标准输出：
  ```bash
  gzip-go -d -stdout hello.txt.gz > hello.txt
  ```
- 压缩多个文件：
  ```bash
  gzip-go a.log b.log c.log
  ```
- 递归压缩目录（跳过已是 .gz 的文件）：
  ```bash
  gzip-go -r /var/log
  ```
- 递归解压目录（仅处理 .gz 文件）：
  ```bash
  gzip-go -d -r /data/backups
  ```

## 行为说明
- 压缩：为每个输入文件生成 `*.gz` 文件；若目标存在且未指定 `-f`，将报错跳过。
- 解压：对 `*.gz` 文件生成去掉 `.gz` 的同名文件；若目标存在且未指定 `-f`，将报错跳过。
- 标准流：当输入为 `-` 或未提供路径时，从 `stdin` 读取；当指定 `-stdout` 时，输出到 `stdout` 并仅允许一个输入。
- 本工具不删除源文件（区别于某些 gzip 实现默认删除源文件的行为）。

## 已知限制
- 非完全兼容 GNU gzip 的全部选项，仅覆盖常用场景。
- 不处理稀疏文件与特殊文件类型；符号链接按解引用后的普通文件处理（由内核与运行时决定）。

## 许可证
MIT OR Apache-2.0（遵循仓库根目录 LICENSE）。
