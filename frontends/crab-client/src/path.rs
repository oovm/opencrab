//! 数据库路径管理模块
//!
//! 提供数据库文件路径的获取和管理功能。

use std::path::PathBuf;

/// 获取数据库文件路径
///
/// 返回标准应用数据目录下的 OpenCrab/opencrab.db 路径。
/// 如果无法获取应用数据目录，则返回 None。
///
/// # 示例
///
/// ```
/// use crab_client::get_database_path;
///
/// if let Some(path) = get_database_path() {
///     println!("数据库路径: {:?}", path);
/// }
/// ```
pub fn get_database_path() -> Option<PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push("OpenCrab");
        path.push("opencrab.db");
        path
    })
}
