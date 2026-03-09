# Oh My Crab - Tauri 命令暴露 - 产品需求文档

## Overview
- **Summary**: 实现 Tauri 命令暴露层，将数据库功能暴露给前端调用，同时确保 CLI 和 GUI 共享同一个 SQLite 数据库。
- **Purpose**: 建立前端与后端数据库的桥梁，提供类型安全的命令接口，实现数据持久化和多用户功能。
- **Target Users**: Oh My Crab GUI 用户、On My Claw CLI 用户

## Goals
- 暴露完整的数据库 CRUD 操作作为 Tauri 命令
- 实现前端类型定义，确保类型安全
- 确保 CLI 和 GUI 共享同一个 SQLite 数据库文件
- 提供数据库状态管理和错误处理
- 支持当前用户会话管理

## Non-Goals (Out of Scope)
- 实现用户认证和密码系统（后续阶段）
- 实现数据加密（后续阶段）
- 实现多设备同步（后续阶段）
- 实现云端备份（后续阶段）

## Background & Context
- 数据库层（types.rs 和 database.rs）已实现完成
- GUI 使用 Tauri + Vue 3，CLI 使用 Rust + Clap
- 两个客户端需要共享同一个 SQLite 数据库文件（limbo.db）
- 数据库文件位于应用数据目录

## Functional Requirements
- **FR-1**: 暴露用户管理 Tauri 命令
- **FR-2**: 暴露会话管理 Tauri 命令
- **FR-3**: 暴露消息管理 Tauri 命令
- **FR-4**: 暴露设置管理 Tauri 命令
- **FR-5**: 实现当前用户会话状态管理
- **FR-6**: 提供统一的数据库路径管理（CLI/GUI 共享）
- **FR-7**: 实现错误处理和结果包装

## Non-Functional Requirements
- **NFR-1**: 所有 Tauri 命令响应时间 < 100ms
- **NFR-2**: 类型安全，无运行时类型错误
- **NFR-3**: 优雅的错误处理和用户友好的错误信息
- **NFR-4**: 数据库连接安全，防止并发写入冲突
- **NFR-5**: 代码遵循 Rust 最佳实践，所有公共项有文档注释

## Constraints
- **Technical**: 
  - 使用 Tauri 2.0 命令系统
  - 使用 rusqlite 进行数据库操作
  - 前端使用 TypeScript 类型定义
- **Business**:
  - 数据库文件必须在应用数据目录
  - CLI 和 GUI 必须能同时访问同一个数据库
- **Dependencies**:
  - 已实现的 database.rs 和 types.rs 模块
  - Tauri 2.0 框架
  - Vue 3 前端

## Assumptions
- 数据库文件位于系统标准应用数据目录
- CLI 和 GUI 运行在同一台机器上
- 用户不会同时从 CLI 和 GUI 进行冲突的写入操作
- SQLite 的文件锁机制足以处理并发访问

## Acceptance Criteria

### AC-1: 用户管理命令暴露
- **Given**: 数据库已初始化
- **When**: 前端调用用户管理 Tauri 命令
- **Then**: 可以创建、读取、更新、删除用户
- **Verification**: `programmatic`
- **Notes**: 所有命令返回正确的类型或错误

### AC-2: 会话管理命令暴露
- **Given**: 存在有效用户
- **When**: 前端调用会话管理 Tauri 命令
- **Then**: 可以创建、读取、更新、删除会话
- **Verification**: `programmatic`
- **Notes**: 会话正确关联到用户

### AC-3: 消息管理命令暴露
- **Given**: 存在有效会话
- **When**: 前端调用消息管理 Tauri 命令
- **Then**: 可以创建、读取、删除消息
- **Verification**: `programmatic`
- **Notes**: 消息正确关联到会话和用户

### AC-4: 设置管理命令暴露
- **Given**: 存在有效用户
- **When**: 前端调用设置管理 Tauri 命令
- **Then**: 可以读取和更新用户设置
- **Verification**: `programmatic`
- **Notes**: 设置正确关联到用户

### AC-5: 当前用户会话管理
- **Given**: 应用已启动
- **When**: 用户选择或切换当前用户
- **Then**: 应用状态更新为当前用户，所有操作使用该用户上下文
- **Verification**: `programmatic`

### AC-6: CLI 和 GUI 共享数据库
- **Given**: GUI 创建了数据
- **When**: CLI 启动并访问数据库
- **Then**: CLI 可以读取和修改 GUI 创建的数据
- **Verification**: `programmatic`
- **Notes**: 两个客户端使用相同的数据库路径

### AC-7: 错误处理
- **Given**: 执行无效操作（如删除不存在的用户）
- **When**: 调用 Tauri 命令
- **Then**: 返回有意义的错误信息，不崩溃
- **Verification**: `programmatic`

### AC-8: 类型安全
- **Given**: 前端 TypeScript 代码
- **When**: 调用 Tauri 命令
- **Then**: 有完整的类型定义，无类型错误
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要实现数据库并发锁机制？
- [ ] 是否需要数据库迁移功能？
- [ ] 当前用户状态是否需要持久化？
