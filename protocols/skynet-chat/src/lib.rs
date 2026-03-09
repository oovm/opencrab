#![warn(missing_docs)]
//! Skynet 聊天协议模块，定义聊天和消息相关的接口 trait 和类型。
//!
//! 该模块仅包含协议定义，不包含具体的业务逻辑实现。
use async_trait::async_trait;
use skynet_types::{ChannelId, Message, MessageContent, MessageId, SkyNetResult, UserId};

/// 聊天服务 trait，定义对话和消息相关的核心接口
#[async_trait]
pub trait ChatService: Send + Sync {
    /// 发送消息
    ///
    /// # 参数
    /// - `channel_id`: 频道 ID（群聊）或 None（私聊）
    /// - `sender_id`: 发送者 ID
    /// - `recipient_id`: 接收者 ID（私聊时使用）
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// - 成功时返回发送的消息
    /// - 失败时返回错误
    async fn send_message(
        &self,
        channel_id: Option<ChannelId>,
        sender_id: UserId,
        recipient_id: Option<UserId>,
        content: MessageContent,
    ) -> SkyNetResult<Message>;

    /// 获取消息
    ///
    /// # 参数
    /// - `message_id`: 消息 ID
    ///
    /// # 返回
    /// - 成功时返回消息
    /// - 失败时返回错误
    async fn get_message(&self, message_id: MessageId) -> SkyNetResult<Message>;

    /// 获取频道或私聊中的消息列表
    ///
    /// # 参数
    /// - `channel_id`: 频道 ID（群聊）或 None（私聊）
    /// - `recipient_id`: 接收者 ID（私聊时使用）
    /// - `limit`: 返回消息数量限制
    /// - `before_id`: 仅返回此 ID 之前的消息（可选）
    ///
    /// # 返回
    /// - 成功时返回消息列表
    /// - 失败时返回错误
    async fn list_messages(
        &self,
        channel_id: Option<ChannelId>,
        recipient_id: Option<UserId>,
        limit: u32,
        before_id: Option<MessageId>,
    ) -> SkyNetResult<Vec<Message>>;

    /// 更新消息
    ///
    /// # 参数
    /// - `message_id`: 消息 ID
    /// - `content`: 新的消息内容
    ///
    /// # 返回
    /// - 成功时返回更新后的消息
    /// - 失败时返回错误
    async fn update_message(
        &self,
        message_id: MessageId,
        content: MessageContent,
    ) -> SkyNetResult<Message>;

    /// 删除消息
    ///
    /// # 参数
    /// - `message_id`: 消息 ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn delete_message(&self, message_id: MessageId) -> SkyNetResult<()>;

    /// 标记消息为已读
    ///
    /// # 参数
    /// - `message_id`: 消息 ID
    /// - `reader_id`: 阅读者 ID
    ///
    /// # 返回
    /// - 成功时返回更新后的消息
    /// - 失败时返回错误
    async fn mark_message_as_read(
        &self,
        message_id: MessageId,
        reader_id: UserId,
    ) -> SkyNetResult<Message>;
}
