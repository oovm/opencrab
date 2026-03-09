#![warn(missing_docs)]

//! Crab Types - OpenCrab 核心类型定义模块
//!
//! 提供 OpenCrab 系统核心业务数据结构和类型定义。

pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;
pub use wae_types::{
    WaeError, WaeErrorKind, WaeResult,
};

mod error;
mod types;

pub use error::*;
pub use types::*;
