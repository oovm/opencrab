# Ironclaw 重构至 Opencrab - The Implementation Plan (Decomposed and Prioritized Task List)

## [x] Task 1: 完善根目录 Cargo.toml workspace 配置
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 参考 wae 的 workspace 配置
  - 配置 workspace.package（version, edition, license 等）
  - 配置 workspace.dependencies 统一管理所有依赖
  - 更新 members 列表包含所有 backends crates
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `programmatic` TR-1.1: `cargo build` 在根目录成功执行
- **Notes**: 保持与现有 opencrab/Cargo.toml 的兼容性

## [x] Task 2: 分析 ironclaw 模块依赖关系并确定拆分方案
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 分析 ironclaw/src/ 下所有模块的依赖关系
  - 参考现有 opencrab/backends/ 的模块划分
  - 确定最终的 backends/* crate 列表
  - 确保无循环依赖
- **Acceptance Criteria Addressed**: [AC-2]
- **Test Requirements**:
  - `human-judgement` TR-2.1: 模块划分方案清晰合理，遵循单一职责原则
- **Notes**: 预计需要创建的 crates: opencrab-types, opencrab-config, opencrab-database, opencrab-llm, opencrab-tools, opencrab-channels, opencrab-agent, opencrab-safety, opencrab-sandbox, opencrab-secrets, opencrab-workspace, opencrab-web 等

## [x] Task 3: 创建/完善 opencrab-types 基础类型库
- **Priority**: P0
- **Depends On**: [Task 2]
- **Description**: 
  - 提取 ironclaw 中跨模块共享的基础类型
  - 包含错误类型、通用数据结构等
  - 添加完整的文档注释
  - 确保无后置注释
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: `cargo test -p opencrab-types` 通过
  - `human-judgement` TR-3.2: 所有 public API 都有文档注释
- **Notes**: 这是最基础的库，其他所有库都可能依赖它

## [x] Task 4: 创建/完善 opencrab-config 配置库
- **Priority**: P0
- **Depends On**: [Task 3]
- **Description**: 
  - 提取 ironclaw/config/ 模块
  - 依赖 opencrab-types
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-4.1: `cargo test -p opencrab-config` 通过
  - `human-judgement` TR-4.2: 配置加载功能正常

## [x] Task 5: 创建/完善 opencrab-database 数据库库
- **Priority**: P0
- **Depends On**: [Task 3, Task 4]
- **Description**: 
  - 提取 ironclaw/db/ 模块
  - 支持 libsql 和 postgres
  - 包含迁移功能
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-5.1: `cargo test -p opencrab-database` 通过

## [x] Task 6: 创建/完善 opencrab-llm LLM 提供商库
- **Priority**: P0
- **Depends On**: [Task 3, Task 4]
- **Description**: 
  - 提取 ironclaw/llm/ 模块
  - 包含多提供商支持
  - 包含智能路由等功能
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-6.1: `cargo test -p opencrab-llm` 通过

## [x] Task 7: 创建/完善 opencrab-tools 工具系统库
- **Priority**: P0
- **Depends On**: [Task 3, Task 4]
- **Description**: 
  - 提取 ironclaw/tools/ 模块
  - 包含内置工具、MCP、WASM 工具支持
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-7.1: `cargo test -p opencrab-tools` 通过

## [x] Task 8: 创建/完善 opencrab-channels 通道库
- **Priority**: P0
- **Depends On**: [Task 3, Task 4, Task 6, Task 7]
- **Description**: 
  - 提取 ironclaw/channels/ 模块
  - 包含 CLI、HTTP、Websocket、Web 通道
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-8.1: `cargo test -p opencrab-channels` 通过

## [x] Task 9: 创建/完善 opencrab-agent 核心代理库
- **Priority**: P0
- **Depends On**: [Task 3, Task 4, Task 5, Task 6, Task 7, Task 8]
- **Description**: 
  - 提取 ironclaw/agent/ 模块
  - 核心代理循环、任务调度等
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-9.1: `cargo test -p opencrab-agent` 通过
- **Notes**: 暂时从 workspace 排除，需要进一步完善

## [/] Task 13: 从 ironclaw 移植代码到 crab-types 模块
- **Priority**: P0
- **Depends On**: [Task 3]
- **Description**: 
  - 移植 ironclaw 中共享的基础类型到 crab-types
  - 包括错误类型、通用数据结构等
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-13.1: `cargo test -p crab-types` 通过

## [/] Task 14: 从 ironclaw 移植代码到 crab-config 模块
- **Priority**: P0
- **Depends On**: [Task 13]
- **Description**: 
  - 移植 ironclaw/config/ 模块到 crab-config
  - 确保依赖 crab-types
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-14.1: `cargo test -p crab-config` 通过

## [x] Task 10: 创建其他辅助库
- **Priority**: P1
- **Depends On**: [Task 3]
- **Description**: 
  - opencrab-safety (safety 模块)
  - opencrab-sandbox (sandbox 模块)
  - opencrab-secrets (secrets 模块)
  - opencrab-workspace (workspace 模块)
  - 等等
- **Acceptance Criteria Addressed**: [AC-2, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-10.1: 每个库的测试都通过

## [x] Task 11: 创建主库 backends/opencrab
- **Priority**: P0
- **Depends On**: [Task 3-10]
- **Description**: 
  - 创建主库，整合所有功能
  - 提供统一的 API
  - 创建二进制文件 opencrab
  - 重新导出所有需要的公共 API
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-11.1: `cargo build -p opencrab` 成功
  - `programmatic` TR-11.2: `cargo run --bin opencrab -- --help` 正常显示帮助

## [ ] Task 12: 功能完整性验证
- **Priority**: P0
- **Depends On**: [Task 11]
- **Description**: 
  - 运行 ironclaw 的所有测试用例
  - 确保 opencrab 功能完全一致
  - 修复发现的问题
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `programmatic` TR-12.1: 所有集成测试通过
  - `human-judgement` TR-12.2: 手动测试关键功能正常
