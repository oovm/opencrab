# OpenCrab Crab-Config 模块完善 - Verification Checklist

## 主模块检查
- [ ] lib.rs 包含所有必要的子模块声明
- [ ] Config 主结构体定义完整
- [ ] from_env 方法已实现
- [ ] from_db 方法已实现
- [ ] from_env_with_toml 方法已实现
- [ ] from_db_with_toml 方法已实现
- [ ] inject_llm_keys_from_secrets 函数已实现
- [ ] inject_os_credentials 函数已实现
- [ ] inject_single_var 函数已实现
- [ ] for_testing 方法已实现
- [ ] re_resolve_llm 方法已实现

## 子模块检查
- [ ] agent.rs 功能完整且有文档注释
- [ ] builder.rs 功能完整且有文档注释
- [ ] channels.rs 功能完整且有文档注释
- [ ] database.rs 功能完整且有文档注释
- [ ] embeddings.rs 功能完整且有文档注释
- [ ] heartbeat.rs 功能完整且有文档注释
- [ ] hygiene.rs 功能完整且有文档注释
- [ ] llm.rs 功能完整且有文档注释
- [ ] routines.rs 功能完整且有文档注释
- [ ] safety.rs 功能完整且有文档注释
- [ ] sandbox.rs 功能完整且有文档注释
- [ ] secrets.rs 功能完整且有文档注释
- [ ] skills.rs 功能完整且有文档注释
- [ ] transcription.rs 功能完整且有文档注释
- [ ] tunnel.rs 功能完整且有文档注释
- [ ] wasm.rs 功能完整且有文档注释
- [ ] error.rs 功能完整且有文档注释
- [ ] helpers.rs 功能完整且有文档注释
- [ ] settings.rs 功能完整且有文档注释

## 代码规范检查
- [ ] 所有 public 的结构体有文档注释
- [ ] 所有 public 的枚举有文档注释
- [ ] 所有 public 的方法有文档注释
- [ ] 所有 public 的字段有文档注释
- [ ] 没有使用后置注释
- [ ] 依赖 crab-types 正确配置

## 测试检查
- [ ] `cargo test -p crab-config` 所有测试通过

## 功能完整性检查
- [ ] 配置加载功能与 ironclaw 一致
- [ ] 环境变量注入功能完整
- [ ] TOML 配置文件覆盖功能完整
- [ ] 数据库配置加载功能完整
