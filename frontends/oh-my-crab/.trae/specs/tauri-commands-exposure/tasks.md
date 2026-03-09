# Oh My Crab - Tauri 命令暴露 - 实现计划

## [x] Task 1: 重构数据库模块为共享库（暂时跳过，直接在 oh-my-crab 中实现）
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 创建共享的 limbo-core 库，包含 types.rs 和 database.rs
  - 确保 CLI 和 GUI 都可以使用这个共享库
  - 统一数据库路径管理逻辑
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `programmatic` TR-1.1: limbo-core 库可以被 oh-my-crab 和 on-my-claw 同时引用
  - `programmatic` TR-1.2: 两个客户端使用相同的数据库路径逻辑
  - `human-judgement` TR-1.3: 代码结构清晰，无重复代码
- **Notes**: 使用 workspace 或者独立 crate 实现共享

## [x] Task 2: 实现 Tauri 状态管理
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**: 
  - 实现 Tauri AppState 来管理数据库连接
  - 实现当前用户会话状态
  - 确保数据库连接线程安全
- **Acceptance Criteria Addressed**: [AC-5]
- **Test Requirements**:
  - `programmatic` TR-2.1: AppState 正确管理 Database 实例
  - `programmatic` TR-2.2: 当前用户状态可以设置和读取
  - `programmatic` TR-2.3: 多线程访问数据库无数据竞争
- **Notes**: 使用 Mutex 或 RwLock 确保线程安全

## [x] Task 3: 暴露用户管理 Tauri 命令
- **Priority**: P0
- **Depends On**: [Task 2]
- **Description**: 
  - 创建 create_user 命令
  - 创建 get_all_users 命令
  - 创建 get_user_by_id 命令
  - 创建 update_user 命令
  - 创建 delete_user 命令
- **Acceptance Criteria Addressed**: [AC-1, AC-7]
- **Test Requirements**:
  - `programmatic` TR-3.1: 所有用户命令可以被前端调用
  - `programmatic` TR-3.2: 命令返回正确的类型或错误
  - `programmatic` TR-3.3: 无效操作返回有意义的错误
- **Notes**: 所有命令都要有完整的文档注释

## [x] Task 4: 暴露会话管理 Tauri 命令
- **Priority**: P0
- **Depends On**: [Task 3]
- **Description**: 
  - 创建 create_conversation 命令
  - 创建 get_conversations_by_user 命令
  - 创建 get_conversation_by_id 命令
  - 创建 update_conversation 命令
  - 创建 delete_conversation 命令
- **Acceptance Criteria Addressed**: [AC-2, AC-7]
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有会话命令可以被前端调用
  - `programmatic` TR-4.2: 会话正确关联到当前用户
  - `programmatic` TR-4.3: 命令返回正确的类型或错误
- **Notes**: 命令应使用当前用户上下文

## [x] Task 5: 暴露消息管理 Tauri 命令
- **Priority**: P0
- **Depends On**: [Task 4]
- **Description**: 
  - 创建 create_message 命令
  - 创建 get_messages_by_conversation 命令
  - 创建 delete_message 命令
- **Acceptance Criteria Addressed**: [AC-3, AC-7]
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有消息命令可以被前端调用
  - `programmatic` TR-5.2: 消息正确关联到会话和用户
  - `programmatic` TR-5.3: 命令返回正确的类型或错误
- **Notes**: 命令应使用当前用户上下文

## [x] Task 6: 暴露设置管理 Tauri 命令
- **Priority**: P0
- **Depends On**: [Task 5]
- **Description**: 
  - 创建 get_settings 命令
  - 创建 upsert_settings 命令
- **Acceptance Criteria Addressed**: [AC-4, AC-7]
- **Test Requirements**:
  - `programmatic` TR-6.1: 设置命令可以被前端调用
  - `programmatic` TR-6.2: 设置正确关联到当前用户
  - `programmatic` TR-6.3: 命令返回正确的类型或错误
- **Notes**: 命令应使用当前用户上下文

## [x] Task 7: 创建前端 TypeScript 类型定义
- **Priority**: P1
- **Depends On**: [Task 3, Task 4, Task 5, Task 6]
- **Description**: 
  - 创建前端类型定义文件
  - 定义所有数据类型的 TypeScript 接口
  - 定义 Tauri 命令调用的类型安全包装器
- **Acceptance Criteria Addressed**: [AC-8]
- **Test Requirements**:
  - `programmatic` TR-7.1: TypeScript 编译无错误
  - `programmatic` TR-7.2: 所有类型与 Rust 类型对应
  - `human-judgement` TR-7.3: 类型定义完整且易用
- **Notes**: 放在 src/types/index.ts

## [x] Task 8: 更新 main.rs 注册所有命令
- **Priority**: P0
- **Depends On**: [Task 3, Task 4, Task 5, Task 6]
- **Description**: 
  - 在 main.rs 中设置 AppState
  - 注册所有 Tauri 命令到 invoke_handler
  - 确保 setup 函数正确初始化
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-8.1: 应用可以正常启动
  - `programmatic` TR-8.2: 所有命令都已注册
  - `programmatic` TR-8.3: 数据库在启动时正确初始化
- **Notes**: 保留开发模式下的 devtools 功能

## [ ] Task 9: 更新 CLI (on-my-claw) 使用共享库
- **Priority**: P1
- **Depends On**: [Task 1]
- **Description**: 
  - 更新 on-my-claw 的 Cargo.toml 引用 limbo-core
  - 实现 CLI 的数据库访问功能
  - 添加用户管理、会话管理等 CLI 命令
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `programmatic` TR-9.1: CLI 可以编译并运行
  - `programmatic` TR-9.2: CLI 可以访问与 GUI 相同的数据库
  - `programmatic` TR-9.3: CLI 命令功能完整
- **Notes**: 确保数据库路径逻辑与 GUI 一致

## [ ] Task 10: 集成测试和验证
- **Priority**: P1
- **Depends On**: [Task 7, Task 8, Task 9]
- **Description**: 
  - 测试所有 Tauri 命令的端到端功能
  - 测试 CLI 和 GUI 数据共享
  - 验证错误处理
  - 性能测试
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-8]
- **Test Requirements**:
  - `programmatic` TR-10.1: 所有功能正常工作
  - `programmatic` TR-10.2: CLI 和 GUI 可以共享数据
  - `programmatic` TR-10.3: 命令响应时间 < 100ms
  - `human-judgement` TR-10.4: 代码质量符合要求
- **Notes**: 手动测试和自动化测试结合
