#![warn(missing_docs)]
use async_trait::async_trait;
use augur_types::{Project, ProjectStatus, Team, TeamType, Workspace, WorkspaceMember};
use serde::{Deserialize, Serialize};
use skynet_types::{
    Agent, Department, Employee, EmployeeStatus, Memory, MemorySearchQuery, MemorySearchResult, MemoryTag, MemoryType, Message,
    Organization, Role, Skill, SkillManifest, SkillRating, SkillSystem, SkillType, SkillVersion, SkyNetError, SkyNetResult,
    WorkStyle, WorkspaceSkillConfig, Conversation,
};
use uuid::Uuid;

/// 对话状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    /// 活跃状态
    Active,
    /// 归档状态
    Archived,
    /// 删除状态
    Deleted,
}

/// 持久化服务 trait，提供统一的持久化管理接口
#[async_trait]
pub trait PersistenceService: Send + Sync {
    /// 开始一个事务
    ///
    /// # 返回
    /// - 成功时返回事务ID
    /// - 失败时返回错误
    async fn begin_transaction(&self) -> SkyNetResult<Uuid>;

    /// 提交事务
    ///
    /// # 参数
    /// - `transaction_id`: 事务ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn commit_transaction(&self, transaction_id: Uuid) -> SkyNetResult<()>;

    /// 回滚事务
    ///
    /// # 参数
    /// - `transaction_id`: 事务ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn rollback_transaction(&self, transaction_id: Uuid) -> SkyNetResult<()>;

    /// 检查连接状态
    ///
    /// # 返回
    /// - 成功时返回连接状态（true 表示已连接）
    /// - 失败时返回错误
    async fn is_connected(&self) -> SkyNetResult<bool>;

    /// 关闭持久化连接
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn close(&self) -> SkyNetResult<()>;

    /// 获取用户仓储
    ///
    /// # 返回
    /// - 用户仓储实例
    fn user_repository(&self) -> Box<dyn UserRepository>;

    /// 获取组织仓储
    ///
    /// # 返回
    /// - 组织仓储实例
    fn organization_repository(&self) -> Box<dyn OrganizationRepository>;

    /// 获取部门仓储
    ///
    /// # 返回
    /// - 部门仓储实例
    fn department_repository(&self) -> Box<dyn DepartmentRepository>;

    /// 获取角色仓储
    ///
    /// # 返回
    /// - 角色仓储实例
    fn role_repository(&self) -> Box<dyn RoleRepository>;

    /// 获取对话仓储
    ///
    /// # 返回
    /// - 对话仓储实例
    fn conversation_repository(&self) -> Box<dyn ConversationRepository>;

    /// 获取消息仓储
    ///
    /// # 返回
    /// - 消息仓储实例
    fn message_repository(&self) -> Box<dyn MessageRepository>;

    /// 获取智能体仓储
    ///
    /// # 返回
    /// - 智能体仓储实例
    fn agent_repository(&self) -> Box<dyn AgentRepository>;

    /// 获取记忆仓储
    ///
    /// # 返回
    /// - 记忆仓储实例
    fn memory_repository(&self) -> Box<dyn MemoryRepository>;

    /// 获取记忆标签仓储
    ///
    /// # 返回
    /// - 记忆标签仓储实例
    fn memory_tag_repository(&self) -> Box<dyn MemoryTagRepository>;

    /// 获取技能仓储
    ///
    /// # 返回
    /// - 技能仓储实例
    fn skill_repository(&self) -> Box<dyn SkillRepository>;

    /// 获取技能清单仓储
    ///
    /// # 返回
    /// - 技能清单仓储实例
    fn skill_manifest_repository(&self) -> Box<dyn SkillManifestRepository>;

    /// 获取工作区技能配置仓储
    ///
    /// # 返回
    /// - 工作区技能配置仓储实例
    fn workspace_skill_config_repository(&self) -> Box<dyn WorkspaceSkillConfigRepository>;

    /// 获取员工仓储
    ///
    /// # 返回
    /// - 员工仓储实例
    fn employee_repository(&self) -> Box<dyn EmployeeRepository>;

    /// 获取工作区仓储
    ///
    /// # 返回
    /// - 工作区仓储实例
    fn workspace_repository(&self) -> Box<dyn WorkspaceRepository>;

    /// 获取工作区成员仓储
    ///
    /// # 返回
    /// - 工作区成员仓储实例
    fn workspace_member_repository(&self) -> Box<dyn WorkspaceMemberRepository>;

    /// 获取项目仓储
    ///
    /// # 返回
    /// - 项目仓储实例
    fn project_repository(&self) -> Box<dyn ProjectRepository>;

    /// 获取团队仓储
    ///
    /// # 返回
    /// - 团队仓储实例
    fn team_repository(&self) -> Box<dyn TeamRepository>;

    /// 获取技能版本仓储
    ///
    /// # 返回
    /// - 技能版本仓储实例
    fn skill_version_repository(&self) -> Box<dyn SkillVersionRepository>;
}

/// 通用仓储 trait，定义基础的 CRUD 操作
#[async_trait]
pub trait Repository<T>: Send + Sync {
    /// 根据ID查找实体
    ///
    /// # 参数
    /// - `id`: 实体ID
    ///
    /// # 返回
    /// - 成功时返回实体，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_id(&self, id: Uuid) -> SkyNetResult<Option<T>>;

    /// 保存实体
    ///
    /// # 参数
    /// - `entity`: 要保存的实体
    ///
    /// # 返回
    /// - 成功时返回保存后的实体
    /// - 失败时返回错误
    async fn save(&self, entity: T) -> SkyNetResult<T>;

    /// 更新实体
    ///
    /// # 参数
    /// - `entity`: 要更新的实体
    ///
    /// # 返回
    /// - 成功时返回更新后的实体
    /// - 失败时返回错误
    async fn update(&self, entity: T) -> SkyNetResult<T>;

    /// 删除实体
    ///
    /// # 参数
    /// - `id`: 实体ID
    ///
    /// # 返回
    /// - 成功时返回是否删除成功
    /// - 失败时返回错误
    async fn delete(&self, id: Uuid) -> SkyNetResult<bool>;

    /// 检查实体是否存在
    ///
    /// # 参数
    /// - `id`: 实体ID
    ///
    /// # 返回
    /// - 成功时返回是否存在
    /// - 失败时返回错误
    async fn exists(&self, id: Uuid) -> SkyNetResult<bool>;

    /// 获取所有实体
    ///
    /// # 参数
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回实体列表
    /// - 失败时返回错误
    async fn find_all(&self, limit: u32, offset: u32) -> SkyNetResult<Vec<T>>;
}

/// 用户仓储 trait（使用 Employee 类型）
#[async_trait]
pub trait UserRepository: Repository<Employee> {
    /// 根据用户名查找用户
    ///
    /// # 参数
    /// - `username`: 用户名
    ///
    /// # 返回
    /// - 成功时返回用户，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_username(&self, username: &str) -> SkyNetResult<Option<Employee>>;

    /// 根据邮箱查找用户
    ///
    /// # 参数
    /// - `email`: 邮箱
    ///
    /// # 返回
    /// - 成功时返回用户，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_email(&self, email: &str) -> SkyNetResult<Option<Employee>>;

    /// 根据部门ID查找用户列表
    ///
    /// # 参数
    /// - `department_id`: 部门ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回用户列表
    /// - 失败时返回错误
    async fn find_by_department(&self, department_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Employee>>;

    /// 根据角色ID查找用户列表
    ///
    /// # 参数
    /// - `role_id`: 角色ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回用户列表
    /// - 失败时返回错误
    async fn find_by_role(&self, role_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Employee>>;
}

/// 组织仓储 trait
#[async_trait]
pub trait OrganizationRepository: Repository<Organization> {
    /// 根据名称查找组织
    ///
    /// # 参数
    /// - `name`: 组织名称
    ///
    /// # 返回
    /// - 成功时返回组织，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_name(&self, name: &str) -> SkyNetResult<Option<Organization>>;
}

/// 部门仓储 trait
#[async_trait]
pub trait DepartmentRepository: Repository<Department> {
    /// 根据组织ID查找部门列表
    ///
    /// # 参数
    /// - `org_id`: 组织ID
    ///
    /// # 返回
    /// - 成功时返回部门列表
    /// - 失败时返回错误
    async fn find_by_organization(&self, org_id: Uuid) -> SkyNetResult<Vec<Department>>;

    /// 根据上级部门ID查找子部门列表
    ///
    /// # 参数
    /// - `parent_id`: 上级部门ID
    ///
    /// # 返回
    /// - 成功时返回部门列表
    /// - 失败时返回错误
    async fn find_by_parent(&self, parent_id: Uuid) -> SkyNetResult<Vec<Department>>;
}

/// 角色仓储 trait
#[async_trait]
pub trait RoleRepository: Repository<Role> {
    /// 根据组织ID查找角色列表
    ///
    /// # 参数
    /// - `org_id`: 组织ID
    ///
    /// # 返回
    /// - 成功时返回角色列表
    /// - 失败时返回错误
    async fn find_by_organization(&self, org_id: Uuid) -> SkyNetResult<Vec<Role>>;

    /// 根据名称和组织ID查找角色
    ///
    /// # 参数
    /// - `name`: 角色名称
    /// - `org_id`: 组织ID
    ///
    /// # 返回
    /// - 成功时返回角色，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_name_and_org(&self, name: &str, org_id: Uuid) -> SkyNetResult<Option<Role>>;
}

/// 对话仓储 trait
#[async_trait]
pub trait ConversationRepository: Repository<Conversation> {
    /// 根据用户ID查找对话列表
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回对话列表
    /// - 失败时返回错误
    async fn find_by_participant(&self, user_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Conversation>>;

    /// 根据状态查找对话列表
    ///
    /// # 参数
    /// - `status`: 对话状态
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回对话列表
    /// - 失败时返回错误
    async fn find_by_status(&self, status: ConversationStatus, limit: u32, offset: u32) -> SkyNetResult<Vec<Conversation>>;

    /// 归档对话（将状态设为 Archived）
    ///
    /// # 参数
    /// - `conversation_id`: 对话ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn archive_conversation(&self, conversation_id: Uuid) -> SkyNetResult<()>;

    /// 恢复归档对话（将状态设为 Active）
    ///
    /// # 参数
    /// - `conversation_id`: 对话ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn unarchive_conversation(&self, conversation_id: Uuid) -> SkyNetResult<()>;

    /// 查询归档对话
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回归档对话列表
    /// - 失败时返回错误
    async fn find_archived_by_participant(&self, user_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Conversation>>;

    /// 查询活跃对话
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回活跃对话列表
    /// - 失败时返回错误
    async fn find_active_by_participant(&self, user_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Conversation>>;

    /// 删除对话（将状态设为 Deleted）
    ///
    /// # 参数
    /// - `id`: 对话ID
    ///
    /// # 返回
    /// - 成功时返回更新后的对话
    /// - 失败时返回错误
    async fn delete_conversation(&self, id: Uuid) -> SkyNetResult<Conversation>;
}

/// 消息仓储 trait
#[async_trait]
pub trait MessageRepository: Repository<Message> {
    /// 根据对话ID查找消息列表
    ///
    /// # 参数
    /// - `conversation_id`: 对话ID
    /// - `limit`: 返回数量限制
    /// - `before_id`: 仅返回此ID之前的消息（可选）
    ///
    /// # 返回
    /// - 成功时返回消息列表
    /// - 失败时返回错误
    async fn find_by_conversation(
        &self,
        conversation_id: Uuid,
        limit: u32,
        before_id: Option<Uuid>,
    ) -> SkyNetResult<Vec<Message>>;

    /// 获取对话的最新消息
    ///
    /// # 参数
    /// - `conversation_id`: 对话ID
    ///
    /// # 返回
    /// - 成功时返回最新消息，不存在时返回None
    /// - 失败时返回错误
    async fn find_latest_by_conversation(&self, conversation_id: Uuid) -> SkyNetResult<Option<Message>>;
}

/// 智能体仓储 trait
#[async_trait]
pub trait AgentRepository: Repository<Agent> {
    /// 根据智能体类型查找智能体列表
    ///
    /// # 参数
    /// - `agent_type`: 智能体类型
    ///
    /// # 返回
    /// - 成功时返回智能体列表
    /// - 失败时返回错误
    async fn find_by_type(&self, agent_type: skynet_types::AgentType) -> SkyNetResult<Vec<Agent>>;

    /// 根据状态查找智能体列表
    ///
    /// # 参数
    /// - `status`: 智能体状态
    ///
    /// # 返回
    /// - 成功时返回智能体列表
    /// - 失败时返回错误
    async fn find_by_status(&self, status: skynet_types::AgentStatus) -> SkyNetResult<Vec<Agent>>;

    /// 根据名称查找智能体
    ///
    /// # 参数
    /// - `name`: 智能体名称
    ///
    /// # 返回
    /// - 成功时返回智能体，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_name(&self, name: &str) -> SkyNetResult<Option<Agent>>;
}

/// 记忆标签仓储 trait
#[async_trait]
pub trait MemoryTagRepository: Repository<MemoryTag> {
    /// 根据名称查找记忆标签
    ///
    /// # 参数
    /// - `name`: 标签名称
    ///
    /// # 返回
    /// - 成功时返回标签，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_name(&self, name: &str) -> SkyNetResult<Option<MemoryTag>>;

    /// 根据用户ID查找记忆标签列表
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回标签列表
    /// - 失败时返回错误
    async fn find_by_user(&self, user_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<MemoryTag>>;
}

/// 记忆仓储 trait
#[async_trait]
pub trait MemoryRepository: Repository<Memory> {
    /// 根据所有者ID和类型查找记忆
    ///
    /// # 参数
    /// - `owner_id`: 所有者ID
    /// - `owner_type`: 所有者类型
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回记忆列表
    /// - 失败时返回错误
    async fn find_by_owner(&self, owner_id: Uuid, owner_type: &str, limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;

    /// 根据记忆类型查找
    ///
    /// # 参数
    /// - `memory_type`: 记忆类型
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回记忆列表
    /// - 失败时返回错误
    async fn find_by_type(&self, memory_type: MemoryType, limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;

    /// 根据标签查找
    ///
    /// # 参数
    /// - `tag_id`: 标签ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回记忆列表
    /// - 失败时返回错误
    async fn find_by_tag(&self, tag_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;

    /// 关键词搜索记忆
    ///
    /// # 参数
    /// - `query`: 搜索查询参数
    ///
    /// # 返回
    /// - 成功时返回记忆搜索结果列表
    /// - 失败时返回错误
    async fn search_by_keyword(&self, query: MemorySearchQuery) -> SkyNetResult<Vec<MemorySearchResult>>;

    /// 查找关联记忆
    ///
    /// # 参数
    /// - `memory_id`: 记忆ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回关联的记忆列表
    /// - 失败时返回错误
    async fn find_related(&self, memory_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Memory>>;
}

/// 技能仓储 trait
#[async_trait]
pub trait SkillRepository: Repository<Skill> {
    /// 根据技能类型查找技能列表
    ///
    /// # 参数
    /// - `skill_type`: 技能类型
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回技能列表
    /// - 失败时返回错误
    async fn find_by_type(&self, skill_type: SkillType, limit: u32, offset: u32) -> SkyNetResult<Vec<Skill>>;

    /// 根据名称搜索技能
    ///
    /// # 参数
    /// - `name`: 技能名称（支持模糊搜索）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回技能列表
    /// - 失败时返回错误
    async fn search_by_name(&self, name: &str, limit: u32, offset: u32) -> SkyNetResult<Vec<Skill>>;

    /// 根据技能ID查找所有版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    ///
    /// # 返回
    /// - 成功时返回技能版本列表
    /// - 失败时返回错误
    async fn find_versions(&self, skill_id: Uuid) -> SkyNetResult<Vec<SkillVersion>>;

    /// 根据技能ID和版本号查找特定版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `version`: 版本号
    ///
    /// # 返回
    /// - 成功时返回技能版本，不存在时返回None
    /// - 失败时返回错误
    async fn find_version(&self, skill_id: Uuid, version: &str) -> SkyNetResult<Option<SkillVersion>>;

    /// 保存技能版本
    ///
    /// # 参数
    /// - `version`: 要保存的技能版本
    ///
    /// # 返回
    /// - 成功时返回保存后的技能版本
    /// - 失败时返回错误
    async fn save_version(&self, version: SkillVersion) -> SkyNetResult<SkillVersion>;

    /// 设置技能的当前版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `version_id`: 版本ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn set_current_version(&self, skill_id: Uuid, version_id: Uuid) -> SkyNetResult<()>;

    /// 查找启用的技能
    ///
    /// # 参数
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回启用的技能列表
    /// - 失败时返回错误
    async fn find_enabled(&self, limit: u32, offset: u32) -> SkyNetResult<Vec<Skill>>;

    /// 启用/禁用技能
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `enabled`: 是否启用
    ///
    /// # 返回
    /// - 成功时返回更新后的技能
    /// - 失败时返回错误
    async fn toggle_enabled(&self, skill_id: Uuid, enabled: bool) -> SkyNetResult<Skill>;
}

/// 技能清单仓储 trait
#[async_trait]
pub trait SkillManifestRepository: Repository<SkillManifest> {
    /// 根据技能ID查找技能清单
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    ///
    /// # 返回
    /// - 成功时返回技能清单，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_skill_id(&self, skill_id: Uuid) -> SkyNetResult<Option<SkillManifest>>;
}

/// 工作区技能配置仓储 trait
#[async_trait]
pub trait WorkspaceSkillConfigRepository: Repository<WorkspaceSkillConfig> {
    /// 根据工作区ID查找技能配置
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回技能配置列表
    /// - 失败时返回错误
    async fn find_by_workspace(&self, workspace_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<WorkspaceSkillConfig>>;

    /// 根据工作区ID和技能ID查找特定配置
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `skill_id`: 技能ID
    ///
    /// # 返回
    /// - 成功时返回技能配置，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_workspace_and_skill(
        &self,
        workspace_id: Uuid,
        skill_id: Uuid,
    ) -> SkyNetResult<Option<WorkspaceSkillConfig>>;

    /// 查找工作区启用的技能配置
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    ///
    /// # 返回
    /// - 成功时返回启用的技能配置列表
    /// - 失败时返回错误
    async fn find_enabled_by_workspace(&self, workspace_id: Uuid) -> SkyNetResult<Vec<WorkspaceSkillConfig>>;
}

/// 员工仓储 trait
#[async_trait]
pub trait EmployeeRepository: Repository<Employee> {
    /// 根据公司ID查找员工列表
    ///
    /// # 参数
    /// - `company_id`: 公司ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回员工列表
    /// - 失败时返回错误
    async fn find_by_company(&self, company_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Employee>>;

    /// 根据角色类型查找员工列表
    ///
    /// # 参数
    /// - `role_type`: 角色类型
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回员工列表
    /// - 失败时返回错误
    async fn find_by_role_type(
        &self,
        role_type: augur_types::RoleType,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<Employee>>;

    /// 根据员工ID保存或更新员工的技能系统
    ///
    /// # 参数
    /// - `skill_system`: 技能系统（包含 employee_id）
    ///
    /// # 返回
    /// - 成功时返回保存后的技能系统
    /// - 失败时返回错误
    async fn save_skill_system(&self, skill_system: SkillSystem) -> SkyNetResult<SkillSystem>;

    /// 根据员工ID获取技能系统
    ///
    /// # 参数
    /// - `employee_id`: 员工ID
    ///
    /// # 返回
    /// - 成功时返回技能系统，不存在时返回None
    /// - 失败时返回错误
    async fn find_skill_system(&self, employee_id: Uuid) -> SkyNetResult<Option<SkillSystem>>;

    /// 根据员工ID保存或更新员工的技能评级
    ///
    /// # 参数
    /// - `skill_ratings`: 技能评级列表（每个都包含 employee_id）
    ///
    /// # 返回
    /// - 成功时返回保存后的技能评级列表
    /// - 失败时返回错误
    async fn save_skill_ratings(&self, skill_ratings: Vec<SkillRating>) -> SkyNetResult<Vec<SkillRating>>;

    /// 根据员工ID获取技能评级列表
    ///
    /// # 参数
    /// - `employee_id`: 员工ID
    ///
    /// # 返回
    /// - 成功时返回技能评级列表
    /// - 失败时返回错误
    async fn find_skill_ratings(&self, employee_id: Uuid) -> SkyNetResult<Vec<SkillRating>>;

    /// 根据员工ID保存或更新员工的工作风格
    ///
    /// # 参数
    /// - `work_style`: 工作风格（包含 employee_id）
    ///
    /// # 返回
    /// - 成功时返回保存后的工作风格
    /// - 失败时返回错误
    async fn save_work_style(&self, work_style: WorkStyle) -> SkyNetResult<WorkStyle>;

    /// 根据员工ID获取工作风格
    ///
    /// # 参数
    /// - `employee_id`: 员工ID
    ///
    /// # 返回
    /// - 成功时返回工作风格，不存在时返回None
    /// - 失败时返回错误
    async fn find_work_style(&self, employee_id: Uuid) -> SkyNetResult<Option<WorkStyle>>;
}

/// 工作区仓储 trait
#[async_trait]
pub trait WorkspaceRepository: Repository<Workspace> {
    /// 根据组织ID查找工作区列表
    ///
    /// # 参数
    /// - `organization_id`: 组织ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回工作区列表
    /// - 失败时返回错误
    async fn find_by_organization(&self, organization_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Workspace>>;

    /// 根据创建者ID查找工作区列表
    ///
    /// # 参数
    /// - `creator_id`: 创建者ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回工作区列表
    /// - 失败时返回错误
    async fn find_by_creator(&self, creator_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Workspace>>;

    /// 根据名称搜索工作区
    ///
    /// # 参数
    /// - `name`: 工作区名称（支持模糊搜索）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回工作区列表
    /// - 失败时返回错误
    async fn search_by_name(&self, name: &str, limit: u32, offset: u32) -> SkyNetResult<Vec<Workspace>>;
}

/// 工作区成员仓储 trait
#[async_trait]
pub trait WorkspaceMemberRepository: Repository<WorkspaceMember> {
    /// 根据工作区ID查找成员列表
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回成员列表
    /// - 失败时返回错误
    async fn find_by_workspace(&self, workspace_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<WorkspaceMember>>;

    /// 根据用户ID查找工作区成员关系
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回成员关系列表
    /// - 失败时返回错误
    async fn find_by_user(&self, user_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<WorkspaceMember>>;

    /// 根据工作区ID和用户ID查找特定成员关系
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回成员关系，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_workspace_and_user(&self, workspace_id: Uuid, user_id: Uuid) -> SkyNetResult<Option<WorkspaceMember>>;

    /// 移除工作区成员
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// - 成功时返回是否删除成功
    /// - 失败时返回错误
    async fn remove_member(&self, workspace_id: Uuid, user_id: Uuid) -> SkyNetResult<bool>;
}

/// 项目仓储 trait
#[async_trait]
pub trait ProjectRepository: Repository<Project> {
    /// 根据工作区ID查找项目列表
    ///
    /// # 参数
    /// - `workspace_id`: 工作区ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回项目列表
    /// - 失败时返回错误
    async fn find_by_workspace(&self, workspace_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Project>>;

    /// 根据状态查找项目列表
    ///
    /// # 参数
    /// - `status`: 项目状态
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回项目列表
    /// - 失败时返回错误
    async fn find_by_status(&self, status: ProjectStatus, limit: u32, offset: u32) -> SkyNetResult<Vec<Project>>;

    /// 根据创建者ID查找项目列表
    ///
    /// # 参数
    /// - `creator_id`: 创建者ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回项目列表
    /// - 失败时返回错误
    async fn find_by_creator(&self, creator_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Project>>;

    /// 根据名称搜索项目
    ///
    /// # 参数
    /// - `name`: 项目名称（支持模糊搜索）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回项目列表
    /// - 失败时返回错误
    async fn search_by_name(&self, name: &str, limit: u32, offset: u32) -> SkyNetResult<Vec<Project>>;
}

/// 团队仓储 trait
#[async_trait]
pub trait TeamRepository: Repository<Team> {
    /// 根据公司ID查找团队列表
    ///
    /// # 参数
    /// - `company_id`: 公司ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回团队列表
    /// - 失败时返回错误
    async fn find_by_company(&self, company_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Team>>;

    /// 根据团队类型查找团队列表
    ///
    /// # 参数
    /// - `team_type`: 团队类型
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回团队列表
    /// - 失败时返回错误
    async fn find_by_type(&self, team_type: TeamType, limit: u32, offset: u32) -> SkyNetResult<Vec<Team>>;

    /// 根据父团队ID查找子团队列表
    ///
    /// # 参数
    /// - `parent_team_id`: 父团队ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回子团队列表
    /// - 失败时返回错误
    async fn find_by_parent(&self, parent_team_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<Team>>;

    /// 根据名称搜索团队
    ///
    /// # 参数
    /// - `name`: 团队名称（支持模糊搜索）
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回团队列表
    /// - 失败时返回错误
    async fn search_by_name(&self, name: &str, limit: u32, offset: u32) -> SkyNetResult<Vec<Team>>;
}

/// 技能版本仓储 trait
#[async_trait]
pub trait SkillVersionRepository: Repository<SkillVersion> {
    /// 根据技能ID查找版本列表
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `limit`: 返回数量限制
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 成功时返回技能版本列表
    /// - 失败时返回错误
    async fn find_by_skill(&self, skill_id: Uuid, limit: u32, offset: u32) -> SkyNetResult<Vec<SkillVersion>>;

    /// 根据技能ID和版本号查找特定版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `version`: 版本号
    ///
    /// # 返回
    /// - 成功时返回技能版本，不存在时返回None
    /// - 失败时返回错误
    async fn find_by_skill_and_version(&self, skill_id: Uuid, version: &str) -> SkyNetResult<Option<SkillVersion>>;

    /// 获取技能的当前版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    ///
    /// # 返回
    /// - 成功时返回当前版本，不存在时返回None
    /// - 失败时返回错误
    async fn find_current_version(&self, skill_id: Uuid) -> SkyNetResult<Option<SkillVersion>>;

    /// 设置技能的当前版本
    ///
    /// # 参数
    /// - `skill_id`: 技能ID
    /// - `version_id`: 版本ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn set_current_version(&self, skill_id: Uuid, version_id: Uuid) -> SkyNetResult<()>;
}
