# Ironclaw 重构至 Opencrab V2 - Product Requirement Document

## Overview
- **Summary**: 将 ironclaw 项目从单体架构完整重构为模块化的 cargo workspace 架构，参考 plot.rs、wae、game-gpt 的优秀实践，保持功能 100% 一致。
- **Purpose**: 解决 ironclaw 架构混乱问题，提高代码可维护性、可测试性和可扩展性，便于团队协作和功能迭代。
- **Target Users**: 开发团队、维护者、贡献者

## Goals
- 使用 cargo workspace 管理多个独立的功能模块，已有基础框架，需完善
- 完整移植 ironclaw 的所有代码到 opencrab 的对应模块
- 创建完整的 backends/opencrab 主库，包含完整的 server bin 和 opencrab.exe 功能
- 保持与原 ironclaw 100% 的功能兼容性
- 遵循 plot.rs、wae、game-gpt 的架构风格
- 确保所有 public 的结构体、枚举、方法、字段都有文档注释，禁止使用后置注释

## Non-Goals (Out of Scope)
- 不修改 ironclaw 原项目代码
- 不添加新功能（仅重构）
- 不改变对外公开的 API 接口
- 不进行性能优化（除非重构过程中必须）

## Background & Context
- ironclaw 是一个功能强大的 AI 助手项目，但所有代码集中在一个 crate 中，导致依赖关系复杂、编译慢、难以测试
- opencrab 项目已经有了很好的基础架构，包括完整的 cargo workspace 配置、模块化的 crate 结构（crab-* 系列）
- plot.rs、wae、game-gpt 提供了优秀的模块化架构参考
- 当前已有多个基础 crate，但代码移植不完整，主程序功能缺失
- 需要完整移植 ironclaw 的 src/ 下所有模块，并完善各个 crab-* 库

## Functional Requirements
- **FR-1**: 完善 cargo workspace 配置，确保所有模块正确构建
- **FR-2**: 完整移植 ironclaw/src/ 所有代码到对应的 crab-* crates
- **FR-3**: 创建 backends/crab-agent 模块（目前是 temp 目录，需要完善）
- **FR-4**: 完善 backends/opencrab 主库，作为统一入口
- **FR-5**: 完整实现 opencrab.exe 二进制文件，功能与 ironclaw.exe 完全一致
- **FR-6**: 所有 public 结构体、枚举、方法、字段都有文档注释
- **FR-7**: 禁止使用后置注释
- **FR-8**: 保持原 ironclaw 的所有功能特性

## Non-Functional Requirements
- **NFR-1**: 编译速度提升，增量编译更快
- **NFR-2**: 模块间依赖清晰，无循环依赖
- **NFR-3**: 代码可测试性提升，独立模块可单独测试
- **NFR-4**: 遵循 Rust 社区最佳实践
- **NFR-5**: 文档完整，可通过 `cargo doc` 生成

## Constraints
- **Technical**: Rust 1.92+, Cargo workspace
- **Business**: 保持功能完全一致
- **Dependencies**: 保留原 ironclaw 的所有依赖
- **Architecture**: 参考 wae 和 game-gpt 的模块化架构

## Assumptions
- ironclaw 现有的测试可以复用
- 功能拆分可以按现有模块边界进行
- opencrab 现有的基础模块可以整合
- crab-agent.temp 可以作为基础进行完善

## Acceptance Criteria

### AC-1: Cargo Workspace 配置正确
- **Given**: 根目录 Cargo.toml
- **When**: 运行 `cargo build --workspace`
- **Then**: 整个 workspace 编译成功，无错误
- **Verification**: `programmatic`

### AC-2: 所有模块完整移植
- **Given**: backends/ 目录下的所有 crab-* crates
- **When**: 检查 ironclaw/src/ 与 crab-* crates 的对应关系
- **Then**: 所有功能模块都已完整移植，代码覆盖率 100%
- **Verification**: `human-judgment`

### AC-3: crab-agent 模块完善
- **Given**: backends/crab-agent 模块
- **When**: 检查 crab-agent 的功能完整性
- **Then**: 包含完整的 agent 功能，与 ironclaw/src/agent/ 一致
- **Verification**: `human-judgment`

### AC-4: 主库功能完整
- **Given**: backends/opencrab
- **When**: 运行 `cargo run --bin opencrab`
- **Then**: 启动与 ironclaw 相同的服务，所有命令可用
- **Verification**: `programmatic`

### AC-5: 文档注释完整
- **Given**: 所有 public API
- **When**: 运行 `cargo doc --no-deps --open`
- **Then**: 所有 public 项都有文档注释
- **Verification**: `human-judgment`

### AC-6: 无后置注释
- **Given**: 所有源文件
- **When**: 检查代码风格
- **Then**: 没有使用后置注释
- **Verification**: `programmatic`

### AC-7: 功能一致
- **Given**: opencrab 二进制
- **When**: 运行 ironclaw 的所有功能测试
- **Then**: 所有测试通过
- **Verification**: `programmatic`

## Open Questions
- [ ] 确定 crab-channels 模块是否需要包含完整的 web 通道实现（包括 web server, websocket 等）
- [ ] 确认 crab-tools 模块是否需要包含完整的内置工具、MCP 和 WASM 工具支持
