use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::id::{ChannelId, MessageId, ResourceId, UserId};

/// 对话结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// 对话唯一标识符
    pub conversation_id: Uuid,
    /// 对话主题（可选）
    pub topic: Option<String>,
    /// 参与者ID列表
    pub participants: Vec<Uuid>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 提及对象类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionType {
    /// 提及特定用户
    User,
    /// 提及频道所有成员
    Channel,
    /// 提及特定角色
    Role,
}

/// 提及结构体，用于提醒特定用户或群组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    /// 提及类型
    pub mention_type: MentionType,
    /// 提及对象 ID（用户 ID 或角色 ID）
    pub mention_id: String,
    /// 提及显示名称
    pub mention_name: String,
    /// 在文本中的起始位置（可选，用于文本高亮）
    pub offset: Option<u32>,
    /// 在文本中的长度（可选，用于文本高亮）
    pub length: Option<u32>,
}

/// 消息表情反应结构体，用户对消息的情感反馈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// 反应唯一 ID
    pub reaction_id: String,
    /// 目标消息 ID
    pub message_id: MessageId,
    /// 添加反应的用户
    pub user_id: UserId,
    /// 表情（如 "👍"、"❤️"、"🎉"）或自定义表情 ID
    pub emoji: String,
    /// 添加时间
    pub created_at: DateTime<Utc>,
}

/// 消息置顶结构体，用于标记重要消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    /// 置顶唯一 ID
    pub pin_id: String,
    /// 被置顶的消息 ID
    pub message_id: MessageId,
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 置顶者 ID
    pub pinned_by: UserId,
    /// 置顶时间
    pub pinned_at: DateTime<Utc>,
    /// 排序位置（可选，用于手动排序）
    pub order: Option<i32>,
}

/// 消息线程结构体，用于在特定消息下进行深入讨论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    /// 线程 ID（即父消息的 message_id）
    pub thread_id: MessageId,
    /// 父消息 ID
    pub parent_message_id: MessageId,
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 线程创建者
    pub created_by: UserId,
    /// 线程创建时间
    pub created_at: DateTime<Utc>,
    /// 线程内消息数量
    pub message_count: u32,
    /// 最后消息时间
    pub last_message_at: DateTime<Utc>,
}

/// 消息状态枚举（可选扩展）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    /// 发送中
    Sending,
    /// 已发送
    Sent,
    /// 已送达
    Delivered,
    /// 已读
    Read,
    /// 已撤回
    Recalled,
}

/// 文本消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    /// 文本内容（纯文本、Markdown、HTML 等，由客户端决定渲染方式）
    pub text: String,
    /// 提及列表
    pub mentions: Vec<Mention>,
}

/// 图片消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    /// 图片资源 ID
    pub resource_id: ResourceId,
    /// 图片说明（可选）
    pub caption: Option<String>,
}

/// 文件消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    /// 文件资源 ID
    pub resource_id: ResourceId,
    /// 文件名称（可选）
    pub filename: Option<String>,
    /// 文件大小（字节，可选）
    pub size: Option<u64>,
}

/// 资源引用消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    /// 资源 ID
    pub resource_id: ResourceId,
}

/// 链接消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkContent {
    /// 链接 URL
    pub url: String,
    /// 标题（可选）
    pub title: Option<String>,
    /// 描述（可选）
    pub description: Option<String>,
    /// 预览图片（可选）
    pub preview_image: Option<String>,
}

/// 系统消息内容（如成员加入、频道创建等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemContent {
    /// 事件类型
    pub event_type: String,
    /// 事件数据
    pub event_data: Value,
}

/// 消息内容枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    /// 文本消息内容
    Text(TextContent),
    /// 图片消息内容
    Image(ImageContent),
    /// 文件消息内容
    File(FileContent),
    /// 资源引用消息内容
    Resource(ResourceContent),
    /// 链接消息内容
    Link(LinkContent),
    /// 系统消息内容
    System(SystemContent),
}

/// 消息结构体，子网内传递的内容单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息唯一 ID
    pub message_id: MessageId,
    /// 频道 ID（群聊）或空（私聊）
    pub channel_id: Option<ChannelId>,
    /// 发送者 ID
    pub sender_id: UserId,
    /// 接收者 ID（私聊时使用）
    pub recipient_id: Option<UserId>,
    /// 消息类型（如 text/image/file/resource 等，开放类型）
    pub message_type: String,
    /// 消息内容（结构化数据，JSON 对象或加密的结构化数据）
    pub content: MessageContent,
    /// 是否加密
    pub content_encrypted: bool,
    /// 资源引用列表（可选）
    pub resource_refs: Vec<ResourceId>,
    /// 回复的消息 ID（可选）
    pub reply_to: Option<MessageId>,
    /// 线程 ID（可选，用于消息线程）
    pub thread_id: Option<MessageId>,
    /// 提及列表（可选）
    pub mentions: Vec<Mention>,
    /// 是否置顶（可选，默认 false）
    pub is_pinned: bool,
    /// 置顶时间（可选）
    pub pinned_at: Option<DateTime<Utc>>,
    /// 置顶者 ID（可选）
    pub pinned_by: Option<UserId>,
    /// 消息状态（可选，如 sending/sent/delivered/read/recalled）
    pub status: Option<MessageStatus>,
    /// 送达时间（可选）
    pub delivered_at: Option<DateTime<Utc>>,
    /// 已读时间（可选）
    pub read_at: Option<DateTime<Utc>>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 编辑时间（可选）
    pub edited_at: Option<DateTime<Utc>>,
    /// 删除时间（可选，软删除）
    pub deleted_at: Option<DateTime<Utc>>,
    /// 自定义元数据（可选）
    pub metadata: Option<Value>,
}
