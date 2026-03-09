# OpenCrab Crab-Config 模块完善 - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 完善 lib.rs 主模块，整合所有子模块
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 查看 ironclaw 的 config/mod.rs 完整结构
  - 更新 crab-config 的 lib.rs，添加所有子模块声明
  - 添加 Config 主结构体定义
  - 实现所有配置加载方法（from_env, from_db, from_env_with_toml, from_db_with_toml）
  - 实现环境变量注入相关函数
  - 确保所有 public 项都有文档注释
  - 确保不使用后置注释
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-6]
- **Test Requirements**:
  - `programmatic` TR-1.1: lib.rs 包含所有必要的子模块声明
  - `programmatic` TR-1.2: Config 结构体包含所有必要的子配置项
  - `programmatic` TR-1.3: 所有配置加载方法已实现
  - `human-judgement` TR-1.4: 所有 public 项有文档注释
  - `human-judgement` TR-1.5: 没有使用后置注释
- **Notes**: 需要仔细对照 ironclaw 的实现，确保功能一致

## [ ] Task 2: 检查和完善所有子模块
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 检查每个子模块（agent.rs, builder.rs, channels.rs, database.rs 等）
  - 确保每个子模块与 ironclaw 的对应模块功能一致
  - 确保所有 public 结构体、枚举、方法、字段都有文档注释
  - 确保不使用后置注释
  - 确保所有子模块都有必要的测试
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3]
- **Test Requirements**:
  - `human-judgement` TR-2.1: 每个子模块功能完整
  - `human-judgement` TR-2.2: 所有 public 项有文档注释
  - `human-judgement` TR-2.3: 没有使用后置注释
- **Notes**: 需要逐个检查 17 个子模块

## [ ] Task 3: 更新 Cargo.toml 依赖
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 检查 crab-config 的 Cargo.toml
  - 确保正确依赖 crab-types
  - 添加任何缺失的依赖（对照 ironclaw 的 Cargo.toml）
- **Acceptance Criteria Addressed**: [AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: Cargo.toml 正确依赖 crab-types
  - `programmatic` TR-3.2: 所有必要的依赖都已添加
- **Notes**: 确保不引入不必要的依赖

## [ ] Task 4: 运行测试并修复问题
- **Priority**: P0
- **Depends On**: Task 1, Task 2, Task 3
- **Description**: 
  - 运行 `cargo test -p crab-config`
  - 修复所有测试失败
  - 确保所有测试通过
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-4.1: `cargo test -p crab-config` 所有测试通过
- **Notes**: 可能需要修复类型错误、依赖问题等

## [ ] Task 5: 最终验证和代码审查
- **Priority**: P1
- **Depends On**: Task 4
- **Description**: 
  - 最终验证所有功能与 ironclaw 一致
  - 检查文档注释的完整性
  - 确认没有使用后置注释
  - 确认配置加载功能完整
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5, AC-6]
- **Test Requirements**:
  - `human-judgement` TR-5.1: 所有功能完整且与 ironclaw 一致
  - `human-judgement` TR-5.2: 文档注释完整
  - `human-judgement` TR-5.3: 没有使用后置注释
- **Notes**: 这是最后的质量检查
