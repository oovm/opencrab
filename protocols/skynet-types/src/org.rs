use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 组织结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// 组织唯一标识符
    pub id: Uuid,
    /// 组织名称
    pub name: String,
    /// 组织描述
    pub description: Option<String>,
    /// 组织Logo URL
    pub logo_url: Option<String>,
    /// 组织下属部门ID列表
    pub department_ids: Vec<Uuid>,
    /// 组织创建时间
    pub created_at: DateTime<Utc>,
    /// 组织最后更新时间
    pub updated_at: DateTime<Utc>,
}

/// 部门结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    /// 部门唯一标识符
    pub id: Uuid,
    /// 部门名称
    pub name: String,
    /// 上级部门ID
    pub parent_id: Option<Uuid>,
    /// 部门描述
    pub description: Option<String>,
    /// 部门成员ID列表
    pub member_ids: Vec<Uuid>,
    /// 部门负责人ID
    pub leader_id: Option<Uuid>,
    /// 部门排序值
    pub order: i32,
}

/// 角色结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// 角色唯一标识符
    pub id: Uuid,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
}
