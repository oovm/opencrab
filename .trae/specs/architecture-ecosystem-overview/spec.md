# 架构总设计文档 Spec

## Why

当前 opencrab 项目的架构文档虽然详细描述了单机智能体节点的架构，但缺少宏观的架构总设计文档来清晰说明整个生态系统中各个项目之间的关系。需要创建一个架构总设计文档，让维护者能够快速理解整个生态系统的架构。

## What Changes

- 新增 `documentation/zh-hans/maintainer/architecture/ecosystem-overview.md` - 生态系统总览文档
- 更新 `documentation/zh-hans/maintainer/index.md` - 添加新文档链接

## Impact

- Affected specs: 无
- Affected code: 仅文档文件

## ADDED Requirements

### Requirement: 生态系统架构文档

系统 SHALL 提供完整的生态系统架构文档，清晰说明各项目之间的关系。

#### Scenario: 维护者查看架构总览

- **WHEN** 维护者打开架构总览文档
- **THEN** 能够清晰理解整个生态系统的架构层次
- **AND** 能够理解各项目的定位和关系
- **AND** 能够理解 wae 运行时与各应用的关系

### Requirement: 项目关系说明

系统 SHALL 清晰说明各项目之间的关系，包括但不限于：

- OpenClaw 产品形态说明
- Skynet 协议的定位
- OpenCrab 单机智能体节点的定位
- ai-company 联合端的定位
- ai-empire、ai-waifu、ai-planet 等应用形态的定位
- wae 运行时的定位

#### Scenario: 理解项目定位

- **WHEN** 维护者阅读项目关系说明
- **THEN** 能够理解每个项目在生态系统中的位置
- **AND** 能够理解项目之间的依赖关系
- **AND** 能够理解前后端一对多/多对多的架构模式

### Requirement: 架构层次图示

系统 SHALL 提供清晰的架构层次图示，展示各层次的依赖关系。

#### Scenario: 查看架构图

- **WHEN** 维护者查看架构图
- **THEN** 能够直观理解整个生态系统的层次结构
- **AND** 能够理解协议层、运行时层、应用层的关系
