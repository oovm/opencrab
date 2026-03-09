use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::id::{ResourceId, UserId};

/// 资源类型枚举，开放类型标识
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    /// 通用文件
    File,
    /// 图片文件
    Image,
    /// 文档文件
    Document,
    /// 视频文件
    Video,
    /// 音频文件
    Audio,
    /// 链接资源
    Link,
    /// 自定义资源类型，用于扩展
    Custom(String),
}

/// 资源结构体，子网内可访问的实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// 资源唯一 ID
    pub resource_id: ResourceId,
    /// 资源类型
    pub resource_type: ResourceType,
    /// 资源名称
    pub name: String,
    /// 资源描述（可选）
    pub description: Option<String>,
    /// 资源内容哈希（blake3 哈希，用于验证完整性）
    pub content_hash: [u8; 32],
    /// 资源内容存储位置（可选，如 IPFS CID、本地存储等）
    pub content_uri: Option<String>,
    /// 资源大小（字节）
    pub size: u64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 创建者
    pub created_by: UserId,
    /// 更新时间（可选）
    pub updated_at: Option<DateTime<Utc>>,
    /// 更新者（可选）
    pub updated_by: Option<UserId>,
    /// 版本号
    pub version: u32,
    /// 权限设置（可选）
    pub permissions: Option<Value>,
    /// 自定义元数据（可选）
    pub metadata: Option<Value>,
}

impl ResourceType {
    /// 从字符串创建资源类型
    pub fn from_str(s: &str) -> Self {
        match s {
            "file" => ResourceType::File,
            "image" => ResourceType::Image,
            "document" => ResourceType::Document,
            "video" => ResourceType::Video,
            "audio" => ResourceType::Audio,
            "link" => ResourceType::Link,
            _ => ResourceType::Custom(s.to_string()),
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::File => "file",
            ResourceType::Image => "image",
            ResourceType::Document => "document",
            ResourceType::Video => "video",
            ResourceType::Audio => "audio",
            ResourceType::Link => "link",
            ResourceType::Custom(s) => s,
        }
    }
}
