use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::id::{ChannelId, MessageId, UserId};

/// WebSocket 事件类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEventType {
    /// 连接事件
    Connect,
    /// 断开连接事件
    Disconnect,
    /// 消息推送事件
    Message,
    /// 状态更新事件
    StatusUpdate,
    /// 输入中事件
    Typing,
    /// 已读回执事件
    ReadReceipt,
}

/// WebSocket 连接握手数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectData {
    /// 认证令牌
    pub token: String,
    /// 客户端唯一标识符
    pub client_id: String,
    /// 子网 ID
    pub subnet_id: String,
}

/// WebSocket 消息推送数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    /// 消息唯一 ID
    pub message_id: MessageId,
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 发送者 ID
    pub sender_id: UserId,
    /// 接收者 ID（私聊时使用）
    pub recipient_id: Option<UserId>,
    /// 消息内容
    pub content: Value,
    /// 消息创建时间
    pub created_at: DateTime<Utc>,
}

/// WebSocket 状态更新数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdateData {
    /// 用户 ID
    pub user_id: UserId,
    /// 在线状态
    pub status: String,
    /// 状态更新时间
    pub updated_at: DateTime<Utc>,
}

/// WebSocket 输入中数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingData {
    /// 用户 ID
    pub user_id: UserId,
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 接收者 ID（私聊时使用）
    pub recipient_id: Option<UserId>,
    /// 是否正在输入
    pub is_typing: bool,
}

/// WebSocket 已读回执数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadReceiptData {
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 消息 ID
    pub message_id: MessageId,
    /// 阅读者 ID
    pub reader_id: UserId,
    /// 阅读时间
    pub read_at: DateTime<Utc>,
}

/// WebSocket 事件数据枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketEventData {
    /// 连接事件数据
    Connect(ConnectData),
    /// 断开连接事件数据
    Disconnect,
    /// 消息推送事件数据
    Message(MessageData),
    /// 状态更新事件数据
    StatusUpdate(StatusUpdateData),
    /// 输入中事件数据
    Typing(TypingData),
    /// 已读回执事件数据
    ReadReceipt(ReadReceiptData),
}

/// WebSocket 事件结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketEvent {
    /// 事件类型
    pub event_type: WebSocketEventType,
    /// 事件数据
    pub data: WebSocketEventData,
}
