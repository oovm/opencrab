# 入门教程

本教程将详细介绍如何从零开始设置和运行 OpenCrab，让您快速拥有一个属于自己的 AI 智能体。

## 前置准备

在开始之前，请确保您的系统已安装以下软件：

- **Rust** (最新稳定版) - 用于编译和运行后端
- **Node.js** (v18 或更高版本) - 用于运行前端
- **Git** - 用于获取代码

### 安装 Rust

访问 [Rust 官网](https://www.rust-lang.org/tools/install) 下载并安装 Rust。

安装完成后，验证安装：
```bash
rustc --version
cargo --version
```

### 安装 Node.js

访问 [Node.js 官网](https://nodejs.org/) 下载并安装 LTS 版本。

安装完成后，验证安装：
```bash
node --version
npm --version
```

## 第一步：获取代码

克隆 OpenCrab 仓库到本地：

```bash
git clone https://github.com/your-org/opencrab.git
cd opencrab
```

## 第二步：安装依赖

### 安装后端依赖

进入后端目录并构建项目：

```bash
cd backends/opencrab
cargo build
```

这可能需要几分钟时间，取决于您的网络速度和计算机性能。

### 安装前端依赖

进入前端目录并安装依赖：

```bash
cd ../../frontends/oh-my-crab
npm install
```

## 第三步：配置 OpenCrab

### 复制配置文件

首先，找到配置文件模板并复制：

```bash
cd ../../backends/opencrab
# 如果有配置示例文件，复制它
cp config.example.toml config.toml 2>/dev/null || echo "配置示例文件不存在，将使用默认配置"
```

### 配置 AI 模型

编辑 `config.toml` 文件（如果没有该文件，OpenCrab 会在首次运行时创建默认配置）。

主要配置项包括：

```toml
[ai]
provider = "openai"  # 或其他支持的提供商
api_key = "your-api-key-here"
model = "gpt-4"

[server]
host = "127.0.0.1"
port = 3000

[database]
path = "./data/opencrab.db"
```

**提示**：您可以先不配置 API 密钥，OpenCrab 会使用内置的模拟模式运行。

## 第四步：启动服务

### 启动后端服务

在 `backends/opencrab` 目录下运行：

```bash
cargo run
```

后端服务启动后，您会看到类似以下的输出：
```
Server listening on http://127.0.0.1:3000
```

### 启动前端应用

打开一个新的终端窗口，进入前端目录：

```bash
cd frontends/oh-my-crab
npm run dev
```

前端应用启动后，您会看到类似以下的输出：
```
VITE v5.x.x  ready in xxx ms

➜  Local:   http://localhost:5173/
```

## 第五步：创建您的第一个智能体

1. 在浏览器中打开 `http://localhost:5173`
2. 点击"创建智能体"按钮
3. 填写智能体信息：
   - **名称**：给您的智能体起个名字，比如"小助手"
   - **描述**：简单描述这个智能体的用途
   - **角色**：选择一个预设角色，或自定义
4. 点击"创建"

恭喜！您现在拥有了自己的第一个 OpenCrab 智能体！

## 第六步：开始对话

1. 在智能体列表中点击刚创建的智能体
2. 在输入框中输入您的问题，比如："你好，介绍一下你自己"
3. 点击发送按钮或按 Enter 键

智能体会根据您的配置进行回复。

## 常见问题

### Q: 后端启动失败怎么办？

A: 请检查：
1. Rust 是否已正确安装
2. 端口 3000 是否被占用
3. 查看终端输出的错误信息

### Q: 前端无法连接后端？

A: 请确保：
1. 后端服务正在运行
2. 后端地址配置正确（默认 http://localhost:3000）
3. 没有防火墙阻止连接

### Q: 如何切换 AI 模型？

A: 编辑 `config.toml` 文件中的 `ai.model` 配置项，然后重启后端服务。

### Q: 数据存储在哪里？

A: 默认情况下，数据存储在 `backends/opencrab/data/` 目录下的 SQLite 数据库文件中。

## 下一步

现在您已经成功运行了 OpenCrab，接下来可以：

- 学习如何 [配置智能体](./configure-agent.md)
- 了解如何 [添加技能](./add-skills.md)
- 探索如何 [使用工具](./use-tools.md)
