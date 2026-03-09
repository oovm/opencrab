# Ironclaw 重构至 Opencrab - Product Requirement Document

## Overview
- **Summary**: 将 ironclaw 项目从单体架构重构为模块化的 cargo workspace 架构，参考 plot.rs、wae、game-gpt 的优秀实践，保持功能完全一致。
- **Purpose**: 解决 ironclaw 架构混乱问题，提高代码可维护性、可测试性和可扩展性，便于团队协作和功能迭代。
- **Target Users**: 开发团队、维护者、贡献者

## Goals
- 使用 cargo workspace 管理多个独立的功能模块
- 拆分功能为独立的 crates（backends/*）
- 创建主库 opencrab 作为入口点和二进制文件
- 保持与原 ironclaw 100% 的功能兼容性
- 遵循 plot.rs、wae、game-gpt 的架构风格

## Non-Goals (Out of Scope)
- 不修改 ironclaw 原项目代码
- 不添加新功能（仅重构）
- 不改变 API 接口
- 不进行性能优化（除非重构过程中必须）

## Background & Context
- ironclaw 是一个功能强大的 AI 助手项目，但所有代码集中在一个 crate 中，导致依赖关系复杂、编译慢、难以测试
- opencrab 项目已经开始了初步的模块化尝试，有几个基础的 crab-* crates
- plot.rs、wae、game-gpt 提供了优秀的模块化架构参考
- 使用 workspace.dependencies 统一管理依赖版本
- 清晰的模块边界和职责分离

## Functional Requirements
- **FR-1**: 完整的 cargo workspace 配置
- **FR-2**: 功能拆分到独立的 backends/* crates
- **FR-3**: 主库 backends/opencrab 作为统一入口
- **FR-4**: 二进制文件 opencrab.exe（与 ironclaw.exe 功能一致）
- **FR-5**: 所有 public 结构体、枚举、方法、字段都有文档注释
- **FR-6**: 禁止使用后置注释

## Non-Functional Requirements
- **NFR-1**: 编译速度显著提升（增量编译更快）
- **NFR-2**: 模块间依赖清晰（无循环依赖）
- **NFR-3**: 代码可测试性提升（独立模块可单独测试）
- **NFR-4**: 遵循 Rust 社区最佳实践

## Constraints
- **Technical**: Rust 1.92+, Cargo workspace
- **Business**: 保持功能完全一致
- **Dependencies**: 保留原 ironclaw 的所有依赖

## Assumptions
- ironclaw 现有的测试可以复用
- 功能拆分可以按现有模块边界进行
- opencrab 现有的基础模块可以整合

## Acceptance Criteria

### AC-1: Cargo Workspace 配置正确
- **Given**: 根目录 Cargo.toml
- **When**: 运行 `cargo build`
- **Then**: 整个 workspace 编译成功
- **Verification**: `programmatic`

### AC-2: 所有模块正确拆分
- **Given**: backends/ 目录
- **When**: 检查模块划分
- **Then**: 每个功能模块都在独立的 crate 中，依赖关系清晰
- **Verification**: `human-judgment`

### AC-3: 主库功能完整
- **Given**: backends/opencrab
- **When**: 运行 `cargo run --bin opencrab`
- **Then**: 启动与 ironclaw 相同的服务
- **Verification**: `programmatic`

### AC-4: 文档注释完整
- **Given**: 所有 public API
- **When**: 运行 `cargo doc --no-deps --open`
- **Then**: 所有 public 项都有文档注释
- **Verification**: `human-judgment`

### AC-5: 无后置注释
- **Given**: 所有源文件
- **When**: 检查代码风格
- **Then**: 没有使用后置注释
- **Verification**: `programmatic`

### AC-6: 功能一致
- **Given**: opencrab 二进制
- **When**: 运行 ironclaw 的所有功能测试
- **Then**: 所有测试通过
- **Verification**: `programmatic`

## Open Questions
- [ ] 确定具体的模块拆分方案（需要进一步分析 ironclaw 的模块依赖）
