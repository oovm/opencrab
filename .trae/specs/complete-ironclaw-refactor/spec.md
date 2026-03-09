# OpenCrab 完整重构 - Product Requirement Document

## Overview
- **Summary**: 将 ironclaw 项目完全重构为 opencrab，采用优秀的模块化架构设计，使用 cargo workspace 管理，保持功能完全一致，同时将所有文档和注释翻译为中文。
- **Purpose**: 解决 ironclaw 架构混乱的问题，参考 wae、game-gpt 等项目的优秀架构，创建一个模块化、可维护、可扩展的 AI 助手框架。
- **Target Users**: 开发者、AI 助手用户、系统集成者

## Goals
- [ ] 采用 cargo workspace 管理，将功能拆分为独立的 crate
- [ ] 保持与 ironclaw 完全一致的功能
- [ ] 使用中文注释和文档
- [ ] 保持 backends/opencrab 作为主库，提供 server bin (opencrab.exe)
- [ ] 参考 wae 和 game-gpt 的模块化架构设计

## Non-Goals (Out of Scope)
- 不修改原 ironclaw 项目
- 不添加新功能
- 不改变用户交互界面
- 不修改 API 接口

## Background & Context
- ironclaw 是一个功能强大但架构混乱的 AI 助手项目
- wae 和 game-gpt 展示了优秀的模块化架构设计
- opencrab 已经有初步的模块化框架，但功能不完整
- 需要将 ironclaw 的所有功能完整迁移到 opencrab 的模块化架构中

## Functional Requirements
- **FR-1**: 完整实现 ironclaw 的所有核心功能
- **FR-2**: 使用 cargo workspace 管理，所有模块独立可编译
- **FR-3**: 提供完整的 opencrab.exe 二进制文件
- **FR-4**: 所有公共结构体、枚举、方法、字段都有中文文档注释
- **FR-5**: 保持与 ironclaw 100% 功能兼容性

## Non-Functional Requirements
- **NFR-1**: 编译通过且无警告
- **NFR-2**: 模块化架构清晰，依赖关系合理
- **NFR-3**: 代码可维护性良好
- **NFR-4**: 中文文档完整清晰

## Constraints
- **Technical**: Rust 2024 edition, 使用现有的依赖版本
- **Business**: 保持功能完全一致
- **Dependencies**: 保留 ironclaw 的所有依赖

## Assumptions
- ironclaw 的所有功能都是需要保留的
- opencrab 的现有模块化框架可以作为基础
- 所有公共 API 保持兼容

## Acceptance Criteria

### AC-1: Cargo Workspace 架构
- **Given**: 项目根目录
- **When**: 查看 Cargo.toml
- **Then**: 正确配置 cargo workspace，包含所有必要的 backends crate
- **Verification**: `programmatic`

### AC-2: 功能完整性
- **Given**: 完整的 opencrab 项目
- **When**: 运行 opencrab 所有功能
- **Then**: 功能与 ironclaw 完全一致
- **Verification**: `programmatic`

### AC-3: 中文文档注释
- **Given**: 所有公共 API
- **When**: 查看代码
- **Then**: 所有公共结构体、枚举、方法、字段都有中文文档注释
- **Verification**: `human-judgment`

### AC-4: 编译成功
- **Given**: 完整的 opencrab 项目
- **When**: 运行 cargo build --release
- **Then**: 编译成功，无警告
- **Verification**: `programmatic`

### AC-5: 主二进制文件
- **Given**: 编译完成的项目
- **When**: 查看 target/release
- **Then**: opencrab.exe 存在且可正常运行
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要保留 ironclaw 的所有测试？
