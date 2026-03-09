# OpenCrab 重构 - 验证清单

## 架构和结构验证
- [ ] 检查项目采用 Cargo workspace 进行管理
- [ ] 检查功能被清晰地划分到不同的 crate 中（backends/ 下）
- [ ] 检查每个 crate 职责单一，符合单一职责原则
- [ ] 检查依赖关系合理，没有循环依赖
- [ ] 检查 backends/opencrab 作为主入口整合了所有功能

## 代码质量验证
- [ ] 检查所有 public 结构体、枚举、方法、字段都有完整的中文文档注释
- [ ] 检查没有使用后置注释
- [ ] 检查所有注释和图示都是中文
- [ ] 检查代码风格一致
- [ ] 检查错误处理完整

## 功能完整性验证
- [ ] 检查 crab-types 包含所有基础类型和错误处理
- [ ] 检查 crab-config 包含所有配置项（agent、llm、channels、database 等）
- [ ] 检查 crab-observability 包含完整的日志和追踪系统
- [ ] 检查 crab-database 支持 PostgreSQL 和 libSQL
- [ ] 检查 crab-llm 包含多提供商支持、故障转移、重试等
- [ ] 检查 crab-tools 包含工具注册表、速率限制、MCP 和 WASM 工具支持
- [ ] 检查 crab-safety 包含输入清理、策略验证、凭据检测
- [ ] 检查 crab-secrets 包含密钥加密存储
- [ ] 检查 crab-sandbox 包含 Docker 容器管理
- [ ] 检查 crab-workspace 包含文档管理、嵌入、搜索
- [ ] 检查 crab-skills 包含技能解析、注册、选择
- [ ] 检查 crab-context 包含上下文管理和内存
- [ ] 检查 crab-history 包含历史记录和分析
- [ ] 检查 crab-estimation 包含成本、时间、价值估算
- [ ] 检查 crab-evaluation 包含指标和成功评估
- [ ] 检查 crab-hooks 包含钩子注册和触发
- [ ] 检查 crab-registry 包含目录和安装器
- [ ] 检查 crab-extensions 包含扩展发现和加载
- [ ] 检查 crab-pairing 包含配对存储
- [ ] 检查 crab-channels 包含 REPL、HTTP、Signal、WebSocket、Web UI 网关、WASM 通道
- [ ] 检查 crab-agent 包含代理循环、消息路由、任务执行、调度器、自我修复
- [ ] 检查 crab-orchestrator 包含 API 和作业管理

## 编译和运行验证
- [ ] 检查 `cargo build` 能够成功编译整个项目
- [ ] 检查 `cargo build --release` 能够成功编译
- [ ] 检查 opencrab.exe 能够正常生成
- [ ] 检查 `cargo run --bin opencrab -- --version` 能够正常输出版本信息
- [ ] 检查 `cargo run --bin opencrab -- --help` 能够正常输出帮助信息
- [ ] 检查所有子命令都能正常工作（repl、serve、doctor 等）

## 测试验证
- [ ] 检查所有单元测试都能通过
- [ ] 检查所有集成测试都能通过
- [ ] 检查测试覆盖率合理

## 功能一致性验证
- [ ] 检查 opencrab 的功能与 ironclaw 完全一致
- [ ] 检查 CLI 接口与 ironclaw 一致
- [ ] 检查 Web UI 功能与 ironclaw 一致
- [ ] 检查工具调用行为与 ironclaw 一致
- [ ] 检查 LLM 集成行为与 ironclaw 一致
