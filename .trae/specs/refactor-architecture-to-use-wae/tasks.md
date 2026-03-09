# OpenCrab 架构重构 - 实施计划

## [ ] Task 1: 分析和准备
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 详细分析所有 crab-* 模块的当前状态
  - 确认哪些模块是基础设施模块，哪些是核心业务模块
  - 分析核心业务模块与基础设施模块的依赖关系
  - 检查 WAE 框架是否提供了所需的所有功能
- **Acceptance Criteria Addressed**: [AC-1, AC-2]
- **Test Requirements**:
  - `programmatic` TR-1.1: 列出所有 backends/ 目录下的模块
  - `programmatic` TR-1.2: 分析每个模块的 Cargo.toml 依赖关系
  - `human-judgement` TR-1.3: 确认 WAE 框架功能覆盖情况
- **Notes**: 这是关键的准备工作，确保我们理解要做什么

## [ ] Task 2: 更新 Cargo.toml 工作空间配置
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 workspace.members 中移除基础设施模块
  - 从 workspace.default-members 中移除基础设施模块
  - 从 workspace.dependencies 中移除基础设施模块
  - 添加 WAE 框架的 workspace 依赖
- **Acceptance Criteria Addressed**: [AC-1, AC-3]
- **Test Requirements**:
  - `programmatic` TR-2.1: 验证 workspace.members 中不再包含基础设施模块
  - `programmatic` TR-2.2: 验证 workspace.dependencies 中包含 WAE 相关模块
- **Notes**: 确保正确配置路径依赖或 git 依赖

## [ ] Task 3: 重构 crab-types 模块
- **Priority**: P0
- **Depends On**: Task 2
- **Description**: 
  - 分析 crab-types 中的内容
  - 保留 OpenCrab 特有的类型定义
  - 移除基础设施相关的类型定义（这些应该来自 WAE）
  - 更新 crab-types 的依赖，从 crab-* 改为 wae-*
- **Acceptance Criteria Addressed**: [AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: crab-types 能够正常编译
  - `programmatic` TR-3.2: 确认只保留 OpenCrab 特有类型
- **Notes**: 仔细分析每个类型，确保不破坏现有代码

## [ ] Task 4: 更新核心业务模块依赖
- **Priority**: P0
- **Depends On**: Task 3
- **Description**: 
  - 更新 crab-agent 的 Cargo.toml
  - 更新 crab-skill 的 Cargo.toml
  - 更新 crab-memory 的 Cargo.toml
  - 更新 crab-chat 的 Cargo.toml
  - 更新 crab-tool 的 Cargo.toml
  - 更新 crab-scheduler 的 Cargo.toml
  - 更新 crab-workspace 的 Cargo.toml
  - 将所有 crab-* 基础设施依赖改为 wae-*
- **Acceptance Criteria Addressed**: [AC-2, AC-3]
- **Test Requirements**:
  - `programmatic` TR-4.1: 每个核心业务模块的 Cargo.toml 已更新
  - `programmatic` TR-4.2: 核心业务模块能够正常编译
- **Notes**: 逐个模块更新，确保每个模块都能正常工作

## [ ] Task 5: 更新核心业务模块代码
- **Priority**: P0
- **Depends On**: Task 4
- **Description**: 
  - 更新核心业务模块的导入语句（use 语句）
  - 从 `crab_config::` 改为 `wae_config::`
  - 从 `crab_database::` 改为 `wae_database::`
  - 从 `crab_cache::` 改为 `wae_cache::`
  - 从 `crab_https::` 改为 `wae_https::`
  - 从 `crab_effect::` 改为 `wae_effect::`
  - 从 `crab_queue::` 改为 `wae_queue::`
  - 从 `crab_event::` 改为 `wae_event::`
  - 从 `crab_storage::` 改为 `wae_storage::`
  - 确保 API 调用正确
- **Acceptance Criteria Addressed**: [AC-2, AC-3]
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有导入语句已更新
  - `programmatic` TR-5.2: 核心业务模块能够正常编译和链接
- **Notes**: 可能需要一些 API 适配工作

## [ ] Task 6: 移除基础设施模块
- **Priority**: P1
- **Depends On**: Task 5
- **Description**: 
  - 删除 crab-config 目录
  - 删除 crab-database 目录
  - 删除 crab-cache 目录
  - 删除 crab-https 目录
  - 删除 crab-effect 目录
  - 删除 crab-queue 目录
  - 删除 crab-event 目录
  - 删除 crab-storage 目录
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `programmatic` TR-6.1: 确认所有基础设施模块目录已删除
  - `programmatic` TR-6.2: 项目仍能正常编译
- **Notes**: 确保在删除前所有依赖都已更新

## [ ] Task 7: 更新架构文档
- **Priority**: P1
- **Depends On**: Task 6
- **Description**: 
  - 更新 architecture/index.md
  - 更新 master-plan.md
  - 更新 infrastructure.md
  - 更新 core-layer.md
  - 反映新的分层结构
  - 明确 OpenCrab 依赖 WAE
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `human-judgement` TR-7.1: 文档准确反映新架构
  - `human-judgement` TR-7.2: 文档清晰说明与 WAE 的关系
- **Notes**: 确保文档与实际代码一致

## [ ] Task 8: 全面编译测试
- **Priority**: P0
- **Depends On**: Task 7
- **Description**: 
  - 运行 cargo build --all
  - 运行 cargo test --all（如果有测试）
  - 修复任何编译错误
  - 确保所有模块正常工作
- **Acceptance Criteria Addressed**: [AC-3]
- **Test Requirements**:
  - `programmatic` TR-8.1: cargo build --all 成功
  - `programmatic` TR-8.2: cargo test --all 成功（如果有测试）
- **Notes**: 这是最终验证步骤

## [ ] Task 9: 更新 opencrab 主模块
- **Priority**: P1
- **Depends On**: Task 8
- **Description**: 
  - 更新 opencrab 模块的 Cargo.toml
  - 更新 opencrab 模块的代码导入
  - 确保主入口点正常工作
- **Acceptance Criteria Addressed**: [AC-2, AC-3]
- **Test Requirements**:
  - `programmatic` TR-9.1: opencrab 模块能够正常编译
  - `programmatic` TR-9.2: opencrab 二进制能够正常构建
- **Notes**: 确保整个应用能够正常启动
