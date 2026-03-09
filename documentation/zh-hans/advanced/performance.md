# 性能优化

本文档深入介绍 OpenCrab 的性能优化策略，包括资源管理、缓存策略、并发处理和调优技巧。

## 概述

OpenCrab 从设计之初就将性能作为重要考虑因素。虽然安全性和可靠性优先，但通过合理的架构设计和优化技术，OpenCrab 能够在保证安全的同时提供出色的性能。

## 性能设计原则

### 1. 零抽象开销

在性能关键路径上保持零抽象：
- 使用 Rust 的零成本抽象
- 避免不必要的堆分配
- 内联热点代码

### 2. 异步优先

所有 I/O 操作都是异步的：
- 使用 Tokio 异步运行时
- 非阻塞网络操作
- 异步文件 I/O

### 3. 资源池化

复用昂贵资源：
- 数据库连接池
- HTTP 客户端连接池
- 线程池

### 4. 智能缓存

多层缓存策略：
- 内存缓存（高频访问）
- 本地缓存（中频访问）
- 分布式缓存（低频访问）

---

## 异步架构

### Tokio 运行时配置

OpenCrab 使用 Tokio 作为异步运行时，针对不同场景进行优化：

```rust
use tokio::runtime::{Builder, Runtime};

fn create_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name("opencrab-worker")
        .thread_stack_size(2 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
}
```

### 任务调度优化

- **计算密集型任务**：使用 `spawn_blocking` 或专用线程池
- **I/O 密集型任务**：使用常规异步任务
- **优先级任务**：使用优先级队列

```rust
// 计算密集型任务
tokio::task::spawn_blocking(|| {
    heavy_computation();
});

// 优先级队列
use priority_queue::PriorityQueue;

let mut queue = PriorityQueue::new();
queue.push(high_priority_task, Priority::High);
queue.push(low_priority_task, Priority::Low);
```

---

## 内存管理

### 减少堆分配

- 使用 `&str` 而非 `String` 当数据不需要所有权时
- 使用 `Cow<'a, T>` 减少克隆
- 使用 `SmallVec` 处理小集合

```rust
use std::borrow::Cow;
use smallvec::SmallVec;

fn process_data(data: &str) -> Cow<str> {
    if data.is_empty() {
        Cow::Borrowed("default")
    } else {
        Cow::Owned(data.to_uppercase())
    }
}

fn small_collection() -> SmallVec<[u8; 16]> {
    let mut vec = SmallVec::new();
    vec.extend_from_slice(&[1, 2, 3, 4]);
    vec
}
```

### 内存池

对于频繁分配释放的对象，使用内存池：

```rust
use object_pool::Pool;

struct Message {
    content: Vec<u8>,
    timestamp: u64,
}

let pool = Pool::new(1024, || Message {
    content: Vec::with_capacity(4096),
    timestamp: 0,
});

{
    let mut msg = pool.pull(|| Message {
        content: Vec::with_capacity(4096),
        timestamp: 0,
    });
    msg.content.extend_from_slice(b"hello");
    msg.timestamp = now();
    process_message(&msg);
}
```

### 内存监控

监控内存使用情况，及时发现泄漏：

```rust
use tracing::info;
use sysinfo::{System, SystemExt};

fn monitor_memory() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    if let Some(process) = sys.process(sysinfo::get_current_pid().unwrap()) {
        info!("Memory used: {} bytes", process.memory());
        info!("Virtual memory: {} bytes", process.virtual_memory());
    }
}
```

---

## 缓存策略

### 多层缓存架构

```
┌─────────────────────────────────────────┐
│         L1: 内存缓存 (μs 级)            │
│    - 高频访问数据                        │
│    - TTL: 秒级                          │
└──────────────┬──────────────────────────┘
               │ 未命中
┌──────────────▼──────────────────────────┐
│         L2: 本地缓存 (ms 级)            │
│    - 中频访问数据                        │
│    - TTL: 分钟级                        │
└──────────────┬──────────────────────────┘
               │ 未命中
┌──────────────▼──────────────────────────┐
│         L3: 分布式缓存 (ms 级)          │
│    - 低频访问数据                        │
│    - TTL: 小时级                        │
└──────────────┬──────────────────────────┘
               │ 未命中
┌──────────────▼──────────────────────────┐
│         源数据 (s 级)                   │
│    - 数据库、文件系统等                  │
└─────────────────────────────────────────┘
```

### L1 内存缓存实现

使用 `moka` 高性能缓存库：

```rust
use moka::sync::Cache;
use std::time::Duration;

struct MemoryCache<K, V> {
    inner: Cache<K, V>,
}

impl<K, V> MemoryCache<K, V>
where
    K: Send + Sync + 'static + std::hash::Hash + Eq,
    V: Send + Sync + 'static + Clone,
{
    fn new(max_capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(ttl)
            .build();
        MemoryCache { inner: cache }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        self.inner.get_or_insert_with(key, f).clone()
    }
}
```

### 缓存失效策略

- **TTL（Time To Live）**：基于时间失效
- **LRU（Least Recently Used）**：基于访问频率失效
- **主动失效**：数据更新时主动失效缓存

```rust
fn invalidate_cache(cache: &MemoryCache<Key, Value>, key: &Key) {
    cache.invalidate(key);
}

fn invalidate_pattern(cache: &MemoryCache<Key, Value>, pattern: &str) {
    cache.iter()
        .filter(|(k, _)| k.to_string().contains(pattern))
        .for_each(|(k, _)| cache.invalidate(&k));
}
```

---

## 数据库优化

### 连接池

使用 `deadpool` 或 `bb8` 数据库连接池：

```rust
use deadpool_postgres::{Config, Pool, Runtime};

fn create_db_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".into());
    cfg.dbname = Some("opencrab".into());
    cfg.user = Some("opencrab".into());
    cfg.password = Some("password".into());
    
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
```

### 查询优化

- 使用索引加速查询
- 避免 N+1 查询
- 使用批量操作
- 分页查询

```rust
// 使用索引
CREATE INDEX idx_memory_agent_id ON memory(agent_id);
CREATE INDEX idx_memory_timestamp ON memory(timestamp DESC);

// 批量插入
async fn batch_insert_memories(pool: &Pool, memories: &[Memory]) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client.prepare(
        "INSERT INTO memory (id, agent_id, content, timestamp) VALUES ($1, $2, $3, $4)"
    ).await?;
    
    for memory in memories {
        client.execute(&stmt, &[
            &memory.id,
            &memory.agent_id,
            &memory.content,
            &memory.timestamp,
        ]).await?;
    }
    Ok(())
}

// 分页查询
async fn query_memories_paginated(
    pool: &Pool,
    agent_id: &AgentId,
    page: i64,
    page_size: i64,
) -> Result<Vec<Memory>> {
    let client = pool.get().await?;
    let offset = (page - 1) * page_size;
    
    let rows = client.query(
        "SELECT id, agent_id, content, timestamp 
         FROM memory 
         WHERE agent_id = $1 
         ORDER BY timestamp DESC 
         LIMIT $2 OFFSET $3",
        &[agent_id, &page_size, &offset],
    ).await?;
    
    rows.into_iter().map(|row| Memory::from_row(row)).collect()
}
```

### 读写分离

对于读多写少的场景，使用读写分离：

```rust
struct DatabasePool {
    writer: Pool,
    readers: Vec<Pool>,
}

impl DatabasePool {
    async fn get_reader(&self) -> Pool {
        let index = rand::thread_rng().gen_range(0..self.readers.len());
        self.readers[index].clone()
    }

    async fn get_writer(&self) -> Pool {
        self.writer.clone()
    }
}
```

---

## 并发处理

### 并发控制

使用适当的并发原语：

```rust
use tokio::sync::{Mutex, RwLock, Semaphore};

// Mutex：独占访问
let data = Mutex::new(Vec::new());
{
    let mut guard = data.lock().await;
    guard.push(1);
}

// RwLock：多读单写
let config = RwLock::new(Config::default());
{
    let read_guard = config.read().await;
    println!("Config: {:?}", *read_guard);
}
{
    let mut write_guard = config.write().await;
    *write_guard = Config::new();
}

// Semaphore：限制并发数
let semaphore = Semaphore::new(10);
let _permit = semaphore.acquire().await?;
// 执行并发限制的操作
```

### 并行处理

对大量数据进行并行处理：

```rust
use futures::stream::{self, StreamExt};

async fn parallel_process<T, F, Fut>(items: Vec<T>, f: F, concurrency: usize)
where
    F: Fn(T) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
    T: Send,
{
    stream::iter(items)
        .for_each_concurrent(concurrency, |item| f(item))
        .await;
}

// 使用示例
let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
parallel_process(items, |i| async move {
    println!("Processing: {}", i);
    tokio::time::sleep(Duration::from_millis(100)).await;
}, 4).await;
```

---

## 网络优化

### HTTP 客户端优化

```rust
use reqwest::Client;
use std::time::Duration;

fn create_http_client() -> Client {
    Client::builder()
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}
```

### 连接复用

- HTTP/2 多路复用
- 连接池复用
- Keep-Alive

---

## 性能监控

### 指标收集

使用 `metrics` 库收集性能指标：

```rust
use metrics::{counter, histogram, timing};
use std::time::Instant;

fn record_metrics() {
    counter!("requests_total", 1);
    histogram!("request_duration_seconds", 0.5);
    
    let start = Instant::now();
    // 执行操作
    timing!("operation_duration_seconds", start.elapsed());
}
```

### 分布式追踪

使用 `tracing` 和 `opentelemetry` 进行分布式追踪：

```rust
use tracing::{info, instrument, span, Level};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::prelude::*;

#[instrument]
async fn process_request(request: Request) -> Response {
    info!("Processing request");
    
    let span = span!(Level::INFO, "database_query");
    let _enter = span.enter();
    let data = query_database().await;
    
    Response::new(data)
}
```

---

## 调优技巧

### 1. 分析瓶颈

使用性能分析工具找到瓶颈：

```bash
# CPU 分析
perf record -g target/release/opencrab
perf report

# 火焰图
perf record -F 99 -g -- target/release/opencrab
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

### 2. 配置调优

根据硬件配置调整参数：

```toml
# opencrab.toml
[performance]
worker_threads = 8
max_connections = 100
cache_size = 10000
batch_size = 1000
```

### 3. 编译优化

使用 Rust 的编译优化：

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

---

## 最佳实践

### 开发阶段

1. **基准测试**：为关键代码编写基准测试
2. **性能预算**：设定性能目标
3. **持续监控**：集成性能测试到 CI

### 部署阶段

1. **压力测试**：进行负载测试
2. **容量规划**：根据负载规划资源
3. **弹性伸缩**：支持自动扩缩容

### 运维阶段

1. **实时监控**：监控关键指标
2. **告警配置**：设置性能告警
3. **定期优化**：定期审查和优化

---

## 总结

OpenCrab 的性能优化策略包括：

1. **异步架构**：Tokio 运行时、非阻塞 I/O
2. **内存管理**：减少分配、内存池
3. **缓存策略**：多层缓存、智能失效
4. **数据库优化**：连接池、查询优化、读写分离
5. **并发处理**：并发控制、并行处理
6. **网络优化**：连接复用、HTTP/2
7. **性能监控**：指标收集、分布式追踪

通过合理应用这些优化技术，OpenCrab 能够在保证安全和可靠性的同时，提供出色的性能表现。
