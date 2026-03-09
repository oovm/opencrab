#![warn(missing_docs)]

//! Crab Chat - OpenCrab 聊天系统模块
//!
//! 提供消息和会话管理功能。

pub use crab_types::Result;

/// 会话结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conversation {
    /// 会话唯一标识符
    pub id: uuid::Uuid,
    /// 所属用户ID
    pub user_id: uuid::Uuid,
    /// 会话标题
    pub title: String,
    /// 会话描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 是否归档
    pub is_archived: bool,
}

/// 消息结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// 消息唯一标识符
    pub id: uuid::Uuid,
    /// 所属会话ID
    pub conversation_id: uuid::Uuid,
    /// 发送者用户ID
    pub user_id: uuid::Uuid,
    /// 消息角色
    pub role: String,
    /// 消息内容
    pub content: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 消息元数据
    pub metadata: Option<String>,
}

/// 聊天服务 trait
#[async_trait::async_trait]
pub trait ChatService: Send + Sync {
    /// 创建会话
    async fn create_conversation(
        &self,
        _user_id: uuid::Uuid,
        _title: &str,
        _description: Option<&str>,
    ) -> Result<Conversation>;

    /// 获取会话
    async fn get_conversation(&self, _conversation_id: uuid::Uuid) -> Result<Conversation>;

    /// 列出会话
    async fn list_conversations(
        &self,
        _user_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Conversation>>;

    /// 发送消息
    async fn send_message(
        &self,
        _conversation_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _role: &str,
        _content: &str,
        _metadata: Option<&str>,
    ) -> Result<Message>;

    /// 列出消息
    async fn list_messages(
        &self,
        _conversation_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Message>>;
}

/// 内存聊天服务实现
pub struct MemoryChatService;

impl MemoryChatService {
    /// 创建新的内存聊天服务
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ChatService for MemoryChatService {
    async fn create_conversation(
        &self,
        _user_id: uuid::Uuid,
        _title: &str,
        _description: Option<&str>,
    ) -> Result<Conversation> {
        let now = chrono::Utc::now();
        Ok(Conversation {
            id: uuid::Uuid::new_v4(),
            user_id: _user_id,
            title: _title.to_string(),
            description: _description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
            is_archived: false,
        })
    }

    async fn get_conversation(&self, _conversation_id: uuid::Uuid) -> Result<Conversation> {
        Err(crab_types::Error::not_implemented("get_conversation"))
    }

    async fn list_conversations(
        &self,
        _user_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Conversation>> {
        Ok(Vec::new())
    }

    async fn send_message(
        &self,
        _conversation_id: uuid::Uuid,
        _user_id: uuid::Uuid,
        _role: &str,
        _content: &str,
        _metadata: Option<&str>,
    ) -> Result<Message> {
        Ok(Message {
            id: uuid::Uuid::new_v4(),
            conversation_id: _conversation_id,
            user_id: _user_id,
            role: _role.to_string(),
            content: _content.to_string(),
            created_at: chrono::Utc::now(),
            metadata: _metadata.map(|s| s.to_string()),
        })
    }

    async fn list_messages(
        &self,
        _conversation_id: uuid::Uuid,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }
}

impl Default for MemoryChatService {
    fn default() -> Self {
        Self::new()
    }
}
