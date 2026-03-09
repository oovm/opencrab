# 修复 OpenCrab 上下游问题并添加测试 - 产品需求文档

## Overview
- **Summary**: 修复 OpenCrab 与 WAE 框架的集成问题，将所有错误类型统一为 WaeError，并在 opencrab 模块中添加完整的测试套件，确保系统真正可用。
- **Purpose**: 解决当前架构重构后出现的编译错误和错误类型不一致问题，确保 OpenCrab 能够正常编译和运行。
- **Target Users**: OpenCrab 维护者、开发者、测试人员

## Goals
- 修复 WAE 框架中的错误类型问题，确保与 OpenCrab 兼容
- 将 OpenCrab 中所有错误类型统一为 WaeError
- 在 opencrab 模块中添加完整的测试套件
- 确保整个项目能够正常编译和通过测试
- 提供清晰的错误处理和转换机制

## Non-Goals (Out of Scope)
- 不重构 WAE 框架的核心架构
- 不改变 OpenCrab 核心业务逻辑
- 不添加新的功能特性
- 不修改 Skynet 协议层

## Background & Context
### 当前问题
1. **WAE 框架错误类型问题**：wae-storage 和 wae-database 模块存在 API 不匹配问题
2. **错误类型不统一**：OpenCrab 仍然使用自己的 Error 类型，而不是 WaeError
3. **缺少测试**：opencrab 模块没有完整的测试套件
4. **编译失败**：由于上述问题，项目无法正常编译

### 错误类型架构
```
WAE 框架: WaeError (中心化错误类型)
    ↓
OpenCrab: 应该完全使用 WaeError
    ↓
应用层: 统一的错误处理
```

## Functional Requirements
- **FR-1**: 修复 WAE 框架中 wae-storage 和 wae-database 模块的错误 API 问题
- **FR-2**: 将 crab-types 中的 Error 类型替换为 WaeError
- **FR-3**: 更新所有核心业务模块使用 WaeError
- **FR-4**: 在 opencrab 模块中添加单元测试
- **FR-5**: 在 opencrab 模块中添加集成测试
- **FR-6**: 确保整个项目能够正常编译

## Non-Functional Requirements
- **NFR-1**: 错误处理保持向后兼容
- **NFR-2**: 测试覆盖率达到 80% 以上
- **NFR-3**: 编译时间不显著增加
- **NFR-4**: 错误类型转换开销最小化

## Constraints
- **Technical**:
  - 必须使用 Rust 1.75+
  - 必须保持与现有代码的兼容性
  - 必须使用 WAE 框架的 WaeError 类型
- **Business**:
  - 不能破坏现有功能
  - 必须在合理时间内完成
- **Dependencies**:
  - 依赖 WAE 框架的修复
  - 依赖现有核心业务模块

## Assumptions
- WAE 框架的错误 API 问题可以修复
- 错误类型转换不会引入性能问题
- 现有核心业务模块可以平滑迁移到 WaeError
- 测试可以验证系统的正确性

## Acceptance Criteria

### AC-1: WAE 框架错误修复
- **Given**: WAE 框架的 wae-storage 和 wae-database 模块有错误 API 问题
- **When**: 修复这些问题
- **Then**: WAE 框架能够正常编译
- **Verification**: `programmatic`
- **Notes**: 验证通过 cargo build 成功

### AC-2: crab-types 错误类型统一
- **Given**: crab-types 模块有自己的 Error 类型
- **When**: 将其替换为 WaeError
- **Then**: crab-types 使用 WaeError 作为唯一错误类型
- **Verification**: `programmatic`
- **Notes**: 验证通过检查代码确认

### AC-3: 核心业务模块错误统一
- **Given**: 核心业务模块使用 crab-types::Error
- **When**: 更新为使用 WaeError
- **Then**: 所有核心业务模块使用 WaeError
- **Verification**: `programmatic`
- **Notes**: 验证通过检查代码确认

### AC-4: opencrab 单元测试
- **Given**: opencrab 模块没有单元测试
- **When**: 添加单元测试
- **Then**: opencrab 模块有完整的单元测试套件
- **Verification**: `programmatic`
- **Notes**: 验证通过 cargo test 成功

### AC-5: opencrab 集成测试
- **Given**: opencrab 模块没有集成测试
- **When**: 添加入集成测试
- **Then**: opencrab 模块有完整的集成测试套件
- **Verification**: `programmatic`
- **Notes**: 验证通过 cargo test 成功

### AC-6: 项目正常编译
- **Given**: 完成所有修复和更新
- **When**: 运行 cargo build --all
- **Then**: 项目能够成功编译
- **Verification**: `programmatic`
- **Notes**: 验证通过编译命令的退出码确认

## Open Questions
- [ ] WAE 框架的错误 API 具体问题是什么？需要详细分析
- [ ] 是否需要保留 crab-types::Error 作为 WaeError 的别名以保持向后兼容？
- [ ] 测试应该覆盖哪些具体场景？
