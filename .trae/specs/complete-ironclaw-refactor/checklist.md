# OpenCrab 完整重构 - Verification Checklist

- [ ] Checkpoint 1: Cargo workspace 配置正确，包含所有 backends crate
- [ ] Checkpoint 2: 所有 crab-* 模块可独立编译
- [ ] Checkpoint 3: 所有 ironclaw 功能都已迁移到 opencrab
- [ ] Checkpoint 4: 所有公共 API 都有中文文档注释
- [ ] Checkpoint 5: opencrab 主库可正常编译
- [ ] Checkpoint 6: opencrab.exe 二进制文件可正常生成
- [ ] Checkpoint 7: cargo build --release 成功且无警告
- [ ] Checkpoint 8: 主要功能测试通过
- [ ] Checkpoint 9: 功能与 ironclaw 一致
- [ ] Checkpoint 10: 原 ironclaw 项目未被修改
