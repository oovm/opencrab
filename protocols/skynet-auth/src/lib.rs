#![warn(missing_docs)]
//! Skynet 认证协议模块，定义身份验证相关的接口 trait 和类型。
//!
//! 该模块仅包含协议定义，不包含具体的业务逻辑实现。
use async_trait::async_trait;
use skynet_types::{AuthId, SkyNetResult, UserId};

/// 身份验证服务 trait，定义用户认证相关的核心接口
#[async_trait]
pub trait AuthService: Send + Sync {
    /// 用户登录
    ///
    /// # 参数
    /// - `auth_id`: 用户全局身份 ID
    /// - `credentials`: 认证凭据
    ///
    /// # 返回
    /// - 成功时返回用户在子网内的本地 ID 和访问令牌
    /// - 失败时返回错误
    async fn login(&self, auth_id: AuthId, credentials: &str) -> SkyNetResult<(UserId, String)>;

    /// 用户登出
    ///
    /// # 参数
    /// - `token`: 访问令牌
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn logout(&self, token: &str) -> SkyNetResult<()>;

    /// 验证访问令牌
    ///
    /// # 参数
    /// - `token`: 访问令牌
    ///
    /// # 返回
    /// - 成功时返回用户全局身份 ID
    /// - 失败时返回错误
    async fn verify_token(&self, token: &str) -> SkyNetResult<AuthId>;

    /// 刷新访问令牌
    ///
    /// # 参数
    /// - `refresh_token`: 刷新令牌
    ///
    /// # 返回
    /// - 成功时返回新的访问令牌
    /// - 失败时返回错误
    async fn refresh_token(&self, refresh_token: &str) -> SkyNetResult<String>;

    /// 注册新用户
    ///
    /// # 参数
    /// - `auth_id`: 用户全局身份 ID
    /// - `credentials`: 认证凭据
    ///
    /// # 返回
    /// - 成功时返回空
    /// - 失败时返回错误
    async fn register(&self, auth_id: AuthId, credentials: &str) -> SkyNetResult<()>;
}
