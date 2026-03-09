# Ironclaw 重构至 Opencrab V2 - Verification Checklist

## Workspace 配置
- [ ] 根目录 Cargo.toml 包含正确的 workspace 配置
- [ ] workspace.package 配置了版本、edition、license 等元信息
- [ ] workspace.dependencies 统一管理所有依赖版本
- [ ] members 列表包含所有 backends crates（包括 crab-agent）
- [ ] `cargo build --workspace` 在根目录成功执行，无错误

## 模块拆分与移植
- [ ] ironclaw/src/agent/ 已完整移植到 crab-agent
- [ ] ironclaw/src/channels/ 已完整移植到 crab-channels
- [ ] ironclaw/src/config/ 已完整移植到 crab-config
- [ ] ironclaw/src/db/ 已完整移植到 crab-database
- [ ] ironclaw/src/llm/ 已完整移植到 crab-llm
- [ ] ironclaw/src/tools/ 已完整移植到 crab-tools
- [ ] ironclaw/src/safety/ 已完整移植到 crab-safety
- [ ] ironclaw/src/sandbox/ 已完整移植到 crab-sandbox
- [ ] ironclaw/src/secrets/ 已完整移植到 crab-secrets
- [ ] ironclaw/src/workspace/ 已完整移植到 crab-workspace
- [ ] ironclaw/src/skills/ 已完整移植到 crab-skills
- [ ] ironclaw/src/context/ 已完整移植到 crab-context
- [ ] ironclaw/src/history/ 已完整移植到 crab-history
- [ ] ironclaw/src/estimation/ 已完整移植到 crab-estimation
- [ ] ironclaw/src/evaluation/ 已完整移植到 crab-evaluation
- [ ] ironclaw/src/extensions/ 已完整移植到 crab-extensions
- [ ] ironclaw/src/hooks/ 已完整移植到 crab-hooks
- [ ] ironclaw/src/registry/ 已完整移植到 crab-registry
- [ ] ironclaw/src/orchestrator/ 已完整移植到 crab-orchestrator
- [ ] ironclaw/src/pairing/ 已完整移植到 crab-pairing
- [ ] ironclaw/src/observability/ 已完整移植到 crab-observability
- [ ] 模块依赖关系清晰，无循环依赖
- [ ] 遵循单一职责原则
- [ ] 参考了 plot.rs、wae、game-gpt 的架构风格

## 代码质量
- [ ] 所有 public 结构体都有文档注释
- [ ] 所有 public 枚举都有文档注释
- [ ] 所有 public 方法都有文档注释
- [ ] 所有 public 字段都有文档注释
- [ ] 没有使用后置注释
- [ ] `cargo doc --no-deps --workspace` 可以正常生成文档
- [ ] `cargo clippy --workspace` 无警告（或已处理）
- [ ] `cargo fmt --check --workspace` 通过

## 主库功能
- [ ] backends/opencrab 主库存在且功能完整
- [ ] 主库正确依赖其他所有子库
- [ ] 主库重新导出必要的公共 API
- [ ] 二进制文件 opencrab 可以正常编译
- [ ] `cargo run --bin opencrab -- --help` 正常显示帮助，与 ironclaw 一致
- [ ] 所有 CLI 命令功能正常

## 功能完整性
- [ ] `cargo test --workspace` 所有单元测试通过
- [ ] 所有集成测试通过
- [ ] 关键功能手动验证通过
- [ ] 与 ironclaw 功能完全一致
- [ ] ironclaw 的所有测试用例在 opencrab 上都能通过
