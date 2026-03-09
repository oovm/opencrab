#![warn(missing_docs)]

//! Crab Skill - OpenCrab 技能系统模块
//!
//! 提供技能的注册和管理功能。

pub use crab_types::Result;

/// 技能类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    /// 核心技能
    Core,
    /// 专业技能
    Professional,
    /// 工具技能
    Tool,
    /// 软技能
    Soft,
}

/// 技能结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Skill {
    /// 技能唯一标识符
    pub id: uuid::Uuid,
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: Option<String>,
    /// 技能类型
    pub skill_type: SkillType,
    /// 技能分类
    pub category: Option<String>,
    /// 技能标签
    pub tags: Vec<String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Skill {
    /// 创建新的技能
    pub fn new(name: String, skill_type: SkillType) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            description: None,
            skill_type,
            category: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// 技能服务 trait
#[async_trait::async_trait]
pub trait SkillService: Send + Sync {
    /// 注册技能
    async fn register_skill(
        &self,
        _name: &str,
        _description: &str,
        _skill_type: SkillType,
        _metadata: serde_json::Value,
    ) -> Result<Skill>;

    /// 获取技能
    async fn get_skill(&self, _skill_id: uuid::Uuid) -> Result<Skill>;

    /// 列出技能
    async fn list_skills(&self, _limit: u32, _offset: u32) -> Result<Vec<Skill>>;
}

/// 内存技能服务实现
pub struct MemorySkillService;

impl MemorySkillService {
    /// 创建新的内存技能服务
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl SkillService for MemorySkillService {
    async fn register_skill(
        &self,
        _name: &str,
        _description: &str,
        _skill_type: SkillType,
        _metadata: serde_json::Value,
    ) -> Result<Skill> {
        let mut skill = Skill::new(_name.to_string(), _skill_type);
        skill.description = Some(_description.to_string());
        Ok(skill)
    }

    async fn get_skill(&self, _skill_id: uuid::Uuid) -> Result<Skill> {
        Err(crab_types::Error::not_implemented("get_skill"))
    }

    async fn list_skills(&self, _limit: u32, _offset: u32) -> Result<Vec<Skill>> {
        Ok(Vec::new())
    }
}

impl Default for MemorySkillService {
    fn default() -> Self {
        Self::new()
    }
}
