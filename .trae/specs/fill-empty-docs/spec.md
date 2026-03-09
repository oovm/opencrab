# OpenCrab 文档回填 - Product Requirement Document

## Overview
- **Summary**: 回填 OpenCrab 文档中目前空缺的目录，包括 concepts/、overview/、tutorials/、tutorials/use-cases/ 和 advanced/，参考 OpenClaw 的设计理念以及相关项目的概念，补充完整的 OpenCrab 文档体系。
- **Purpose**: 完善 OpenCrab 的文档体系，确保用户能够全面理解 OpenCrab 的核心概念、使用方法和进阶特性。
- **Target Users**: OpenCrab 的开发者、用户和维护者。

## Goals
- 创建完整的 OpenCrab 核心概念文档 (concepts/)
- 创建 OpenCrab 概述文档 (overview/)
- 创建 OpenCrab 教程文档 (tutorials/)
- 创建 OpenCrab 使用场景文档 (tutorials/use-cases/)
- 创建 OpenCrab 进阶主题文档 (advanced/)

## Non-Goals (Out of Scope)
- 不创建与 AI Company 相关的内容（这些已移动到 ai-company 项目）
- 不重复维护者文档 (maintainer/) 中已有的内容
- 不涉及具体代码实现细节（留给维护者文档）

## Background & Context
- OpenCrab 是一个单机智能体节点系统，是 OpenClaw 生态系统的基础节点实现
- 之前的文档整理将 AI Company 相关的内容移动到了 ai-company 项目，导致 OpenCrab 的一些文档目录空缺
- 需要参考 OpenClaw 的设计理念以及相关项目的概念，补充适合 OpenCrab 的文档内容
- 可以提到 OpenClaw，但不能提到 ironclaw

## Functional Requirements
- **FR-1**: 提供 OpenCrab 核心概念的清晰解释
- **FR-2**: 提供 OpenCrab 的项目概述和快速入门
- **FR-3**: 提供 OpenCrab 的使用教程
- **FR-4**: 提供 OpenCrab 的使用场景示例
- **FR-5**: 提供 OpenCrab 的进阶主题文档

## Non-Functional Requirements
- **NFR-1**: 文档语言为中文
- **NFR-2**: 文档结构清晰，易于导航
- **NFR-3**: 内容准确反映 OpenCrab 的设计理念
- **NFR-4**: 文档风格与现有维护者文档保持一致

## Constraints
- **Technical**: 必须参考现有维护者文档的风格和内容
- **Business**: 必须在 OpenCrab 项目范围内，不能涉及 AI Company 特有内容
- **Dependencies**: 依赖现有的维护者文档 (maintainer/) 作为技术参考

## Assumptions
- OpenCrab 的核心设计理念与 OpenClaw 一致
- 可以参考相关项目的概念，但必须重新组织为适合 OpenCrab 的内容
- 文档主要面向中文用户

## Acceptance Criteria

### AC-1: 核心概念文档完整
- **Given**: 用户需要了解 OpenCrab 的核心概念
- **When**: 用户访问 concepts/ 目录
- **Then**: 能够找到完整的 OpenCrab 核心概念解释，包括智能体、技能、能力、记忆、工具等
- **Verification**: `human-judgment`

### AC-2: 概述文档完整
- **Given**: 用户需要快速了解 OpenCrab
- **When**: 用户访问 overview/ 目录
- **Then**: 能够找到 OpenCrab 的项目介绍、快速入门和核心特点
- **Verification**: `human-judgment`

### AC-3: 教程文档完整
- **Given**: 用户需要学习如何使用 OpenCrab
- **When**: 用户访问 tutorials/ 目录
- **Then**: 能够找到逐步的使用教程，包括基础设置、智能体配置等
- **Verification**: `human-judgment`

### AC-4: 使用场景文档完整
- **Given**: 用户需要了解 OpenCrab 的实际应用
- **When**: 用户访问 tutorials/use-cases/ 目录
- **Then**: 能够找到多种使用场景的示例，包括个人助手、开发辅助等
- **Verification**: `human-judgment`

### AC-5: 进阶文档完整
- **Given**: 用户需要深入了解 OpenCrab 的高级特性
- **When**: 用户访问 advanced/ 目录
- **Then**: 能够找到进阶主题文档，包括扩展性、安全性等
- **Verification**: `human-judgment`

### AC-6: 文档索引更新
- **Given**: 所有文档已创建完成
- **When**: 用户访问主文档索引
- **Then**: 主索引中包含新创建文档的链接
- **Verification**: `human-judgment`

## Open Questions
- [ ] 是否需要创建英文版本的文档？
- [ ] 文档的详细程度应该如何把握？
