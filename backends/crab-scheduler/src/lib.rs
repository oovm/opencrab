#![warn(missing_docs)]

//! Crab Scheduler - OpenCrab 调度系统模块
//!
//! 提供定时任务调度功能。

pub use crab_types::Result;

/// 任务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 待执行
    Pending,
    /// 执行中
    Running,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 调度任务结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledTask {
    /// 任务唯一标识符
    pub id: uuid::Uuid,
    /// 任务名称
    pub name: String,
    /// 任务描述
    pub description: Option<String>,
    /// 任务状态
    pub status: TaskStatus,
    /// Cron 表达式
    pub cron_expression: Option<String>,
    /// 下次执行时间
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ScheduledTask {
    /// 创建新的调度任务
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            description: None,
            status: TaskStatus::Pending,
            cron_expression: None,
            next_run_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 调度服务 trait
#[async_trait::async_trait]
pub trait SchedulerService: Send + Sync {
    /// 创建调度任务
    async fn create_task(
        &self,
        _name: &str,
        _description: Option<&str>,
        _cron_expression: Option<&str>,
    ) -> Result<ScheduledTask>;

    /// 获取任务
    async fn get_task(&self, _task_id: uuid::Uuid) -> Result<ScheduledTask>;

    /// 列出任务
    async fn list_tasks(&self, _limit: u32, _offset: u32) -> Result<Vec<ScheduledTask>>;

    /// 取消任务
    async fn cancel_task(&self, _task_id: uuid::Uuid) -> Result<()>;
}

/// 内存调度服务实现
pub struct MemorySchedulerService;

impl MemorySchedulerService {
    /// 创建新的内存调度服务
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl SchedulerService for MemorySchedulerService {
    async fn create_task(
        &self,
        _name: &str,
        _description: Option<&str>,
        _cron_expression: Option<&str>,
    ) -> Result<ScheduledTask> {
        let mut task = ScheduledTask::new(_name.to_string());
        task.description = _description.map(|s| s.to_string());
        task.cron_expression = _cron_expression.map(|s| s.to_string());
        Ok(task)
    }

    async fn get_task(&self, _task_id: uuid::Uuid) -> Result<ScheduledTask> {
        Err(crab_types::Error::not_implemented("get_task"))
    }

    async fn list_tasks(&self, _limit: u32, _offset: u32) -> Result<Vec<ScheduledTask>> {
        Ok(Vec::new())
    }

    async fn cancel_task(&self, _task_id: uuid::Uuid) -> Result<()> {
        Ok(())
    }
}

impl Default for MemorySchedulerService {
    fn default() -> Self {
        Self::new()
    }
}
