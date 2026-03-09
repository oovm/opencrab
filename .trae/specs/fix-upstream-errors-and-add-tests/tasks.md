# 修复 OpenCrab 上下游问题并添加测试 - 实施计划

## [ ] Task 1: 分析 WAE 框架错误
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 运行 cargo build 在 WAE 项目中，详细分析编译错误
  - 定位 wae-storage 模块中的具体问题
  - 定位 wae-database 模块中的具体问题
  - 理解错误 API 的不匹配之处
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `programmatic` TR-1.1: 记录所有编译错误
  - `programmatic` TR-1.2: 定位错误的具体代码位置
- **Notes**: 这是关键的第一步，确保我们理解要修复什么

## [ ] Task 2: 修复 WAE 框架错误
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 修复 wae-storage 模块中的错误 API 问题
  - 修复 wae-database 模块中的错误 API 问题
  - 确保 WAE 框架能够正常编译
  - 运行 WAE 的测试确保修复没有破坏其他功能
- **Acceptance Criteria Addressed**: [AC-1]
- **Test Requirements**:
  - `programmatic` TR-2.1: WAE 项目 cargo build 成功
  - `programmatic` TR-2.2: WAE 项目 cargo test 成功
- **Notes**: 只修复必要的部分，不要过度重构

## [ ] Task 3: 重构 crab-types 错误类型
- **Priority**: P0
- **Depends On**: Task 2
- **Description**: 
  - 移除 crab-types 中的 Error 枚举
  - 将 Error 类型替换为 WaeError 的类型别名
  - 更新 Result 类型别名使用 WaeResult
  - 保持向后兼容性（如有需要）
  - 更新 crab-types 的 Cargo.toml 依赖
- **Acceptance Criteria Addressed**: [AC-2]
- **Test Requirements**:
  - `programmatic` TR-3.1: crab-types 能够正常编译
  - `programmatic` TR-3.2: 类型别名正确指向 WaeError
- **Notes**: 仔细考虑向后兼容性

## [ ] Task 4: 更新核心业务模块错误类型
- **Priority**: P0
- **Depends On**: Task 3
- **Description**: 
  - 更新 crab-agent 使用 WaeError
  - 更新 crab-skill 使用 WaeError
  - 更新 crab-memory 使用 WaeError
  - 更新 crab-chat 使用 WaeError
  - 更新 crab-tool 使用 WaeError
  - 更新 crab-scheduler 使用 WaeError
  - 更新 crab-workspace 使用 WaeError
  - 更新所有导入语句
  - 更新错误创建和处理代码
- **Acceptance Criteria Addressed**: [AC-3]
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有核心业务模块能够正常编译
  - `programmatic` TR-4.2: 错误处理代码已更新
- **Notes**: 逐个模块更新，确保每个模块都能正常工作

## [ ] Task 5: 更新 opencrab 模块
- **Priority**: P0
- **Depends On**: Task 4
- **Description**: 
  - 更新 opencrab 的 lib.rs 使用 WaeError
  - 更新 prelude 模块的导出
  - 确保所有重新导出正确
- **Acceptance Criteria Addressed**: [AC-3]
- **Test Requirements**:
  - `programmatic` TR-5.1: opencrab 模块能够正常编译
  - `programmatic` TR-5.2: 所有重新导出正确
- **Notes**: 确保公共 API 保持一致

## [ ] Task 6: 添加 opencrab 单元测试
- **Priority**: P1
- **Depends On**: Task 5
- **Description**: 
  - 创建 tests 目录结构
  - 为每个核心模块添加单元测试
  - 测试类型定义
  - 测试错误处理
  - 测试内存实现
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-6.1: 单元测试文件已创建
  - `programmatic` TR-6.2: cargo test 成功运行单元测试
- **Notes**: 从简单的测试开始，逐步完善

## [ ] Task 7: 添加 opencrab 集成测试
- **Priority**: P1
- **Depends On**: Task 6
- **Description**: 
  - 创建集成测试目录
  - 测试模块间的集成
  - 测试完整的工作流
  - 测试错误传播
- **Acceptance Criteria Addressed**: [AC-5]
- **Test Requirements**:
  - `programmatic` TR-7.1: 集成测试文件已创建
  - `programmatic` TR-7.2: cargo test 成功运行集成测试
- **Notes**: 确保测试覆盖关键场景

## [ ] Task 8: 全面编译和测试
- **Priority**: P0
- **Depends On**: Task 7
- **Description**: 
  - 运行 cargo build --all 在 OpenCrab 项目
  - 运行 cargo test --all 在 OpenCrab 项目
  - 修复任何剩余的编译错误
  - 确保所有测试通过
- **Acceptance Criteria Addressed**: [AC-6]
- **Test Requirements**:
  - `programmatic` TR-8.1: cargo build --all 成功
  - `programmatic` TR-8.2: cargo test --all 成功
- **Notes**: 这是最终验证步骤
