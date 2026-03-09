use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::{SkyNetError, SkyNetErrorKind, SkyNetResult};

/// 生成新的 UUID v4
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

/// 生成新的 UUID v4 字符串
pub fn generate_uuid_string() -> String {
    Uuid::new_v4().to_string()
}

/// 获取当前 UTC 时间
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// 获取当前 UTC 时间的 RFC3339 格式字符串
pub fn now_utc_string() -> String {
    Utc::now().to_rfc3339()
}

/// 获取当前时间戳（秒级）
pub fn timestamp_seconds() -> i64 {
    Utc::now().timestamp()
}

/// 获取当前时间戳（毫秒级）
pub fn timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// 从字符串解析 UUID
///
/// # Arguments
///
/// * `s` - UUID 字符串
pub fn parse_uuid(s: &str) -> SkyNetResult<Uuid> {
    Uuid::parse_str(s).map_err(|e| SkyNetError::new(SkyNetErrorKind::UuidParse { msg: e.to_string() }))
}
