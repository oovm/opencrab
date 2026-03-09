#![warn(missing_docs)]

//! Crab Memory - OpenCrab 记忆系统模块
//!
//! 提供短期和长期记忆管理功能。

pub use crab_types::Result;

/// 记忆类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// 消息记忆
    Message,
    /// 任务记忆
    Task,
    /// 经验记忆
    Experience,
    /// 知识记忆
    Knowledge,
}

/// 记忆结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Memory {
    /// 记忆唯一标识符
    pub id: uuid::Uuid,
    /// 所有者ID
    pub owner_id: uuid::Uuid,
    /// 所有者类型
    pub owner_type: String,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 记忆内容
    pub content: String,
    /// 记忆摘要
    pub summary: Option<String>,
    /// 记忆元数据
    pub metadata: Option<serde_json::Value>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 记忆服务 trait
#[async_trait::async_trait]
pub trait MemoryService: Send + Sync {
    /// 创建记忆
    async fn create_memory(
        &self,
        _memory_type: MemoryType,
        _owner_id: uuid::Uuid,
        _owner_type: &str,
        _content: &str,
        _summary: Option<&str>,
        _metadata: serde_json::Value,
    ) -> Result<Memory>;

    /// 获取记忆
    async fn get_memory(&self, _memory_id: uuid::Uuid) -> Result<Memory>;

    /// 列出记忆
    async fn list_memories(
        &self,
        _owner_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Memory>>;
}

/// 内存记忆服务实现
pub struct MemoryMemoryService;

impl MemoryMemoryService {
    /// 创建新的内存记忆服务
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MemoryService for MemoryMemoryService {
    async fn create_memory(
        &self,
        _memory_type: MemoryType,
        _owner_id: uuid::Uuid,
        _owner_type: &str,
        _content: &str,
        _summary: Option<&str>,
        _metadata: serde_json::Value,
    ) -> Result<Memory> {
        let now = chrono::Utc::now();
        Ok(Memory {
            id: uuid::Uuid::new_v4(),
            owner_id: _owner_id,
            owner_type: _owner_type.to_string(),
            memory_type: _memory_type,
            content: _content.to_string(),
            summary: _summary.map(|s| s.to_string()),
            metadata: Some(_metadata),
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_memory(&self, _memory_id: uuid::Uuid) -> Result<Memory> {
        Err(crab_types::Error::not_implemented("get_memory"))
    }

    async fn list_memories(
        &self,
        _owner_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Memory>> {
        Ok(Vec::new())
    }
}

impl Default for MemoryMemoryService {
    fn default() -> Self {
        Self::new()
    }
}
