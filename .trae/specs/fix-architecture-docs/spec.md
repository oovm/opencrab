# 架构文档修正 Spec

## Why
当前架构文档中关于生态系统层次的描述存在错误：
1. ai-empire、ai-waifu、ai-planet 等应用形态被错误地描述为独立应用，实际上它们与 OpenCrab 是同级关系
2. augur-* 模块被描述为核心实现层，但实际上是过时概念，将被 crab-* 模块吸收
3. 客户端命名规范不清晰，应统一为 oh-my-* 系列

## What Changes
- **BREAKING**: 重构生态系统层次结构，ai-empire、ai-home、ai-waifu、ai-planet 与 OpenCrab 同级
- **BREAKING**: 移除 augur-* 相关描述，统一使用 crab-* 模块
- 统一客户端命名为 oh-my-* 系列
- 修正架构图示中的层次关系
- 更新各文档中的模块依赖关系描述

## Impact
- Affected docs:
  - ecosystem-overview.md
  - index.md
  - master-plan.md
  - core-layer.md
  - infrastructure.md
  - implementation-layer.md
  - presentation-layer.md
  - protocol-layer.md
  - security-model.md
  - decentralization.md

## ADDED Requirements

### Requirement: 统一的应用层次结构
生态系统 SHALL 采用统一的层次结构描述：

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           应用层 (Applications)                          │
│                                                                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐        │
│  │   oh-my-empire  │  │   oh-my-waifu   │  │   oh-my-planet  │        │
│  │   (帝国形态)     │  │   (角色形态)     │  │   (星球形态)     │        │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘        │
│                                                                          │
│  ┌─────────────────┐  ┌─────────────────┐                              │
│  │   oh-my-home    │  │   oh-my-crab    │                              │
│  │   (家庭形态)     │  │   (个人形态)     │                              │
│  └─────────────────┘  └─────────────────┘                              │
│                                                                          │
│  所有 oh-my-* 应用都是同级关系，共享 crab-* 核心模块                      │
└─────────────────────────────────────────────────────────────────────────┘
                                   ↓ 基于
┌─────────────────────────────────────────────────────────────────────────┐
│                      核心层 (Core Layer - crab-*)                        │
│  crab-agent  crab-skill  crab-memory  crab-chat  crab-tool  ...        │
└─────────────────────────────────────────────────────────────────────────┘
                                   ↓ 基于
┌─────────────────────────────────────────────────────────────────────────┐
│                    基础设施层 (Infrastructure - crab-*)                  │
│  crab-config  crab-database  crab-cache  crab-https  crab-effect  ...  │
└─────────────────────────────────────────────────────────────────────────┘
                                   ↓ 使用
┌─────────────────────────────────────────────────────────────────────────┐
│                      协议层 (Protocols - skynet-*)                       │
│  skynet-types  skynet-auth  skynet-chat  skynet-gateway  ...           │
└─────────────────────────────────────────────────────────────────────────┘
```

#### Scenario: 应用层次关系
- **WHEN** 用户查看架构文档
- **THEN** 应看到 oh-my-empire、oh-my-waifu、oh-my-planet、oh-my-home、oh-my-crab 为同级应用
- **AND** 所有应用都基于 crab-* 核心模块构建

### Requirement: 统一的模块命名规范
系统 SHALL 使用 crab-* 作为核心模块命名前缀：
- crab-agent - 智能体管理
- crab-skill - 技能系统
- crab-memory - 记忆系统
- crab-chat - 聊天系统
- crab-tool - 工具系统
- crab-scheduler - 调度系统
- crab-workspace - 工作区管理
- crab-config - 配置管理
- crab-database - 数据库抽象
- crab-cache - 缓存抽象
- crab-https - HTTP 服务
- crab-effect - 代数效应
- crab-queue - 队列系统
- crab-event - 事件系统
- crab-storage - 存储抽象

#### Scenario: 模块命名一致性
- **WHEN** 开发者查看架构文档
- **THEN** 应只看到 crab-* 模块命名
- **AND** 不应看到 augur-* 模块命名

### Requirement: 统一的客户端命名规范
系统 SHALL 使用 oh-my-* 作为客户端应用命名前缀：
- oh-my-crab - 个人形态客户端
- oh-my-empire - 帝国形态客户端
- oh-my-waifu - 角色形态客户端
- oh-my-planet - 星球形态客户端
- oh-my-home - 家庭形态客户端

#### Scenario: 客户端命名一致性
- **WHEN** 用户查看架构文档
- **THEN** 应看到所有客户端使用 oh-my-* 前缀
- **AND** 不应看到 on-my-claw 或其他命名

## MODIFIED Requirements

### Requirement: 架构层次简化
原三层架构（表现层、实现层、协议层）SHALL 简化为四层架构：
1. 应用层 - oh-my-* 系列应用
2. 核心层 - crab-* 核心业务模块
3. 基础设施层 - crab-* 基础设施模块
4. 协议层 - skynet-* 协议模块

### Requirement: 移除 augur-* 相关内容
所有 augur-* 模块描述 SHALL 被移除或替换为对应的 crab-* 模块：
- augur-agent → crab-agent
- augur-orchestrator → crab-orchestrator（或合并到 crab-agent）
- augur-organization → crab-organization
- augur-skill → crab-skill
- augur-memory → crab-memory
- augur-persistence → crab-database
- augur-file-system → crab-storage
- augur-types → crab-types

## REMOVED Requirements

### Requirement: ai-company 联合端概念
**Reason**: ai-company 作为独立概念已过时，其功能由各 oh-my-* 应用共享 crab-* 模块实现
**Migration**: 将 ai-company 相关描述迁移到各 oh-my-* 应用的共享特性说明

### Requirement: 前后端多对多架构描述
**Reason**: 所有 oh-my-* 应用采用相同的前后端一对多架构
**Migration**: 统一描述为前后端一对多架构，每个应用独立部署
