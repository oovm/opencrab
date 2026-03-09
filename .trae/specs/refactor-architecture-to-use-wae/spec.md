# OpenCrab 架构重构 - 产品需求文档

## Overview
- **Summary**: 重构 OpenCrab 架构，停止重复造轮子，将基础设施层的职责移交给 WAE 框架，让 OpenCrab 专注于智能体核心业务逻辑。
- **Purpose**: 解决当前架构中 crab-* 基础设施模块与 wae-* 模块职责重叠的问题，遵循单一职责原则，提高代码复用性和可维护性。
- **Target Users**: OpenCrab 维护者、开发者、贡献者

## Goals
- 移除所有重复的 crab-* 基础设施模块（crab-config, crab-database, crab-cache, crab-https, crab-effect, crab-queue, crab-event, crab-storage）
- 让 OpenCrab 的核心层（crab-agent, crab-skill, crab-memory, crab-chat, crab-tool, crab-scheduler, crab-workspace）直接依赖 WAE
- 更新架构文档，反映新的分层结构
- 确保所有现有核心业务模块能够正常工作
- 简化项目依赖关系，降低维护成本

## Non-Goals (Out of Scope)
- 不修改 WAE 框架本身
- 不重构核心业务逻辑（只调整依赖）
- 不改变 Skynet 协议层
- 不重构前端应用层
- 不改变用户可见的功能

## Background & Context
### 当前架构问题
OpenCrab 目前的架构设计存在职责重叠问题：
1. **基础设施层重复**：crab-* 基础设施模块（crab-config, crab-database, crab-cache 等）与 wae-* 模块功能完全重叠
2. **违背单一职责原则**：OpenCrab 既是智能体框架，又是基础设施框架
3. **维护成本高**：需要同时维护两套基础设施模块
4. **学习曲线陡峭**：开发者需要同时理解两套基础设施

### 参考设计
- **IronClaw**：是一个单体应用，核心是智能体功能，不包含基础设施框架
- **WAE**：是一个完整的微服务优先的 Rust 异步框架，提供所有必要的基础设施
- **正确的架构关系**：OpenCrab 应该是 WAE 之上的智能体应用，而不是重新实现基础设施

### 项目关系
```
应用层 (oh-my-*)
    ↓
OpenCrab 核心层 (crab-agent, crab-skill, ...)
    ↓
WAE 基础设施层 (wae-config, wae-database, ...)
    ↓
协议层 (Skynet)
```

## Functional Requirements
- **FR-1**: 移除所有重复的 crab-* 基础设施模块
- **FR-2**: 更新核心业务模块依赖，从 crab-* 改为 wae-*
- **FR-3**: 更新 Cargo.toml 工作空间配置
- **FR-4**: 更新架构文档，反映新的分层结构
- **FR-5**: 确保核心业务模块能够正常编译

## Non-Functional Requirements
- **NFR-1**: 重构后编译时间不显著增加
- **NFR-2**: 重构后运行时性能不下降
- **NFR-3**: 代码变更应保持最小化，仅修改必要的部分
- **NFR-4**: 所有公共 API 应保持向后兼容（核心业务层）

## Constraints
- **Technical**:
  - 必须使用 Rust 1.75+
  - 必须使用 WAE 作为基础设施框架
  - 必须保持与现有 Skynet 协议的兼容性
- **Business**:
  - 不能破坏现有功能
  - 不能增加用户可见的复杂性
- **Dependencies**:
  - 依赖 WAE 框架的稳定版本
  - 依赖现有核心业务模块

## Assumptions
- WAE 框架提供了 OpenCrab 所需的所有基础设施功能
- 核心业务模块与基础设施模块的耦合是松耦合的
- 修改依赖关系不会破坏核心业务逻辑
- 现有测试可以验证重构的正确性

## Acceptance Criteria

### AC-1: 基础设施模块移除
- **Given**: 当前存在 crab-config, crab-database, crab-cache, crab-https, crab-effect, crab-queue, crab-event, crab-storage 模块
- **When**: 执行架构重构
- **Then**: 这些模块从 backends/ 目录中完全移除
- **Verification**: `programmatic`
- **Notes**: 验证通过检查目录结构确认

### AC-2: 核心模块依赖更新
- **Given**: 核心业务模块 (crab-agent, crab-skill, crab-memory, crab-chat, crab-tool, crab-scheduler, crab-workspace) 依赖 crab-* 基础设施模块
- **When**: 执行架构重构
- **Then**: 这些模块的 Cargo.toml 中的依赖从 crab-* 改为 wae-*
- **Verification**: `programmatic`
- **Notes**: 验证通过检查 Cargo.toml 文件确认

### AC-3: 项目正常编译
- **Given**: 完成架构重构
- **When**: 运行 cargo build --all
- **Then**: 项目能够成功编译，无错误
- **Verification**: `programmatic`
- **Notes**: 验证通过编译命令的退出码确认

### AC-4: 架构文档更新
- **Given**: 完成架构重构
- **When**: 查看 architecture/index.md 和 master-plan.md
- **Then**: 文档反映新的分层结构，明确 OpenCrab 依赖 WAE
- **Verification**: `human-judgment`
- **Notes**: 验证通过人工审阅文档内容确认

### AC-5: 核心类型保留
- **Given**: 当前存在 crab-types 模块
- **When**: 执行架构重构
- **Then**: crab-types 模块保留，但只包含 OpenCrab 特有的类型定义，基础设施类型移到 WAE
- **Verification**: `programmatic`
- **Notes**: 验证通过检查 crab-types 的内容确认

## Open Questions
- [ ] crab-types 中哪些类型是 OpenCrab 特有，哪些应该移到 WAE？
- [ ] 是否需要创建一个 opencrab-types 模块来替代 crab-types，专门用于 OpenCrab 特有的类型？
- [ ] 如何平滑过渡，确保现有代码能够正常工作？
