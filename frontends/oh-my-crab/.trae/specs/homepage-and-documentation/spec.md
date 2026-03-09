# Homepage 和文档迁移 - 产品需求文档

## Overview
- **Summary**: 将项目文档从 VitePress 框架迁移到纯 Markdown，并在前端添加一个独立的 Homepage 页面，将文档作为 Homepage 的一个板块。
- **Purpose**: 简化项目结构，统一使用 Vue + Element Plus + TailwindCSS 技术栈，移除对 VitePress 和 Vutex 的依赖。
- **Target Users**: 项目开发者、维护者以及使用 OpenCrab 的最终用户。

## Goals
- 移除 documentation 目录下的 VitePress 和 Vutex 配置及依赖
- 在 oh-my-crab 前端项目中添加独立的 Homepage 页面
- 将文档作为 Homepage 的一个功能板块，支持 Markdown 渲染
- 善用 import.meta.glob 实现文档的动态加载
- 保持技术栈的一致性（Vite + Vue 3 + Element Plus + TailwindCSS）

## Non-Goals (Out of Scope)
- 不创建新的独立前端项目，将 Homepage 集成到现有的 oh-my-crab 项目中
- 不重写现有文档内容，仅调整文档组织方式
- 不实现复杂的文档搜索和高级导航功能（MVP 版本）

## Background & Context
- 当前 documentation 目录使用 VitePress 构建，包含多语言支持
- frontends 目录下的 oh-my-crab 项目已经使用 Vue 3 + Element Plus + TailwindCSS 技术栈
- 项目希望统一技术栈，减少框架依赖
- oh-my-crab 项目已安装 marked 库，可用于 Markdown 渲染

## Functional Requirements
- **FR-1**: 清理 documentation 目录，移除 VitePress 相关文件，保留纯 Markdown 文档
- **FR-2**: 在 oh-my-crab 项目中创建 Homepage 组件
- **FR-3**: Homepage 包含文档浏览板块，支持文档导航和渲染
- **FR-4**: 使用 import.meta.glob 动态加载 Markdown 文档
- **FR-5**: 将 Homepage 集成到 oh-my-crab 的路由或导航系统中

## Non-Functional Requirements
- **NFR-1**: 文档加载要快速，避免不必要的延迟
- **NFR-2**: 代码结构清晰，易于维护和扩展
- **NFR-3**: Markdown 渲染要美观，与整体 UI 风格一致
- **NFR-4**: 保持现有 oh-my-crab 功能不受影响

## Constraints
- **Technical**: 必须使用现有的技术栈（Vite + Vue 3 + Element Plus + TailwindCSS），必须使用 marked 进行 Markdown 渲染，不引入新的依赖
- **Business**: 保持现有文档内容不变，仅调整组织方式
- **Dependencies**: 依赖 oh-my-crab 项目的现有结构和配置

## Assumptions
- Markdown 文档可以直接从 documentation 目录复制到 oh-my-crab 项目中或通过相对路径访问
- 用户希望 Homepage 作为 oh-my-crab 的一部分，而不是独立项目
- 使用 import.meta.glob 可以实现文档的静态导入和打包

## Acceptance Criteria

### AC-1: 清理 documentation 目录
- **Given**: documentation 目录包含 VitePress 和 Vutex 配置
- **When**: 执行清理操作
- **Then**: documentation 目录下只保留纯 Markdown 文档和基本的 README
- **Verification**: `programmatic`

### AC-2: Homepage 组件创建
- **Given**: oh-my-crab 项目已存在
- **When**: 创建 Homepage 组件
- **Then**: Homepage 组件可以正常渲染，包含基本的布局和内容
- **Verification**: `human-judgment`

### AC-3: 文档浏览功能
- **Given**: Homepage 组件已创建
- **When**: 用户访问文档板块
- **Then**: 可以看到文档目录结构并浏览文档内容
- **Verification**: `human-judgment`

### AC-4: Markdown 渲染
- **Given**: Markdown 文档已加载
- **When**: 用户选择文档查看
- **Then**: Markdown 内容被正确渲染为 HTML，样式美观
- **Verification**: `human-judgment`

### AC-5: 集成到 oh-my-crab
- **Given**: Homepage 已完成
- **When**: 用户启动 oh-my-crab
- **Then**: 可以通过导航访问 Homepage
- **Verification**: `human-judgment`

## Open Questions
- [ ] 文档是直接复制到 oh-my-crab 项目中，还是通过相对路径从 documentation 目录访问？
- [ ] 是否需要支持多语言文档切换？
- [ ] Homepage 的具体布局和设计风格有什么要求？
