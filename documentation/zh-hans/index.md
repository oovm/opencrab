# OpenCrab 文档

欢迎阅读 OpenCrab 项目文档！本目录包含 OpenCrab 系统的完整中文文档。

## 文档结构

### 概述 (overview/)

- [index.md](overview/index.md) - 概述索引
- [introduction.md](overview/introduction.md) - 项目介绍
- [quick-start.md](overview/quick-start.md) - 快速入门
- [features.md](overview/features.md) - 核心特性

### 核心概念 (concepts/)

- [index.md](concepts/index.md) - 核心概念索引
- [agent.md](concepts/agent.md) - 智能体
- [skills.md](concepts/skills.md) - 技能
- [capabilities.md](concepts/capabilities.md) - 能力
- [memory.md](concepts/memory.md) - 记忆
- [tool.md](concepts/tool.md) - 工具
- [workspace.md](concepts/workspace.md) - 工作区
- [chat.md](concepts/chat.md) - 聊天
- [scheduler.md](concepts/scheduler.md) - 调度

### 教程 (tutorials/)

- [index.md](tutorials/index.md) - 教程索引
- [getting-started.md](tutorials/getting-started.md) - 入门教程
- [configure-agent.md](tutorials/configure-agent.md) - 配置智能体
- [add-skills.md](tutorials/add-skills.md) - 添加技能
- [use-tools.md](tutorials/use-tools.md) - 使用工具

### 使用场景 (tutorials/use-cases/)

- [index.md](tutorials/use-cases/index.md) - 使用场景索引
- [personal-assistant.md](tutorials/use-cases/personal-assistant.md) - 个人助手
- [development-helper.md](tutorials/use-cases/development-helper.md) - 开发辅助
- [knowledge-base.md](tutorials/use-cases/knowledge-base.md) - 知识库
- [task-automation.md](tutorials/use-cases/task-automation.md) - 任务自动化
- [best-practices.md](tutorials/use-cases/best-practices.md) - 最佳实践

### 进阶主题 (advanced/)

- [index.md](advanced/index.md) - 进阶主题索引
- [extensibility.md](advanced/extensibility.md) - 扩展性
- [security.md](advanced/security.md) - 安全性
- [performance.md](advanced/performance.md) - 性能优化

### 架构设计 (maintainer/architecture/)

- [ecosystem-overview.md](maintainer/architecture/ecosystem-overview.md) - 生态系统总览：整个生态系统的架构层次和项目关系（必读）
- [master-plan.md](maintainer/architecture/master-plan.md) - 架构总设计：OpenCrab 整体架构设计（必读）
- [core-layer.md](maintainer/architecture/core-layer.md) - 核心层设计：智能体核心功能模块设计
- [infrastructure.md](maintainer/architecture/infrastructure.md) - 基础设施层设计：crab-* 基础设施模块设计
- [decentralization.md](maintainer/architecture/decentralization.md) - 去中心化设计：系统的核心安全设计理念

### 维护者文档 (maintainer/)

- [index.md](maintainer/index.md) - 维护者文档索引
- [data-models.md](maintainer/data-models.md) - 数据模型与存储
- [agent-core.md](maintainer/agent-core.md) - 智能体（Agent）核心定义
- [technology-choices.md](maintainer/technology-choices.md) - 技术选型

### Skynet 协议 (maintainer/skynet/)

- [index.md](maintainer/skynet/index.md) - Skynet 协议文档索引
- [skynet.md](maintainer/skynet/skynet.md) - Skynet 协议设计草案
- [subnets.md](maintainer/skynet/subnets.md) - 子网模型
- [messages.md](maintainer/skynet/messages.md) - 消息协议
- [profile.md](maintainer/skynet/profile.md) - 配置文件
- [resources.md](maintainer/skynet/resources.md) - 资源定义
- [threat-model.md](maintainer/skynet/threat-model.md) - 威胁模型
- [uri.md](maintainer/skynet/uri.md) - URI 设计

## 快速开始

1. 从 [概述](overview/index.md) 开始，快速了解 OpenCrab
2. 阅读 [快速入门](overview/quick-start.md) 快速上手
3. 查看 [核心概念](concepts/index.md) 深入理解系统
4. 参考 [教程](tutorials/index.md) 学习使用

## 关于 OpenCrab

OpenCrab 是一个单机智能体节点系统，是 OpenClaw 生态系统的基础节点实现。它实现了 Skynet 协议，支持与其他节点互联，为用户提供本地化的 AI 智能体服务。

### 核心特点

- **单机节点**：适合个人使用，数据本地存储
- **前后端一对多**：类似个人微信，支持多账号切换
- **模块化设计**：crab-* 系列模块，高内聚低耦合
- **协议兼容**：实现 Skynet 协议，支持去中心化互联
