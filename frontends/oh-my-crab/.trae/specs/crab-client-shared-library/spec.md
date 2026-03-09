# Crab Client - 共享库 - 产品需求文档

## Overview
- **Summary**: 创建名为 `crab-client` 的共享 Rust 库，为 oh-my-crab (GUI) 和 on-my-claw (CLI) 提供共享的数据类型、数据库访问和配置管理功能。
- **Purpose**: 避免代码重复，确保两个客户端使用相同的数据模型和数据库逻辑，实现配置和数据共享。
- **Target Users**: oh-my-crab GUI 用户、on-my-claw CLI 用户、开发者

## Goals
- 创建 `crab-client` 共享库
- 提供统一的数据类型定义
- 提供统一的 SQLite 数据库访问
- 提供统一的数据库路径管理
- oh-my-crab 可以独立安装和运行
- on-my-claw 可以独立安装和运行
- 两个客户端共享同一个数据库和配置

## Non-Goals (Out of Scope)
- oh-my-crab 不通过调用 on-my-claw 实现功能
- 两个客户端除了数据和配置共享外，没有其他交集
- 不实现远程通信或进程间通信
- 不实现用户认证系统（后续阶段）

## Background & Context
- oh-my-crab 是 Tauri GUI 客户端
- on-my-claw 是 Rust CLI 客户端
- 两个客户端都是 OpenCrab 的前端
- 需要共享用户数据、会话、消息和设置
- 两个客户端都是独立应用，可以单独安装

## Functional Requirements
- **FR-1**: 创建 `crab-client` 共享库
- **FR-2**: 提供所有数据类型定义（User、Conversation、Message、AppSettings 等）
- **FR-3**: 提供 SQLite 数据库连接和初始化
- **FR-4**: 提供完整的数据库 CRUD 操作
- **FR-5**: 提供统一的数据库路径管理
- **FR-6**: oh-my-crab 引用并使用 crab-client
- **FR-7**: on-my-claw 引用并使用 crab-client

## Non-Functional Requirements
- **NFR-1**: crab-client 可以被两个客户端独立引用
- **NFR-2**: 数据库操作响应时间 < 100ms
- **NFR-3**: 所有公共项有完整的文档注释
- **NFR-4**: 无后置注释
- **NFR-5**: 代码风格一致

## Constraints
- **Technical**: 
  - 使用 Rust 编写共享库
  - 使用 rusqlite 进行 SQLite 操作
  - oh-my-crab 使用 Tauri 2.0
  - on-my-claw 使用 Rust + Clap
- **Business**:
  - 两个客户端都是独立应用
  - 只共享数据和配置，无其他交集
- **Dependencies**:
  - rusqlite
  - serde
  - chrono
  - uuid
  - dirs

## Assumptions
- 两个客户端运行在同一台机器上
- 数据库文件位于标准应用数据目录
- SQLite 的文件锁机制足以处理并发访问
- 用户不会同时进行冲突的写入操作

## Acceptance Criteria

### AC-1: crab-client 库创建
- **Given**: 项目环境已准备好
- **When**: 创建 crab-client 库
- **Then**: 库可以正常编译，包含所有必要的依赖
- **Verification**: `programmatic`

### AC-2: 数据类型定义
- **Given**: crab-client 库已创建
- **When**: 定义数据类型
- **Then**: 所有数据类型（User、Conversation、Message、AppSettings 等）都有完整定义和文档注释
- **Verification**: `programmatic`

### AC-3: 数据库功能
- **Given**: crab-client 库已创建
- **When**: 实现数据库功能
- **Then**: 提供完整的 CRUD 操作，数据库可以正常初始化和使用
- **Verification**: `programmatic`

### AC-4: 数据库路径管理
- **Given**: crab-client 库已创建
- **When**: 实现路径管理
- **Then**: 两个客户端使用相同的数据库路径逻辑
- **Verification**: `programmatic`

### AC-5: oh-my-crab 集成
- **Given**: crab-client 库已创建
- **When**: oh-my-crab 引用 crab-client
- **Then**: oh-my-crab 可以正常编译和运行，使用 crab-client 的功能
- **Verification**: `programmatic`

### AC-6: on-my-claw 集成
- **Given**: crab-client 库已创建
- **When**: on-my-claw 引用 crab-client
- **Then**: on-my-claw 可以正常编译和运行，使用 crab-client 的功能
- **Verification**: `programmatic`

### AC-7: 数据共享
- **Given**: 两个客户端都使用 crab-client
- **When**: GUI 创建数据
- **Then**: CLI 可以读取和修改相同的数据
- **Verification**: `programmatic`

### AC-8: 代码质量
- **Given**: 所有代码已完成
- **When**: 审查代码
- **Then**: 所有公共结构体、枚举、方法、字段都有文档注释，无后置注释
- **Verification**: `human-judgment`

## Open Questions
- [ ] 是否需要实现数据库并发锁机制？
- [ ] 是否需要数据库迁移功能？
