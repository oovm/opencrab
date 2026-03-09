# 添加中文 README 并清理 ironclaw 引用 - 产品需求文档

## Overview
- **Summary**: 为 OpenCrab 项目的所有 crate 添加中文 README 文档，并清理项目中所有提到 "ironclaw" 的地方
- **Purpose**: 改善中文用户的文档体验，并完成项目从 Ironclaw 到 OpenCrab 的品牌转换
- **Target Users**: OpenCrab 的开发人员、贡献者和中文用户

## Goals
- 为指定的 25 个 crate 各添加一个完整的中文 README
- 清理项目中所有提到 "ironclaw" 的地方（包括代码注释、文档字符串、配置文件等）
- README 文档应包含项目功能介绍、当前状态、维护指南等内容
- 确保所有 public API 已有文档注释（遵循项目现有规范）

## Non-Goals (Out of Scope)
- 不修改代码功能逻辑
- 不添加新的功能特性
- 不修改非中文文档
- 不重构现有代码结构

## Background & Context
- OpenCrab 是一个基于模块化架构的 AI 助手框架，由原 Ironclaw 项目重构而来
- 项目使用 Rust 语言开发，包含多个独立的 crate 模块
- 目前缺少完整的中文文档，且代码中仍有对原项目 "ironclaw" 的引用
- 项目已有规范：所有 public 的结构体、枚举、方法、字段都需要文档注释

## Functional Requirements
- **FR-1**: 为以下 25 个 crate 各添加一个中文 README.md：
  - crab-agent
  - crab-channels
  - crab-config
  - crab-context
  - crab-database
  - crab-estimation
  - crab-evaluation
  - crab-extensions
  - crab-history
  - crab-hooks
  - crab-llm
  - crab-observability
  - crab-orchestrator
  - crab-pairing
  - crab-registry
  - crab-safety
  - crab-sandbox
  - crab-secrets
  - crab-skills
  - crab-tools
  - crab-types
  - crab-workspace
  - opencrab
- **FR-2**: README 文档应包含以下章节：
  - 项目介绍
  - 核心功能
  - 当前状态
  - 维护指南（如何贡献、如何测试等）
- **FR-3**: 清理项目中所有提到 "ironclaw" 的文件，包括但不限于：
  - Cargo.toml
  - 源代码文件中的注释和文档字符串
  - 配置文件
  - 环境变量引用

## Non-Functional Requirements
- **NFR-1**: README 文档应简洁明了，易于理解
- **NFR-2**: 清理 ironclaw 引用时不应破坏代码功能
- **NFR-3**: 所有更改应保持项目的代码风格一致性

## Constraints
- **Technical**: 仅修改文档和注释，不改变代码逻辑
- **Business**: 使用中文编写 README 文档
- **Dependencies**: 无外部依赖

## Assumptions
- 项目的主要功能和架构不会在短期内发生重大变化
- 所有 crate 的 Cargo.toml 已有基本描述信息
- 代码库中已有的文档注释规范应继续遵循

## Acceptance Criteria

### AC-1: 所有目标 crate 都有中文 README
- **Given**: 项目存在 25 个目标 crate
- **When**: 任务完成后
- **Then**: 每个 crate 目录下都有一个 README.md 文件，且内容为中文
- **Verification**: `programmatic`
- **Notes**: 检查文件存在性和内容语言

### AC-2: README 包含必要章节
- **Given**: crate 已有 README.md 文件
- **When**: 查看 README 内容
- **Then**: README 包含项目介绍、核心功能、当前状态、维护指南等章节
- **Verification**: `human-judgment`

### AC-3: 清理所有 ironclaw 引用
- **Given**: 项目代码库
- **When**: 搜索 "ironclaw"（不区分大小写）
- **Then**: 没有找到相关匹配结果
- **Verification**: `programmatic`
- **Notes**: 使用 grep 工具验证

### AC-4: 代码功能不受影响
- **Given**: 已完成 ironclaw 引用清理
- **When**: 运行项目构建
- **Then**: 项目可以成功编译和运行
- **Verification**: `programmatic`

## Open Questions
- 无
