# Homepage 和文档迁移 - 验证清单

## 清理阶段
- [ ] .vitepress 目录已完全删除
- [ ] .vutex 目录已完全删除
- [ ] documentation/package.json 已更新，移除 vitepress 依赖
- [ ] 所有 Markdown 文档文件完整保留

## 配置阶段
- [ ] vite.config.ts 已更新，支持 Markdown 文件加载
- [ ] 可以使用 import.meta.glob 加载 .md 文件
- [ ] 项目可以正常构建和运行

## 功能阶段
- [ ] 文档文件已复制到 oh-my-crab/src/docs/ 目录
- [ ] 文档加载工具可以正确解析目录结构
- [ ] MarkdownViewer 组件可以正确渲染 Markdown
- [ ] Markdown 渲染样式美观，与整体 UI 一致
- [ ] Homepage 组件布局合理，包含文档浏览板块
- [ ] 文档侧边栏导航功能正常
- [ ] 点击文档可以正确显示内容

## 集成阶段
- [ ] 导航栏包含 Homepage 选项
- [ ] 可以通过导航访问 Homepage
- [ ] 现有功能（对话、记忆等）正常工作
- [ ] 没有引入新的错误或警告

## 测试阶段
- [ ] 整个功能流程测试通过
- [ ] 性能和用户体验良好
- [ ] 代码符合项目规范
- [ ] TypeScript 类型检查通过
