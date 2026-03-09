#![warn(missing_docs)]
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use skynet_types::{Conversation, Memory, MemorySearchResult, MemoryType, Message, SkyNetError, SkyNetErrorKind, SkyNetResult};
use uuid::Uuid;

/// 对话上下文结构体，包含从对话中提取的关键信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// 对话唯一标识符
    pub conversation_id: Uuid,
    /// 对话主题（从对话中提取）
    pub topic: Option<String>,
    /// 参与者ID列表
    pub participants: Vec<Uuid>,
    /// 重要决策列表
    pub key_decisions: Vec<String>,
    /// 对话摘要
    pub summary: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 记忆服务 trait，定义记忆相关的核心接口
#[async_trait]
pub trait MemoryService: Send + Sync {
    /// 创建记忆
    ///
    /// # 参数
    /// - `memory_type`: 记忆类型
    /// - `owner_id`: 所有者ID
    /// - `owner_type`: 所有者类型
    /// - `content`: 记忆内容
    /// - `summary`: 记忆摘要（可选）
    /// - `metadata`: 记忆元数据
    ///
    /// # 返回
    /// - 成功时返回创建的记忆
    /// - 失败时返回错误
    async fn create_memory(
        &self,
        memory_type: MemoryType,
        owner_id: Uuid,
        owner_type: &str,
        content: &str,
        summary: Option<&str>,
        metadata: serde_json::Value,
    ) -> SkyNetResult<Memory>;

    /// 获取记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    ///
    /// # 返回
    /// - 成功时返回记忆信息
    /// - 失败时返回错误
    async fn get_memory(&self, memory_id: Uuid) -> SkyNetResult<Memory>;

    /// 更新记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    /// - `content`: 新的记忆内容（可选）
    /// - `summary`: 新的记忆摘要（可选）
    /// - `metadata`: 新的记忆元数据（可选）
    ///
    /// # 返回
    /// - 成功时返回更新后的记忆
    /// - 失败时返回错误
    async fn update_memory(
        &self,
        memory_id: Uuid,
        content: Option<&str>,
        summary: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> SkyNetResult<Memory>;

    /// 删除记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn delete_memory(&self, memory_id: Uuid) -> SkyNetResult<()>;

    /// 列出记忆（支持分页）
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回记忆列表
    /// - 失败时返回错误
    async fn list_memories(&self, owner_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;

    /// 添加标签到记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    /// - `tag_name`: 标签名称
    /// - `tag_description`: 标签描述（可选）
    ///
    /// # 返回
    /// - 成功时返回更新后的记忆
    /// - 失败时返回错误
    async fn add_tag(&self, memory_id: Uuid, tag_name: &str, tag_description: Option<&str>) -> SkyNetResult<Memory>;

    /// 从记忆移除标签
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    /// - `tag_id`: 标签ID
    ///
    /// # 返回
    /// - 成功时返回更新后的记忆
    /// - 失败时返回错误
    async fn remove_tag(&self, memory_id: Uuid, tag_id: Uuid) -> SkyNetResult<Memory>;

    /// 添加记忆关联
    ///
    /// # 参数
    /// - `source_memory_id`: 源记忆ID
    /// - `target_memory_id`: 目标记忆ID
    /// - `relation_type`: 关联类型
    /// - `weight`: 关联权重
    ///
    /// # 返回
    /// - 成功时返回更新后的源记忆
    /// - 失败时返回错误
    async fn add_relation(
        &self,
        source_memory_id: Uuid,
        target_memory_id: Uuid,
        relation_type: &str,
        weight: f64,
    ) -> SkyNetResult<Memory>;

    /// 移除记忆关联
    ///
    /// # 参数
    /// - `source_memory_id`: 源记忆ID
    /// - `target_memory_id`: 目标记忆ID
    ///
    /// # 返回
    /// - 成功时返回更新后的源记忆
    /// - 失败时返回错误
    async fn remove_relation(&self, source_memory_id: Uuid, target_memory_id: Uuid) -> SkyNetResult<Memory>;

    /// 获取关联记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    ///
    /// # 返回
    /// - 成功时返回关联的记忆列表
    /// - 失败时返回错误
    async fn get_related_memories(&self, memory_id: Uuid) -> SkyNetResult<Vec<Memory>>;

    /// 根据关键词搜索记忆
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `keyword`: 搜索关键词
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回匹配的记忆搜索结果列表（包含相关性分数）
    /// - 失败时返回错误
    async fn search_by_keyword(
        &self,
        owner_id: Uuid,
        keyword: &str,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<MemorySearchResult>>;

    /// 根据标签列表过滤记忆
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `tag_ids`: 标签ID列表
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回匹配的记忆列表
    /// - 失败时返回错误
    async fn filter_by_tags(&self, owner_id: Uuid, tag_ids: &[Uuid], limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;

    /// 根据时间范围过滤记忆
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `start_time`: 时间范围起始（可选）
    /// - `end_time`: 时间范围结束（可选）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回匹配的记忆列表
    /// - 失败时返回错误
    async fn filter_by_time_range(
        &self,
        owner_id: Uuid,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<Memory>>;

    /// 组合搜索记忆（关键词 + 标签 + 时间范围）
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `keyword`: 搜索关键词（可选）
    /// - `tag_ids`: 标签ID列表（可选）
    /// - `start_time`: 时间范围起始（可选）
    /// - `end_time`: 时间范围结束（可选）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回匹配的记忆搜索结果列表（包含相关性分数）
    /// - 失败时返回错误
    async fn search_with_filters(
        &self,
        owner_id: Uuid,
        keyword: Option<&str>,
        tag_ids: Option<&[Uuid]>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<MemorySearchResult>>;

    /// 导出记忆为 JSON 字符串
    ///
    /// # 参数
    /// - `memory_ids`: 要导出的记忆ID列表
    ///
    /// # 返回
    /// - 成功时返回 JSON 格式的字符串
    /// - 失败时返回错误
    async fn export_memories(&self, memory_ids: &[Uuid]) -> SkyNetResult<String>;

    /// 从 JSON 字符串导入记忆
    ///
    /// # 参数
    /// - `json_str`: 包含记忆数据的 JSON 字符串
    ///
    /// # 返回
    /// - 成功时返回导入的记忆列表
    /// - 失败时返回错误
    async fn import_memories(&self, json_str: &str) -> SkyNetResult<Vec<Memory>>;

    /// 导出所有者的所有记忆
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `owner_type`: 所有者类型
    ///
    /// # 返回
    /// - 成功时返回 JSON 格式的字符串
    /// - 失败时返回错误
    async fn export_all_memories(&self, owner_id: Uuid, owner_type: &str) -> SkyNetResult<String>;

    /// 从对话中提取上下文信息
    ///
    /// # 参数
    /// - `conversation`: 对话对象
    /// - `messages`: 对话消息列表
    ///
    /// # 返回
    /// - 成功时返回提取的对话上下文
    /// - 失败时返回错误
    async fn extract_context(&self, conversation: &Conversation, messages: &[Message]) -> SkyNetResult<ConversationContext>;

    /// 将对话转换为记忆
    ///
    /// # 参数
    /// - `context`: 对话上下文
    /// - `owner_id`: 所有者ID
    /// - `owner_type`: 所有者类型
    ///
    /// # 返回
    /// - 成功时返回创建的记忆
    /// - 失败时返回错误
    async fn create_memory_from_conversation(
        &self,
        context: &ConversationContext,
        owner_id: Uuid,
        owner_type: &str,
    ) -> SkyNetResult<Memory>;
}
