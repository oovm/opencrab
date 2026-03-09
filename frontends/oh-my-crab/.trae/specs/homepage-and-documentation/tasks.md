# Homepage 和文档迁移 - 实施计划

## [ ] Task 1: 清理 documentation 目录
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 删除 .vitepress 目录
  - 删除 .vutex 目录
  - 更新 package.json，移除 vitepress 相关依赖和脚本
  - 保留所有 Markdown 文档文件
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: 确认 .vitepress 和 .vutex 目录已删除
  - `programmatic` TR-1.2: 确认 package.json 中 vitepress 相关内容已移除
  - `human-judgement` TR-1.3: 确认所有 Markdown 文档文件完整保留
- **Notes**: 确保只删除构建配置文件，保留文档内容

## [ ] Task 2: 配置 Vite 以支持 Markdown 文档加载
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 更新 vite.config.ts，添加对 Markdown 文件的处理支持
  - 配置 import.meta.glob 可以正确加载 .md 文件
- **Acceptance Criteria Addressed**: AC-2, AC-4
- **Test Requirements**:
  - `human-judgement` TR-2.1: 确认 vite.config.ts 已正确配置
  - `programmatic` TR-2.2: 可以使用 import.meta.glob 加载 .md 文件
- **Notes**: 可以使用 ?raw 后缀加载原始 Markdown 内容

## [ ] Task 3: 创建文档加载和管理工具
- **Priority**: P0
- **Depends On**: Task 2
- **Description**: 
  - 在 src/utils/ 下创建文档管理工具
  - 实现使用 import.meta.glob 动态加载文档
  - 实现文档目录结构的解析和导航
  - 将文档复制到 oh-my-crab/src/docs/ 目录下
- **Acceptance Criteria Addressed**: AC-3, AC-4
- **Test Requirements**:
  - `human-judgement` TR-3.1: 确认文档文件已复制到正确位置
  - `programmatic` TR-3.2: 可以正确加载和解析文档目录结构
  - `programmatic` TR-3.3: 可以通过路径获取单个文档内容

## [ ] Task 4: 创建 Markdown 渲染组件
- **Priority**: P0
- **Depends On**: Task 3
- **Description**: 
  - 在 src/components/ 下创建 MarkdownViewer 组件
  - 使用 marked 库渲染 Markdown 内容
  - 应用 TailwindCSS 样式使渲染结果美观
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgement` TR-4.1: 确认 Markdown 内容被正确渲染
  - `human-judgement` TR-4.2: 确认渲染结果样式美观，与整体 UI 一致

## [ ] Task 5: 创建 Homepage 组件
- **Priority**: P0
- **Depends On**: Task 3, Task 4
- **Description**: 
  - 在 src/views/ 下创建 Homepage 组件
  - 设计 Homepage 布局，包含文档浏览板块
  - 实现文档侧边栏导航
  - 实现文档内容展示区域
- **Acceptance Criteria Addressed**: AC-2, AC-3
- **Test Requirements**:
  - `human-judgement` TR-5.1: 确认 Homepage 可以正常渲染
  - `human-judgement` TR-5.2: 确认文档侧边栏导航功能正常
  - `human-judgement` TR-5.3: 确认点击文档可以正确显示内容

## [ ] Task 6: 集成 Homepage 到 oh-my-crab
- **Priority**: P0
- **Depends On**: Task 5
- **Description**: 
  - 更新 App.vue，添加 Homepage 到导航标签
  - 确保现有功能不受影响
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `human-judgement` TR-6.1: 确认导航栏包含 Homepage 选项
  - `human-judgement` TR-6.2: 确认可以通过导航访问 Homepage
  - `human-judgement` TR-6.3: 确认现有功能（对话、记忆等）正常工作

## [ ] Task 7: 测试和优化
- **Priority**: P1
- **Depends On**: Task 6
- **Description**: 
  - 测试整个功能流程
  - 优化性能和用户体验
  - 修复发现的问题
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3, AC-4, AC-5
- **Test Requirements**:
  - `human-judgement` TR-7.1: 整个功能流程顺畅
  - `human-judgement` TR-7.2: 性能和用户体验良好
