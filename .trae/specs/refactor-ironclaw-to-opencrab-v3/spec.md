# OpenCrab 架构重构 - Product Requirement Document

## Overview
- **Summary**: 将 Ironclaw 项目从混乱的单体架构重构为 OpenCrab 项目的模块化架构，参考 WAE 和 GameGPT 等优秀项目的设计模式，使用 Cargo Workspace 管理，将功能拆分重组，最终实现功能完全一致但架构更清晰可维护的 OpenCrab。
- **Purpose**: 解决 Ironclaw 项目架构混乱的问题，提高代码可维护性、可扩展性和可测试性，为未来功能演进奠定坚实的基础。
- **Target Users**: OpenCrab 开发者、维护者、贡献者。

## Goals
- 使用 Cargo Workspace 管理项目，将功能拆分为多个独立的 crate
- 重构代码结构，实现清晰的模块划分和依赖关系
- 保持与 Ironclaw 完全一致的功能
- 将所有注释和图示改为中文
- 参考 WAE 和 GameGPT 的优秀架构设计模式

## Non-Goals (Out of Scope)
- 修改或添加新的功能特性
- 修改 API 接口和行为
- 修改数据库结构和迁移
- 修改外部依赖库版本（除非必要）

## Background & Context
- Ironclaw 项目是一个功能强大的 AI 助手框架，支持多通道交互、工具调用、LLM 集成等功能
- 但是 Ironclaw 的架构非常混乱，所有功能都在一个巨大的 crate 中，难以维护和扩展
- WAE 和 GameGPT 项目展示了优秀的 Rust 项目架构设计，使用 Cargo Workspace 管理，功能模块化清晰
- OpenCrab 项目已经有部分模块化的基础，但需要进一步完善和完整重构

## Functional Requirements
- **FR-1**: 建立完整的 Cargo Workspace 结构，包含 backends/opencrab 主库
- **FR-2**: 将 Ironclaw 的所有功能模块拆分到独立的 crate 中
- **FR-3**: 实现 backends/opencrab 主库，包含 server bin 和 opencrab.exe
- **FR-4**: 保持所有功能与 Ironclaw 完全一致
- **FR-5**: 将所有注释和图示改为中文

## Non-Functional Requirements
- **NFR-1**: 代码结构清晰，模块职责单一，依赖关系明确
- **NFR-2**: 编译成功，无警告
- **NFR-3**: 所有测试通过
- **NFR-4**: 文档注释完整，所有 public 的结构体、枚举、方法、字段都有文档注释

## Constraints
- **Technical**: 使用 Rust 语言，遵循 Cargo Workspace 最佳实践
- **Business**: 保持与 Ironclaw 完全相同的功能
- **Dependencies**: 保持与 Ironclaw 相同的外部依赖库版本

## Assumptions
- Ironclaw 项目的所有功能都是稳定且正确的
- 用户不需要添加新功能，只需要重构架构
- OpenCrab 目录可以完全重写（除了必要的保留文件）

## Acceptance Criteria

### AC-1: Cargo Workspace 结构完整
- **Given**: 项目根目录
- **When**: 查看 Cargo.toml 和目录结构
- **Then**: 可以看到完整的 Cargo Workspace 配置，包含 backends 目录下的所有 crate
- **Verification**: `programmatic`

### AC-2: 功能模块拆分完整
- **Given**: backends 目录
- **When**: 查看各个 crate 的内容
- **Then**: 所有 Ironclaw 的功能都被正确拆分到相应的 crate 中
- **Verification**: `programmatic`

### AC-3: 主库功能完整
- **Given**: backends/opencrab crate
- **When**: 查看 lib.rs 和 bin/opencrab.rs
- **Then**: 主库正确导出所有必要的模块，server bin 可以正常运行
- **Verification**: `programmatic`

### AC-4: 功能一致性
- **Given**: 重构完成的 OpenCrab
- **When**: 运行所有 Ironclaw 的功能测试
- **Then**: 所有测试通过，功能行为与 Ironclaw 完全一致
- **Verification**: `programmatic`

### AC-5: 中文注释完整
- **Given**: 所有 Rust 源文件
- **When**: 检查代码注释
- **Then**: 所有注释和图示都是中文，所有 public 的结构体、枚举、方法、字段都有文档注释
- **Verification**: `human-judgment`

### AC-6: 编译成功
- **Given**: 重构完成的 OpenCrab
- **When**: 运行 cargo build --release
- **Then**: 编译成功，无警告
- **Verification**: `programmatic`

## Open Questions
- 无
