# 整理融合文档结构 Spec

## Why

当前 OpenCrab 文档存在两个主要问题：
1. **文档错位**：OpenCrab 文档中混入了大量 AI Company 特有的内容（如 `augur-*` 模块、`ai-company`/`ai-empire`/`ai-waifu` 前端应用等），这些应该放在 `ai-company` 项目中
2. **文档碎片化**：文档结构过于细碎，需要整理融合

## What Changes

### 移动到 ai-company 项目的文档
- `implementation-layer.md` - 描述 augur-* 模块，这是 ai-company 的实现层
- `presentation-layer.md` - 描述 ai-company/ai-empire/ai-waifu 等前端应用
- `security-model.md` - 明确说明是 "AI Company 作为 Skynet 协议上的一种子网实现"
- `protocol-layer.md` - 需要改写，当前描述的是 AI Company 视角

### 需要删除或合并的文档
- `architecture/index.md` 中的 AI Company 概念映射表应移到 ai-company
- concepts/ 目录下的文档需要评估是否属于 OpenCrab 还是 ai-company

### OpenCrab 文档应保留的内容
- 生态系统总览（ecosystem-overview.md）
- 架构总设计（master-plan.md）
- 核心层设计（core-layer.md）- crab-* 模块
- 基础设施层设计（infrastructure.md）
- 去中心化设计（decentralization.md）
- Skynet 协议文档（maintainer/skynet/）

## Impact

- Affected files: 
  - OpenCrab: `documentation/zh-hans/` 下多个文件
  - ai-company: `documentation/zh-hans/` 需要接收移动的文档

## ADDED Requirements

### Requirement: 文档归属正确

文档应该放在正确的项目中：
- OpenCrab 文档只描述 OpenCrab 相关内容（crab-* 模块、oh-my-crab 客户端等）
- ai-company 文档描述 ai-company 相关内容（augur-* 模块、ai-company/ai-empire 等前端）

#### Scenario: 查看 OpenCrab 架构文档

- **WHEN** 开发者阅读 OpenCrab 架构文档
- **THEN** 只看到 crab-* 模块和 OpenCrab 相关的设计
- **AND** 不会看到 augur-* 或 ai-company 前端应用的内容

### Requirement: 文档结构清晰

文档目录结构应该清晰、不过度碎片化。

#### Scenario: 浏览文档目录

- **WHEN** 开发者浏览文档目录
- **THEN** 能够快速理解文档结构
- **AND** 相关内容聚合在一起

## MODIFIED Requirements

### Requirement: ai-company 文档充实

ai-company 项目应该有完整的文档，包含从 OpenCrab 移动过来的相关内容。
