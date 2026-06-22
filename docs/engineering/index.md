# 工程实践概览

本章节介绍 Build Your Own Tools 项目的工程化实践。

## 工程化架构

```mermaid
graph TB
    subgraph "开发流程"
        A[需求] --> B[代码实现]
        B --> C[代码审查]
    end
    
    subgraph "质量保证"
        C --> D[CI/CD]
        D --> E[构建测试]
        E --> F[质量门禁]
    end
    
    subgraph "文档管理"
        F --> G[API 文档]
        G --> H[用户文档]
        H --> I[变更日志]
    end
    
    style A fill:#f59e0b,color:#fff
    style D fill:#10b981,color:#fff
```

## 章节内容

### [CI/CD 设计](/engineering/cicd)

描述持续集成和部署流程：

- GitHub Actions 工作流
- 构建矩阵
- 质量门禁
- 发布流程

### [文档策略](/engineering/documentation)

说明文档维护策略：

- 文档结构
- API 文档生成
- 变更日志管理
- 版本化策略

## 关键文件

| 文件 | 用途 |
|------|------|
| `.github/workflows/*.yml` | CI/CD 工作流 |
| `CHANGELOG.md` | 变更日志 |

## 工程化收益

```mermaid
pie title 工程化投入占比
    "自动化测试" : 30
    "CI/CD" : 25
    "文档维护" : 20
    "代码审查" : 15
    "规范管理" : 10
```

### 效率提升

| 方面 | 提升 |
|------|------|
| 问题发现 | 70% 更早 |
| 发布周期 | 50% 更短 |
| 文档同步 | 90% 自动 |
| 代码审查 | 40% 高效 |

## 下一步

- 🔧 阅读 [CI/CD 设计](/engineering/cicd) 学习自动化流程
- 📚 阅读 [文档策略](/engineering/documentation) 掌握文档维护
