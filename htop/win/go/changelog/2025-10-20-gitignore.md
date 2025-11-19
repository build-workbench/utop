# 2025-10-20 Git 忽略规则

- 新增：在仓库根目录创建 `.gitignore`。
- 内容：忽略常见构建产物（`bin/`、`dist/`、`build/`、`*.exe` 等二进制）、临时文件（`tmp/`、`*.log`、`*.cover`、`*.prof`）、以及常用 IDE 配置目录（`.vscode/`、`.idea/`）。
- 目的：保持版本库整洁，避免提交生成文件与个人开发环境配置。

影响文件：
- `.gitignore`

验证：
- 运行 `git status` 可见新增 `.gitignore`，并且位于忽略列表中的现有文件（如 `htop-win-go.exe`）将被排除出未跟踪文件显示。
