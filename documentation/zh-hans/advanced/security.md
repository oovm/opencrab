# 安全性

本文档深入介绍 OpenCrab 的安全架构、威胁模型和最佳安全实践。

## 概述

OpenCrab 从设计之初就将安全性作为核心原则，采用零信任架构、端到端加密、权限最小化等现代安全最佳实践，为用户提供企业级的安全保障。

## 安全设计原则

### 1. 零信任架构

不假设任何参与方是可信的，包括：
- 不相信服务节点
- 不相信网络环境
- 不相信其他用户
- 验证一切，信任极少

### 2. 权限最小化

每个组件和用户只拥有完成任务所需的最小权限：
- 工具调用需要明确授权
- 文件操作限制在工作区内
- 网络访问需要显式许可

### 3. 纵深防御

多层安全机制，任何一层被突破都不会导致完全失效：
- 传输层加密
- 应用层验证
- 数据层保护
- 审计日志记录

### 4. 透明安全

安全机制对用户可见且可审计：
- 所有操作有日志记录
- 权限变更可追溯
- 安全事件可监控

---

## Skynet 协议安全

Skynet 协议是 OpenCrab 生态系统的通信基础设施，其安全性是整个系统安全的基础。

### 密码学原语选择

| 用途 | 算法 | 说明 |
|------|------|------|
| 数字签名 | Ed25519 | 高效安全的椭圆曲线签名 |
| 密钥交换 | X25519 | 高效的 Diffie-Hellman 密钥交换 |
| 哈希 | Blake3-256 | 高性能加密哈希 |
| 对称加密 | ChaCha20-Poly1305 | 认证加密，软件实现高效 |
| Noise 协议 | Noise_XX_25519_ChaChaPoly_BLAKE2s | 轻量级握手协议 |

### 身份认证

每个节点和用户都拥有唯一的 Ed25519 密钥对：

```
NodeID = blake3(public_key)
UserID = blake3(public_key)
```

所有操作都需要用私钥签名，接收方验证签名后才接受。

### 端到端加密

#### 私聊加密（X3DH + Double Ratchet）

1. **X3DH 密钥协商**：建立初始会话密钥
2. **Double Ratchet**：消息密钥前向保密

```
发送方:
1. 获取接收方的预密钥
2. 生成临时密钥对
3. 计算共享密钥
4. 使用 ChaCha20-Poly1305 加密消息
5. 发送加密消息和密钥材料

接收方:
1. 使用私钥计算共享密钥
2. 解密消息
3. 更新棘轮状态
```

#### 群聊加密（MLS）

使用 MLS（Messaging Layer Security）协议提供高效的群组加密：
- 支持大群组成员
- 成员加入/离开高效
- 前向保密和后向保密

### 威胁模型（STRIDE）

详见 [Skynet 协议威胁模型](../maintainer/skynet/threat-model.md)

---

## 核心层安全

### 智能体权限控制

每个智能体都有明确的权限边界：

```rust
pub struct AgentPermissions {
    file_access: FileAccessMode,
    network_access: NetworkAccessMode,
    tool_execution: ToolExecutionMode,
    memory_access: MemoryAccessMode,
}

pub enum FileAccessMode {
    None,
    ReadOnly,
    ReadWrite(PathBuf),
}

pub enum NetworkAccessMode {
    None,
    Allowlist(Vec<Url>),
    All,
}
```

### 工具调用安全

工具调用前进行多层验证：

1. **参数校验**：验证工具参数的类型和范围
2. **权限检查**：确认智能体有调用该工具的权限
3. **沙箱执行**：在受限环境中执行工具
4. **结果审计**：记录工具调用和返回结果

### 工作区隔离

每个工作区都是独立的安全边界：

- 智能体只能在授权的工作区内操作
- 路径校验防止路径穿越攻击
- 工作区间数据传输需要显式授权

```rust
fn validate_path(workspace_root: &Path, path: &Path) -> Result<PathBuf> {
    let canonical = workspace_root.join(path).canonicalize()?;
    if !canonical.starts_with(workspace_root) {
        return Err(Error::PathTraversal);
    }
    Ok(canonical)
}
```

---

## 数据安全

### 静态数据加密

敏感数据在存储前进行加密：

- 数据库字段加密
- 文件内容加密
- 记忆加密

```rust
pub struct EncryptedStorage {
    inner: Box<dyn StorageBackend>,
    encryption_key: [u8; 32],
}

impl EncryptedStorage {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut rand::thread_rng());
        let cipher = ChaCha20Poly1305::new(&self.encryption_key.into());
        let ciphertext = cipher.encrypt(&nonce, data)?;
        Ok([nonce.as_slice(), &ciphertext].concat())
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(&data[..12])?;
        let ciphertext = &data[12..];
        let cipher = ChaCha20Poly1305::new(&self.encryption_key.into());
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }
}
```

### 密钥管理

密钥管理遵循最佳实践：

- 使用密钥派生函数（Argon2）从密码派生密钥
- 密钥分离：不同用途使用不同密钥
- 密钥轮换：定期更换密钥
- 密钥备份：安全备份恢复密钥

---

## 审计和日志

### 结构化日志

所有安全相关操作都有结构化日志：

```rust
pub struct SecurityEvent {
    timestamp: DateTime<Utc>,
    event_type: SecurityEventType,
    actor: ActorId,
    resource: ResourceId,
    action: Action,
    result: Result<(), ErrorCode>,
    metadata: HashMap<String, serde_json::Value>,
}

pub enum SecurityEventType {
    Authentication,
    Authorization,
    ToolExecution,
    FileAccess,
    NetworkAccess,
    PermissionChange,
}
```

### 日志完整性

日志使用 Merkle 树确保完整性：

```
日志条目1 → 哈希1 ──┐
                     ├─ 根哈希
日志条目2 → 哈希2 ──┘
```

任何日志篡改都会被检测到。

---

## 插件安全

### 插件沙箱

第三方插件在受限环境中运行：

- WASM 沙箱隔离
- 资源使用限制（CPU、内存）
- API 调用白名单
- 网络访问限制

### 插件签名

所有官方插件都经过数字签名：

```rust
fn verify_plugin_signature(plugin: &Plugin, signature: &[u8]) -> Result<()> {
    let public_key = load_official_public_key()?;
    let plugin_hash = blake3::hash(plugin.code());
    public_key.verify(plugin_hash.as_bytes(), signature)?;
    Ok(())
}
```

---

## 部署安全

### 服务器安全

- 使用最小化容器镜像
- 定期更新依赖和补丁
- 禁用不必要的服务
- 使用防火墙限制网络访问

### 传输安全

- 强制使用 TLS 1.3
- HSTS（HTTP Strict Transport Security）
- 证书 pinned
- 定期轮换证书

### 数据库安全

- 数据库用户权限最小化
- 数据库连接加密
- 定期备份
- 敏感字段加密

---

## 最佳安全实践

### 用户端

1. **使用强密码**：密码长度至少 16 位，包含多种字符
2. **启用 2FA**：使用双因素认证
3. **定期备份**：备份密钥和重要数据
4. **审查权限**：定期审查应用权限
5. **更新软件**：保持 OpenCrab 和依赖库最新

### 开发者

1. **输入验证**：验证所有外部输入
2. **错误处理**：不泄露敏感信息到错误消息
3. **依赖审计**：定期审计第三方依赖
4. **安全测试**：进行渗透测试和代码审计
5. **文档安全**：不记录敏感信息到文档

### 管理员

1. **访问控制**：最小权限原则
2. **监控告警**：设置安全事件告警
3. **日志审计**：定期审查安全日志
4. **应急响应**：制定安全事件响应计划
5. **合规性**：遵守相关法律法规

---

## 安全更新

### 漏洞报告

发现安全漏洞请通过安全邮箱报告，不要公开披露。

### 安全公告

重要安全更新会通过以下渠道发布：
- 项目 GitHub Releases
- 安全公告邮件列表
- 官方社交媒体账号

---

## 总结

OpenCrab 通过多层安全机制提供企业级安全保障：

1. **Skynet 协议**：零信任、端到端加密、身份认证
2. **核心层**：权限控制、工具安全、工作区隔离
3. **数据安全**：静态加密、密钥管理
4. **审计日志**：结构化日志、完整性保护
5. **插件安全**：沙箱、签名验证
6. **部署安全**：服务器、传输、数据库安全

遵循最佳安全实践，用户可以安全地使用 OpenCrab 进行各种任务。
