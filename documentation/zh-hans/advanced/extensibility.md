# 扩展性

本文档深入介绍 OpenCrab 的扩展机制，包括插件系统、自定义模块开发和架构扩展性设计。

## 概述

OpenCrab 采用模块化、插件化的架构设计，从底层协议层到上层应用层都提供了丰富的扩展点。这种设计使开发者能够：

- 不修改核心代码即可扩展功能
- 根据需求定制和替换组件
- 构建生态系统，共享扩展模块

## 核心扩展原则

OpenCrab 的扩展性设计遵循以下原则：

### 1. 接口抽象优先

所有可扩展的功能都先定义 trait 接口，再提供默认实现。例如：

```rust
pub trait MemorySystem {
    fn save(&self, memory: Memory) -> Result<MemoryId>;
    fn retrieve(&self, id: MemoryId) -> Result<Option<Memory>>;
    fn search(&self, query: &str) -> Result<Vec<Memory>>;
}
```

### 2. 依赖注入

通过代数效应（wae 提供）实现声明式依赖注入，避免硬编码依赖：

```rust
#[effect]
trait MemorySystemEffect {
    fn save_memory(memory: Memory) -> MemoryId;
}
```

### 3. 插件注册表

所有可扩展组件都通过统一的注册表管理，支持运行时注册和卸载：

```rust
pub struct PluginRegistry {
    skills: HashMap<SkillId, Box<dyn Skill>>,
    tools: HashMap<ToolId, Box<dyn Tool>>,
    storage_backends: HashMap<BackendId, Box<dyn StorageBackend>>,
}
```

---

## 技能插件开发

技能是 OpenCrab 智能体的内在能力，开发者可以通过实现 `Skill` trait 来创建自定义技能。

### 技能 trait 定义

```rust
pub trait Skill: Send + Sync {
    fn id(&self) -> SkillId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, ctx: SkillContext) -> Result<SkillOutput>;
    fn can_handle(&self, task: &Task) -> bool;
}
```

### 创建自定义技能示例

```rust
use crab_skill::{Skill, SkillId, SkillContext, SkillOutput};
use crab_types::Task;

pub struct CodeReviewSkill;

impl Skill for CodeReviewSkill {
    fn id(&self) -> SkillId {
        SkillId::new("code-review")
    }

    fn name(&self) -> &str {
        "代码审查"
    }

    fn description(&self) -> &str {
        "分析代码质量，提供改进建议"
    }

    fn can_handle(&self, task: &Task) -> bool {
        task.tags.contains(&"code-review".into())
    }

    fn execute(&self, ctx: SkillContext) -> Result<SkillOutput> {
        let files = ctx.workspace.list_files()?;
        let mut issues = Vec::new();
        
        for file in files {
            if file.path.extension() == Some("rs".as_ref()) {
                issues.push(Self::analyze_rust_file(&file)?);
            }
        }
        
        Ok(SkillOutput {
            issues,
            suggestions: vec![],
        })
    }
}

impl CodeReviewSkill {
    fn analyze_rust_file(file: &FileInfo) -> Result<Issue> {
        let content = file.read()?;
        let issues = Self::check_style(&content);
        Ok(Issue {
            file: file.path.clone(),
            issues,
        })
    }
}
```

### 注册技能

```rust
use crab_skill::SkillRegistry;

fn register_skills(registry: &mut SkillRegistry) {
    registry.register(Box::new(CodeReviewSkill));
    registry.register(Box::new(DocumentationSkill));
    registry.register(Box::new(TestingSkill));
}
```

---

## 工具插件开发

工具是 OpenCrab 智能体的外部扩展功能，开发者可以通过实现 `Tool` trait 来创建自定义工具。

### 工具 trait 定义

```rust
pub trait Tool: Send + Sync {
    fn id(&self) -> ToolId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> ToolParameters;
    fn execute(&self, params: serde_json::Value) -> Result<ToolOutput>;
    fn permissions(&self) -> Vec<Permission>;
}
```

### 创建自定义工具示例

```rust
use crab_tool::{Tool, ToolId, ToolParameters, ToolOutput, Parameter};
use crab_types::Permission;

pub struct WeatherTool;

impl Tool for WeatherTool {
    fn id(&self) -> ToolId {
        ToolId::new("weather")
    }

    fn name(&self) -> &str {
        "天气查询"
    }

    fn description(&self) -> &str {
        "查询指定城市的天气信息"
    }

    fn parameters(&self) -> ToolParameters {
        ToolParameters {
            parameters: vec![
                Parameter {
                    name: "city".into(),
                    description: "城市名称".into(),
                    required: true,
                    param_type: "string".into(),
                },
            ],
        }
    }

    fn permissions(&self) -> Vec<Permission> {
        vec![Permission::NetworkAccess]
    }

    fn execute(&self, params: serde_json::Value) -> Result<ToolOutput> {
        let city = params["city"].as_str().ok_or_else(|| Error::InvalidParameter)?;
        let weather = Self::fetch_weather(city)?;
        
        Ok(ToolOutput {
            success: true,
            data: weather,
        })
    }
}

impl WeatherTool {
    fn fetch_weather(city: &str) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let response = client.get(&format!("https://api.weather.com/{}", city))
            .send()?
            .json()?;
        Ok(response)
    }
}
```

---

## 存储后端扩展

OpenCrab 支持多种存储后端，开发者可以实现 `StorageBackend` trait 来添加自定义存储。

### 存储后端 trait 定义

```rust
pub trait StorageBackend: Send + Sync {
    fn name(&self) -> &str;
    fn read(&self, path: &Path) -> Result<Vec<u8>>;
    fn write(&self, path: &Path, data: &[u8]) -> Result<()>;
    fn exists(&self, path: &Path) -> Result<bool>;
    fn delete(&self, path: &Path) -> Result<()>;
    fn list(&self, path: &Path) -> Result<Vec<PathBuf>>;
}
```

### 自定义存储后端示例

```rust
use crab_storage::{StorageBackend, Result};
use std::path::{Path, PathBuf};

pub struct S3StorageBackend {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3StorageBackend {
    pub async fn new(bucket: String) -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);
        Ok(Self { client, bucket })
    }
}

impl StorageBackend for S3StorageBackend {
    fn name(&self) -> &str {
        "s3"
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        let key = path.to_str().ok_or_else(|| Error::InvalidPath)?;
        let rt = tokio::runtime::Runtime::new()?;
        let response = rt.block_on(async {
            self.client
                .get_object()
                .bucket(&self.bucket)
                .key(key)
                .send()
                .await
        })?;
        
        let bytes = rt.block_on(response.body.collect())?.into_bytes();
        Ok(bytes.to_vec())
    }

    fn write(&self, path: &Path, data: &[u8]) -> Result<()> {
        let key = path.to_str().ok_or_else(|| Error::InvalidPath)?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(key)
                .body(data.to_vec().into())
                .send()
                .await
        })?;
        Ok(())
    }

    fn exists(&self, path: &Path) -> Result<bool> {
        let key = path.to_str().ok_or_else(|| Error::InvalidPath)?;
        let rt = tokio::runtime::Runtime::new()?;
        let result = rt.block_on(async {
            self.client
                .head_object()
                .bucket(&self.bucket)
                .key(key)
                .send()
                .await
        });
        
        match result {
            Ok(_) => Ok(true),
            Err(aws_sdk_s3::error::HeadObjectError::NoSuchKey(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    fn delete(&self, path: &Path) -> Result<()> {
        let key = path.to_str().ok_or_else(|| Error::InvalidPath)?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            self.client
                .delete_object()
                .bucket(&self.bucket)
                .key(key)
                .send()
                .await
        })?;
        Ok(())
    }

    fn list(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let prefix = path.to_str().ok_or_else(|| Error::InvalidPath)?;
        let rt = tokio::runtime::Runtime::new()?;
        let response = rt.block_on(async {
            self.client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix)
                .send()
                .await
        })?;
        
        let mut paths = Vec::new();
        if let Some(contents) = response.contents {
            for object in contents {
                if let Some(key) = object.key {
                    paths.push(PathBuf::from(key));
                }
            }
        }
        Ok(paths)
    }
}
```

---

## 数据库后端扩展

OpenCrab 的数据库层也支持多种后端实现。

### 数据库 trait 定义

```rust
pub trait DatabaseBackend: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, query: &str, params: &[Param]) -> Result<()>;
    fn query(&self, query: &str, params: &[Param]) -> Result<Vec<Row>>;
    fn transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Transaction) -> Result<T>;
}
```

---

## AI 提供商扩展

OpenCrab 支持多种 AI 模型提供商，开发者可以添加自定义 AI 提供商。

### AI Provider trait 定义

```rust
pub trait AiProvider: Send + Sync {
    fn name(&self) -> &str;
    fn chat(&self, messages: Vec<Message>) -> Result<Message>;
    fn chat_stream(&self, messages: Vec<Message>) -> Result<impl Stream<Item = Result<Chunk>>>;
    fn embeddings(&self, texts: Vec<String>) -> Result<Vec<Embedding>>;
}
```

---

## 架构扩展性

### 前后端分离

OpenCrab 采用前后端一对多架构，通过 `crab-client` 共享库隔离前端和后端：

```
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   oh-my-crab    │  │  oh-my-empire   │  │   oh-my-waifu   │
│  (前端应用1)     │  │  (前端应用2)     │  │  (前端应用3)     │
└────────┬────────┘  └────────┬────────┘  └────────┬────────┘
         │                     │                     │
         └─────────────────────┼─────────────────────┘
                               │
                    ┌──────────▼──────────┐
                    │    crab-client      │
                    │   (共享客户端库)     │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │     crab-*          │
                    │   (核心模块)         │
                    └─────────────────────┘
```

### 协议层扩展

Skynet 协议层定义了标准化的通信接口，新的协议模块可以通过实现 `Protocol` trait 来扩展：

```rust
pub trait Protocol {
    fn name(&self) -> &str;
    fn version(&self) -> Version;
    fn handle_message(&self, msg: Message) -> Result<Response>;
}
```

---

## 最佳实践

### 1. 遵循 trait 设计

- 先定义 trait，再提供实现
- trait 要小而专注，遵循单一职责原则
- 使用关联类型和泛型提高灵活性

### 2. 错误处理

- 使用统一的错误类型
- 提供有意义的错误信息
- 实现 `std::error::Error` trait

### 3. 测试

- 为每个扩展编写单元测试
- 使用 mock trait 隔离依赖
- 提供集成测试示例

### 4. 文档

- 为公共 API 编写文档注释
- 提供使用示例
- 记录配置选项和限制

---

## 总结

OpenCrab 的扩展性设计使开发者能够：

1. 通过 trait 接口扩展任何功能
2. 使用插件系统动态加载扩展
3. 利用依赖注入实现松耦合
4. 通过分层架构保持代码清晰

这种设计确保了 OpenCrab 生态系统的持续演进和繁荣。
