/**
 * 用户信息接口
 */
export interface User {
    id: string;
    name: string;
    role: "admin" | "user";
    avatar: string;
}

/**
 * 维度信息接口
 */
export interface DIM {
    id: string;
    name: string;
    key: string;
    values: string[];
    isMandatory: boolean;
}

/**
 * 认证响应接口
 */
export interface AuthResponse {
    success: boolean;
    user?: User;
    token?: string;
    message?: string;
}

/**
 * 登录凭据接口
 */
export interface LoginCredentials {
    username: string;
    password: string;
}

/**
 * 客户信息接口
 */
export interface Client {
    id: string;
    name: string;
    description: string;
}

/**
 * 团队信息接口（用于组织结构）
 */
export interface Team {
    id: string;
    name: string;
    clientId: string;
    description: string;
}

/**
 * 明星团队接口
 */
export interface StarTeam {
    /**
     * 团队唯一标识
     */
    id: string;
    /**
     * 团队名称
     */
    name: string;
    /**
     * 团队标语
     */
    tagline?: string;
    /**
     * 团队描述
     */
    description?: string;
    /**
     * 团队头像/Logo
     */
    logo?: string;
    /**
     * 团队成员 ID 列表
     */
    memberIds: string[];
    /**
     * 团队专长领域
     */
    expertise: string[];
    /**
     * 团队成立时间
     */
    founded?: string;
    /**
     * 团队项目 ID 列表
     */
    projectIds: string[];
    /**
     * 创建者用户 ID
     */
    creatorId: string;
    /**
     * 创建时间
     */
    createdAt: Date;
    /**
     * 更新时间
     */
    updatedAt: Date;
}

/**
 * 明星项目接口
 */
export interface StarProject {
    /**
     * 项目唯一标识
     */
    id: string;
    /**
     * 项目名称
     */
    name: string;
    /**
     * 项目标语
     */
    tagline?: string;
    /**
     * 项目描述
     */
    description?: string;
    /**
     * 项目封面图片
     */
    cover?: string;
    /**
     * 项目状态
     */
    status: "planning" | "in_progress" | "completed" | "on_hold";
    /**
     * 项目开始时间
     */
    startDate?: string;
    /**
     * 项目预计完成时间
     */
    endDate?: string;
    /**
     * 项目团队 ID
     */
    teamId?: string;
    /**
     * 项目成员 ID 列表
     */
    memberIds: string[];
    /**
     * 项目标签
     */
    tags: string[];
    /**
     * 项目里程碑
     */
    milestones: string[];
    /**
     * 创建者用户 ID
     */
    creatorId: string;
    /**
     * 创建时间
     */
    createdAt: Date;
    /**
     * 更新时间
     */
    updatedAt: Date;
}

/**
 * 组织信息接口
 */
export interface Organization {
    id: string;
    name: string;
    teamId: string;
    description: string;
}

/**
 * 组织结构接口
 */
export interface OrgStructure {
    client: Client;
    team: Team;
    organization: Organization;
}

/**
 * 角色类型枚举（员工或顾问）
 */
export type RoleType = "employee" | "consultant";

/**
 * 角色接口（员工/顾问的统一表示）
 * 员工在 ai-company 中称为员工，在 ai-empire 中称为顾问，本质上都是角色
 */
export interface Role {
    /**
     * 角色唯一标识
     */
    id: string;
    /**
     * 角色标题/名称
     */
    title: string;
    /**
     * 角色类型（员工或顾问）
     */
    type: RoleType;
    /**
     * 角色头像
     */
    avatar?: string;
    /**
     * 角色描述
     */
    description?: string;
    /**
     * 技能集合
     */
    skills: Skill[];
    /**
     * Token 上限（薪酬）
     */
    tokenLimit: number;
    /**
     * 创建者用户 ID
     */
    creatorId: string;
    /**
     * 创建时间
     */
    createdAt: Date;
    /**
     * 更新时间
     */
    updatedAt: Date;
}

/**
 * 技能接口
 */
export interface Skill {
    /**
     * 技能唯一标识
     */
    id: string;
    /**
     * 技能名称
     */
    name: string;
    /**
     * 技能描述
     */
    description?: string;
    /**
     * 技能分类
     */
    category?: string;
}

/**
 * 评分接口
 */
export interface Rating {
    /**
     * 评分唯一标识
     */
    id: string;
    /**
     * 评分用户 ID
     */
    userId: string;
    /**
     * 被评分角色 ID
     */
    roleId: string;
    /**
     * 评分分数（1-5）
     */
    score: number;
    /**
     * 创建时间
     */
    createdAt: Date;
}

/**
 * 评论接口
 */
export interface Review {
    /**
     * 评论唯一标识
     */
    id: string;
    /**
     * 评论用户 ID
     */
    userId: string;
    /**
     * 被评论角色 ID
     */
    roleId: string;
    /**
     * 评论内容
     */
    content: string;
    /**
     * 关联的评分
     */
    rating?: Rating;
    /**
     * 创建时间
     */
    createdAt: Date;
}

/**
 * 标签接口
 */
export interface Tag {
    /**
     * 标签唯一标识
     */
    id: string;
    /**
     * 标签名称
     */
    name: string;
    /**
     * 标签颜色
     */
    color?: string;
}

/**
 * 用户标签映射接口（每个用户最多为每个角色贴 5 个标签）
 */
export interface UserTag {
    /**
     * 用户 ID
     */
    userId: string;
    /**
     * 角色 ID
     */
    roleId: string;
    /**
     * 用户贴的标签列表（最多 5 个）
     */
    tags: Tag[];
}

/**
 * 组织架构节点接口
 */
export interface OrgNode {
    /**
     * 节点唯一标识
     */
    id: string;
    /**
     * 部门/团队名称
     */
    name: string;
    /**
     * 部门/团队描述
     */
    description?: string;
    /**
     * 负责人 ID
     */
    leadId?: string;
    /**
     * 成员 ID 列表
     */
    memberIds: string[];
    /**
     * 子节点
     */
    children: OrgNode[];
}

/**
 * 用户评价接口
 */
export interface CompanyReview {
    /**
     * 评价唯一标识
     */
    id: string;
    /**
     * 评价用户 ID
     */
    userId: string;
    /**
     * 评价用户名称
     */
    userName: string;
    /**
     * 评价用户头像
     */
    userAvatar?: string;
    /**
     * 评分（1-5）
     */
    rating: number;
    /**
     * 评价内容
     */
    content: string;
    /**
     * 评价标签
     */
    tags: string[];
    /**
     * 创建时间
     */
    createdAt: Date;
}

/**
 * 公司接口
 */
export interface Company {
    /**
     * 公司唯一标识
     */
    id: string;
    /**
     * 公司名称
     */
    name: string;
    /**
     * 公司标语
     */
    tagline?: string;
    /**
     * 公司描述
     */
    description?: string;
    /**
     * 公司头像/Logo
     */
    logo?: string;
    /**
     * 成立时间
     */
    founded?: string;
    /**
     * 所属行业
     */
    industry?: string;
    /**
     * 公司使命
     */
    mission?: string;
    /**
     * 公司愿景
     */
    vision?: string;
    /**
     * 核心价值观
     */
    values: string[];
    /**
     * 经营理念
     */
    philosophy?: string;
    /**
     * 企业文化
     */
    culture?: string;
    /**
     * 组织架构
     */
    orgStructure?: OrgNode;
    /**
     * 用户评价
     */
    reviews: CompanyReview[];
    /**
     * 创建者用户 ID
     */
    creatorId: string;
    /**
     * 创建时间
     */
    createdAt: Date;
    /**
     * 更新时间
     */
    updatedAt: Date;
}

/**
 * 论坛帖子接口
 */
export interface ForumPost {
    /**
     * 帖子唯一标识
     */
    id: string;
    /**
     * 帖子标题
     */
    title: string;
    /**
     * 帖子内容
     */
    content: string;
    /**
     * 发帖用户 ID
     */
    userId: string;
    /**
     * 发帖用户名称
     */
    userName: string;
    /**
     * 发帖用户头像
     */
    userAvatar?: string;
    /**
     * 帖子标签
     */
    tags: string[];
    /**
     * 浏览次数
     */
    views: number;
    /**
     * 点赞数
     */
    likes: number;
    /**
     * 评论数
     */
    commentCount: number;
    /**
     * 是否置顶
     */
    isPinned: boolean;
    /**
     * 是否精华
     */
    isFeatured: boolean;
    /**
     * 创建时间
     */
    createdAt: Date;
    /**
     * 更新时间
     */
    updatedAt: Date;
}

/**
 * 论坛评论接口
 */
export interface ForumComment {
    /**
     * 评论唯一标识
     */
    id: string;
    /**
     * 所属帖子 ID
     */
    postId: string;
    /**
     * 评论内容
     */
    content: string;
    /**
     * 评论用户 ID
     */
    userId: string;
    /**
     * 评论用户名称
     */
    userName: string;
    /**
     * 评论用户头像
     */
    userAvatar?: string;
    /**
     * 回复的评论 ID（可选，用于楼中楼）
     */
    replyToId?: string;
    /**
     * 点赞数
     */
    likes: number;
    /**
     * 创建时间
     */
    createdAt: Date;
}
