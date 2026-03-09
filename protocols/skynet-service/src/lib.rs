#![warn(missing_docs)]
//! SkyNet 协议层服务 trait 定义
//!
//! 本 crate 定义了 Skynet 协议的核心服务接口，包括：
//! - Service: 基础服务 trait
//! - SubnetService: 子网管理服务
//! - MessageService: 消息管理服务
//! - ResourceService: 资源管理服务

use async_trait::async_trait;
use skynet_types::{
    chat::{Message, Pin, Reaction, Thread},
    error::SkyNetError,
    id::{AuthId, ChannelId, MessageId, ResourceId, SubnetId, UserId},
    resource::Resource,
    subnet::{Channel, Member, PermissionPolicy, Subnet, SubnetMetadata},
};

/// 基础服务 trait，定义服务的生命周期和基础信息接口
#[async_trait]
pub trait Service: Send + Sync {
    /// 获取服务名称
    ///
    /// # 返回
    /// - 服务名称字符串
    fn name(&self) -> &str;

    /// 获取服务版本
    ///
    /// # 返回
    /// - 服务版本字符串
    fn version(&self) -> &str;

    /// 初始化服务
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn initialize(&self) -> Result<(), SkyNetError>;

    /// 启动服务
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn start(&self) -> Result<(), SkyNetError>;

    /// 停止服务
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn stop(&self) -> Result<(), SkyNetError>;

    /// 检查服务健康状态
    ///
    /// # 返回
    /// - 成功时返回健康状态（true 表示健康）
    /// - 失败时返回错误
    async fn health_check(&self) -> Result<bool, SkyNetError>;

    /// 获取服务状态
    ///
    /// # 返回
    /// - 成功时返回服务状态信息
    /// - 失败时返回错误
    async fn get_status(&self) -> Result<ServiceStatus, SkyNetError>;
}

/// 服务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// 未初始化
    Uninitialized,
    /// 初始化中
    Initializing,
    /// 已初始化但未启动
    Initialized,
    /// 启动中
    Starting,
    /// 运行中
    Running,
    /// 停止中
    Stopping,
    /// 已停止
    Stopped,
    /// 错误状态
    Error,
}

/// 子网管理服务 trait
///
/// 负责子网的创建、查询、成员管理、频道管理等功能。
#[async_trait]
pub trait SubnetService: Send + Sync {
    /// 创建一个新的子网
    ///
    /// # 参数
    /// - `metadata`: 子网元信息
    /// - `created_by`: 创建者的全局身份 ID
    ///
    /// # 返回
    /// - 成功时返回创建的子网
    /// - 失败时返回错误
    async fn create_subnet(&self, metadata: SubnetMetadata, created_by: AuthId) -> Result<Subnet, SkyNetError>;

    /// 获取子网信息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    ///
    /// # 返回
    /// - 成功时返回子网信息
    /// - 失败时返回错误
    async fn get_subnet(&self, subnet_id: SubnetId) -> Result<Subnet, SkyNetError>;

    /// 更新子网元信息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `metadata`: 新的子网元信息
    ///
    /// # 返回
    /// - 成功时返回更新后的子网
    /// - 失败时返回错误
    async fn update_subnet_metadata(&self, subnet_id: SubnetId, metadata: SubnetMetadata) -> Result<Subnet, SkyNetError>;

    /// 删除子网
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn delete_subnet(&self, subnet_id: SubnetId) -> Result<(), SkyNetError>;

    /// 添加成员到子网
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `member`: 要添加的成员
    ///
    /// # 返回
    /// - 成功时返回更新后的成员列表
    /// - 失败时返回错误
    async fn add_member(&self, subnet_id: SubnetId, member: Member) -> Result<Vec<Member>, SkyNetError>;

    /// 移除子网成员
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `user_id`: 要移除的成员的本地 ID
    ///
    /// # 返回
    /// - 成功时返回更新后的成员列表
    /// - 失败时返回错误
    async fn remove_member(&self, subnet_id: SubnetId, user_id: UserId) -> Result<Vec<Member>, SkyNetError>;

    /// 获取子网成员列表
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    ///
    /// # 返回
    /// - 成功时返回成员列表
    /// - 失败时返回错误
    async fn list_members(&self, subnet_id: SubnetId) -> Result<Vec<Member>, SkyNetError>;

    /// 创建频道
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `channel`: 要创建的频道
    ///
    /// # 返回
    /// - 成功时返回创建的频道
    /// - 失败时返回错误
    async fn create_channel(&self, subnet_id: SubnetId, channel: Channel) -> Result<Channel, SkyNetError>;

    /// 获取频道信息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `channel_id`: 频道 ID
    ///
    /// # 返回
    /// - 成功时返回频道信息
    /// - 失败时返回错误
    async fn get_channel(&self, subnet_id: SubnetId, channel_id: ChannelId) -> Result<Channel, SkyNetError>;

    /// 获取子网中的所有频道
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    ///
    /// # 返回
    /// - 成功时返回频道列表
    /// - 失败时返回错误
    async fn list_channels(&self, subnet_id: SubnetId) -> Result<Vec<Channel>, SkyNetError>;

    /// 删除频道
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `channel_id`: 频道 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn delete_channel(&self, subnet_id: SubnetId, channel_id: ChannelId) -> Result<(), SkyNetError>;

    /// 更新权限策略
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `policy`: 新的权限策略
    ///
    /// # 返回
    /// - 成功时返回更新后的权限策略
    /// - 失败时返回错误
    async fn update_permission_policy(
        &self,
        subnet_id: SubnetId,
        policy: PermissionPolicy,
    ) -> Result<PermissionPolicy, SkyNetError>;
}

/// 消息管理服务 trait
///
/// 负责消息的发送、接收、编辑、删除以及消息表情反应、置顶、线程等功能。
#[async_trait]
pub trait MessageService: Send + Sync {
    /// 发送消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message`: 要发送的消息
    ///
    /// # 返回
    /// - 成功时返回发送后的消息
    /// - 失败时返回错误
    async fn send_message(&self, subnet_id: SubnetId, message: Message) -> Result<Message, SkyNetError>;

    /// 获取消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message_id`: 消息 ID
    ///
    /// # 返回
    /// - 成功时返回消息
    /// - 失败时返回错误
    async fn get_message(&self, subnet_id: SubnetId, message_id: MessageId) -> Result<Message, SkyNetError>;

    /// 获取频道的历史消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `channel_id`: 频道 ID
    /// - `limit`: 返回消息数量限制
    /// - `before`: 只返回此时间戳之前的消息（可选）
    ///
    /// # 返回
    /// - 成功时返回消息列表
    /// - 失败时返回错误
    async fn get_channel_messages(
        &self,
        subnet_id: SubnetId,
        channel_id: ChannelId,
        limit: u32,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<Message>, SkyNetError>;

    /// 获取私聊消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `user_id1`: 用户 1 的 ID
    /// - `user_id2`: 用户 2 的 ID
    /// - `limit`: 返回消息数量限制
    /// - `before`: 只返回此时间戳之前的消息（可选）
    ///
    /// # 返回
    /// - 成功时返回消息列表
    /// - 失败时返回错误
    async fn get_private_messages(
        &self,
        subnet_id: SubnetId,
        user_id1: UserId,
        user_id2: UserId,
        limit: u32,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<Message>, SkyNetError>;

    /// 编辑消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message_id`: 要编辑的消息 ID
    /// - `new_content`: 新的消息内容
    ///
    /// # 返回
    /// - 成功时返回更新后的消息
    /// - 失败时返回错误
    async fn edit_message(
        &self,
        subnet_id: SubnetId,
        message_id: MessageId,
        new_content: skynet_types::chat::MessageContent,
    ) -> Result<Message, SkyNetError>;

    /// 删除消息（软删除）
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message_id`: 要删除的消息 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn delete_message(&self, subnet_id: SubnetId, message_id: MessageId) -> Result<(), SkyNetError>;

    /// 添加消息表情反应
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `reaction`: 要添加的表情反应
    ///
    /// # 返回
    /// - 成功时返回添加后的表情反应
    /// - 失败时返回错误
    async fn add_reaction(&self, subnet_id: SubnetId, reaction: Reaction) -> Result<Reaction, SkyNetError>;

    /// 移除消息表情反应
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `reaction_id`: 要移除的表情反应 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn remove_reaction(&self, subnet_id: SubnetId, reaction_id: String) -> Result<(), SkyNetError>;

    /// 获取消息的所有表情反应
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message_id`: 消息 ID
    ///
    /// # 返回
    /// - 成功时返回表情反应列表
    /// - 失败时返回错误
    async fn get_reactions(&self, subnet_id: SubnetId, message_id: MessageId) -> Result<Vec<Reaction>, SkyNetError>;

    /// 置顶消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `pin`: 要添加的置顶信息
    ///
    /// # 返回
    /// - 成功时返回添加后的置顶信息
    /// - 失败时返回错误
    async fn pin_message(&self, subnet_id: SubnetId, pin: Pin) -> Result<Pin, SkyNetError>;

    /// 取消置顶消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `pin_id`: 要移除的置顶 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn unpin_message(&self, subnet_id: SubnetId, pin_id: String) -> Result<(), SkyNetError>;

    /// 获取频道或私聊的置顶消息列表
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `channel_id`: 频道 ID（私聊时为 None）
    ///
    /// # 返回
    /// - 成功时返回置顶列表
    /// - 失败时返回错误
    async fn get_pinned_messages(&self, subnet_id: SubnetId, channel_id: Option<ChannelId>) -> Result<Vec<Pin>, SkyNetError>;

    /// 创建消息线程
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `thread`: 要创建的线程
    ///
    /// # 返回
    /// - 成功时返回创建的线程
    /// - 失败时返回错误
    async fn create_thread(&self, subnet_id: SubnetId, thread: Thread) -> Result<Thread, SkyNetError>;

    /// 获取线程消息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `thread_id`: 线程 ID（即父消息的 ID）
    /// - `limit`: 返回消息数量限制
    ///
    /// # 返回
    /// - 成功时返回线程内的消息列表
    /// - 失败时返回错误
    async fn get_thread_messages(
        &self,
        subnet_id: SubnetId,
        thread_id: MessageId,
        limit: u32,
    ) -> Result<Vec<Message>, SkyNetError>;

    /// 更新消息状态
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `message_id`: 消息 ID
    /// - `status`: 新的消息状态
    ///
    /// # 返回
    /// - 成功时返回更新后的消息
    /// - 失败时返回错误
    async fn update_message_status(
        &self,
        subnet_id: SubnetId,
        message_id: MessageId,
        status: skynet_types::chat::MessageStatus,
    ) -> Result<Message, SkyNetError>;
}

/// 资源管理服务 trait
///
/// 负责资源的上传、下载、更新、删除等功能。
#[async_trait]
pub trait ResourceService: Send + Sync {
    /// 创建资源
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource`: 要创建的资源
    /// - `content`: 资源内容（字节数据）
    ///
    /// # 返回
    /// - 成功时返回创建的资源
    /// - 失败时返回错误
    async fn create_resource(&self, subnet_id: SubnetId, resource: Resource, content: Vec<u8>)
    -> Result<Resource, SkyNetError>;

    /// 获取资源信息
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 资源 ID
    ///
    /// # 返回
    /// - 成功时返回资源信息
    /// - 失败时返回错误
    async fn get_resource(&self, subnet_id: SubnetId, resource_id: ResourceId) -> Result<Resource, SkyNetError>;

    /// 下载资源内容
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 资源 ID
    ///
    /// # 返回
    /// - 成功时返回资源内容（字节数据）
    /// - 失败时返回错误
    async fn download_resource(&self, subnet_id: SubnetId, resource_id: ResourceId) -> Result<Vec<u8>, SkyNetError>;

    /// 更新资源
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 要更新的资源 ID
    /// - `new_resource`: 新的资源信息
    /// - `new_content`: 新的资源内容（可选）
    ///
    /// # 返回
    /// - 成功时返回更新后的资源
    /// - 失败时返回错误
    async fn update_resource(
        &self,
        subnet_id: SubnetId,
        resource_id: ResourceId,
        new_resource: Resource,
        new_content: Option<Vec<u8>>,
    ) -> Result<Resource, SkyNetError>;

    /// 删除资源（软删除）
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 要删除的资源 ID
    ///
    /// # 返回
    /// - 成功时返回 ()
    /// - 失败时返回错误
    async fn delete_resource(&self, subnet_id: SubnetId, resource_id: ResourceId) -> Result<(), SkyNetError>;

    /// 列出子网中的资源
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_type`: 资源类型过滤（可选）
    /// - `limit`: 返回数量限制
    ///
    /// # 返回
    /// - 成功时返回资源列表
    /// - 失败时返回错误
    async fn list_resources(
        &self,
        subnet_id: SubnetId,
        resource_type: Option<skynet_types::resource::ResourceType>,
        limit: u32,
    ) -> Result<Vec<Resource>, SkyNetError>;

    /// 搜索资源
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `query`: 搜索查询字符串
    /// - `limit`: 返回数量限制
    ///
    /// # 返回
    /// - 成功时返回匹配的资源列表
    /// - 失败时返回错误
    async fn search_resources(&self, subnet_id: SubnetId, query: String, limit: u32) -> Result<Vec<Resource>, SkyNetError>;

    /// 获取资源的历史版本
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 资源 ID
    ///
    /// # 返回
    /// - 成功时返回资源的历史版本列表
    /// - 失败时返回错误
    async fn get_resource_versions(&self, subnet_id: SubnetId, resource_id: ResourceId) -> Result<Vec<Resource>, SkyNetError>;

    /// 恢复资源到特定版本
    ///
    /// # 参数
    /// - `subnet_id`: 子网 ID
    /// - `resource_id`: 资源 ID
    /// - `version`: 要恢复到的版本号
    ///
    /// # 返回
    /// - 成功时返回恢复后的资源
    /// - 失败时返回错误
    async fn restore_resource_version(
        &self,
        subnet_id: SubnetId,
        resource_id: ResourceId,
        version: u32,
    ) -> Result<Resource, SkyNetError>;
}
