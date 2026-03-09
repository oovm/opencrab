#![warn(missing_docs)]

//! Crab Client - OpenCrab 客户端共享库
//!
//! 提供客户端共享的功能和数据结构，包括数据库操作、类型定义等。

pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

mod database;
mod error;
mod path;
mod types;

pub use database::*;
pub use error::*;
pub use path::*;
pub use types::*;

/// 通用结果类型别名
pub type Result<T> = std::result::Result<T, Error>;
