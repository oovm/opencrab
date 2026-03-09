import type { DIM, User, Role, Skill, Rating, Review, UserTag, Company, StarTeam, StarProject } from "../../types";

const KEYS = {
    DIMS: "pm_dims",
    USERS: "pm_users",
    CURRENT_USER: "pm_current_user",
    ROLES: "pm_roles",
    SKILLS: "pm_skills",
    RATINGS: "pm_ratings",
    REVIEWS: "pm_reviews",
    USER_TAGS: "pm_user_tags",
    COMPANIES: "pm_companies",
    STAR_TEAMS: "pm_star_teams",
    STAR_PROJECTS: "pm_star_projects",
    FORUM_POSTS: "pm_forum_posts",
    FORUM_COMMENTS: "pm_forum_comments",
};

const isBrowser = typeof window !== "undefined";

export const Storage = {
    getDIMS(): DIM[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.DIMS);
        let dims: DIM[] = [];

        if (!data) {
            dims = [
                {
                    id: "dim-1",
                    name: "Cost Center",
                    key: "cost_center",
                    values: ["Project A", "Project B", "Innovation Lab"],
                    isMandatory: true,
                },
                {
                    id: "dim-2",
                    name: "Version",
                    key: "version",
                    values: ["v1.0", "v2.0", "Backlog"],
                    isMandatory: false,
                },
                {
                    id: "dim-3",
                    name: "Team",
                    key: "team",
                    values: ["Frontend", "Backend", "Design", "Product"],
                    isMandatory: true,
                },
            ];
            localStorage.setItem(KEYS.DIMS, JSON.stringify(dims));
            return dims;
        }

        dims = JSON.parse(data);
        if (!dims.find((d: DIM) => d.key === "team")) {
            dims.push({
                id: "dim-3",
                name: "Team",
                key: "team",
                values: ["Frontend", "Backend", "Design", "Product"],
                isMandatory: true,
            });
            localStorage.setItem(KEYS.DIMS, JSON.stringify(dims));
        }
        return dims;
    },

    setDIMS(dims: DIM[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.DIMS, JSON.stringify(dims));
    },

    getUsers(): User[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.USERS);
        return data ? JSON.parse(data) : [];
    },

    setUsers(users: User[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.USERS, JSON.stringify(users));
    },

    getCurrentUser(): User | null {
        if (!isBrowser) return null;
        const data = localStorage.getItem(KEYS.CURRENT_USER);
        return data ? JSON.parse(data) : null;
    },

    setCurrentUser(user: User | null) {
        if (!isBrowser) return;
        if (user) {
            localStorage.setItem(KEYS.CURRENT_USER, JSON.stringify(user));
        } else {
            localStorage.removeItem(KEYS.CURRENT_USER);
        }
    },

    getRoles(): Role[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.ROLES);
        let roles: Role[] = [];

        if (!data) {
            const now = new Date();
            roles = [
                {
                    id: "role-1",
                    title: "Vue 前端开发工程师",
                    type: "employee",
                    avatar: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20vue%20developer%20avatar&image_size=square",
                    description: "专注于 Vue 3 和 TypeScript 的前端开发专家",
                    skills: [
                        { id: "skill-1", name: "Vue 3", description: "Vue 3 Composition API", category: "前端框架" },
                        {
                            id: "skill-2",
                            name: "TypeScript",
                            description: "类型安全的 JavaScript",
                            category: "编程语言",
                        },
                        { id: "skill-3", name: "Element Plus", description: "Vue 3 UI 组件库", category: "UI 组件" },
                    ],
                    tokenLimit: 100000,
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
                {
                    id: "role-2",
                    title: "AI 咨询顾问",
                    type: "consultant",
                    avatar: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20AI%20consultant%20avatar&image_size=square",
                    description: "人工智能战略和实施顾问",
                    skills: [
                        { id: "skill-4", name: "机器学习", description: "ML 算法和模型", category: "AI" },
                        { id: "skill-5", name: "Prompt Engineering", description: "优化 LLM 提示词", category: "AI" },
                    ],
                    tokenLimit: 500000,
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
            ];
            localStorage.setItem(KEYS.ROLES, JSON.stringify(roles));
            return roles;
        }

        return JSON.parse(data);
    },

    setRoles(roles: Role[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.ROLES, JSON.stringify(roles));
    },

    getSkills(): Skill[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.SKILLS);
        let skills: Skill[] = [];

        if (!data) {
            skills = [
                { id: "skill-1", name: "Vue 3", description: "Vue 3 Composition API", category: "前端框架" },
                { id: "skill-2", name: "TypeScript", description: "类型安全的 JavaScript", category: "编程语言" },
                { id: "skill-3", name: "Element Plus", description: "Vue 3 UI 组件库", category: "UI 组件" },
                { id: "skill-4", name: "机器学习", description: "ML 算法和模型", category: "AI" },
                { id: "skill-5", name: "Prompt Engineering", description: "优化 LLM 提示词", category: "AI" },
                { id: "skill-6", name: "React", description: "React 前端框架", category: "前端框架" },
                { id: "skill-7", name: "Node.js", description: "后端 JavaScript 运行时", category: "后端" },
            ];
            localStorage.setItem(KEYS.SKILLS, JSON.stringify(skills));
            return skills;
        }

        return JSON.parse(data);
    },

    setSkills(skills: Skill[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.SKILLS, JSON.stringify(skills));
    },

    getRatings(): Rating[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.RATINGS);
        return data ? JSON.parse(data) : [];
    },

    setRatings(ratings: Rating[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.RATINGS, JSON.stringify(ratings));
    },

    getReviews(): Review[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.REVIEWS);
        return data ? JSON.parse(data) : [];
    },

    setReviews(reviews: Review[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.REVIEWS, JSON.stringify(reviews));
    },

    getUserTags(): UserTag[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.USER_TAGS);
        return data ? JSON.parse(data) : [];
    },

    setUserTags(userTags: UserTag[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.USER_TAGS, JSON.stringify(userTags));
    },

    getCompanies(): Company[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.COMPANIES);
        let companies: Company[] = [];

        if (!data) {
            const now = new Date();
            companies = [
                {
                    id: "company-1",
                    name: "未来科技工作室",
                    tagline: "用 AI 打造未来",
                    description: "专注于 AI 驱动的创新产品开发",
                    logo: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=modern%20tech%20company%20logo%20minimal%20design&image_size=square",
                    founded: "2025年",
                    industry: "科技/AI",
                    mission: "用 AI 技术赋能每一个创作者",
                    vision: "成为全球领先的一人公司标杆",
                    values: ["创新", "品质", "用户至上", "持续学习"],
                    philosophy: "以用户需求为核心，用技术创新解决实际问题",
                    culture: "开放、包容、追求卓越的工程师文化",
                    orgStructure: {
                        id: "org-1",
                        name: "未来科技工作室",
                        description: "公司整体架构",
                        memberIds: [],
                        children: [
                            {
                                id: "org-2",
                                name: "产品研发部",
                                description: "负责产品设计和技术研发",
                                leadId: "role-1",
                                memberIds: ["role-1"],
                                children: [
                                    {
                                        id: "org-3",
                                        name: "前端开发组",
                                        description: "负责前端应用开发",
                                        memberIds: ["role-1"],
                                        children: [],
                                    },
                                    {
                                        id: "org-4",
                                        name: "AI 研发组",
                                        description: "负责 AI 模型和算法研发",
                                        memberIds: ["role-2"],
                                        children: [],
                                    },
                                ],
                            },
                            {
                                id: "org-5",
                                name: "市场运营部",
                                description: "负责市场推广和用户运营",
                                memberIds: [],
                                children: [],
                            },
                        ],
                    },
                    reviews: [
                        {
                            id: "review-1",
                            userId: "user-1",
                            userName: "张明",
                            userAvatar:
                                "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20male&image_size=square",
                            rating: 5,
                            content: "非常棒的 AI 公司，产品创新能力强，用户体验优秀！",
                            tags: ["创新", "用户体验好", "技术强"],
                            createdAt: new Date(now.getTime() - 86400000 * 7),
                        },
                        {
                            id: "review-2",
                            userId: "user-2",
                            userName: "李华",
                            userAvatar:
                                "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female&image_size=square",
                            rating: 4,
                            content: "团队专业，服务响应快，产品质量高，期待更多创新产品。",
                            tags: ["专业", "服务好", "质量高"],
                            createdAt: new Date(now.getTime() - 86400000 * 3),
                        },
                    ],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
                {
                    id: "company-2",
                    name: "数字营销实验室",
                    tagline: "数据驱动增长",
                    description: "专注于数字营销和品牌建设",
                    logo: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=modern%20marketing%20company%20logo%20blue%20theme&image_size=square",
                    founded: "2024年",
                    industry: "营销/品牌",
                    mission: "帮助品牌在数字时代脱颖而出",
                    vision: "成为最受信赖的数字营销伙伴",
                    values: ["数据驱动", "创意无限", "结果导向"],
                    philosophy: "用数据说话，用创意打动人心",
                    culture: "创意驱动、结果导向的营销文化",
                    orgStructure: {
                        id: "org-6",
                        name: "数字营销实验室",
                        description: "公司整体架构",
                        memberIds: [],
                        children: [
                            {
                                id: "org-7",
                                name: "品牌策略部",
                                description: "负责品牌定位和策略规划",
                                memberIds: [],
                                children: [],
                            },
                            {
                                id: "org-8",
                                name: "数字投放部",
                                description: "负责广告投放和效果优化",
                                memberIds: [],
                                children: [],
                            },
                            {
                                id: "org-9",
                                name: "内容创意部",
                                description: "负责内容创作和创意设计",
                                memberIds: [],
                                children: [],
                            },
                        ],
                    },
                    reviews: [
                        {
                            id: "review-3",
                            userId: "user-3",
                            userName: "王芳",
                            userAvatar:
                                "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female%20asian&image_size=square",
                            rating: 5,
                            content: "营销方案非常专业，带来了显著的业务增长！",
                            tags: ["专业", "效果好", "增长快"],
                            createdAt: new Date(now.getTime() - 86400000 * 14),
                        },
                    ],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
            ];
            localStorage.setItem(KEYS.COMPANIES, JSON.stringify(companies));
            return companies;
        }

        return JSON.parse(data);
    },

    setCompanies(companies: Company[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.COMPANIES, JSON.stringify(companies));
    },

    getStarTeams(): StarTeam[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.STAR_TEAMS);
        let starTeams: StarTeam[] = [];

        if (!data) {
            const now = new Date();
            starTeams = [
                {
                    id: "star-team-1",
                    name: "AI 创新团队",
                    tagline: "用 AI 定义未来",
                    description: "专注于 AI 产品研发的精英团队",
                    logo: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20AI%20team%20logo%20modern%20design&image_size=square",
                    memberIds: ["role-1", "role-2"],
                    expertise: ["人工智能", "机器学习", "产品设计"],
                    founded: "2025年",
                    projectIds: ["star-project-1"],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
                {
                    id: "star-team-2",
                    name: "全栈开发团队",
                    tagline: "从前端到后端，我们都在行",
                    description: "精通全栈开发的综合性团队",
                    logo: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=modern%20fullstack%20development%20team%20logo&image_size=square",
                    memberIds: ["role-1"],
                    expertise: ["Vue 3", "TypeScript", "Node.js", "数据库"],
                    founded: "2024年",
                    projectIds: ["star-project-2"],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
            ];
            localStorage.setItem(KEYS.STAR_TEAMS, JSON.stringify(starTeams));
            return starTeams;
        }

        return JSON.parse(data);
    },

    setStarTeams(starTeams: StarTeam[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.STAR_TEAMS, JSON.stringify(starTeams));
    },

    getStarProjects(): StarProject[] {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.STAR_PROJECTS);
        let starProjects: StarProject[] = [];

        if (!data) {
            const now = new Date();
            starProjects = [
                {
                    id: "star-project-1",
                    name: "AI 智能助手项目",
                    tagline: "打造下一代智能交互体验",
                    description: "开发一个基于大语言模型的智能助手系统",
                    cover: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=AI%20assistant%20project%20cover%20modern%20tech%20design&image_size=square",
                    status: "in_progress",
                    startDate: "2025-01-01",
                    endDate: "2025-12-31",
                    teamId: "star-team-1",
                    memberIds: ["role-1", "role-2"],
                    tags: ["AI", "LLM", "智能助手"],
                    milestones: ["需求分析", "原型设计", "开发完成", "测试上线"],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
                {
                    id: "star-project-2",
                    name: "电商平台重构",
                    tagline: "全新的购物体验",
                    description: "对现有电商平台进行全面技术升级和用户体验优化",
                    cover: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=e-commerce%20platform%20project%20cover%20modern%20design&image_size=square",
                    status: "planning",
                    startDate: "2025-03-01",
                    endDate: "2025-09-30",
                    teamId: "star-team-2",
                    memberIds: ["role-1"],
                    tags: ["Vue 3", "电商", "重构"],
                    milestones: ["技术选型", "架构设计", "开发", "测试", "上线"],
                    creatorId: "user-1",
                    createdAt: now,
                    updatedAt: now,
                },
            ];
            localStorage.setItem(KEYS.STAR_PROJECTS, JSON.stringify(starProjects));
            return starProjects;
        }

        return JSON.parse(data);
    },

    setStarProjects(starProjects: StarProject[]) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.STAR_PROJECTS, JSON.stringify(starProjects));
    },

    getForumPosts() {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.FORUM_POSTS);
        let forumPosts = [];

        if (!data) {
            const now = new Date();
            forumPosts = [
                {
                    id: "post-1",
                    title: "未来科技工作室，用 AI 重新定义工作方式！",
                    content:
                        "作为一家专注于 AI 驱动创新的工作室，我们的使命是用 AI 技术赋能每一个创作者。我们相信，AI 不只是工具，更是创意的伙伴！",
                    userId: "user-1",
                    userName: "张明",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20male&image_size=square",
                    tags: ["AI", "创新", "未来科技"],
                    views: 1234,
                    likes: 89,
                    commentCount: 23,
                    isPinned: true,
                    isFeatured: true,
                    createdAt: new Date(now.getTime() - 86400000 * 3),
                    updatedAt: new Date(now.getTime() - 86400000 * 3),
                },
                {
                    id: "post-2",
                    title: "数字营销实验室分享：如何用数据驱动业务增长",
                    content:
                        "数据驱动增长是我们的核心价值观。今天想和大家分享一下我们如何通过数据分析和创意策略，帮助品牌在数字时代脱颖而出。",
                    userId: "user-2",
                    userName: "李华",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female&image_size=square",
                    tags: ["数字营销", "数据驱动", "增长"],
                    views: 856,
                    likes: 56,
                    commentCount: 12,
                    isPinned: false,
                    isFeatured: true,
                    createdAt: new Date(now.getTime() - 86400000 * 2),
                    updatedAt: new Date(now.getTime() - 86400000 * 2),
                },
                {
                    id: "post-3",
                    title: "一人公司也能做大做强！我的创业经验分享",
                    content:
                        "很多人问我一人公司如何运营。其实关键在于找到自己的核心竞争力，然后用技术放大自己的能力。AI 是最好的伙伴！",
                    userId: "user-3",
                    userName: "王芳",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female%20asian&image_size=square",
                    tags: ["创业", "一人公司", "经验分享"],
                    views: 2341,
                    likes: 156,
                    commentCount: 45,
                    isPinned: false,
                    isFeatured: false,
                    createdAt: new Date(now.getTime() - 86400000 * 1),
                    updatedAt: new Date(now.getTime() - 86400000 * 1),
                },
            ];
            localStorage.setItem(KEYS.FORUM_POSTS, JSON.stringify(forumPosts));
            return forumPosts;
        }

        return JSON.parse(data);
    },

    setForumPosts(forumPosts) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.FORUM_POSTS, JSON.stringify(forumPosts));
    },

    getForumComments() {
        if (!isBrowser) return [];
        const data = localStorage.getItem(KEYS.FORUM_COMMENTS);
        let forumComments = [];

        if (!data) {
            const now = new Date();
            forumComments = [
                {
                    id: "comment-1",
                    postId: "post-1",
                    content: "说得太对了！AI 确实是创意的最佳伙伴。期待看到更多产品！",
                    userId: "user-2",
                    userName: "李华",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female&image_size=square",
                    likes: 12,
                    createdAt: new Date(now.getTime() - 86400000 * 2.5),
                },
                {
                    id: "comment-2",
                    postId: "post-1",
                    content: "加油！未来科技工作室的产品我一直在用，非常棒！",
                    userId: "user-3",
                    userName: "王芳",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20female%20asian&image_size=square",
                    likes: 8,
                    createdAt: new Date(now.getTime() - 86400000 * 2),
                },
                {
                    id: "comment-3",
                    postId: "post-2",
                    content: "数据驱动确实是关键！能不能分享一下具体用了哪些工具？",
                    userId: "user-1",
                    userName: "张明",
                    userAvatar:
                        "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=professional%20user%20avatar%20male&image_size=square",
                    likes: 5,
                    createdAt: new Date(now.getTime() - 86400000 * 1.5),
                },
            ];
            localStorage.setItem(KEYS.FORUM_COMMENTS, JSON.stringify(forumComments));
            return forumComments;
        }

        return JSON.parse(data);
    },

    setForumComments(forumComments) {
        if (!isBrowser) return;
        localStorage.setItem(KEYS.FORUM_COMMENTS, JSON.stringify(forumComments));
    },
};
