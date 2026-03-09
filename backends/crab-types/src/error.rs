//! 错误类型定义
//!
//! 基于 WAE 框架提供的统一错误类型，扩展 Crab 特定功能。

pub use wae_types::{ErrorCategory, WaeError, WaeErrorKind, WaeResult};

/// Crab 错误类型
///
/// 基于 WAE 框架的 WaeError，保持向后兼容性。
pub type CrabError = WaeError;

/// Crab 错误类型枚举
///
/// 基于 WAE 框架的 WaeErrorKind，保持向后兼容性。
pub type CrabErrorKind = WaeErrorKind;

/// Crab 结果类型
///
/// 基于 WAE 框架的 WaeResult，保持向后兼容性。
pub type CrabResult<T> = WaeResult<T>;

/// 错误类型别名（保持向后兼容性）
pub type Error = WaeError;

/// 结果类型别名（保持向后兼容性）
pub type Result<T> = WaeResult<T>;
