# OpenCrab Crab-Config 模块完善 - Product Requirement Document

## Overview
- **Summary**: 完善 opencrab/backends/crab-config 模块，完整移植 ironclaw/src/config/ 模块的所有功能到 crab-config，确保与 OpenCrab 生态系统的兼容性。
- **Purpose**: 为 OpenCrab 框架提供完整、独立的配置管理模块，支持从多种来源（环境变量、TOML 文件、数据库）加载配置，并提供统一的配置接口。
- **Target Users**: OpenCrab 框架开发者、使用 OpenCrab 构建应用的开发者

## Goals
- 完整移植 ironclaw/src/config/ 模块的所有功能到 crab-config
- 确保依赖 crab-types
- 确保所有 public 的结构体、枚举、方法、字段都有文档注释
- 确保没有使用后置注释
- 确保 `cargo test -p crab-config` 能正常通过
- 确保配置加载功能完整，与 ironclaw 一致

## Non-Goals (Out of Scope)
- 不修改 ironclaw 项目的代码
- 不添加超出 ironclaw 原有功能的新特性
- 不重构 OpenCrab 其他模块
- 不修改配置模块的 API 语义（保持与 ironclaw 一致）

## Background & Context
- 项目已有基础的 crab-config 模块框架，包含多个配置子模块
- 需要移植 ironclaw 的完整配置管理功能，包括主配置结构体、配置加载逻辑、环境变量注入等
- 项目使用 pnpm workspace，依赖 crab-types
- 所有代码需要符合 OpenCrab 的代码规范（文档注释、无后置注释等）

## Functional Requirements
- **FR-1**: 提供完整的 Config 主结构体，包含所有子配置项
- **FR-2**: 支持从环境变量加载配置（from_env）
- **FR-3**: 支持从数据库加载配置（from_db）
- **FR-4**: 支持 TOML 配置文件覆盖
- **FR-5**: 提供环境变量注入机制（inject_llm_keys_from_secrets, inject_os_credentials）
- **FR-6**: 提供测试友好的配置构造方法（for_testing）
- **FR-7**: 提供 LLM 配置重新解析功能（re_resolve_llm）

## Non-Functional Requirements
- **NFR-1**: 所有 public 的结构体、枚举、方法、字段都必须有文档注释
- **NFR-2**: 禁止使用后置注释（// 注释放在代码行后面）
- **NFR-3**: 所有测试必须通过（cargo test -p crab-config）
- **NFR-4**: 配置加载性能与 ironclaw 相当
- **NFR-5**: 错误处理必须完善，提供清晰的错误信息

## Constraints
- **Technical**: 
  - 使用 Rust 语言
  - 依赖 crab-types 模块
  - 遵循 OpenCrab 的工作空间结构
  - 使用 pnpm workspace（非 npm）
- **Business**: 
  - 保持与 ironclaw 功能一致
  - 不引入破坏性变更
- **Dependencies**: 
  - crab-types 模块
  - 现有 ironclaw 配置模块作为参考

## Assumptions
- ironclaw 的配置模块是完整和正确的
- 现有的 crab-config 子模块基础代码是可用的
- crab-types 模块提供了必要的类型支持

## Acceptance Criteria

### AC-1: 完整移植 Config 结构体和加载方法
- **Given**: ironclaw 的 config/mod.rs 包含完整的 Config 结构体和加载方法
- **When**: 移植完成后
- **Then**: crab-config 包含相同功能的 Config 结构体和加载方法（from_env, from_db, from_env_with_toml, from_db_with_toml）
- **Verification**: `programmatic`

### AC-2: 所有 public 项有文档注释
- **Given**: 所有配置相关的代码
- **When**: 检查所有 public 的结构体、枚举、方法、字段
- **Then**: 每个 public 项都有适当的文档注释
- **Verification**: `human-judgment`

### AC-3: 无后置注释
- **Given**: 所有配置相关的代码
- **When**: 检查代码中的注释位置
- **Then**: 没有使用后置注释（// 注释放在代码行后面）
- **Verification**: `human-judgment`

### AC-4: 测试通过
- **Given**: 完整的 crab-config 模块
- **When**: 运行 `cargo test -p crab-config`
- **Then**: 所有测试通过
- **Verification**: `programmatic`

### AC-5: 依赖 crab-types
- **Given**: crab-config 的 Cargo.toml
- **When**: 检查依赖关系
- **Then**: 正确依赖 crab-types 模块
- **Verification**: `programmatic`

### AC-6: 环境变量注入功能完整
- **Given**: 完整的 crab-config 模块
- **When**: 检查 inject_llm_keys_from_secrets, inject_os_credentials, inject_single_var 等函数
- **Then**: 这些函数功能完整，与 ironclaw 一致
- **Verification**: `programmatic`

## Open Questions
- 无
