#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use skynet_types::{SkyNetError, SkyNetErrorKind, SkyNetResult};
use uuid::Uuid;

/// 通知类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    /// 系统通知
    System,
    /// 聊天通知
    Chat,
    /// 任务通知
    Task,
    /// 文件通知
    File,
    /// 组织通知
    Organization,
    /// 自定义通知
    Custom,
}

/// 通知优先级枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPriority {
    /// 低优先级
    Low,
    /// 中优先级
    Medium,
    /// 高优先级
    High,
    /// 紧急优先级
    Urgent,
}

/// 通知状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    /// 未读
    Unread,
    /// 已读
    Read,
    /// 已归档
    Archived,
    /// 已删除
    Deleted,
}

/// 通知结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 通知唯一标识符
    pub id: Uuid,
    /// 接收者ID
    pub recipient_id: Uuid,
    /// 通知类型
    pub notification_type: NotificationType,
    /// 通知优先级
    pub priority: NotificationPriority,
    /// 通知标题
    pub title: String,
    /// 通知内容
    pub content: String,
    /// 通知元数据
    pub metadata: Option<Value>,
    /// 通知状态
    pub status: NotificationStatus,
    /// 通知创建时间
    pub created_at: DateTime<Utc>,
    /// 通知读取时间
    pub read_at: Option<DateTime<Utc>>,
    /// 关联的动作URL
    pub action_url: Option<String>,
}

/// 通知服务 trait，定义通知管理和发送相关的核心接口
#[async_trait]
pub trait NotificationService: Send + Sync {
    /// 发送通知给单个用户
    ///
    /// # 参数
    /// - `recipient_id`: 接收者ID
    /// - `notification_type`: 通知类型
    /// - `priority`: 通知优先级
    /// - `title`: 通知标题
    /// - `content`: 通知内容
    /// - `metadata`: 通知元数据
    /// - `action_url`: 关联的动作URL
    ///
    /// # 返回
    /// - 成功时返回创建的通知
    /// - 失败时返回错误
    async fn send_notification(
        &self,
        recipient_id: Uuid,
        notification_type: NotificationType,
        priority: NotificationPriority,
        title: &str,
        content: &str,
        metadata: Option<Value>,
        action_url: Option<&str>,
    ) -> SkyNetResult<Notification>;

    /// 批量发送通知
    ///
    /// # 参数
    /// - `recipient_ids`: 接收者ID列表
    /// - `notification_type`: 通知类型
    /// - `priority`: 通知优先级
    /// - `title`: 通知标题
    /// - `content`: 通知内容
    /// - `metadata`: 通知元数据
    /// - `action_url`: 关联的动作URL
    ///
    /// # 返回
    /// - 成功时返回创建的通知列表
    /// - 失败时返回错误
    async fn send_bulk_notifications(
        &self,
        recipient_ids: Vec<Uuid>,
        notification_type: NotificationType,
        priority: NotificationPriority,
        title: &str,
        content: &str,
        metadata: Option<Value>,
        action_url: Option<&str>,
    ) -> SkyNetResult<Vec<Notification>>;

    /// 获取通知信息
    ///
    /// # 参数
    /// - `notification_id`: 通知ID
    ///
    /// # 返回
    /// - 成功时返回通知信息
    /// - 失败时返回错误
    async fn get_notification(&self, notification_id: Uuid) -> SkyNetResult<Notification>;

    /// 获取用户的通知列表
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `status`: 通知状态过滤器（可选）
    /// - `notification_type`: 通知类型过滤器（可选）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回通知列表
    /// - 失败时返回错误
    async fn list_user_notifications(
        &self,
        user_id: Uuid,
        status: Option<NotificationStatus>,
        notification_type: Option<NotificationType>,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<Notification>>;

    /// 标记通知为已读
    ///
    /// # 参数
    /// - `notification_id`: 通知ID
    ///
    /// # 返回
    /// - 成功时返回更新后的通知
    /// - 失败时返回错误
    async fn mark_as_read(&self, notification_id: Uuid) -> SkyNetResult<Notification>;

    /// 标记用户的所有通知为已读
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn mark_all_as_read(&self, user_id: Uuid) -> SkyNetResult<()>;

    /// 归档通知
    ///
    /// # 参数
    /// - `notification_id`: 通知ID
    ///
    /// # 返回
    /// - 成功时返回更新后的通知
    /// - 失败时返回错误
    async fn archive_notification(&self, notification_id: Uuid) -> SkyNetResult<Notification>;

    /// 删除通知
    ///
    /// # 参数
    /// - `notification_id`: 通知ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn delete_notification(&self, notification_id: Uuid) -> SkyNetResult<()>;

    /// 批量删除通知
    ///
    /// # 参数
    /// - `notification_ids`: 通知ID列表
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn delete_notifications(&self, notification_ids: Vec<Uuid>) -> SkyNetResult<()>;

    /// 获取用户未读通知数量
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回未读通知数量
    /// - 失败时返回错误
    async fn get_unread_count(&self, user_id: Uuid) -> SkyNetResult<u64>;

    /// 订阅实时通知
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回通知流
    /// - 失败时返回错误
    async fn subscribe_notifications(&self, user_id: Uuid) -> SkyNetResult<()>;

    /// 取消订阅实时通知
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn unsubscribe_notifications(&self, user_id: Uuid) -> SkyNetResult<()>;
}
