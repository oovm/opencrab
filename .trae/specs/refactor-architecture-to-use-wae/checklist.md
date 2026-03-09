# OpenCrab 架构重构 - 验证清单

## 代码变更验证
- [ ] 所有重复的 crab-* 基础设施模块已从 backends/ 目录移除
- [ ] Cargo.toml 工作空间配置已更新，不再包含基础设施模块
- [ ] Cargo.toml 工作空间配置已添加 WAE 依赖
- [ ] 核心业务模块的 Cargo.toml 已更新，依赖从 crab-* 改为 wae-*
- [ ] 核心业务模块的导入语句已更新
- [ ] crab-types 模块已重构，只保留 OpenCrab 特有类型
- [ ] opencrab 主模块已更新

## 编译验证
- [ ] cargo build --all 成功完成，无错误
- [ ] cargo check --all 成功完成，无警告
- [ ] 所有核心业务模块能够独立编译

## 文档验证
- [ ] architecture/index.md 已更新，反映新的分层结构
- [ ] master-plan.md 已更新，明确 OpenCrab 依赖 WAE
- [ ] infrastructure.md 已更新或删除
- [ ] core-layer.md 已更新

## 架构原则验证
- [ ] 不再有重复的基础设施模块
- [ ] OpenCrab 专注于智能体核心业务逻辑
- [ ] 所有基础设施功能由 WAE 提供
- [ ] 依赖关系清晰，单向向下依赖
- [ ] 遵循单一职责原则

## 向后兼容性验证
- [ ] 核心业务模块的公共 API 保持不变
- [ ] 类型定义保持兼容
- [ ] 没有破坏性变更（除非必要并有文档说明）
