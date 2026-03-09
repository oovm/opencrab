# OpenCrab 重构 - 实现计划（分解和优先级任务列表）

## [ ] 任务 1: 完善 crab-types 基础类型模块
- **优先级**: P0
- **依赖**: None
- **描述**:
  - 将 ironclaw 中所有基础类型、错误处理、时间处理等迁移到 crab-types
  - 添加完整的中文文档注释
  - 确保所有类型都有适当的测试
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-1.1: 所有类型都能正常编译和使用
  - `programmatic` TR-1.2: 错误类型处理完整
  - `human-judgement` TR-1.3: 所有 public API 都有中文文档注释
- **备注**: 这是其他模块的基础，必须优先完成

## [ ] 任务 2: 完善 crab-config 配置管理模块
- **优先级**: P0
- **依赖**: Task 1
- **描述**:
  - 将 ironclaw 的配置系统完整迁移到 crab-config
  - 包括所有配置项：agent、llm、channels、database、embeddings、heartbeat、hygiene、routines、safety、sandbox、secrets、skills、tunnel、wasm 等
  - 添加完整的中文文档注释
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-2.1: 配置能够从环境变量和 TOML 文件正确加载
  - `programmatic` TR-2.2: 所有配置项都有默认值
  - `human-judgement` TR-2.3: 配置结构清晰，注释完整

## [ ] 任务 3: 完善 crab-observability 可观测性模块
- **优先级**: P0
- **依赖**: Task 1
- **描述**:
  - 将 ironclaw 的日志和追踪系统迁移到 crab-observability
  - 包括日志层、多追踪器支持、无操作追踪器等
  - 添加完整的中文文档注释
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-3.1: 日志能够正常输出
  - `human-judgement` TR-3.2: 追踪器 API 设计合理

## [ ] 任务 4: 完善 crab-database 数据库模块
- **优先级**: P0
- **依赖**: Task 1, Task 2
- **描述**:
  - 将 ironclaw 的数据库抽象层迁移到 crab-database
  - 包括 PostgreSQL 和 libSQL 支持
  - 数据库迁移脚本
  - TLS 支持
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-4.1: 能够连接到 PostgreSQL 和 libSQL 数据库
  - `programmatic` TR-4.2: 数据库迁移能够正常执行
  - `human-judgement` TR-4.3: 数据库抽象层设计合理

## [ ] 任务 5: 完善 crab-llm LLM 提供商模块
- **优先级**: P0
- **依赖**: Task 1, Task 2, Task 3
- **描述**:
  - 将 ironclaw 的 LLM 集成系统完整迁移到 crab-llm
  - 包括多提供商支持、故障转移、重试、响应缓存、智能路由、成本计算等
  - 会话管理
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-5.1: 能够与多个 LLM 提供商通信
  - `programmatic` TR-5.2: 故障转移和重试机制正常工作
  - `human-judgement` TR-5.3: LLM API 设计清晰

## [ ] 任务 6: 完善 crab-tools 工具系统模块
- **优先级**: P0
- **依赖**: Task 1, Task 2, Task 3
- **描述**:
  - 将 ironclaw 的工具系统迁移到 crab-tools
  - 包括工具注册表、速率限制、模式验证、内置工具等
  - MCP 工具支持
  - WASM 工具支持
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-6.1: 工具能够被正确注册和调用
  - `programmatic` TR-6.2: 速率限制正常工作
  - `human-judgement` TR-6.3: 工具系统设计合理

## [ ] 任务 7: 完善 crab-safety 安全模块
- **优先级**: P0
- **依赖**: Task 1
- **描述**:
  - 将 ironclaw 的安全系统迁移到 crab-safety
  - 包括输入清理、策略验证、凭据检测、泄漏检测等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-7.1: 输入能够被正确清理
  - `programmatic` TR-7.2: 策略验证正常工作
  - `human-judgement` TR-7.3: 安全层设计合理

## [ ] 任务 8: 完善 crab-secrets 密钥管理模块
- **优先级**: P0
- **依赖**: Task 1, Task 2, Task 4
- **描述**:
  - 将 ironclaw 的密钥管理系统迁移到 crab-secrets
  - 包括加密存储、密钥链集成、类型安全等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-8.1: 密钥能够被正确加密和存储
  - `programmatic` TR-8.2: 密钥能够被正确解密和检索
  - `human-judgement` TR-8.3: 密钥管理 API 设计合理

## [ ] 任务 9: 完善 crab-sandbox 沙箱模块
- **优先级**: P1
- **依赖**: Task 1, Task 2
- **描述**:
  - 将 ironclaw 的沙箱系统迁移到 crab-sandbox
  - 包括 Docker 容器管理、检测、配置、HTTP 代理等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-9.1: Docker 检测正常工作
  - `programmatic` TR-9.2: 容器能够被正确创建和管理
  - `human-judgement` TR-9.3: 沙箱 API 设计合理

## [ ] 任务 10: 完善 crab-workspace 工作区模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 4
- **描述**:
  - 将 ironclaw 的工作区系统迁移到 crab-workspace
  - 包括文档管理、分块、嵌入、搜索、卫生检查等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-10.1: 文档能够被正确索引和搜索
  - `programmatic` TR-10.2: 嵌入功能正常工作
  - `human-judgement` TR-10.3: 工作区 API 设计合理

## [ ] 任务 11: 完善 crab-skills 技能模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 6
- **描述**:
  - 将 ironclaw 的技能系统迁移到 crab-skills
  - 包括技能解析、注册、目录、选择、衰减等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-11.1: 技能能够被正确解析和注册
  - `programmatic` TR-11.2: 技能选择逻辑正常工作
  - `human-judgement` TR-11.3: 技能系统 API 设计合理

## [ ] 任务 12: 完善 crab-context 上下文模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 4
- **描述**:
  - 将 ironclaw 的上下文管理系统迁移到 crab-context
  - 包括上下文管理器、内存、状态等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-12.1: 上下文能够被正确管理
  - `programmatic` TR-12.2: 内存操作正常工作
  - `human-judgement` TR-12.3: 上下文 API 设计合理

## [ ] 任务 13: 完善 crab-history 历史记录模块
- **优先级**: P1
- **依赖**: Task 1, Task 4
- **描述**:
  - 将 ironclaw 的历史记录系统迁移到 crab-history
  - 包括历史存储、分析等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-13.1: 历史记录能够被正确存储和检索
  - `programmatic` TR-13.2: 分析功能正常工作
  - `human-judgement` TR-13.3: 历史记录 API 设计合理

## [ ] 任务 14: 完善 crab-estimation 估算模块
- **优先级**: P1
- **依赖**: Task 1
- **描述**:
  - 将 ironclaw 的估算系统迁移到 crab-estimation
  - 包括成本、时间、价值估算和学习器
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-14.1: 估算功能正常工作
  - `programmatic` TR-14.2: 学习器能够从历史数据中学习
  - `human-judgement` TR-14.3: 估算 API 设计合理

## [ ] 任务 15: 完善 crab-evaluation 评估模块
- **优先级**: P1
- **依赖**: Task 1
- **描述**:
  - 将 ironclaw 的评估系统迁移到 crab-evaluation
  - 包括指标和成功评估
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-15.1: 评估指标能够被正确计算
  - `human-judgement` TR-15.2: 评估 API 设计合理

## [ ] 任务 16: 完善 crab-hooks 钩子模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 10
- **描述**:
  - 将 ironclaw 的钩子系统迁移到 crab-hooks
  - 包括钩子注册、引导钩子、捆绑钩子等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-16.1: 钩子能够被正确注册和触发
  - `human-judgement` TR-16.2: 钩子系统 API 设计合理

## [ ] 任务 17: 完善 crab-registry 注册表模块
- **优先级**: P1
- **依赖**: Task 1, Task 2
- **描述**:
  - 将 ironclaw 的注册表系统迁移到 crab-registry
  - 包括目录、嵌入式注册表、安装器、清单等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-17.1: 注册表功能正常工作
  - `human-judgement` TR-17.2: 注册表 API 设计合理

## [ ] 任务 18: 完善 crab-extensions 扩展模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 6, Task 17
- **描述**:
  - 将 ironclaw 的扩展管理系统迁移到 crab-extensions
  - 包括发现、管理器、注册表等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-18.1: 扩展能够被正确发现和加载
  - `human-judgement` TR-18.2: 扩展 API 设计合理

## [ ] 任务 19: 完善 crab-pairing 配对模块
- **优先级**: P1
- **依赖**: Task 1, Task 4
- **描述**:
  - 将 ironclaw 的配对系统迁移到 crab-pairing
  - 包括配对存储等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-19.1: 配对功能正常工作
  - `human-judgement` TR-19.2: 配对 API 设计合理

## [ ] 任务 20: 完善 crab-channels 通道模块
- **优先级**: P0
- **依赖**: Task 1, Task 2, Task 3, Task 5, Task 6, Task 10, Task 12, Task 19
- **描述**:
  - 将 ironclaw 的通道系统完整迁移到 crab-channels
  - 包括通道管理器、REPL、HTTP、Signal、WebSocket、Web UI 网关、WASM 通道等
- **验收标准**: AC-1, AC-3, AC-4
- **测试要求**:
  - `programmatic` TR-20.1: 所有通道类型都能正常工作
  - `programmatic` TR-20.2: Web UI 网关正常运行
  - `human-judgement` TR-20.3: 通道 API 设计合理

## [ ] 任务 21: 完善 crab-agent 代理模块
- **优先级**: P0
- **依赖**: Task 1-20
- **描述**:
  - 将 ironclaw 的核心代理逻辑迁移到 crab-agent
  - 包括代理循环、消息路由、任务执行、调度器、例程、会话管理、自我修复等
  - 所有核心代理功能
- **验收标准**: AC-1, AC-3, AC-4
- **测试要求**:
  - `programmatic` TR-21.1: 代理能够正常启动和运行
  - `programmatic` TR-21.2: 消息路由和任务执行正常工作
  - `human-judgement` TR-21.3: 代理逻辑设计合理

## [ ] 任务 22: 完善 crab-orchestrator 编排器模块
- **优先级**: P1
- **依赖**: Task 1, Task 2, Task 4, Task 5, Task 9, Task 21
- **描述**:
  - 将 ironclaw 的编排器系统迁移到 crab-orchestrator
  - 包括 API、认证、作业管理器等
- **验收标准**: AC-1, AC-3
- **测试要求**:
  - `programmatic` TR-22.1: 编排器 API 正常工作
  - `human-judgement` TR-22.2: 编排器 API 设计合理

## [ ] 任务 23: 完善 backends/opencrab 主库
- **优先级**: P0
- **依赖**: Task 1-22
- **描述**:
  - 整合所有子模块，提供完整的公共 API
  - 实现主入口点（main.rs）
  - 包括 CLI、应用构建器、引导流程等
  - 将 ironclaw 的 main.rs 和 app.rs 完整迁移
  - 添加完整的中文文档注释
- **验收标准**: AC-1, AC-2, AC-3, AC-4, AC-5
- **测试要求**:
  - `programmatic` TR-23.1: opencrab 能够成功编译
  - `programmatic` TR-23.2: opencrab.exe 能够正常运行
  - `programmatic` TR-23.3: 所有 CLI 命令正常工作
  - `human-judgement` TR-23.4: 主库 API 设计合理，整合完整

## [ ] 任务 24: 集成测试和验证
- **优先级**: P0
- **依赖**: Task 23
- **描述**:
  - 运行完整的测试套件
  - 验证功能与 ironclaw 一致
  - 修复所有发现的问题
- **验收标准**: AC-1, AC-4
- **测试要求**:
  - `programmatic` TR-24.1: 所有测试通过
  - `programmatic` TR-24.2: 功能与 ironclaw 行为一致
  - `human-judgement` TR-24.3: 整体系统稳定运行
