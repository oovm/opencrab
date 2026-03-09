# Ironclaw 重构至 Opencrab V2 - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 完善 crab-types 模块，完整移植基础类型
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 从 ironclaw/src/ 提取所有跨模块共享的基础类型到 crab-types
  - 确保包含所有必要的错误类型和工具函数
  - 添加完整的文档注释，禁止使用后置注释
  - 检查是否遗漏任何重要类型
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-1.1: `cargo test -p crab-types` 通过
  - `human-judgement` TR-1.2: 所有 public API 都有文档注释
- **Notes**: 这是最基础的库，其他所有库都可能依赖它

## [ ] Task 2: 完善 crab-config 模块，完整移植配置管理
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**: 
  - 完整移植 ironclaw/src/config/ 模块到 crab-config
  - 确保所有配置项和加载逻辑完整
  - 依赖 crab-types
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-2.1: `cargo test -p crab-config` 通过
  - `human-judgement` TR-2.2: 配置加载功能完整，与 ironclaw 一致

## [ ] Task 3: 完善 crab-database 模块，完整移植数据库功能
- **Priority**: P0
- **Depends On**: [Task 1, Task 2]
- **Description**: 
  - 完整移植 ironclaw/src/db/ 模块到 crab-database
  - 支持 libsql 和 postgres 两种数据库
  - 包含完整的迁移功能
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-3.1: `cargo test -p crab-database` 通过

## [ ] Task 4: 完善 crab-llm 模块，完整移植 LLM 提供商功能
- **Priority**: P0
- **Depends On**: [Task 1, Task 2]
- **Description**: 
  - 完整移植 ironclaw/src/llm/ 模块到 crab-llm
  - 包含所有 LLM 提供商支持
  - 包含智能路由、重试、熔断器等功能
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-4.1: `cargo test -p crab-llm` 通过

## [ ] Task 5: 完善 crab-tools 模块，完整移植工具系统
- **Priority**: P0
- **Depends On**: [Task 1, Task 2]
- **Description**: 
  - 完整移植 ironclaw/src/tools/ 模块到 crab-tools
  - 包含内置工具、MCP、WASM 工具支持
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-5.1: `cargo test -p crab-tools` 通过

## [ ] Task 6: 完善 crab-channels 模块，完整移植通道系统
- **Priority**: P0
- **Depends On**: [Task 1, Task 2, Task 4, Task 5]
- **Description**: 
  - 完整移植 ironclaw/src/channels/ 模块到 crab-channels
  - 包含 CLI、HTTP、Websocket、Web 通道
  - 包含 web server、sse、ws 等功能
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-6.1: `cargo test -p crab-channels` 通过

## [ ] Task 7: 完善 crab-agent 模块（从 temp 转正）
- **Priority**: P0
- **Depends On**: [Task 1, Task 2, Task 3, Task 4, Task 5, Task 6]
- **Description**: 
  - 将 crab-agent.temp 重命名为 crab-agent
  - 完整移植 ironclaw/src/agent/ 模块
  - 包含核心代理循环、任务调度等功能
  - 添加完整的文档注释
  - 更新 workspace.members 包含 crab-agent
- **Acceptance Criteria Addressed**: [AC-2, AC-3, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-7.1: `cargo test -p crab-agent` 通过

## [ ] Task 8: 完善其他辅助库（crab-safety, crab-sandbox 等）
- **Priority**: P1
- **Depends On**: [Task 1]
- **Description**: 
  - 完整移植 ironclaw/src/safety/ 到 crab-safety
  - 完整移植 ironclaw/src/sandbox/ 到 crab-sandbox
  - 完整移植 ironclaw/src/secrets/ 到 crab-secrets
  - 完整移植 ironclaw/src/workspace/ 到 crab-workspace
  - 完整移植 ironclaw/src/skills/ 到 crab-skills
  - 完整移植 ironclaw/src/context/ 到 crab-context
  - 完整移植 ironclaw/src/history/ 到 crab-history
  - 完整移植 ironclaw/src/estimation/ 到 crab-estimation
  - 完整移植 ironclaw/src/evaluation/ 到 crab-evaluation
  - 完整移植 ironclaw/src/extensions/ 到 crab-extensions
  - 完整移植 ironclaw/src/hooks/ 到 crab-hooks
  - 完整移植 ironclaw/src/registry/ 到 crab-registry
  - 完整移植 ironclaw/src/orchestrator/ 到 crab-orchestrator
  - 完整移植 ironclaw/src/pairing/ 到 crab-pairing
  - 完整移植 ironclaw/src/observability/ 到 crab-observability
  - 为所有库添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-2, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-8.1: 每个库的测试都通过

## [ ] Task 9: 完善 backends/opencrab 主库
- **Priority**: P0
- **Depends On**: [Task 1-8]
- **Description**: 
  - 完整移植 ironclaw/src/ 主入口代码
  - 整合所有 crab-* 子库
  - 重新导出所有需要的公共 API
  - 确保主库依赖正确
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-4, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-9.1: `cargo build -p opencrab` 成功
  - `programmatic` TR-9.2: `cargo doc --no-deps -p opencrab` 成功生成

## [ ] Task 10: 完善 opencrab.exe 二进制文件
- **Priority**: P0
- **Depends On**: [Task 9]
- **Description**: 
  - 完整移植 ironclaw/src/main.rs 和 cli 模块
  - 实现所有 CLI 命令
  - 确保与 ironclaw.exe 功能 100% 一致
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-4, AC-5, AC-6]
- **Test Requirements**:
  - `programmatic` TR-10.1: `cargo run --bin opencrab -- --help` 正常显示帮助
  - `programmatic` TR-10.2: 所有 CLI 命令功能正常

## [ ] Task 11: 功能完整性验证
- **Priority**: P0
- **Depends On**: [Task 10]
- **Description**: 
  - 运行 ironclaw 的所有测试用例
  - 确保 opencrab 功能完全一致
  - 修复发现的问题
- **Acceptance Criteria Addressed**: [AC-7]
- **Test Requirements**:
  - `programmatic` TR-11.1: 所有集成测试通过
  - `human-judgement` TR-11.2: 手动测试关键功能正常
