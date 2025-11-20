# rgzip

一个简洁的 Rust gzip 命令行工具，主打可维护、易分发，默认面向 Linux（亦可跨平台构建）。

## 项目亮点
- **最少依赖**：仅基于 `flate2` 与 `clap`，遵循 KISS。
- **双向流支持**：既可处理文件，也可处理标准输入/输出。
- **可复用核心库**：压缩/解压逻辑位于 `src/lib.rs`，方便嵌入其他项目。

## 目录结构
```text
rgzip/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── changelog/
│   ├── 2025-09-25-init.md
│   └── ...
├── src/
│   ├── lib.rs      # gzip 核心逻辑
│   └── main.rs     # 命令行入口
└── target/         # 构建输出（忽略于版本控制）
```

## 快速开始（Linux）
1. 安装 Rust 工具链（推荐 `rustup`）：<https://rustup.rs>
2. 克隆仓库并进入目录：
   ```bash
   git clone https://github.com/<your-org>/rgzip.git
   cd rgzip
   ```
3. 构建发布版本：
   ```bash
   cargo build --release
   ```
4. 将可执行文件加入 `PATH` 或直接运行 `target/release/rgzip`。

## 使用示例
- **压缩文件（生成 `file.txt.gz`）**
  ```bash
  rgzip file.txt
  ```
- **压缩到指定输出**
  ```bash
  rgzip -o out.gz file.txt
  ```
- **从标准输入压缩到文件**
  ```bash
  echo "hello" | rgzip -o hello.gz
  ```
- **从标准输入压缩到标准输出**
  ```bash
  echo "hello" | rgzip > hello.gz
  ```
- **解压文件（默认去掉 `.gz` 后缀）**
  ```bash
  rgzip -d file.txt.gz
  ```
- **解压到指定输出**
  ```bash
  rgzip -d -o out.txt file.txt.gz
  ```
- **从标准输入解压到标准输出**
  ```bash
  rgzip -d < hello.gz > hello.txt
  ```

## 命令行参数
- `-d, --decompress`：解压模式（默认压缩）。
- `-o, --output <PATH>`：指定输出文件。
- `-k, --keep`：成功后保留源文件。
- `-f, --force`：覆盖已有输出文件。
- `-l, --level <0-9>`：压缩级别，默认 `6`。

## 开发 & 测试
1. 保持代码格式：
   ```bash
   cargo fmt
   ```
2. 运行静态检查（可选）：
   ```bash
   cargo clippy --all-targets --all-features
   ```
3. 运行自测（如添加单元/集成测试）：
   ```bash
   cargo test
   ```

## 贡献指南
1. Fork 仓库并创建特性分支。
2. 在提交前运行最小验证流程（`cargo fmt` / `cargo clippy` / `cargo test`）。
3. 在 `changelog/` 目录新增记录，描述你的变更。
4. 创建 Pull Request，简述动机和测试结果。

## 发布提示
- 自定义版本号于 `Cargo.toml`。
- 使用 `cargo publish` 前请确保 changelog 与 README 同步更新。

## 许可证
MIT OR Apache-2.0（双许可证任选其一，遵循仓库根目录 LICENSE）。
