#![warn(missing_docs)]

//! OpenCrab - 基于模块化架构的 AI 助手框架
//!
//! OpenCrab 是一个功能强大、模块化的 AI 助手框架，提供完整的 AI 代理功能。
//!
//! ## 核心功能
//!
//! - **智能代理**: 智能体管理和生命周期控制
//! - **记忆系统**: 灵活的记忆存储和检索
//! - **技能系统**: 可扩展的技能注册和执行
//! - **聊天系统**: 会话管理和消息处理
//! - **工具系统**: 工具注册和调用机制
//! - **调度系统**: 任务调度和定时执行
//! - **工作区管理**: 工作区和文件管理
//!
//! ## 模块结构
//!
//! - `types` - 基础类型和错误处理
//! - `config` - 配置和设置管理
//! - `database` - 数据库抽象和持久化
//! - `cache` - 缓存抽象和内存缓存
//! - `effect` - 代数效应和依赖注入
//! - `https` - HTTP 服务和 API 响应
//! - `queue` - 任务队列和异步处理
//! - `event` - 事件总线和事件分发
//! - `storage` - 存储抽象和文件系统存储
//! - `agent` - 智能体管理和生命周期
//! - `skill` - 技能系统和技能注册
//! - `memory` - 记忆系统和记忆检索
//! - `chat` - 聊天系统和会话管理
//! - `tool` - 工具系统和工具调用
//! - `scheduler` - 调度系统和任务执行
//! - `workspace` - 工作区管理和文件操作

pub mod prelude {
    //! 常用类型和 trait 的便捷导入
    //!
    //! 使用 `use opencrab::prelude::*;` 可以一次性导入最常用的类型。

    pub use crab_agent::{Agent, AgentService, AgentStatus, AgentType, MemoryAgentService};
    pub use wae_cache::{CacheService, memory_cache};
    pub use crab_chat::{ChatService, Conversation, MemoryChatService, Message as ChatMessage};
    pub use wae_config::ConfigLoader;
    pub use wae_database::{DatabaseConnection, Entity, Repository};
    pub use wae_effect::{AlgebraicEffect, Effectful};
    pub use wae_event::{Event, EventBus, event_bus, memory_event_store};
    pub use wae_https::{ApiResponse, HttpsServer};
    pub use crab_memory::{Memory, MemoryMemoryService, MemoryService, MemoryType};
    pub use wae_queue::{Message as QueueMessage, MessageProducer, MessageConsumer, QueueService, memory_queue_service};
    pub use crab_scheduler::{MemorySchedulerService, ScheduledTask, SchedulerService, TaskStatus};
    pub use crab_skill::{MemorySkillService, Skill, SkillService, SkillType};
    pub use wae_storage::{LocalStorageProvider, StorageProvider, StorageService};
    pub use crab_tool::{MemoryToolService, Tool, ToolExecutionResult, ToolService};
    pub use crab_types::{DateTime, Error, Result, Utc, Uuid, CrabError, CrabErrorKind, CrabResult};
    pub use wae_types::{WaeError, WaeErrorKind, WaeResult};
    pub use crab_workspace::{MemoryWorkspaceService, Workspace, WorkspaceService};
}

/// 基础类型模块
pub mod types {
    pub use crab_types::*;
}

/// 配置管理模块
pub mod config {
    pub use wae_config::*;
}

/// 数据库模块
pub mod database {
    pub use wae_database::*;
}

/// 缓存模块
pub mod cache {
    pub use wae_cache::*;
}

/// 效应模块
pub mod effect {
    pub use wae_effect::*;
}

/// HTTP 服务模块
pub mod https {
    pub use wae_https::*;
}

/// 队列模块
pub mod queue {
    pub use wae_queue::*;
}

/// 事件模块
pub mod event {
    pub use wae_event::*;
}

/// 存储模块
pub mod storage {
    pub use wae_storage::*;
}

/// 智能体模块
pub mod agent {
    pub use crab_agent::*;
}

/// 技能模块
pub mod skill {
    pub use crab_skill::*;
}

/// 记忆模块
pub mod memory {
    pub use crab_memory::*;
}

/// 聊天模块
pub mod chat {
    pub use crab_chat::*;
}

/// 工具模块
pub mod tool {
    pub use crab_tool::*;
}

/// 调度模块
pub mod scheduler {
    pub use crab_scheduler::*;
}

/// 工作区模块
pub mod workspace {
    pub use crab_workspace::*;
}
