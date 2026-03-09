# Ironclaw 重构至 Opencrab - Verification Checklist

## Workspace 配置
- [ ] 根目录 Cargo.toml 包含正确的 workspace 配置
- [ ] workspace.package 配置了版本、edition、license 等元信息
- [ ] workspace.dependencies 统一管理所有依赖版本
- [ ] members 列表包含所有 backends crates
- [ ] `cargo build` 在根目录成功执行

## 模块拆分
- [ ] 所有功能都拆分到独立的 backends/* crates
- [ ] 模块依赖关系清晰，无循环依赖
- [ ] 遵循单一职责原则
- [ ] 参考了 plot.rs、wae、game-gpt 的架构风格

## 代码质量
- [ ] 所有 public 结构体都有文档注释
- [ ] 所有 public 枚举都有文档注释
- [ ] 所有 public 方法都有文档注释
- [ ] 所有 public 字段都有文档注释
- [ ] 没有使用后置注释
- [ ] `cargo doc --no-deps` 可以正常生成文档

## 主库功能
- [ ] backends/opencrab 主库存在
- [ ] 主库正确依赖其他子库
- [ ] 主库重新导出必要的公共 API
- [ ] 二进制文件 opencrab 可以编译
- [ ] `cargo run --bin opencrab -- --help` 正常显示帮助

## 功能完整性
- [ ] 所有单元测试通过
- [ ] 所有集成测试通过
- [ ] 关键功能手动验证通过
- [ ] 与 ironclaw 功能完全一致

## 构建与测试
- [ ] `cargo test --workspace` 全部通过
- [ ] `cargo clippy --workspace` 无警告（或已处理）
- [ ] `cargo fmt --check` 通过
