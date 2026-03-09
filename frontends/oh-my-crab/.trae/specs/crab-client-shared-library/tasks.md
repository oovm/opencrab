# Crab Client - 共享库 - 实现计划

## [ ] Task 1: 创建 crab-client 共享库
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 在 frontends 目录下创建 crab-client 库
  - 配置 Cargo.toml 依赖
  - 添加到 workspace members
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `programmatic` TR-1.1: crab-client 库可以正常编译
  - `programmatic` TR-1.2: 依赖配置正确
  - `human-judgement` TR-1.3: 库结构清晰
- **Notes**: 使用原来 oh-my-crab 中可以工作的代码

## [ ] Task 2: 实现数据类型定义
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**: 
  - 创建 types.rs 模块
  - 定义 UserRole 枚举
  - 定义 User、Conversation、Message、AppSettings 结构体
  - 定义所有请求结构体
- **Acceptance Criteria Addressed**: [AC-2, AC-8]
- **Test Requirements**:
  - `programmatic` TR-2.1: 所有类型定义完整
  - `human-judgement` TR-2.2: 所有公共项有文档注释
  - `human-judgement` TR-2.3: 无后置注释
- **Notes**: 从 oh-my-crab 复制，确保所有文档注释完整

## [ ] Task 3: 实现数据库路径管理
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**: 
  - 创建 path.rs 模块
  - 实现 get_database_path() 函数
  - 使用 dirs crate 获取标准应用数据目录
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-3.1: 路径管理函数可以正常工作
  - `programmatic` TR-3.2: 路径指向正确的应用数据目录
  - `human-judgement` TR-3.3: 函数有文档注释
- **Notes**: 确保两个客户端使用相同的路径逻辑

## [ ] Task 4: 实现数据库模块
- **Priority**: P0
- **Depends On**: [Task 2, Task 3]
- **Description**: 
  - 创建 database.rs 模块
  - 实现 Database 结构体
  - 实现数据库初始化和表创建
  - 实现所有 CRUD 操作
- **Acceptance Criteria Addressed**: [AC-3, AC-8]
- **Test Requirements**:
  - `programmatic` TR-4.1: 数据库可以正常初始化
  - `programmatic` TR-4.2: 所有 CRUD 操作正常工作
  - `human-judgement` TR-4.3: 所有公共项有文档注释
  - `human-judgement` TR-4.4: 无后置注释
- **Notes**: 从 oh-my-crab 复制可以工作的代码

## [ ] Task 5: 创建 lib.rs 导出公共 API
- **Priority**: P0
- **Depends On**: [Task 2, Task 3, Task 4]
- **Description**: 
  - 创建 lib.rs
  - 导出所有公共类型
  - 导出 Database 结构体
  - 导出 get_database_path 函数
- **Acceptance Criteria Addressed**: [AC-1, AC-8]
- **Test Requirements**:
  - `programmatic` TR-5.1: 库可以被其他 crate 引用
  - `programmatic` TR-5.2: 所有公共 API 正确导出
  - `human-judgement` TR-5.3: 模块有文档注释
- **Notes**: 确保 API 设计合理

## [ ] Task 6: 更新 oh-my-crab 使用 crab-client
- **Priority**: P0
- **Depends On**: [Task 5]
- **Description**: 
  - 更新 oh-my-crab 的 Cargo.toml
  - 删除 oh-my-crab 中的 types.rs 和 database.rs
  - 更新 main.rs 使用 crab-client
  - 确保 Tauri 命令正常工作
- **Acceptance Criteria Addressed**: [AC-5]
- **Test Requirements**:
  - `programmatic` TR-6.1: oh-my-crab 可以正常编译
  - `programmatic` TR-6.2: Tauri 命令正常工作
  - `human-judgement` TR-6.3: 代码简洁，无重复
- **Notes**: oh-my-crab 是独立应用，不依赖 on-my-claw

## [ ] Task 7: 更新 on-my-claw 使用 crab-client
- **Priority**: P1
- **Depends On**: [Task 5]
- **Description**: 
  - 更新 on-my-claw 的 Cargo.toml
  - 实现 CLI 命令使用 crab-client
  - 添加用户管理、会话管理等 CLI 子命令
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `programmatic` TR-7.1: on-my-claw 可以正常编译
  - `programmatic` TR-7.2: CLI 命令正常工作
  - `human-judgement` TR-7.3: 代码简洁，无重复
- **Notes**: on-my-claw 是独立应用，不依赖 oh-my-crab

## [x] Task 8: 验证数据共享
- **Priority**: P1
- **Depends On**: [Task 6, Task 7]
- **Description**: 
  - 测试 oh-my-crab 创建数据
  - 测试 on-my-claw 读取相同数据
  - 验证两个客户端可以共享数据库
- **Acceptance Criteria Addressed**: [AC-7]
- **Test Requirements**:
  - `programmatic` TR-8.1: GUI 创建的数据可以被 CLI 读取
  - `programmatic` TR-8.2: CLI 创建的数据可以被 GUI 读取
  - `programmatic` TR-8.3: 两个客户端使用相同的数据库文件
- **Notes**: 手动测试两个客户端的数据共享

## [ ] Task 9: 代码质量检查
- **Priority**: P1
- **Depends On**: [Task 6, Task 7]
- **Description**: 
  - 检查所有公共项的文档注释
  - 检查无后置注释
  - 检查代码风格一致性
- **Acceptance Criteria Addressed**: [AC-8]
- **Test Requirements**:
  - `human-judgement` TR-9.1: 所有公共结构体、枚举、方法、字段都有文档注释
  - `human-judgement` TR-9.2: 无后置注释
  - `human-judgement` TR-9.3: 代码风格一致
- **Notes**: 手动审查代码
