//! 错误类型定义
//!
//! 直接使用 WAE 框架提供的统一错误类型。

pub use wae_types::{
    WaeError, WaeErrorKind, WaeResult,
};

/// Crab 错误类型别名（保持向后兼容性）
pub type CrabError = WaeError;

/// Crab 错误类型枚举别名（保持向后兼容性）
pub type CrabErrorKind = WaeErrorKind;

/// Crab 结果类型别名（保持向后兼容性）
pub type CrabResult<T> = WaeResult<T>;

/// 错误类型别名（保持向后兼容性）
pub type Error = WaeError;

/// 结果类型别名（保持向后兼容性）
pub type Result<T> = WaeResult<T>;
