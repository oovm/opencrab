use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// 智能体类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    /// 助手
    Assistant,
    /// 工作者
    Worker,
    /// 管理者
    Manager,
    /// 自定义
    Custom,
}

/// 智能体状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    /// 离线
    Offline,
    /// 在线
    Online,
    /// 忙碌
    Busy,
    /// 错误
    Error,
}

/// 智能体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// 智能体设置
    pub settings: Value,
    /// 最大并发任务数
    pub max_concurrent_tasks: u32,
    /// 超时时间（秒）
    pub timeout_seconds: u32,
}

/// 智能体结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// 智能体唯一标识符
    pub id: Uuid,
    /// 智能体名称
    pub name: String,
    /// 智能体描述
    pub description: Option<String>,
    /// 智能体头像URL
    pub avatar_url: Option<String>,
    /// 智能体类型
    pub agent_type: AgentType,
    /// 智能体能力列表
    pub capabilities: Vec<String>,
    /// 智能体配置
    pub config: AgentConfig,
    /// 智能体状态
    pub status: AgentStatus,
    /// 智能体创建时间
    pub created_at: DateTime<Utc>,
    /// 智能体最后更新时间
    pub updated_at: DateTime<Utc>,
}

/// 调用状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationStatus {
    /// 待处理
    Pending,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

/// 智能体调用请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInvocationParams {
    /// 智能体ID
    pub agent_id: Uuid,
    /// 任务ID
    pub task_id: Uuid,
    /// 对话ID
    pub conversation_id: Option<Uuid>,
    /// 上下文数据
    pub context: Vec<Value>,
    /// 输入内容
    pub input: String,
    /// 调用参数
    pub parameters: Value,
}

/// 智能体调用响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInvocationResult {
    /// 任务ID
    pub task_id: Uuid,
    /// 调用状态
    pub status: InvocationStatus,
    /// 输出内容
    pub output: Option<String>,
    /// 相关消息列表
    pub messages: Vec<Value>,
    /// 执行动作列表
    pub actions: Vec<Value>,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 角色类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    /// 领导者
    Leader,
    /// 开发者
    Developer,
    /// 设计师
    Designer,
    /// 产品经理
    ProductManager,
    /// 研究员
    Researcher,
    /// 运营
    Operator,
    /// 支持者
    Supporter,
    /// 其他
    Other,
}

/// 员工状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeStatus {
    /// 试用中
    Probation,
    /// 在职
    Active,
    /// 休假
    OnLeave,
    /// 离职
    Resigned,
}

/// 员工结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    /// 员工唯一标识符
    pub id: Uuid,
    /// 员工名称
    pub name: String,
    /// 员工头像
    pub avatar: Option<String>,
    /// 职位
    pub title: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 角色类型
    pub role_type: RoleType,
    /// 所属公司ID
    pub company_id: Uuid,
    /// 员工创建时间
    pub created_at: DateTime<Utc>,
    /// 员工最后更新时间
    pub updated_at: DateTime<Utc>,
}

/// 技能体系数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSystem {
    /// 所属员工ID
    pub employee_id: Uuid,
    /// 核心技能
    pub core_skills: Vec<Uuid>,
    /// 专业技能
    pub professional_skills: Vec<Uuid>,
    /// 工具技能
    pub tool_skills: Vec<Uuid>,
    /// 软技能
    pub soft_skills: Vec<Uuid>,
}

/// 技能级别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillLevel {
    /// 入门
    Beginner,
    /// 初级
    Novice,
    /// 中级
    Intermediate,
    /// 高级
    Advanced,
    /// 专家
    Expert,
    /// 大师
    Master,
}

/// 技能评级数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRating {
    /// 所属员工ID
    pub employee_id: Uuid,
    /// 技能ID
    pub skill_id: Uuid,
    /// 技能级别
    pub level: SkillLevel,
    /// 最后更新时间
    pub last_updated: DateTime<Utc>,
}

/// 响应速度枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSpeed {
    /// 即时
    Immediate,
    /// 快速
    Fast,
    /// 适中
    Moderate,
    /// 慢速
    Slow,
    /// 谨慎
    Careful,
}

/// 决策方式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionStyle {
    /// 快速决策
    Quick,
    /// 数据驱动
    DataDriven,
    /// 协作式
    Collaborative,
    /// 审慎
    Deliberative,
    /// 直觉
    Intuitive,
}

/// 沟通风格枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunicationStyle {
    /// 直接
    Direct,
    /// 间接
    Indirect,
    /// 详细
    Detailed,
    /// 简洁
    Concise,
    /// 协作式
    Collaborative,
}

/// 风险偏好枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskPreference {
    /// 保守
    Conservative,
    /// 审慎
    Cautious,
    /// 平衡
    Balanced,
    /// 冒险
    Aggressive,
    /// 高风险
    HighRisk,
}

/// 工作风格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStyle {
    /// 所属员工ID
    pub employee_id: Uuid,
    /// 响应速度
    pub response_speed: ResponseSpeed,
    /// 决策方式
    pub decision_style: DecisionStyle,
    /// 沟通风格
    pub communication_style: CommunicationStyle,
    /// 风险偏好
    pub risk_preference: RiskPreference,
}

/// 技能状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillStatus {
    /// 未加载
    Unloaded,
    /// 已加载
    Loaded,
    /// 运行中
    Running,
    /// 错误
    Error,
    /// 已禁用
    Disabled,
}

/// 技能类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    /// 内置技能
    Builtin,
    /// 自定义技能
    Custom,
    /// 工作区技能
    Workspace,
}

/// 技能权限枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillPermission {
    /// 读取文件
    ReadFile,
    /// 写入文件
    WriteFile,
    /// 执行命令
    ExecuteCommand,
    /// 网络访问
    NetworkAccess,
    /// 数据库访问
    DatabaseAccess,
    /// 内存访问
    MemoryAccess,
    /// 自定义权限
    Custom(String),
}

/// 技能清单结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    /// 技能清单唯一标识符
    pub id: Uuid,
    /// 技能名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 技能描述
    pub description: String,
    /// 作者
    pub author: String,
    /// 技能类型
    pub skill_type: SkillType,
    /// 权限列表
    pub permissions: Vec<SkillPermission>,
    /// 入口点
    pub entry_point: String,
    /// 依赖列表
    pub dependencies: Vec<Uuid>,
    /// 元数据
    pub metadata: serde_json::Value,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 技能版本结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillVersion {
    /// 版本唯一标识符
    pub id: Uuid,
    /// 技能ID
    pub skill_id: Uuid,
    /// 版本号
    pub version: String,
    /// 更新日志
    pub changelog: String,
    /// 是否为当前版本
    pub is_current: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 工作区技能配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSkillConfig {
    /// 配置唯一标识符
    pub id: Uuid,
    /// 工作区ID
    pub workspace_id: Uuid,
    /// 技能ID
    pub skill_id: Uuid,
    /// 是否启用
    pub enabled: bool,
    /// 技能配置
    pub config: serde_json::Value,
    /// 授权的权限
    pub granted_permissions: Vec<SkillPermission>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
}

/// 技能结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// 技能唯一标识符
    pub id: Uuid,
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: String,
    /// 技能类型
    pub skill_type: SkillType,
    /// 当前版本
    pub current_version: SkillVersion,
    /// 所有版本
    pub versions: Vec<SkillVersion>,
    /// 技能清单
    pub manifest: SkillManifest,
    /// 技能状态
    pub status: SkillStatus,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 元数据
    pub metadata: serde_json::Value,
}
