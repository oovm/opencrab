use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// 记忆类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// 短期记忆
    ShortTerm,
    /// 长期记忆
    LongTerm,
    /// 工作记忆
    Working,
    /// 情景记忆
    Episodic,
    /// 语义记忆
    Semantic,
}

/// 记忆标签结构体用于标签管理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTag {
    /// 标签唯一标识符
    pub id: Uuid,
    /// 标签名称
    pub name: String,
    /// 标签描述
    pub description: Option<String>,
    /// 标签创建时间
    pub created_at: DateTime<Utc>,
}

/// 记忆关联结构体用于记忆关联
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRelation {
    /// 关联唯一标识符
    pub id: Uuid,
    /// 关联的源记忆ID
    pub source_memory_id: Uuid,
    /// 关联的目标记忆ID
    pub target_memory_id: Uuid,
    /// 关联类型
    pub relation_type: String,
    /// 关联权重
    pub weight: f64,
    /// 关联创建时间
    pub created_at: DateTime<Utc>,
}

/// 记忆结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 记忆唯一标识符
    pub id: Uuid,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 所有者ID
    pub owner_id: Uuid,
    /// 所有者类型
    pub owner_type: String,
    /// 记忆内容
    pub content: String,
    /// 记忆摘要
    pub summary: Option<String>,
    /// 标签列表
    pub tags: Vec<MemoryTag>,
    /// 关联列表
    pub relations: Vec<MemoryRelation>,
    /// 记忆创建时间
    pub created_at: DateTime<Utc>,
    /// 记忆最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 记忆元数据
    pub metadata: Value,
    /// 访问次数
    pub access_count: u64,
}

/// 记忆搜索查询结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchQuery {
    /// 用户ID
    pub user_id: Uuid,
    /// 搜索关键词
    pub keywords: Option<String>,
    /// 记忆类型过滤
    pub memory_types: Option<Vec<MemoryType>>,
    /// 标签ID过滤
    pub tag_ids: Option<Vec<Uuid>>,
    /// 时间范围起始
    pub start_time: Option<DateTime<Utc>>,
    /// 时间范围结束
    pub end_time: Option<DateTime<Utc>>,
    /// 返回数量限制
    pub limit: u32,
    /// 偏移量
    pub offset: u32,
}

/// 记忆搜索结果结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResult {
    /// 匹配的记忆
    pub memory: Memory,
    /// 相关性分数
    pub score: f64,
}
