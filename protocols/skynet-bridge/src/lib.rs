#![warn(missing_docs)]
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use skynet_types::{
    BridgeConfig, Message, PlatformConversation, PlatformEvent, PlatformMessage, PlatformType,
    PlatformUser, SkyNetError, SkyNetErrorKind, SkyNetResult,
};
use uuid::Uuid;

/// 消息转换器 trait，支持 PlatformMessage 到内部 Message 的双向转换
pub trait MessageConverter {
    /// 将外部平台消息转换为内部消息
    ///
    /// # 参数
    /// - `platform_message`: 外部平台消息
    ///
    /// # 返回
    /// - 成功时返回内部消息
    /// - 失败时返回转换错误
    fn to_internal_message(&self, platform_message: PlatformMessage) -> SkyNetResult<Message>;

    /// 将内部消息转换为外部平台消息
    ///
    /// # 参数
    /// - `message`: 内部消息
    /// - `platform_conversation_id`: 目标平台会话 ID
    /// - `platform_user_id`: 目标平台用户 ID
    ///
    /// # 返回
    /// - 成功时返回外部平台消息
    /// - 失败时返回转换错误
    fn to_platform_message(
        &self,
        message: Message,
        platform_conversation_id: String,
        platform_user_id: String,
    ) -> SkyNetResult<PlatformMessage>;
}

/// 桥接会话状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    /// 未初始化
    Uninitialized,
    /// 初始化中
    Initializing,
    /// 断开连接
    Disconnected,
    /// 连接中
    Connecting,
    /// 已连接
    Connected,
    /// 错误状态
    Error,
    /// 重连中
    Reconnecting,
}

/// 外部用户与内部用户的映射结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMapping {
    /// 映射唯一标识符
    pub id: Uuid,
    /// 外部平台类型
    pub platform: PlatformType,
    /// 外部平台用户 ID
    pub platform_user_id: String,
    /// 内部用户 ID
    pub internal_user_id: Uuid,
    /// 映射创建时间
    pub created_at: DateTime<Utc>,
    /// 映射最后更新时间
    pub updated_at: DateTime<Utc>,
}

impl UserMapping {
    /// 创建新的用户映射
    ///
    /// # 参数
    /// - `platform`: 外部平台类型
    /// - `platform_user_id`: 外部平台用户 ID
    /// - `internal_user_id`: 内部用户 ID
    ///
    /// # 返回
    /// - 新的用户映射实例
    pub fn new(platform: PlatformType, platform_user_id: String, internal_user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            platform,
            platform_user_id,
            internal_user_id,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 外部会话与内部对话的映射结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMapping {
    /// 映射唯一标识符
    pub id: Uuid,
    /// 外部平台类型
    pub platform: PlatformType,
    /// 外部平台会话 ID
    pub platform_conversation_id: String,
    /// 内部对话 ID
    pub internal_conversation_id: Uuid,
    /// 映射创建时间
    pub created_at: DateTime<Utc>,
    /// 映射最后更新时间
    pub updated_at: DateTime<Utc>,
}

impl ConversationMapping {
    /// 创建新的会话映射
    ///
    /// # 参数
    /// - `platform`: 外部平台类型
    /// - `platform_conversation_id`: 外部平台会话 ID
    /// - `internal_conversation_id`: 内部对话 ID
    ///
    /// # 返回
    /// - 新的会话映射实例
    pub fn new(
        platform: PlatformType,
        platform_conversation_id: String,
        internal_conversation_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            platform,
            platform_conversation_id,
            internal_conversation_id,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 桥接服务 trait，定义外部平台消息桥接的核心接口
#[async_trait]
pub trait BridgeService: Send + Sync {
    /// 初始化桥接服务
    ///
    /// # 参数
    /// - `config`: 桥接配置
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn initialize(&self, config: BridgeConfig) -> SkyNetResult<()>;

    /// 连接到外部平台
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn connect(&self) -> SkyNetResult<()>;

    /// 断开与外部平台的连接
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn disconnect(&self) -> SkyNetResult<()>;

    /// 检查连接状态
    ///
    /// # 返回
    /// - 成功时返回连接状态（true 表示已连接）
    /// - 失败时返回错误
    async fn is_connected(&self) -> SkyNetResult<bool>;

    /// 获取当前会话状态
    ///
    /// # 返回
    /// - 成功时返回会话状态
    /// - 失败时返回错误
    async fn get_session_state(&self) -> SkyNetResult<SessionState>;

    /// 接收外部平台消息
    ///
    /// # 参数
    /// - `platform_conversation_id`: 平台会话 ID
    ///
    /// # 返回
    /// - 成功时返回平台消息
    /// - 失败时返回错误
    async fn receive_message(
        &self,
        platform_conversation_id: &str,
    ) -> SkyNetResult<PlatformMessage>;

    /// 发送消息到外部平台
    ///
    /// # 参数
    /// - `platform_conversation_id`: 平台会话 ID
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// - 成功时返回发送的平台消息
    /// - 失败时返回错误
    async fn send_message(
        &self,
        platform_conversation_id: &str,
        content: &str,
    ) -> SkyNetResult<PlatformMessage>;

    /// 处理平台事件
    ///
    /// # 参数
    /// - `event`: 平台事件
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn handle_event(&self, event: PlatformEvent) -> SkyNetResult<()>;

    /// 获取平台用户信息
    ///
    /// # 参数
    /// - `platform_user_id`: 平台用户 ID
    ///
    /// # 返回
    /// - 成功时返回平台用户信息
    /// - 失败时返回错误
    async fn get_platform_user(&self, platform_user_id: &str) -> SkyNetResult<PlatformUser>;

    /// 获取平台会话信息
    ///
    /// # 参数
    /// - `platform_conversation_id`: 平台会话 ID
    ///
    /// # 返回
    /// - 成功时返回平台会话信息
    /// - 失败时返回错误
    async fn get_platform_conversation(
        &self,
        platform_conversation_id: &str,
    ) -> SkyNetResult<PlatformConversation>;

    /// 列出所有可用的平台会话
    ///
    /// # 返回
    /// - 成功时返回平台会话列表
    /// - 失败时返回错误
    async fn list_platform_conversations(&self) -> SkyNetResult<Vec<PlatformConversation>>;

    /// 列出平台会话的所有参与者
    ///
    /// # 参数
    /// - `platform_conversation_id`: 平台会话 ID
    ///
    /// # 返回
    /// - 成功时返回平台用户列表
    /// - 失败时返回错误
    async fn list_platform_participants(
        &self,
        platform_conversation_id: &str,
    ) -> SkyNetResult<Vec<PlatformUser>>;

    /// 创建用户映射
    ///
    /// # 参数
    /// - `mapping`: 用户映射
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn create_user_mapping(&self, mapping: UserMapping) -> SkyNetResult<()>;

    /// 获取用户映射
    ///
    /// # 参数
    /// - `platform_user_id`: 平台用户 ID
    ///
    /// # 返回
    /// - 成功时返回用户映射
    /// - 失败时返回错误
    async fn get_user_mapping(&self, platform_user_id: &str) -> SkyNetResult<UserMapping>;

    /// 创建会话映射
    ///
    /// # 参数
    /// - `mapping`: 会话映射
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn create_conversation_mapping(&self, mapping: ConversationMapping) -> SkyNetResult<()>;

    /// 获取会话映射
    ///
    /// # 参数
    /// - `platform_conversation_id`: 平台会话 ID
    ///
    /// # 返回
    /// - 成功时返回会话映射
    /// - 失败时返回错误
    async fn get_conversation_mapping(
        &self,
        platform_conversation_id: &str,
    ) -> SkyNetResult<ConversationMapping>;

    /// 设置消息转换器
    ///
    /// # 参数
    /// - `converter`: 消息转换器
    fn set_message_converter(&self, converter: Box<dyn MessageConverter + Send + Sync>);
}
