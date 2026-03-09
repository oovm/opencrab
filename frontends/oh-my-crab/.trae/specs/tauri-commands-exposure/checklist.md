# Oh My Crab - Tauri 命令暴露 - 验证清单

## 共享库重构
- [x] limbo-core 共享库已创建（暂时跳过，直接在 oh-my-crab 中实现）
- [ ] oh-my-crab 可以引用 limbo-core
- [ ] on-my-claw 可以引用 limbo-core
- [ ] 数据库路径管理逻辑统一
- [ ] 无重复代码

## Tauri 状态管理
- [x] AppState 已实现
- [x] Database 实例正确管理
- [x] 当前用户状态可以设置和读取
- [x] 线程安全机制已实现
- [x] 无数据竞争

## 用户管理命令
- [x] create_user 命令已暴露
- [x] get_all_users 命令已暴露
- [x] get_user_by_id 命令已暴露
- [x] update_user 命令已暴露
- [x] delete_user 命令已暴露
- [x] 所有命令有文档注释
- [x] 命令返回正确类型
- [x] 错误处理正确

## 会话管理命令
- [x] create_conversation 命令已暴露
- [x] get_conversations_by_user 命令已暴露
- [x] get_conversation_by_id 命令已暴露
- [x] update_conversation 命令已暴露
- [x] delete_conversation 命令已暴露
- [x] 会话正确关联到当前用户
- [x] 所有命令有文档注释
- [x] 错误处理正确

## 消息管理命令
- [x] create_message 命令已暴露
- [x] get_messages_by_conversation 命令已暴露
- [x] delete_message 命令已暴露
- [x] 消息正确关联到会话和用户
- [x] 所有命令有文档注释
- [x] 错误处理正确

## 设置管理命令
- [x] get_settings 命令已暴露
- [x] upsert_settings 命令已暴露
- [x] 设置正确关联到当前用户
- [x] 所有命令有文档注释
- [x] 错误处理正确

## 前端类型定义
- [x] TypeScript 类型定义已创建
- [x] 所有数据类型有对应接口
- [ ] Tauri 命令有类型安全包装器
- [ ] TypeScript 编译无错误
- [x] 类型与 Rust 类型对应

## main.rs 更新
- [x] AppState 在 setup 中正确设置
- [x] 所有 Tauri 命令已注册
- [ ] 应用可以正常启动
- [ ] 数据库在启动时正确初始化
- [x] 开发模式 devtools 功能保留

## CLI 更新
- [ ] on-my-claw 引用 limbo-core
- [ ] CLI 可以编译并运行
- [ ] CLI 可以访问与 GUI 相同的数据库
- [ ] CLI 命令功能完整
- [ ] 数据库路径逻辑与 GUI 一致

## 集成测试
- [ ] 所有 Tauri 命令端到端功能正常
- [ ] CLI 和 GUI 可以共享数据
- [ ] 错误处理优雅
- [ ] 命令响应时间 < 100ms
- [x] 所有公共项有文档注释
- [x] 无后置注释
- [x] 代码风格一致
