use crate::id::{AuthId, ChannelId, SubnetId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 子网类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubnetType {
    /// 组织子网，用于组织、公司的通信和协作
    Organization,
    /// 社区子网，用于普通社区、兴趣小组的通信
    Community,
    /// 私有子网，用于小型私密群组通信
    Private,
    /// 自定义子网类型
    Custom(String),
}

/// 成员角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    /// 管理员
    Admin,
    /// 普通成员
    Member,
}

/// 频道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// 私聊
    Private,
    /// 群聊
    Group,
    /// 公告频道
    Announcement,
}

/// 子网元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetMetadata {
    /// 子网类型
    pub subnet_type: SubnetType,
    /// 子网类型版本
    pub subnet_version: String,
    /// 子网名称
    pub name: String,
    /// 子网描述
    pub description: Option<String>,
    /// 子网图标
    pub icon: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 创建者
    pub created_by: AuthId,
    /// 功能特性列表
    pub features: Option<Vec<String>>,
    /// 自定义元数据
    pub metadata: Option<Value>,
}

/// 成员结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// 用户在子网内的本地 ID
    pub user_id: Option<UserId>,
    /// 用户全局身份 ID
    pub auth_id: AuthId,
    /// 用户的 Ed25519 公钥
    pub public_key: Vec<u8>,
    /// 角色
    pub role: MemberRole,
    /// 加入时间
    pub joined_at: DateTime<Utc>,
    /// 用户在该子网内使用的设备公钥列表
    pub device_public_keys: Vec<Vec<u8>>,
    /// 该用户在子网内的联系人列表
    pub contacts: Option<Vec<UserId>>,
}

/// 频道结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// 频道/群组的唯一 ID
    pub channel_id: ChannelId,
    /// 频道名称
    pub name: String,
    /// 创建者
    pub created_by: UserId,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 频道成员列表
    pub members: Vec<UserId>,
    /// 频道类型
    pub channel_type: ChannelType,
}

/// 权限策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPolicy {
    /// 谁可以添加/移除成员
    pub can_manage_members: Vec<MemberRole>,
    /// 谁可以创建/删除频道
    pub can_manage_channels: Vec<MemberRole>,
    /// 谁可以修改频道信息
    pub can_modify_channel_info: Vec<MemberRole>,
}

/// 子网结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    /// 子网唯一 ID
    pub subnet_id: SubnetId,
    /// 子网元信息
    pub metadata: SubnetMetadata,
    /// 成员列表
    pub members: Vec<Member>,
    /// 频道列表
    pub channels: Vec<Channel>,
    /// 权限策略
    pub permission_policy: PermissionPolicy,
}
