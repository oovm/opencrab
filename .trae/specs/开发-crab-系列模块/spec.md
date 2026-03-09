# 开发 Crab 系列模块 - Product Requirement Document

## Overview
- **Summary**: 基于架构文档设计，在 `e:\灵之镜有限公司\opencrab\backends` 目录中开发 crab-* 系列模块，包括基础设施层和核心层模块，可参考和迁移 `e:\灵之镜有限公司\ai-company\backends` 中的 augur 系列代码。
- **Purpose**: 实现 OpenCrab 系统的完整后端架构，提供智能体节点的核心功能和基础设施能力。
- **Target Users**: OpenCrab 维护者和开发者。

## Goals
- 完成基础设施层所有 crab-* 模块的开发
- 完成核心层所有 crab-* 模块的开发
- 确保所有模块遵循架构文档设计原则
- 保持与 augur 系列代码的兼容性以便迁移
- 所有公共结构体、枚举、方法、字段都有完整文档注释

## Non-Goals (Out of Scope)
- 暂时不开发前端应用（on-my-claw, oh-my-crab）
- 暂时不实现 Skynet 协议集成
- 暂时不进行性能优化和生产环境部署

## Background & Context
- OpenCrab 项目采用五层架构：前端应用层 → 服务端层 → 核心层 → 基础设施层 → 协议层
- 基础设施层包括：crab-types, crab-config, crab-database, crab-cache, crab-https, crab-effect, crab-queue, crab-event, crab-storage
- 核心层包括：crab-agent, crab-skill, crab-memory, crab-chat, crab-tool, crab-scheduler, crab-workspace
- 可以参考和迁移 `ai-company/backends` 中的 augur 系列代码

## Functional Requirements
- **FR-1**: 实现基础设施层所有模块，提供抽象 trait 和默认实现
- **FR-2**: 实现核心层所有模块，提供智能体核心功能
- **FR-3**: 完善现有的 crab-types 模块，补充缺失的类型定义
- **FR-4**: 更新工作空间 Cargo.toml，添加新模块依赖
- **FR-5**: 确保所有模块可以正常编译和基本测试

## Non-Functional Requirements
- **NFR-1**: 所有公共 API 都有完整的文档注释
- **NFR-2**: 遵循 Rust 2024 版规范
- **NFR-3**: 使用 workspace 依赖管理
- **NFR-4**: 代码风格一致，遵循现有约定

## Constraints
- **Technical**: 使用 Rust 语言，基于现有工作空间结构
- **Business**: 优先实现基础设施层，再实现核心层
- **Dependencies**: 可以参考和迁移 augur 系列代码

## Assumptions
- augur 系列代码是可参考的成熟实现
- 架构文档中的设计是可行的
- 现有 crab-types 模块的基础是正确的

## Acceptance Criteria

### AC-1: 基础设施层模块完整实现
- **Given**: 工作空间配置正确
- **When**: 实现所有基础设施层模块
- **Then**: 所有模块可以正常编译，且包含完整的文档注释
- **Verification**: `programmatic`

### AC-2: 核心层模块完整实现
- **Given**: 基础设施层模块已完成
- **When**: 实现所有核心层模块
- **Then**: 所有模块可以正常编译，且包含完整的文档注释
- **Verification**: `programmatic`

### AC-3: 工作空间配置正确
- **Given**: 所有模块已创建
- **When**: 运行 `cargo build`
- **Then**: 整个工作空间可以正常编译通过
- **Verification**: `programmatic`

### AC-4: 代码文档完整
- **Given**: 所有模块已实现
- **When**: 检查代码文档
- **Then**: 所有公共结构体、枚举、方法、字段都有文档注释
- **Verification**: `human-judgment`

## Open Questions
- 是否需要完全重写还是直接迁移 augur 代码？（建议：参考设计，基于架构文档重新实现）
