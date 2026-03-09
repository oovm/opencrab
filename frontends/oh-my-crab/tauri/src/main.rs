//! Oh My Crab - OpenCrab GUI Client
//! 
//! A Tauri-based GUI client for connecting to OpenCrab or OpenCrab servers.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
use uuid::Uuid;

use crab_client::*;

/// 应用状态管理
struct AppState {
    /// 数据库连接
    db: Mutex<Option<Database>>,
    /// 当前用户 ID
    current_user_id: Mutex<Option<Uuid>>,
}

impl AppState {
    /// 创建新的应用状态
    fn new() -> Self {
        AppState {
            db: Mutex::new(None),
            current_user_id: Mutex::new(None),
        }
    }
}

// ==================== 数据库初始化命令 ====================

/// 初始化数据库连接
#[tauri::command]
async fn init_database(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut db_lock = state.db.lock().unwrap();
    if db_lock.is_none() {
        let db = Database::new().map_err(|e| e.to_string())?;
        *db_lock = Some(db);
    }
    Ok(())
}

/// 获取当前用户 ID
#[tauri::command]
async fn get_current_user_id(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let user_id_lock = state.current_user_id.lock().unwrap();
    Ok(user_id_lock.map(|id| id.to_string()))
}

/// 设置当前用户 ID
#[tauri::command]
async fn set_current_user_id(state: tauri::State<'_, AppState>, user_id: Option<String>) -> Result<(), String> {
    let mut user_id_lock = state.current_user_id.lock().unwrap();
    *user_id_lock = user_id.and_then(|id| Uuid::parse_str(&id).ok());
    Ok(())
}

// ==================== 用户管理命令 ====================

/// 创建新用户
#[tauri::command]
async fn create_user(state: tauri::State<'_, AppState>, req: CreateUserRequest) -> Result<User, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    db.create_user(req).map_err(|e| e.to_string())
}

/// 获取所有用户
#[tauri::command]
async fn get_all_users(state: tauri::State<'_, AppState>) -> Result<Vec<User>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    db.get_all_users().map_err(|e| e.to_string())
}

/// 根据 ID 获取用户
#[tauri::command]
async fn get_user_by_id(state: tauri::State<'_, AppState>, user_id: String) -> Result<Option<User>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    db.get_user_by_id(uuid).map_err(|e| e.to_string())
}

/// 更新用户
#[tauri::command]
async fn update_user(state: tauri::State<'_, AppState>, user_id: String, req: UpdateUserRequest) -> Result<Option<User>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    db.update_user(uuid, req).map_err(|e| e.to_string())
}

/// 删除用户
#[tauri::command]
async fn delete_user(state: tauri::State<'_, AppState>, user_id: String) -> Result<bool, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    db.delete_user(uuid).map_err(|e| e.to_string())
}

// ==================== 会话管理命令 ====================

/// 创建新会话
#[tauri::command]
async fn create_conversation(state: tauri::State<'_, AppState>, req: CreateConversationRequest) -> Result<Conversation, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let user_id_lock = state.current_user_id.lock().unwrap();
    let user_id = user_id_lock.ok_or("未选择当前用户")?;
    db.create_conversation(user_id, req).map_err(|e| e.to_string())
}

/// 获取当前用户的所有会话
#[tauri::command]
async fn get_conversations_by_user(state: tauri::State<'_, AppState>, include_archived: bool) -> Result<Vec<Conversation>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let user_id_lock = state.current_user_id.lock().unwrap();
    let user_id = user_id_lock.ok_or("未选择当前用户")?;
    db.get_conversations_by_user(user_id, include_archived).map_err(|e| e.to_string())
}

/// 根据 ID 获取会话
#[tauri::command]
async fn get_conversation_by_id(state: tauri::State<'_, AppState>, conversation_id: String) -> Result<Option<Conversation>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&conversation_id).map_err(|e| e.to_string())?;
    db.get_conversation_by_id(uuid).map_err(|e| e.to_string())
}

/// 更新会话
#[tauri::command]
async fn update_conversation(state: tauri::State<'_, AppState>, conversation_id: String, req: UpdateConversationRequest) -> Result<Option<Conversation>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&conversation_id).map_err(|e| e.to_string())?;
    db.update_conversation(uuid, req).map_err(|e| e.to_string())
}

/// 删除会话
#[tauri::command]
async fn delete_conversation(state: tauri::State<'_, AppState>, conversation_id: String) -> Result<bool, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&conversation_id).map_err(|e| e.to_string())?;
    db.delete_conversation(uuid).map_err(|e| e.to_string())
}

// ==================== 消息管理命令 ====================

/// 创建新消息
#[tauri::command]
async fn create_message(state: tauri::State<'_, AppState>, conversation_id: String, req: CreateMessageRequest) -> Result<Message, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let conv_uuid = Uuid::parse_str(&conversation_id).map_err(|e| e.to_string())?;
    let user_id_lock = state.current_user_id.lock().unwrap();
    let user_id = user_id_lock.ok_or("未选择当前用户")?;
    db.create_message(conv_uuid, user_id, req).map_err(|e| e.to_string())
}

/// 获取会话的所有消息
#[tauri::command]
async fn get_messages_by_conversation(state: tauri::State<'_, AppState>, conversation_id: String) -> Result<Vec<Message>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&conversation_id).map_err(|e| e.to_string())?;
    db.get_messages_by_conversation(uuid).map_err(|e| e.to_string())
}

/// 删除消息
#[tauri::command]
async fn delete_message(state: tauri::State<'_, AppState>, message_id: String) -> Result<bool, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let uuid = Uuid::parse_str(&message_id).map_err(|e| e.to_string())?;
    db.delete_message(uuid).map_err(|e| e.to_string())
}

// ==================== 设置管理命令 ====================

/// 获取当前用户设置
#[tauri::command]
async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Option<AppSettings>, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let user_id_lock = state.current_user_id.lock().unwrap();
    let user_id = user_id_lock.ok_or("未选择当前用户")?;
    db.get_settings(user_id).map_err(|e| e.to_string())
}

/// 创建或更新当前用户设置
#[tauri::command]
async fn upsert_settings(state: tauri::State<'_, AppState>, settings: AppSettings) -> Result<AppSettings, String> {
    let db_lock = state.db.lock().unwrap();
    let db = db_lock.as_ref().ok_or("数据库未初始化")?;
    let user_id_lock = state.current_user_id.lock().unwrap();
    let user_id = user_id_lock.ok_or("未选择当前用户")?;
    db.upsert_settings(user_id, settings).map_err(|e| e.to_string())
}

/// 问候用户
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            get_current_user_id,
            set_current_user_id,
            create_user,
            get_all_users,
            get_user_by_id,
            update_user,
            delete_user,
            create_conversation,
            get_conversations_by_user,
            get_conversation_by_id,
            update_conversation,
            delete_conversation,
            create_message,
            get_messages_by_conversation,
            delete_message,
            get_settings,
            upsert_settings,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
