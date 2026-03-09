//! OpenCrab 命令行入口
//!
//! MVP 版本 - 简单演示各个模块的使用

use opencrab::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("========================================");
    println!("  OpenCrab MVP 版本");
    println!("========================================");
    println!();

    println!("📦 初始化基础设施层模块...");

    let config_loader = ConfigLoader::new();
    println!("  ✓ ConfigLoader 初始化成功");

    let _database = SqliteConnection::new("sqlite::memory:").await?;
    println!("  ✓ SqliteConnection 初始化成功");

    let cache = MemoryCache::new();
    println!("  ✓ MemoryCache 初始化成功");

    let event_bus = MemoryEventBus::new();
    println!("  ✓ MemoryEventBus 初始化成功");

    let storage = FileSystemStorage::new();
    println!("  ✓ FileSystemStorage 初始化成功");

    let queue = MemoryQueue::new();
    println!("  ✓ MemoryQueue 初始化成功");

    println!();
    println!("🎯 初始化核心层模块...");

    let agent_service = MemoryAgentService::new();
    println!("  ✓ MemoryAgentService 初始化成功");

    let skill_service = MemorySkillService::new();
    println!("  ✓ MemorySkillService 初始化成功");

    let memory_service = MemoryMemoryService::new();
    println!("  ✓ MemoryMemoryService 初始化成功");

    let chat_service = MemoryChatService::new();
    println!("  ✓ MemoryChatService 初始化成功");

    let tool_service = MemoryToolService::new();
    println!("  ✓ MemoryToolService 初始化成功");

    let scheduler_service = MemorySchedulerService::new();
    println!("  ✓ MemorySchedulerService 初始化成功");

    let workspace_service = MemoryWorkspaceService::new();
    println!("  ✓ MemoryWorkspaceService 初始化成功");

    println!();
    println!("🚀 创建测试智能体...");

    let agent = agent_service.create_agent(
        "Test Agent",
        Some("一个测试用的智能体"),
        AgentType::General
    ).await?;
    println!("  ✓ 智能体 '{}' 创建成功", agent.name);

    println!();
    println!("💾 测试缓存功能...");

    cache.set("test_key", b"Hello, OpenCrab!", std::time::Duration::from_secs(60)).await?;
    let value = cache.get("test_key").await?;
    println!("  ✓ 缓存写入成功");
    if let Some(v) = value {
        println!("  ✓ 缓存读取成功: {}", String::from_utf8_lossy(&v));
    }

    println!();
    println!("========================================");
    println!("  OpenCrab MVP 版本启动成功！");
    println!("========================================");

    Ok(())
}
