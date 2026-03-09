# 快速入门

本指南将帮助您快速开始使用 OpenCrab。

## 前置要求

在开始之前，请确保您的系统已安装以下软件：

- **Rust** (最新稳定版)
- **Node.js** (v18 或更高版本)
- **Git**

## 第一步：获取代码

克隆 OpenCrab 仓库到本地：

```bash
git clone https://github.com/your-org/opencrab.git
cd opencrab
```

## 第二步：安装依赖

### 后端依赖

```bash
cd backends/opencrab
cargo build
```

### 前端依赖

```bash
cd frontends/oh-my-crab
npm install
```

## 第三步：配置 OpenCrab

1. 复制配置文件模板：

```bash
cp config.example.toml config.toml
```

2. 编辑 `config.toml` 文件，配置您的 AI 模型提供商和其他设置。

## 第四步：启动服务

### 启动后端服务

```bash
cd backends/opencrab
cargo run
```

后端服务将默认在 `http://localhost:3000` 启动。

### 启动前端应用

在新的终端窗口中：

```bash
cd frontends/oh-my-crab
npm run dev
```

前端应用将在 `http://localhost:5173` 启动。

## 第五步：开始使用

1. 在浏览器中打开 `http://localhost:5173`
2. 创建您的第一个智能体
3. 开始与 AI 助手对话！

## 下一步

- 阅读 [核心特性](./features.md) 了解 OpenCrab 的更多功能
- 查看 [概念文档](../concepts/index.md) 深入理解核心概念
- 探索 [维护者文档](../maintainer/index.md) 了解技术细节

## 常见问题

### Q: 如何切换 AI 模型提供商？

A: 在 `config.toml` 文件中修改 `ai.provider` 配置项即可。

### Q: 数据存储在哪里？

A: OpenCrab 默认使用 SQLite 数据库，数据存储在本地文件系统中。

### Q: 如何添加自定义工具？

A: 参考 [工具系统](../concepts/tool.md) 文档了解如何扩展工具系统。
