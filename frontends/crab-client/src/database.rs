//! 数据库模块
//!
//! 处理 SQLite 数据库连接、初始化和 CRUD 操作。

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use std::fs::create_dir_all;
use uuid::Uuid;

use crate::error::Error;
use crate::path::get_database_path;
use crate::types::*;

/// 数据库管理器
pub struct Database {
    conn: Connection,
}

impl Database {
    /// 创建新的数据库管理器实例
    ///
    /// 使用 `get_database_path()` 获取数据库路径，并自动创建数据库目录（如果不存在）。
    /// 数据库文件名为 `opencrab.db`。
    pub fn new() -> Result<Self, Error> {
        let db_path =
            get_database_path().ok_or_else(|| Error::Other("无法获取数据库路径".into()))?;

        if let Some(parent) = db_path.parent() {
            create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;

        let mut db = Database { conn };
        db.init_tables()?;

        Ok(db)
    }

    /// 初始化数据库表
    fn init_tables(&mut self) -> Result<(), Error> {
        self.conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            
            -- 用户表
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                display_name TEXT,
                role TEXT NOT NULL,
                email TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 1
            );
            
            -- 聊天会话表
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_archived INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            );
            
            -- 聊天消息表
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                metadata TEXT,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            );
            
            -- 应用设置表
            CREATE TABLE IF NOT EXISTS app_settings (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL UNIQUE,
                theme TEXT NOT NULL DEFAULT 'dark',
                language TEXT NOT NULL DEFAULT 'zh-CN',
                api_endpoint TEXT,
                api_key TEXT,
                settings_json TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            );
            
            -- 创建索引
            CREATE INDEX IF NOT EXISTS idx_conversations_user_id ON conversations(user_id);
            CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON conversations(updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
            CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at ASC);
            ",
        )?;

        Ok(())
    }

    // ==================== 用户操作 ====================

    /// 创建新用户
    pub fn create_user(&self, req: CreateUserRequest) -> Result<User, Error> {
        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            username: req.username,
            display_name: req.display_name,
            role: req.role,
            email: req.email,
            created_at: now,
            updated_at: now,
            is_active: true,
        };

        self.conn.execute(
            "INSERT INTO users (id, username, display_name, role, email, created_at, updated_at, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                user.id.to_string(),
                user.username,
                user.display_name,
                serde_json::to_string(&user.role)?,
                user.email,
                user.created_at.to_rfc3339(),
                user.updated_at.to_rfc3339(),
                user.is_active,
            ],
        )?;

        Ok(user)
    }

    /// 获取所有用户
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, display_name, role, email, created_at, updated_at, is_active
             FROM users ORDER BY created_at DESC",
        )?;

        let users = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let username: String = row.get(1)?;
            let display_name: Option<String> = row.get(2)?;
            let role_str: String = row.get(3)?;
            let email: Option<String> = row.get(4)?;
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            let is_active: bool = row.get(7)?;

            let id = Uuid::parse_str(&id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let role = serde_json::from_str(&role_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        6,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);

            Ok(User {
                id,
                username,
                display_name,
                role,
                email,
                created_at,
                updated_at,
                is_active,
            })
        })?;

        users.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }

    /// 根据 ID 获取用户
    pub fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, display_name, role, email, created_at, updated_at, is_active
             FROM users WHERE id = ?1",
        )?;

        let result = stmt
            .query_row(params![user_id.to_string()], |row| {
                let id_str: String = row.get(0)?;
                let username: String = row.get(1)?;
                let display_name: Option<String> = row.get(2)?;
                let role_str: String = row.get(3)?;
                let email: Option<String> = row.get(4)?;
                let created_at_str: String = row.get(5)?;
                let updated_at_str: String = row.get(6)?;
                let is_active: bool = row.get(7)?;

                let id = Uuid::parse_str(&id_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let role = serde_json::from_str(&role_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        3,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            6,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);

                Ok(User {
                    id,
                    username,
                    display_name,
                    role,
                    email,
                    created_at,
                    updated_at,
                    is_active,
                })
            })
            .optional()?;

        Ok(result)
    }

    /// 更新用户
    pub fn update_user(
        &self,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<Option<User>, Error> {
        let now = Utc::now();

        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(display_name) = req.display_name {
            updates.push("display_name = ?".to_string());
            params.push(display_name);
        }
        if let Some(email) = req.email {
            updates.push("email = ?".to_string());
            params.push(email);
        }
        if let Some(is_active) = req.is_active {
            updates.push("is_active = ?".to_string());
            params.push(is_active.to_string());
        }

        if updates.is_empty() {
            return self.get_user_by_id(user_id);
        }

        updates.push("updated_at = ?".to_string());
        params.push(now.to_rfc3339());
        params.push(user_id.to_string());

        let sql = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));

        self.conn
            .execute(&sql, rusqlite::params_from_iter(params))?;

        self.get_user_by_id(user_id)
    }

    /// 删除用户
    pub fn delete_user(&self, user_id: Uuid) -> Result<bool, Error> {
        let rows_affected = self.conn.execute(
            "DELETE FROM users WHERE id = ?1",
            params![user_id.to_string()],
        )?;

        Ok(rows_affected > 0)
    }

    // ==================== 会话操作 ====================

    /// 创建新会话
    pub fn create_conversation(
        &self,
        user_id: Uuid,
        req: CreateConversationRequest,
    ) -> Result<Conversation, Error> {
        let now = Utc::now();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            user_id,
            title: req.title,
            description: req.description,
            created_at: now,
            updated_at: now,
            is_archived: false,
        };

        self.conn.execute(
            "INSERT INTO conversations (id, user_id, title, description, created_at, updated_at, is_archived)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                conversation.id.to_string(),
                conversation.user_id.to_string(),
                conversation.title,
                conversation.description,
                conversation.created_at.to_rfc3339(),
                conversation.updated_at.to_rfc3339(),
                conversation.is_archived,
            ],
        )?;

        Ok(conversation)
    }

    /// 获取用户的所有会话
    pub fn get_conversations_by_user(
        &self,
        user_id: Uuid,
        include_archived: bool,
    ) -> Result<Vec<Conversation>, Error> {
        let sql = if include_archived {
            "SELECT id, user_id, title, description, created_at, updated_at, is_archived
             FROM conversations WHERE user_id = ?1 ORDER BY updated_at DESC"
        } else {
            "SELECT id, user_id, title, description, created_at, updated_at, is_archived
             FROM conversations WHERE user_id = ?1 AND is_archived = 0 ORDER BY updated_at DESC"
        };

        let mut stmt = self.conn.prepare(sql)?;

        let conversations = stmt.query_map(params![user_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let user_id_str: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: Option<String> = row.get(3)?;
            let created_at_str: String = row.get(4)?;
            let updated_at_str: String = row.get(5)?;
            let is_archived: bool = row.get(6)?;

            let id = Uuid::parse_str(&id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let user_id = Uuid::parse_str(&user_id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        4,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);

            Ok(Conversation {
                id,
                user_id,
                title,
                description,
                created_at,
                updated_at,
                is_archived,
            })
        })?;

        conversations
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into())
    }

    /// 根据 ID 获取会话
    pub fn get_conversation_by_id(
        &self,
        conversation_id: Uuid,
    ) -> Result<Option<Conversation>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, title, description, created_at, updated_at, is_archived
             FROM conversations WHERE id = ?1",
        )?;

        let result = stmt
            .query_row(params![conversation_id.to_string()], |row| {
                let id_str: String = row.get(0)?;
                let user_id_str: String = row.get(1)?;
                let title: String = row.get(2)?;
                let description: Option<String> = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                let updated_at_str: String = row.get(5)?;
                let is_archived: bool = row.get(6)?;

                let id = Uuid::parse_str(&id_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let user_id = Uuid::parse_str(&user_id_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        1,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);

                Ok(Conversation {
                    id,
                    user_id,
                    title,
                    description,
                    created_at,
                    updated_at,
                    is_archived,
                })
            })
            .optional()?;

        Ok(result)
    }

    /// 更新会话
    pub fn update_conversation(
        &self,
        conversation_id: Uuid,
        req: UpdateConversationRequest,
    ) -> Result<Option<Conversation>, Error> {
        let now = Utc::now();

        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(title) = req.title {
            updates.push("title = ?".to_string());
            params.push(title);
        }
        if let Some(description) = req.description {
            updates.push("description = ?".to_string());
            params.push(description);
        }
        if let Some(is_archived) = req.is_archived {
            updates.push("is_archived = ?".to_string());
            params.push(is_archived.to_string());
        }

        if updates.is_empty() {
            return self.get_conversation_by_id(conversation_id);
        }

        updates.push("updated_at = ?".to_string());
        params.push(now.to_rfc3339());
        params.push(conversation_id.to_string());

        let sql = format!(
            "UPDATE conversations SET {} WHERE id = ?",
            updates.join(", ")
        );

        self.conn
            .execute(&sql, rusqlite::params_from_iter(params))?;

        self.get_conversation_by_id(conversation_id)
    }

    /// 删除会话
    pub fn delete_conversation(&self, conversation_id: Uuid) -> Result<bool, Error> {
        let rows_affected = self.conn.execute(
            "DELETE FROM conversations WHERE id = ?1",
            params![conversation_id.to_string()],
        )?;

        Ok(rows_affected > 0)
    }

    // ==================== 消息操作 ====================

    /// 创建新消息
    pub fn create_message(
        &self,
        conversation_id: Uuid,
        user_id: Uuid,
        req: CreateMessageRequest,
    ) -> Result<Message, Error> {
        let now = Utc::now();
        let message = Message {
            id: Uuid::new_v4(),
            conversation_id,
            user_id,
            role: req.role,
            content: req.content,
            created_at: now,
            metadata: req.metadata,
        };

        self.conn.execute(
            "INSERT INTO messages (id, conversation_id, user_id, role, content, created_at, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                message.id.to_string(),
                message.conversation_id.to_string(),
                message.user_id.to_string(),
                message.role,
                message.content,
                message.created_at.to_rfc3339(),
                message.metadata,
            ],
        )?;

        Ok(message)
    }

    /// 获取会话的所有消息
    pub fn get_messages_by_conversation(
        &self,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, user_id, role, content, created_at, metadata
             FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC",
        )?;

        let messages = stmt.query_map(params![conversation_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let conversation_id_str: String = row.get(1)?;
            let user_id_str: String = row.get(2)?;
            let role: String = row.get(3)?;
            let content: String = row.get(4)?;
            let created_at_str: String = row.get(5)?;
            let metadata: Option<String> = row.get(6)?;

            let id = Uuid::parse_str(&id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let conversation_id = Uuid::parse_str(&conversation_id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let user_id = Uuid::parse_str(&user_id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);

            Ok(Message {
                id,
                conversation_id,
                user_id,
                role,
                content,
                created_at,
                metadata,
            })
        })?;

        messages
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into())
    }

    /// 删除消息
    pub fn delete_message(&self, message_id: Uuid) -> Result<bool, Error> {
        let rows_affected = self.conn.execute(
            "DELETE FROM messages WHERE id = ?1",
            params![message_id.to_string()],
        )?;

        Ok(rows_affected > 0)
    }

    // ==================== 设置操作 ====================

    /// 获取用户设置
    pub fn get_settings(&self, user_id: Uuid) -> Result<Option<AppSettings>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, theme, language, api_endpoint, api_key, settings_json, created_at, updated_at
             FROM app_settings WHERE user_id = ?1"
        )?;

        let result = stmt
            .query_row(params![user_id.to_string()], |row| {
                let id_str: String = row.get(0)?;
                let user_id_str: String = row.get(1)?;
                let theme: String = row.get(2)?;
                let language: String = row.get(3)?;
                let api_endpoint: Option<String> = row.get(4)?;
                let api_key: Option<String> = row.get(5)?;
                let settings_json: Option<String> = row.get(6)?;
                let created_at_str: String = row.get(7)?;
                let updated_at_str: String = row.get(8)?;

                let id = Uuid::parse_str(&id_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let user_id = Uuid::parse_str(&user_id_str).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        1,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            8,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&Utc);

                Ok(AppSettings {
                    id,
                    user_id,
                    theme,
                    language,
                    api_endpoint,
                    api_key,
                    settings_json,
                    created_at,
                    updated_at,
                })
            })
            .optional()?;

        Ok(result)
    }

    /// 创建或更新用户设置
    pub fn upsert_settings(
        &self,
        _user_id: Uuid,
        settings: AppSettings,
    ) -> Result<AppSettings, Error> {
        let now = Utc::now();

        self.conn.execute(
            "INSERT OR REPLACE INTO app_settings 
             (id, user_id, theme, language, api_endpoint, api_key, settings_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                settings.id.to_string(),
                settings.user_id.to_string(),
                settings.theme,
                settings.language,
                settings.api_endpoint,
                settings.api_key,
                settings.settings_json,
                settings.created_at.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let mut result = settings;
        result.updated_at = now;
        Ok(result)
    }
}
