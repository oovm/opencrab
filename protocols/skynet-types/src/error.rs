use std::fmt;

/// SkyNet 统一错误结构体
#[derive(Debug, Clone)]
pub struct SkyNetError {
    /// 错误类型（包含 i18n key 和参数）
    pub kind: Box<SkyNetErrorKind>,
}

/// SkyNet 错误类型枚举（每个变体对应一个 i18n 国际化键）
#[derive(Debug, Clone)]
pub enum SkyNetErrorKind {
    /// 验证错误
    Validation {
        /// 验证失败的字段名
        field: String,
    },
    /// 通用的未找到错误
    NotFound {
        /// 未找到的资源 ID
        id: String,
    },
    /// 未授权错误，表示缺少必要的权限
    Unauthorized,
    /// 内部错误
    Internal {
        /// 错误消息
        msg: String,
    },

    /// 文件未找到错误
    FileNotFound {
        /// 文件 ID
        id: String,
    },
    /// 文件已存在错误
    FileAlreadyExists {
        /// 文件名称
        name: String,
    },
    /// 文件过大错误
    FileTooLarge {
        /// 文件大小（字节）
        size: u64,
    },
    /// 不支持的文件类型错误
    UnsupportedFileType {
        /// 文件扩展名
        ext: String,
    },
    /// 无权限错误
    NoPermission,
    /// 存储错误
    StorageError {
        /// 错误消息
        msg: String,
    },

    /// 记忆未找到错误
    MemoryNotFound {
        /// 记忆 ID
        id: String,
    },
    /// 标签未找到错误
    TagNotFound {
        /// 标签 ID
        id: String,
    },
    /// 关系未找到错误
    RelationNotFound {
        /// 关系源 ID
        from: String,
        /// 关系目标 ID
        to: String,
    },
    /// 关系已存在错误
    RelationAlreadyExists {
        /// 关系源 ID
        from: String,
        /// 关系目标 ID
        to: String,
    },

    /// 实体未找到错误
    EntityNotFound {
        /// 实体 ID
        id: String,
    },
    /// 实体已存在错误
    EntityAlreadyExists {
        /// 实体 ID
        id: String,
    },
    /// 数据库错误
    DatabaseError {
        /// 错误消息
        msg: String,
    },
    /// 连接错误
    ConnectionError {
        /// 错误消息
        msg: String,
    },
    /// 序列化错误
    SerializationError {
        /// 错误消息
        msg: String,
    },
    /// 约束违反错误
    ConstraintViolation {
        /// 错误消息
        msg: String,
    },

    /// JSON 序列化错误
    JsonSerialization {
        /// 错误消息
        msg: String,
    },
    /// JSON 反序列化错误
    JsonDeserialization {
        /// 错误消息
        msg: String,
    },

    /// IO 错误
    Io {
        /// 错误消息
        msg: String,
    },

    /// 配置错误
    Config {
        /// 错误消息
        msg: String,
    },

    /// UUID 解析错误
    UuidParse {
        /// 错误消息
        msg: String,
    },

    /// 智能体未找到错误
    AgentNotFound {
        /// 智能体 ID
        id: String,
    },
    /// 智能体已存在错误
    AgentAlreadyExists {
        /// 智能体 ID
        id: String,
    },
    /// 智能体离线错误
    AgentOffline,
    /// 智能体忙碌错误
    AgentBusy,
    /// 任务未找到错误
    TaskNotFound {
        /// 任务 ID
        id: String,
    },
    /// 任务已完成错误
    TaskAlreadyCompleted,
    /// 任务已取消错误
    TaskAlreadyCanceled,
    /// 能力不支持错误
    CapabilityNotSupported,

    /// 会话未找到错误
    ConversationNotFound {
        /// 会话 ID
        id: String,
    },
    /// 消息未找到错误
    MessageNotFound {
        /// 消息 ID
        id: String,
    },
    /// 参与者未找到错误
    ParticipantNotFound {
        /// 参与者 ID
        id: String,
    },
    /// 参与者已存在错误
    ParticipantAlreadyExists {
        /// 参与者 ID
        id: String,
    },

    /// 凭据无效错误
    InvalidCredentials,
    /// 令牌无效错误
    InvalidToken,
    /// 令牌过期错误
    TokenExpired,
    /// 用户已存在错误
    UserAlreadyExists {
        /// 用户 ID
        id: String,
    },
    /// 用户未找到错误
    UserNotFound {
        /// 用户 ID
        id: String,
    },

    /// 组织未找到错误
    OrganizationNotFound {
        /// 组织 ID
        id: String,
    },
    /// 部门未找到错误
    DepartmentNotFound {
        /// 部门 ID
        id: String,
    },
    /// 角色未找到错误
    RoleNotFound {
        /// 角色 ID
        id: String,
    },
    /// 组织已存在错误
    OrganizationAlreadyExists {
        /// 组织 ID
        id: String,
    },
    /// 部门已存在错误
    DepartmentAlreadyExists {
        /// 部门 ID
        id: String,
    },
    /// 角色已存在错误
    RoleAlreadyExists {
        /// 角色 ID
        id: String,
    },

    /// 通知未找到错误
    NotificationNotFound {
        /// 通知 ID
        id: String,
    },
    /// 发送失败错误
    SendFailed {
        /// 错误消息
        msg: String,
    },

    /// 技能未找到错误
    SkillNotFound {
        /// 技能 ID
        id: String,
    },
    /// 技能版本未找到错误
    SkillVersionNotFound {
        /// 技能版本 ID
        id: String,
    },
    /// 技能清单未找到错误
    SkillManifestNotFound {
        /// 技能清单 ID
        id: String,
    },
    /// 技能已存在错误
    SkillAlreadyExists {
        /// 技能 ID
        id: String,
    },
    /// 技能版本已存在错误
    SkillVersionAlreadyExists {
        /// 技能版本 ID
        id: String,
    },
    /// 技能未启用错误
    SkillNotEnabled,
    /// 技能未加载错误
    SkillNotLoaded,
    /// 执行超时错误
    ExecutionTimeout,
    /// 执行失败错误
    ExecutionFailed {
        /// 错误消息
        msg: String,
    },
    /// 执行已取消错误
    ExecutionCancelled,
}

impl SkyNetErrorKind {
    /// 获取 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            SkyNetErrorKind::Validation { .. } => "error.validation",
            SkyNetErrorKind::NotFound { .. } => "error.not_found",
            SkyNetErrorKind::Unauthorized => "error.unauthorized",
            SkyNetErrorKind::Internal { .. } => "error.internal",

            SkyNetErrorKind::FileNotFound { .. } => "error.file_not_found",
            SkyNetErrorKind::FileAlreadyExists { .. } => "error.file_already_exists",
            SkyNetErrorKind::FileTooLarge { .. } => "error.file_too_large",
            SkyNetErrorKind::UnsupportedFileType { .. } => "error.unsupported_file_type",
            SkyNetErrorKind::NoPermission => "error.no_permission",
            SkyNetErrorKind::StorageError { .. } => "error.storage_error",

            SkyNetErrorKind::MemoryNotFound { .. } => "error.memory_not_found",
            SkyNetErrorKind::TagNotFound { .. } => "error.tag_not_found",
            SkyNetErrorKind::RelationNotFound { .. } => "error.relation_not_found",
            SkyNetErrorKind::RelationAlreadyExists { .. } => "error.relation_already_exists",

            SkyNetErrorKind::EntityNotFound { .. } => "error.entity_not_found",
            SkyNetErrorKind::EntityAlreadyExists { .. } => "error.entity_already_exists",
            SkyNetErrorKind::DatabaseError { .. } => "error.database_error",
            SkyNetErrorKind::ConnectionError { .. } => "error.connection_error",
            SkyNetErrorKind::SerializationError { .. } => "error.serialization_error",
            SkyNetErrorKind::ConstraintViolation { .. } => "error.constraint_violation",

            SkyNetErrorKind::JsonSerialization { .. } => "error.json_serialization",
            SkyNetErrorKind::JsonDeserialization { .. } => "error.json_deserialization",

            SkyNetErrorKind::Io { .. } => "error.io",
            SkyNetErrorKind::Config { .. } => "error.config",
            SkyNetErrorKind::UuidParse { .. } => "error.uuid_parse",

            SkyNetErrorKind::AgentNotFound { .. } => "error.agent_not_found",
            SkyNetErrorKind::AgentAlreadyExists { .. } => "error.agent_already_exists",
            SkyNetErrorKind::AgentOffline => "error.agent_offline",
            SkyNetErrorKind::AgentBusy => "error.agent_busy",
            SkyNetErrorKind::TaskNotFound { .. } => "error.task_not_found",
            SkyNetErrorKind::TaskAlreadyCompleted => "error.task_already_completed",
            SkyNetErrorKind::TaskAlreadyCanceled => "error.task_already_canceled",
            SkyNetErrorKind::CapabilityNotSupported => "error.capability_not_supported",

            SkyNetErrorKind::ConversationNotFound { .. } => "error.conversation_not_found",
            SkyNetErrorKind::MessageNotFound { .. } => "error.message_not_found",
            SkyNetErrorKind::ParticipantNotFound { .. } => "error.participant_not_found",
            SkyNetErrorKind::ParticipantAlreadyExists { .. } => "error.participant_already_exists",

            SkyNetErrorKind::InvalidCredentials => "error.invalid_credentials",
            SkyNetErrorKind::InvalidToken => "error.invalid_token",
            SkyNetErrorKind::TokenExpired => "error.token_expired",
            SkyNetErrorKind::UserAlreadyExists { .. } => "error.user_already_exists",
            SkyNetErrorKind::UserNotFound { .. } => "error.user_not_found",

            SkyNetErrorKind::OrganizationNotFound { .. } => "error.organization_not_found",
            SkyNetErrorKind::DepartmentNotFound { .. } => "error.department_not_found",
            SkyNetErrorKind::RoleNotFound { .. } => "error.role_not_found",
            SkyNetErrorKind::OrganizationAlreadyExists { .. } => {
                "error.organization_already_exists"
            }
            SkyNetErrorKind::DepartmentAlreadyExists { .. } => "error.department_already_exists",
            SkyNetErrorKind::RoleAlreadyExists { .. } => "error.role_already_exists",

            SkyNetErrorKind::NotificationNotFound { .. } => "error.notification_not_found",
            SkyNetErrorKind::SendFailed { .. } => "error.send_failed",

            SkyNetErrorKind::SkillNotFound { .. } => "error.skill_not_found",
            SkyNetErrorKind::SkillVersionNotFound { .. } => "error.skill_version_not_found",
            SkyNetErrorKind::SkillManifestNotFound { .. } => "error.skill_manifest_not_found",
            SkyNetErrorKind::SkillAlreadyExists { .. } => "error.skill_already_exists",
            SkyNetErrorKind::SkillVersionAlreadyExists { .. } => {
                "error.skill_version_already_exists"
            }
            SkyNetErrorKind::SkillNotEnabled => "error.skill_not_enabled",
            SkyNetErrorKind::SkillNotLoaded => "error.skill_not_loaded",
            SkyNetErrorKind::ExecutionTimeout => "error.execution_timeout",
            SkyNetErrorKind::ExecutionFailed { .. } => "error.execution_failed",
            SkyNetErrorKind::ExecutionCancelled => "error.execution_cancelled",
        }
    }
}

impl SkyNetError {
    /// 创建一个新的 SkyNetError
    pub fn new(kind: SkyNetErrorKind) -> Self {
        Self {
            kind: Box::new(kind),
        }
    }
}

impl fmt::Display for SkyNetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.kind {
            SkyNetErrorKind::Validation { field } => write!(f, "Validation error: {}", field),
            SkyNetErrorKind::NotFound { id } => write!(f, "Not found: {}", id),
            SkyNetErrorKind::Unauthorized => write!(f, "Unauthorized"),
            SkyNetErrorKind::Internal { msg } => write!(f, "Internal error: {}", msg),

            SkyNetErrorKind::FileNotFound { id } => write!(f, "File not found: {}", id),
            SkyNetErrorKind::FileAlreadyExists { name } => {
                write!(f, "File already exists: {}", name)
            }
            SkyNetErrorKind::FileTooLarge { size } => write!(f, "File too large: {} bytes", size),
            SkyNetErrorKind::UnsupportedFileType { ext } => {
                write!(f, "Unsupported file type: {}", ext)
            }
            SkyNetErrorKind::NoPermission => write!(f, "No permission"),
            SkyNetErrorKind::StorageError { msg } => write!(f, "Storage error: {}", msg),

            SkyNetErrorKind::MemoryNotFound { id } => write!(f, "Memory not found: {}", id),
            SkyNetErrorKind::TagNotFound { id } => write!(f, "Tag not found: {}", id),
            SkyNetErrorKind::RelationNotFound { from, to } => {
                write!(f, "Relation not found: {} -> {}", from, to)
            }
            SkyNetErrorKind::RelationAlreadyExists { from, to } => {
                write!(f, "Relation already exists: {} -> {}", from, to)
            }

            SkyNetErrorKind::EntityNotFound { id } => write!(f, "Entity not found: {}", id),
            SkyNetErrorKind::EntityAlreadyExists { id } => {
                write!(f, "Entity already exists: {}", id)
            }
            SkyNetErrorKind::DatabaseError { msg } => write!(f, "Database error: {}", msg),
            SkyNetErrorKind::ConnectionError { msg } => write!(f, "Connection error: {}", msg),
            SkyNetErrorKind::SerializationError { msg } => {
                write!(f, "Serialization error: {}", msg)
            }
            SkyNetErrorKind::ConstraintViolation { msg } => {
                write!(f, "Constraint violation: {}", msg)
            }

            SkyNetErrorKind::JsonSerialization { msg } => {
                write!(f, "JSON serialization error: {}", msg)
            }
            SkyNetErrorKind::JsonDeserialization { msg } => {
                write!(f, "JSON deserialization error: {}", msg)
            }

            SkyNetErrorKind::Io { msg } => write!(f, "IO error: {}", msg),
            SkyNetErrorKind::Config { msg } => write!(f, "Config error: {}", msg),
            SkyNetErrorKind::UuidParse { msg } => write!(f, "UUID parse error: {}", msg),

            SkyNetErrorKind::AgentNotFound { id } => write!(f, "Agent not found: {}", id),
            SkyNetErrorKind::AgentAlreadyExists { id } => write!(f, "Agent already exists: {}", id),
            SkyNetErrorKind::AgentOffline => write!(f, "Agent is offline"),
            SkyNetErrorKind::AgentBusy => write!(f, "Agent is busy"),
            SkyNetErrorKind::TaskNotFound { id } => write!(f, "Task not found: {}", id),
            SkyNetErrorKind::TaskAlreadyCompleted => write!(f, "Task already completed"),
            SkyNetErrorKind::TaskAlreadyCanceled => write!(f, "Task already canceled"),
            SkyNetErrorKind::CapabilityNotSupported => write!(f, "Capability not supported"),

            SkyNetErrorKind::ConversationNotFound { id } => {
                write!(f, "Conversation not found: {}", id)
            }
            SkyNetErrorKind::MessageNotFound { id } => write!(f, "Message not found: {}", id),
            SkyNetErrorKind::ParticipantNotFound { id } => {
                write!(f, "Participant not found: {}", id)
            }
            SkyNetErrorKind::ParticipantAlreadyExists { id } => {
                write!(f, "Participant already exists: {}", id)
            }

            SkyNetErrorKind::InvalidCredentials => write!(f, "Invalid credentials"),
            SkyNetErrorKind::InvalidToken => write!(f, "Invalid token"),
            SkyNetErrorKind::TokenExpired => write!(f, "Token expired"),
            SkyNetErrorKind::UserAlreadyExists { id } => write!(f, "User already exists: {}", id),
            SkyNetErrorKind::UserNotFound { id } => write!(f, "User not found: {}", id),

            SkyNetErrorKind::OrganizationNotFound { id } => {
                write!(f, "Organization not found: {}", id)
            }
            SkyNetErrorKind::DepartmentNotFound { id } => write!(f, "Department not found: {}", id),
            SkyNetErrorKind::RoleNotFound { id } => write!(f, "Role not found: {}", id),
            SkyNetErrorKind::OrganizationAlreadyExists { id } => {
                write!(f, "Organization already exists: {}", id)
            }
            SkyNetErrorKind::DepartmentAlreadyExists { id } => {
                write!(f, "Department already exists: {}", id)
            }
            SkyNetErrorKind::RoleAlreadyExists { id } => write!(f, "Role already exists: {}", id),

            SkyNetErrorKind::NotificationNotFound { id } => {
                write!(f, "Notification not found: {}", id)
            }
            SkyNetErrorKind::SendFailed { msg } => {
                write!(f, "Failed to send notification: {}", msg)
            }

            SkyNetErrorKind::SkillNotFound { id } => write!(f, "Skill not found: {}", id),
            SkyNetErrorKind::SkillVersionNotFound { id } => {
                write!(f, "Skill version not found: {}", id)
            }
            SkyNetErrorKind::SkillManifestNotFound { id } => {
                write!(f, "Skill manifest not found: {}", id)
            }
            SkyNetErrorKind::SkillAlreadyExists { id } => write!(f, "Skill already exists: {}", id),
            SkyNetErrorKind::SkillVersionAlreadyExists { id } => {
                write!(f, "Skill version already exists: {}", id)
            }
            SkyNetErrorKind::SkillNotEnabled => write!(f, "Skill not enabled"),
            SkyNetErrorKind::SkillNotLoaded => write!(f, "Skill not loaded"),
            SkyNetErrorKind::ExecutionTimeout => write!(f, "Execution timeout"),
            SkyNetErrorKind::ExecutionFailed { msg } => write!(f, "Execution failed: {}", msg),
            SkyNetErrorKind::ExecutionCancelled => write!(f, "Execution cancelled"),
        }
    }
}

impl std::error::Error for SkyNetError {}

/// SkyNet 统一结果类型
pub type SkyNetResult<T> = Result<T, SkyNetError>;
