---
layout: home
hero:
  name: Build Your Own Tools
  text: 系统编程技术白皮书
  tagline: Rust × Go 双实现架构对比研究
  image:
    src: /logo.svg
    alt: Build Your Own Tools
  actions:
    - theme: brand
      text: 开始学习
      link: /academy/
    - theme: alt
      text: 查看架构
      link: /whitepaper/architecture
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: 🏗️
    title: 架构对比
    details: 同一问题的 Rust 与 Go 实现，深入分析两种语言的设计哲学差异
  - icon: 📊
    title: 性能分析
    details: 跨语言性能基准，内存模型与并发模型对比研究
  - icon: 🧪
    title: 可构建实现
    details: 三个真实 CLI 工具的完整源码，cargo 与 go 命令即可构建测试
  - icon: 📖
    title: 渐进学习
    details: 从流式 I/O 到跨平台 TUI，复杂度逐级递增的学习路径
---

## 技术白皮书概览

本项目是一个**系统编程学习仓库**，通过重新实现三个真实的 CLI 工具（dos2unix、gzip、htop）来展示 Rust 和 Go 两种语言的系统编程风格差异。

### 核心特性

```mermaid
graph LR
    A[Monorepo] --> B[dos2unix]
    A --> C[gzip]
    A --> D[htop]
    
    B --> B1[Rust]
    C --> C1[Rust]
    C --> C2[Go]
    D --> D1[Rust]
    D --> D2[Go]
    
    classDef primary fill:#f59e0b,color:#fff,stroke:#d97706,stroke-width:2px
    classDef secondary fill:#3b82f6,color:#fff,stroke:#2563eb,stroke-width:2px
    
    class A primary
    class B,C,D secondary
```

### 学习路径

| 阶段 | 工具 | 学习重点 | 复杂度 |
|------|------|----------|--------|
| 1 | dos2unix | 流式 I/O、换行符处理 | ⭐ |
| 2 | gzip | 压缩流程、CLI 设计、错误处理 | ⭐⭐ |
| 3 | htop | TUI、系统 API、跨平台架构 | ⭐⭐⭐ |

### 技术栈

- **Rust**: 系统编程、内存安全、零成本抽象
- **Go**: 并发模型、简洁语法、快速开发
- **VitePress**: 文档站点、Mermaid 图表、LLM 友好输出

## 快速导航

<div class="quick-links">

[学院](/academy/){.VPButton}
[白皮书](/whitepaper/){.VPButton .alt}
[对比研究](/comparison/){.VPButton .alt}

</div>

<style>
.quick-links {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin: 2rem 0;
}
</style>
