# Changelog

## 0.1.0 - 2025-09-25
- 初始化项目结构（Cargo.toml、src/、changelog/）。
- 实现基础功能：CRLF -> LF 转换，支持 stdin/stdout；CLI 选项 -h/-v/-q。

## 0.2.0 - 2025-10-12
- 新增 `--check/-c` 模式，检测含 CRLF 的目标并使用退出码区分结果。
- 更新 `README.md` 文档，说明检测模式的用法与注意事项。
- 为 `convert_crlf_to_lf()` 函数补充单元测试。
