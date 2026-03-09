//! 错误类型定义

use thiserror::Error;

/// 库的错误类型
#[derive(Debug, Error)]
pub enum Error {
    /// 数据库错误
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// 序列化/反序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// UUID 解析错误
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    /// 日期时间解析错误
    #[error("DateTime error: {0}")]
    DateTime(#[from] chrono::ParseError),

    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 通用错误
    #[error("{0}")]
    Other(String),
}
