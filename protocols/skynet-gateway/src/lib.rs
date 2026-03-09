#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use skynet_types::{Conversation, Memory, MemorySearchResult, MemoryType, Message, SkyNetResult};
use uuid::Uuid;

/// 对话上下文结构体，包含从对话中提取的关键信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// 对话唯一标识符
    pub conversation_id: Uuid,
    /// 对话主题（从对话中提取）
    pub topic: Option<String>,
    /// 参与者ID列表
    pub participants: Vec<Uuid>,
    /// 重要决策列表
    pub key_decisions: Vec<String>,
    /// 对话摘要
    pub summary: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 请求类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    /// HTTP请求
    Http,
    /// WebSocket请求
    WebSocket,
    /// JSON-RPC请求
    JsonRpc,
    /// gRPC请求
    Grpc,
}

/// 路由规则结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    /// 路由唯一标识符
    pub id: Uuid,
    /// 路由路径模式
    pub path_pattern: String,
    /// 目标服务名称
    pub target_service: String,
    /// 目标服务路径
    pub target_path: String,
    /// 请求方法列表
    pub methods: Vec<String>,
    /// 是否需要认证
    pub requires_auth: bool,
    /// 请求超时时间（毫秒）
    pub timeout_ms: u32,
    /// 路由规则是否启用
    pub enabled: bool,
    /// 路由优先级（数字越大优先级越高）
    pub priority: i32,
}

/// 网关服务 trait，定义请求路由和转发相关的核心接口
#[async_trait]
pub trait GatewayService: Send + Sync {
    /// 注册新的路由规则
    ///
    /// # 参数
    /// - `path_pattern`: 路径模式
    /// - `target_service`: 目标服务
    /// - `target_path`: 目标路径
    /// - `methods`: 允许的HTTP方法
    /// - `requires_auth`: 是否需要认证
    /// - `timeout_ms`: 超时时间（毫秒）
    /// - `priority`: 路由优先级
    ///
    /// # 返回
    /// - 成功时返回创建的路由规则
    /// - 失败时返回错误
    async fn register_route(
        &self,
        path_pattern: &str,
        target_service: &str,
        target_path: &str,
        methods: Vec<String>,
        requires_auth: bool,
        timeout_ms: u32,
        priority: i32,
    ) -> SkyNetResult<RouteRule>;

    /// 获取路由规则
    ///
    /// # 参数
    /// - `route_id`: 路由ID
    ///
    /// # 返回
    /// - 成功时返回路由规则
    /// - 失败时返回错误
    async fn get_route(&self, route_id: Uuid) -> SkyNetResult<RouteRule>;

    /// 获取所有路由规则
    ///
    /// # 返回
    /// - 成功时返回路由规则列表
    /// - 失败时返回错误
    async fn list_routes(&self) -> SkyNetResult<Vec<RouteRule>>;

    /// 更新路由规则
    ///
    /// # 参数
    /// - `route_id`: 路由ID
    /// - `path_pattern`: 新的路径模式（可选）
    /// - `target_service`: 新的目标服务（可选）
    /// - `target_path`: 新的目标路径（可选）
    /// - `enabled`: 是否启用（可选）
    /// - `priority`: 新的优先级（可选）
    ///
    /// # 返回
    /// - 成功时返回更新后的路由规则
    /// - 失败时返回错误
    async fn update_route(
        &self,
        route_id: Uuid,
        path_pattern: Option<&str>,
        target_service: Option<&str>,
        target_path: Option<&str>,
        enabled: Option<bool>,
        priority: Option<i32>,
    ) -> SkyNetResult<RouteRule>;

    /// 删除路由规则
    ///
    /// # 参数
    /// - `route_id`: 路由ID
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn delete_route(&self, route_id: Uuid) -> SkyNetResult<()>;

    /// 路由请求
    ///
    /// # 参数
    /// - `path`: 请求路径
    /// - `method`: 请求方法
    /// - `headers`: 请求头
    /// - `body`: 请求体
    /// - `request_type`: 请求类型
    ///
    /// # 返回
    /// - 成功时返回响应
    /// - 失败时返回错误
    async fn route_request(
        &self,
        path: &str,
        method: &str,
        headers: Vec<(String, String)>,
        body: Option<Value>,
        request_type: RequestType,
    ) -> SkyNetResult<GatewayResponse>;

    /// 健康检查
    ///
    /// # 返回
    /// - 成功时返回健康状态
    /// - 失败时返回错误
    async fn health_check(&self) -> SkyNetResult<GatewayHealth>;

    /// 获取服务状态
    ///
    /// # 参数
    /// - `service_name`: 服务名称
    ///
    /// # 返回
    /// - 成功时返回服务状态
    /// - 失败时返回错误
    async fn get_service_status(&self, service_name: &str) -> SkyNetResult<ServiceStatus>;

    /// 获取所有服务状态
    ///
    /// # 返回
    /// - 成功时返回服务状态列表
    /// - 失败时返回错误
    async fn list_service_statuses(&self) -> SkyNetResult<Vec<ServiceStatus>>;

    /// 限流配置
    ///
    /// # 参数
    /// - `service_name`: 服务名称
    /// - `max_requests_per_second`: 每秒最大请求数
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn configure_rate_limit(&self, service_name: &str, max_requests_per_second: u32) -> SkyNetResult<()>;
}

/// 网关响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResponse {
    /// 响应状态码
    pub status_code: u16,
    /// 响应头
    pub headers: Vec<(String, String)>,
    /// 响应体
    pub body: Option<Value>,
    /// 处理耗时（毫秒）
    pub duration_ms: u64,
}

/// 服务状态结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// 服务名称
    pub name: String,
    /// 服务是否健康
    pub healthy: bool,
    /// 最后一次健康检查时间
    pub last_check: Option<String>,
    /// 活跃连接数
    pub active_connections: u32,
    /// 请求总数
    pub total_requests: u64,
    /// 错误请求数
    pub error_requests: u64,
}

/// 网关健康状态结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayHealth {
    /// 网关是否健康
    pub healthy: bool,
    /// 活跃路由数
    pub active_routes: u32,
    /// 健康服务数
    pub healthy_services: u32,
    /// 总服务数
    pub total_services: u32,
    /// 版本信息
    pub version: String,
}

/// 创建记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemoryRequest {
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 所有者ID
    pub owner_id: Uuid,
    /// 所有者类型
    pub owner_type: String,
    /// 记忆内容
    pub content: String,
    /// 记忆摘要（可选）
    pub summary: Option<String>,
    /// 记忆元数据
    pub metadata: Value,
}

/// 更新记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemoryRequest {
    /// 新的记忆内容（可选）
    pub content: Option<String>,
    /// 新的记忆摘要（可选）
    pub summary: Option<String>,
    /// 新的记忆元数据（可选）
    pub metadata: Option<Value>,
}

/// 添加标签请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagRequest {
    /// 标签名称
    pub tag_name: String,
    /// 标签描述（可选）
    pub tag_description: Option<String>,
}

/// 添加记忆关联请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRelationRequest {
    /// 目标记忆ID
    pub target_memory_id: Uuid,
    /// 关联类型
    pub relation_type: String,
    /// 关联权重
    pub weight: f64,
}

/// 搜索记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoryRequest {
    /// 所有者ID
    pub owner_id: Uuid,
    /// 搜索关键词（可选）
    pub keyword: Option<String>,
    /// 标签ID列表（可选）
    pub tag_ids: Option<Vec<Uuid>>,
    /// 时间范围起始（可选）
    pub start_time: Option<DateTime<Utc>>,
    /// 时间范围结束（可选）
    pub end_time: Option<DateTime<Utc>>,
    /// 返回数量限制
    pub limit: u32,
    /// 偏移量
    pub offset: u32,
}

/// 导出记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMemoriesRequest {
    /// 要导出的记忆ID列表
    pub memory_ids: Vec<Uuid>,
}

/// 导入记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportMemoriesRequest {
    /// 包含记忆数据的 JSON 字符串
    pub json_str: String,
}

/// 从对话提取上下文请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractContextRequest {
    /// 对话对象
    pub conversation: Conversation,
    /// 对话消息列表
    pub messages: Vec<Message>,
}

/// 从对话创建记忆请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemoryFromConversationRequest {
    /// 对话上下文
    pub context: ConversationContext,
    /// 所有者ID
    pub owner_id: Uuid,
    /// 所有者类型
    pub owner_type: String,
}

/// 记忆响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResponse {
    /// 响应的记忆对象
    pub memory: Memory,
}

/// 记忆列表响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryListResponse {
    /// 记忆列表
    pub memories: Vec<Memory>,
}

/// 记忆搜索结果响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResponse {
    /// 记忆搜索结果列表
    pub results: Vec<MemorySearchResult>,
}

/// 导出记忆响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMemoriesResponse {
    /// JSON 格式的字符串
    pub json_str: String,
}

/// 导入记忆响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportMemoriesResponse {
    /// 导入的记忆列表
    pub memories: Vec<Memory>,
}

/// 提取上下文响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractContextResponse {
    /// 提取的对话上下文
    pub context: ConversationContext,
}
