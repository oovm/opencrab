use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{SubnetId, UserId};

/// 用户在线状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    /// 用户当前活跃，正在使用客户端
    Online,
    /// 用户在线但忙碌，可能无法及时回复
    Busy,
    /// 用户在线但离开一段时间（如 5 分钟无操作）
    Away,
    /// 用户不在线或没有活动连接
    Offline,
}

/// 用户元信息（Profile）描述用户的公开信息，包括头像、昵称、个人简介、在线状态等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 用户 ID
    pub user_id: UserId,
    /// 子网 ID
    pub subnet_id: SubnetId,
    /// 头像：资源引用或 URL（可选）
    pub avatar: Option<String>,
    /// 昵称：子网内显示名称（可选）
    pub nickname: Option<String>,
    /// 个人简介（可选）
    pub bio: Option<String>,
    /// 自定义状态文本（可选，如"正在开会"、"外出吃饭"）
    pub status_text: Option<String>,
    /// 在线状态（可选）
    pub presence_status: Option<PresenceStatus>,
    /// 最后活跃时间（可选）
    pub last_active_at: Option<DateTime<Utc>>,
    /// 元信息更新时间（可选）
    pub updated_at: Option<DateTime<Utc>>,
    /// 设备信息：JSON 对象（可选，如设备类型、操作系统等）
    pub device_info: Option<Value>,
}
