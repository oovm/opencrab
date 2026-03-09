//! 数据类型定义
//!
//! 定义应用程序中使用的所有数据结构。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    /// 管理员用户
    Admin,
    /// 普通用户
    User,
    /// 访客用户
    Guest,
}

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一标识
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 显示名称
    pub display_name: Option<String>,
    /// 用户角色
    pub role: UserRole,
    /// 邮箱（可选）
    pub email: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 是否激活
    pub is_active: bool,
}

/// 聊天会话模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// 会话唯一标识
    pub id: Uuid,
    /// 所属用户 ID
    pub user_id: Uuid,
    /// 会话标题
    pub title: String,
    /// 会话描述（可选）
    pub description: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 是否归档
    pub is_archived: bool,
}

/// 聊天消息模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息唯一标识
    pub id: Uuid,
    /// 所属会话 ID
    pub conversation_id: Uuid,
    /// 发送者用户 ID
    pub user_id: Uuid,
    /// 消息角色（user/assistant/system）
    pub role: String,
    /// 消息内容
    pub content: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 消息元数据（JSON 格式）
    pub metadata: Option<String>,
}

/// 应用程序设置模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 设置 ID
    pub id: Uuid,
    /// 所属用户 ID
    pub user_id: Uuid,
    /// 主题设置
    pub theme: String,
    /// 语言设置
    pub language: String,
    /// API 端点
    pub api_endpoint: Option<String>,
    /// API 密钥（加密存储）
    pub api_key: Option<String>,
    /// 设置 JSON 数据
    pub settings_json: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户名
    pub username: String,
    /// 显示名称（可选）
    pub display_name: Option<String>,
    /// 用户角色
    pub role: UserRole,
    /// 邮箱（可选）
    pub email: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// 显示名称（可选）
    pub display_name: Option<String>,
    /// 邮箱（可选）
    pub email: Option<String>,
    /// 是否激活
    pub is_active: Option<bool>,
}

/// 创建会话请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    /// 会话标题
    pub title: String,
    /// 会话描述（可选）
    pub description: Option<String>,
}

/// 更新会话请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConversationRequest {
    /// 会话标题（可选）
    pub title: Option<String>,
    /// 会话描述（可选）
    pub description: Option<String>,
    /// 是否归档
    pub is_archived: Option<bool>,
}

/// 创建消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// 消息角色
    pub role: String,
    /// 消息内容
    pub content: String,
    /// 消息元数据（可选）
    pub metadata: Option<String>,
}
