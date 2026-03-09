use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 平台类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformType {
    /// Discord 平台
    Discord,
    /// Telegram 平台
    Telegram,
    /// WhatsApp 平台
    WhatsApp,
    /// 飞书平台
    Feishu,
    /// 钉钉平台
    DingTalk,
    /// QQ 平台
    QQ,
    /// 微信平台
    WeChat,
    /// 自定义平台
    Custom,
}

/// 平台用户结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformUser {
    /// 平台用户唯一标识符
    pub id: String,
    /// 用户所属平台类型
    pub platform: PlatformType,
    /// 用户名
    pub username: String,
    /// 显示名称
    pub display_name: String,
    /// 头像 URL
    pub avatar_url: Option<String>,
}

/// 平台会话类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformConversationType {
    /// 私聊会话
    Direct,
    /// 群组会话
    Group,
    /// 频道会话
    Channel,
}

/// 平台会话结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConversation {
    /// 平台会话唯一标识符
    pub id: String,
    /// 会话所属平台类型
    pub platform: PlatformType,
    /// 会话名称
    pub name: String,
    /// 会话类型
    pub conversation_type: PlatformConversationType,
    /// 参与者数量
    pub participant_count: u32,
}

/// 平台消息结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMessage {
    /// 平台消息唯一标识符
    pub id: String,
    /// 消息所属平台类型
    pub platform: PlatformType,
    /// 平台会话 ID
    pub platform_conversation_id: String,
    /// 平台用户 ID
    pub platform_user_id: String,
    /// 消息内容
    pub content: String,
    /// 消息时间戳
    pub timestamp: DateTime<Utc>,
    /// 元数据
    pub metadata: Option<Value>,
}

/// 平台事件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlatformEvent {
    /// 用户加入事件
    UserJoined {
        /// 平台会话 ID
        platform_conversation_id: String,
        /// 加入的用户
        user: PlatformUser,
    },
    /// 用户离开事件
    UserLeft {
        /// 平台会话 ID
        platform_conversation_id: String,
        /// 离开的用户
        user: PlatformUser,
    },
    /// 会话创建事件
    ConversationCreated {
        /// 创建的会话
        conversation: PlatformConversation,
    },
    /// 会话更新事件
    ConversationUpdated {
        /// 更新的会话
        conversation: PlatformConversation,
    },
    /// 消息已读事件
    MessageRead {
        /// 平台会话 ID
        platform_conversation_id: String,
        /// 已读的消息 ID
        message_id: String,
        /// 阅读者用户 ID
        reader_user_id: String,
    },
    /// 自定义事件
    Custom {
        /// 事件名称
        name: String,
        /// 事件数据
        data: Value,
    },
}

/// 桥接配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// 桥接唯一标识符
    pub id: String,
    /// 目标平台类型
    pub platform: PlatformType,
    /// 桥接名称
    pub name: String,
    /// 平台认证令牌
    pub auth_token: String,
    /// 桥接配置
    pub config: Value,
    /// 桥接是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
