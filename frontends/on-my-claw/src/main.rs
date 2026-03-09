//! On My Claw - OpenCrab 命令行客户端
//!
//! 提供命令行界面用于连接和操作 OpenCrab 或 OpenCrab 服务器。

#![warn(missing_docs)]

use clap::Parser;
use crab_client::*;

/// On My Claw CLI 命令行参数
#[derive(Parser, Debug)]
#[command(name = "on-my-claw", about = "OpenCrab 命令行客户端", version)]
struct Cli {
    /// 要执行的子命令
    #[command(subcommand)]
    command: Commands,
}

/// 可用的子命令
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// 连接到 OpenCrab 服务器
    Connect(ConnectArgs),
    /// 用户管理相关命令
    #[command(subcommand)]
    User(UserCommands),
    /// 会话管理相关命令
    #[command(subcommand)]
    Conversation(ConversationCommands),
    /// 消息管理相关命令
    #[command(subcommand)]
    Message(MessageCommands),
    /// 设置管理相关命令
    #[command(subcommand)]
    Settings(SettingsCommands),
}

/// 连接命令参数
#[derive(Parser, Debug)]
struct ConnectArgs {
    /// 服务器主机地址
    #[arg(long, short = 'h', default_value = "localhost")]
    host: String,

    /// 服务器端口
    #[arg(long, short = 'p', default_value = "8080")]
    port: u16,
}

/// 用户管理子命令
#[derive(clap::Subcommand, Debug)]
enum UserCommands {
    /// 创建新用户
    Create(CreateUserArgs),
    /// 列出所有用户
    List,
    /// 获取单个用户
    Get(GetUserArgs),
    /// 更新用户信息
    Update(UpdateUserArgs),
    /// 删除用户
    Delete(DeleteUserArgs),
}

/// 创建用户参数
#[derive(Parser, Debug)]
struct CreateUserArgs {
    /// 用户名
    #[arg(long, short = 'u')]
    username: String,
    /// 显示名称
    #[arg(long)]
    display_name: Option<String>,
    /// 用户角色
    #[arg(long, default_value = "user")]
    role: String,
    /// 邮箱
    #[arg(long)]
    email: Option<String>,
}

/// 获取用户参数
#[derive(Parser, Debug)]
struct GetUserArgs {
    /// 用户 ID
    #[arg(long, short = 'i')]
    user_id: String,
}

/// 更新用户参数
#[derive(Parser, Debug)]
struct UpdateUserArgs {
    /// 用户 ID
    #[arg(long, short = 'i')]
    user_id: String,
    /// 显示名称
    #[arg(long)]
    display_name: Option<String>,
    /// 邮箱
    #[arg(long)]
    email: Option<String>,
    /// 是否激活
    #[arg(long)]
    is_active: Option<bool>,
}

/// 删除用户参数
#[derive(Parser, Debug)]
struct DeleteUserArgs {
    /// 用户 ID
    #[arg(long, short = 'i')]
    user_id: String,
}

/// 会话管理子命令
#[derive(clap::Subcommand, Debug)]
enum ConversationCommands {
    /// 创建新会话
    Create(CreateConversationArgs),
    /// 列出用户的所有会话
    List(ListConversationsArgs),
    /// 获取单个会话
    Get(GetConversationArgs),
    /// 更新会话
    Update(UpdateConversationArgs),
    /// 删除会话
    Delete(DeleteConversationArgs),
}

/// 创建会话参数
#[derive(Parser, Debug)]
struct CreateConversationArgs {
    /// 用户 ID
    #[arg(long, short = 'u')]
    user_id: String,
    /// 会话标题
    #[arg(long, short = 't')]
    title: String,
    /// 会话描述
    #[arg(long, short = 'd')]
    description: Option<String>,
}

/// 列出会话参数
#[derive(Parser, Debug)]
struct ListConversationsArgs {
    /// 用户 ID
    #[arg(long, short = 'u')]
    user_id: String,
    /// 是否包含已归档的会话
    #[arg(long, default_value = "false")]
    include_archived: bool,
}

/// 获取会话参数
#[derive(Parser, Debug)]
struct GetConversationArgs {
    /// 会话 ID
    #[arg(long, short = 'i')]
    conversation_id: String,
}

/// 更新会话参数
#[derive(Parser, Debug)]
struct UpdateConversationArgs {
    /// 会话 ID
    #[arg(long, short = 'i')]
    conversation_id: String,
    /// 会话标题
    #[arg(long, short = 't')]
    title: Option<String>,
    /// 会话描述
    #[arg(long, short = 'd')]
    description: Option<String>,
    /// 是否归档
    #[arg(long)]
    is_archived: Option<bool>,
}

/// 删除会话参数
#[derive(Parser, Debug)]
struct DeleteConversationArgs {
    /// 会话 ID
    #[arg(long, short = 'i')]
    conversation_id: String,
}

/// 消息管理子命令
#[derive(clap::Subcommand, Debug)]
enum MessageCommands {
    /// 创建新消息
    Create(CreateMessageArgs),
    /// 列出会话的所有消息
    List(ListMessagesArgs),
    /// 删除消息
    Delete(DeleteMessageArgs),
}

/// 创建消息参数
#[derive(Parser, Debug)]
struct CreateMessageArgs {
    /// 会话 ID
    #[arg(long, short = 'c')]
    conversation_id: String,
    /// 用户 ID
    #[arg(long, short = 'u')]
    user_id: String,
    /// 消息角色
    #[arg(long, short = 'r')]
    role: String,
    /// 消息内容
    #[arg(long, short = 'C')]
    content: String,
    /// 消息元数据
    #[arg(long)]
    metadata: Option<String>,
}

/// 列出消息参数
#[derive(Parser, Debug)]
struct ListMessagesArgs {
    /// 会话 ID
    #[arg(long, short = 'c')]
    conversation_id: String,
}

/// 删除消息参数
#[derive(Parser, Debug)]
struct DeleteMessageArgs {
    /// 消息 ID
    #[arg(long, short = 'i')]
    message_id: String,
}

/// 设置管理子命令
#[derive(clap::Subcommand, Debug)]
enum SettingsCommands {
    /// 获取用户设置
    Get(GetSettingsArgs),
    /// 设置用户设置
    Set(SetSettingsArgs),
}

/// 获取设置参数
#[derive(Parser, Debug)]
struct GetSettingsArgs {
    /// 用户 ID
    #[arg(long, short = 'u')]
    user_id: String,
}

/// 设置设置参数
#[derive(Parser, Debug)]
struct SetSettingsArgs {
    /// 用户 ID
    #[arg(long, short = 'u')]
    user_id: String,
    /// 主题
    #[arg(long)]
    theme: Option<String>,
    /// 语言
    #[arg(long)]
    language: Option<String>,
    /// API 端点
    #[arg(long)]
    api_endpoint: Option<String>,
    /// API 密钥
    #[arg(long)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect(args) => {
            println!("正在连接到 {}:{}...", args.host, args.port);
        }
        Commands::User(cmd) => {
            let db = Database::new()?;
            handle_user_command(&db, cmd)?;
        }
        Commands::Conversation(cmd) => {
            let db = Database::new()?;
            handle_conversation_command(&db, cmd)?;
        }
        Commands::Message(cmd) => {
            let db = Database::new()?;
            handle_message_command(&db, cmd)?;
        }
        Commands::Settings(cmd) => {
            let db = Database::new()?;
            handle_settings_command(&db, cmd)?;
        }
    }

    Ok(())
}

/// 处理用户管理命令
fn handle_user_command(db: &Database, cmd: &UserCommands) -> anyhow::Result<()> {
    match cmd {
        UserCommands::Create(args) => {
            let role = match args.role.to_lowercase().as_str() {
                "admin" => UserRole::Admin,
                "guest" => UserRole::Guest,
                _ => UserRole::User,
            };

            let req = CreateUserRequest {
                username: args.username.clone(),
                display_name: args.display_name.clone(),
                role,
                email: args.email.clone(),
            };

            let user = db.create_user(req)?;
            println!("用户创建成功:");
            print_user(&user);
        }
        UserCommands::List => {
            let users = db.get_all_users()?;
            println!("用户列表 (共 {} 个):", users.len());
            for user in users {
                println!();
                print_user(&user);
            }
        }
        UserCommands::Get(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            if let Some(user) = db.get_user_by_id(user_id)? {
                println!("用户信息:");
                print_user(&user);
            } else {
                println!("未找到用户 ID: {}", args.user_id);
            }
        }
        UserCommands::Update(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            let req = UpdateUserRequest {
                display_name: args.display_name.clone(),
                email: args.email.clone(),
                is_active: args.is_active,
            };

            if let Some(user) = db.update_user(user_id, req)? {
                println!("用户更新成功:");
                print_user(&user);
            } else {
                println!("未找到用户 ID: {}", args.user_id);
            }
        }
        UserCommands::Delete(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            if db.delete_user(user_id)? {
                println!("用户删除成功");
            } else {
                println!("未找到用户 ID: {}", args.user_id);
            }
        }
    }
    Ok(())
}

/// 处理会话管理命令
fn handle_conversation_command(db: &Database, cmd: &ConversationCommands) -> anyhow::Result<()> {
    match cmd {
        ConversationCommands::Create(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            let req = CreateConversationRequest {
                title: args.title.clone(),
                description: args.description.clone(),
            };

            let conversation = db.create_conversation(user_id, req)?;
            println!("会话创建成功:");
            print_conversation(&conversation);
        }
        ConversationCommands::List(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            let conversations = db.get_conversations_by_user(user_id, args.include_archived)?;
            println!("会话列表 (共 {} 个):", conversations.len());
            for conversation in conversations {
                println!();
                print_conversation(&conversation);
            }
        }
        ConversationCommands::Get(args) => {
            let conversation_id = Uuid::parse_str(&args.conversation_id)?;
            if let Some(conversation) = db.get_conversation_by_id(conversation_id)? {
                println!("会话信息:");
                print_conversation(&conversation);
            } else {
                println!("未找到会话 ID: {}", args.conversation_id);
            }
        }
        ConversationCommands::Update(args) => {
            let conversation_id = Uuid::parse_str(&args.conversation_id)?;
            let req = UpdateConversationRequest {
                title: args.title.clone(),
                description: args.description.clone(),
                is_archived: args.is_archived,
            };

            if let Some(conversation) = db.update_conversation(conversation_id, req)? {
                println!("会话更新成功:");
                print_conversation(&conversation);
            } else {
                println!("未找到会话 ID: {}", args.conversation_id);
            }
        }
        ConversationCommands::Delete(args) => {
            let conversation_id = Uuid::parse_str(&args.conversation_id)?;
            if db.delete_conversation(conversation_id)? {
                println!("会话删除成功");
            } else {
                println!("未找到会话 ID: {}", args.conversation_id);
            }
        }
    }
    Ok(())
}

/// 处理消息管理命令
fn handle_message_command(db: &Database, cmd: &MessageCommands) -> anyhow::Result<()> {
    match cmd {
        MessageCommands::Create(args) => {
            let conversation_id = Uuid::parse_str(&args.conversation_id)?;
            let user_id = Uuid::parse_str(&args.user_id)?;
            let req = CreateMessageRequest {
                role: args.role.clone(),
                content: args.content.clone(),
                metadata: args.metadata.clone(),
            };

            let message = db.create_message(conversation_id, user_id, req)?;
            println!("消息创建成功:");
            print_message(&message);
        }
        MessageCommands::List(args) => {
            let conversation_id = Uuid::parse_str(&args.conversation_id)?;
            let messages = db.get_messages_by_conversation(conversation_id)?;
            println!("消息列表 (共 {} 个):", messages.len());
            for message in messages {
                println!();
                print_message(&message);
            }
        }
        MessageCommands::Delete(args) => {
            let message_id = Uuid::parse_str(&args.message_id)?;
            if db.delete_message(message_id)? {
                println!("消息删除成功");
            } else {
                println!("未找到消息 ID: {}", args.message_id);
            }
        }
    }
    Ok(())
}

/// 处理设置管理命令
fn handle_settings_command(db: &Database, cmd: &SettingsCommands) -> anyhow::Result<()> {
    match cmd {
        SettingsCommands::Get(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            if let Some(settings) = db.get_settings(user_id)? {
                println!("用户设置:");
                print_settings(&settings);
            } else {
                println!("未找到用户 ID {} 的设置", args.user_id);
            }
        }
        SettingsCommands::Set(args) => {
            let user_id = Uuid::parse_str(&args.user_id)?;
            let existing = db.get_settings(user_id)?;
            let now = Utc::now();

            let settings = if let Some(existing) = existing {
                AppSettings {
                    theme: args.theme.clone().unwrap_or(existing.theme),
                    language: args.language.clone().unwrap_or(existing.language),
                    api_endpoint: args.api_endpoint.clone().or(existing.api_endpoint),
                    api_key: args.api_key.clone().or(existing.api_key),
                    ..existing
                }
            } else {
                AppSettings {
                    id: Uuid::new_v4(),
                    user_id,
                    theme: args.theme.clone().unwrap_or_else(|| "dark".to_string()),
                    language: args.language.clone().unwrap_or_else(|| "zh-CN".to_string()),
                    api_endpoint: args.api_endpoint.clone(),
                    api_key: args.api_key.clone(),
                    settings_json: None,
                    created_at: now,
                    updated_at: now,
                }
            };

            let updated = db.upsert_settings(user_id, settings)?;
            println!("设置更新成功:");
            print_settings(&updated);
        }
    }
    Ok(())
}

/// 打印用户信息
fn print_user(user: &User) {
    println!("  ID: {}", user.id);
    println!("  用户名: {}", user.username);
    if let Some(display_name) = &user.display_name {
        println!("  显示名称: {}", display_name);
    }
    println!("  角色: {:?}", user.role);
    if let Some(email) = &user.email {
        println!("  邮箱: {}", email);
    }
    println!("  创建时间: {}", user.created_at);
    println!("  更新时间: {}", user.updated_at);
    println!("  激活状态: {}", user.is_active);
}

/// 打印会话信息
fn print_conversation(conversation: &Conversation) {
    println!("  ID: {}", conversation.id);
    println!("  用户 ID: {}", conversation.user_id);
    println!("  标题: {}", conversation.title);
    if let Some(description) = &conversation.description {
        println!("  描述: {}", description);
    }
    println!("  创建时间: {}", conversation.created_at);
    println!("  更新时间: {}", conversation.updated_at);
    println!("  归档状态: {}", conversation.is_archived);
}

/// 打印消息信息
fn print_message(message: &Message) {
    println!("  ID: {}", message.id);
    println!("  会话 ID: {}", message.conversation_id);
    println!("  用户 ID: {}", message.user_id);
    println!("  角色: {}", message.role);
    println!("  内容: {}", message.content);
    if let Some(metadata) = &message.metadata {
        println!("  元数据: {}", metadata);
    }
    println!("  创建时间: {}", message.created_at);
}

/// 打印设置信息
fn print_settings(settings: &AppSettings) {
    println!("  ID: {}", settings.id);
    println!("  用户 ID: {}", settings.user_id);
    println!("  主题: {}", settings.theme);
    println!("  语言: {}", settings.language);
    if let Some(api_endpoint) = &settings.api_endpoint {
        println!("  API 端点: {}", api_endpoint);
    }
    if let Some(api_key) = &settings.api_key {
        println!("  API 密钥: {}", api_key);
    }
    if let Some(settings_json) = &settings.settings_json {
        println!("  设置 JSON: {}", settings_json);
    }
    println!("  创建时间: {}", settings.created_at);
    println!("  更新时间: {}", settings.updated_at);
}
