#![warn(missing_docs)]

//! OpenCrab 集成测试

use opencrab::prelude::*;

#[tokio::test]
async fn test_agent_creation() {
    let agent_service = MemoryAgentService::new();
    let agent = agent_service
        .create_agent("Test Agent", Some("A test agent"), AgentType::General)
        .await
        .unwrap();
    
    assert_eq!(agent.name, "Test Agent");
    assert_eq!(agent.agent_type, AgentType::General);
    assert_eq!(agent.status, AgentStatus::Uninitialized);
}

#[tokio::test]
async fn test_skill_registration() {
    let skill_service = MemorySkillService::new();
    let skill = skill_service
        .register_skill(
            "Test Skill",
            "A test skill",
            SkillType::Core,
            serde_json::json!({}),
        )
        .await
        .unwrap();
    
    assert_eq!(skill.name, "Test Skill");
    assert_eq!(skill.skill_type, SkillType::Core);
}

#[tokio::test]
async fn test_memory_creation() {
    let memory_service = MemoryMemoryService::new();
    let owner_id = Uuid::new_v4();
    let memory = memory_service
        .create_memory(
            MemoryType::Message,
            owner_id,
            "agent",
            "Test memory content",
            Some("Test summary"),
            serde_json::json!({}),
        )
        .await
        .unwrap();
    
    assert_eq!(memory.owner_id, owner_id);
    assert_eq!(memory.memory_type, MemoryType::Message);
    assert_eq!(memory.content, "Test memory content");
}

#[tokio::test]
async fn test_conversation_creation() {
    let chat_service = MemoryChatService::new();
    let user_id = Uuid::new_v4();
    let conversation = chat_service
        .create_conversation(user_id, "Test Conversation", Some("A test conversation"))
        .await
        .unwrap();
    
    assert_eq!(conversation.user_id, user_id);
    assert_eq!(conversation.title, "Test Conversation");
    assert!(!conversation.is_archived);
}

#[tokio::test]
async fn test_tool_registration() {
    let tool_service = MemoryToolService::new();
    let tool = tool_service
        .register_tool(
            "Test Tool",
            Some("A test tool"),
            serde_json::json!({}),
        )
        .await
        .unwrap();
    
    assert_eq!(tool.name, "Test Tool");
}

#[tokio::test]
async fn test_task_creation() {
    let scheduler_service = MemorySchedulerService::new();
    let task = scheduler_service
        .create_task("Test Task", Some("A test task"), None)
        .await
        .unwrap();
    
    assert_eq!(task.name, "Test Task");
    assert_eq!(task.status, TaskStatus::Pending);
}

#[tokio::test]
async fn test_workspace_creation() {
    let workspace_service = MemoryWorkspaceService::new();
    let creator_id = Uuid::new_v4();
    let workspace = workspace_service
        .create_workspace(
            "Test Workspace",
            Some("A test workspace"),
            "/test/path",
            creator_id,
        )
        .await
        .unwrap();
    
    assert_eq!(workspace.name, "Test Workspace");
    assert_eq!(workspace.path, "/test/path");
    assert_eq!(workspace.creator_id, creator_id);
}

#[test]
fn test_error_creation() {
    let error = Error::not_implemented("test_feature");
    assert_eq!(error.http_status(), 500);
    assert_eq!(error.category(), ErrorCategory::Internal);
}
