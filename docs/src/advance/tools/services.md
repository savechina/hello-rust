# 服务框架

## 开篇故事

想象你在经营餐厅。单个厨师可以炒菜，但要有秩序地运营餐厅，你需要：前台接待、后厨管理、传菜员、收银员。服务框架就像餐厅管理系统——协调各个"服务"（组件）有序工作，处理顾客（请求）订单。

在 Rust 中，服务框架提供应用生命周期管理、依赖注入、错误处理等基础设施，让你专注于业务逻辑而非样板代码。

---

## 本章适合谁

如果你要构建中型到大型的 Rust 应用（Web 服务、微服务、后台进程），本章适合你。服务框架帮助你组织代码、管理依赖、处理错误。

---

## 你会学到什么

完成本章后，你可以：

1. 理解服务框架的核心组件
2. 实现依赖注入模式
3. 管理服务生命周期
4. 设计可测试的服务架构

---

## 前置要求

学习本章前，你需要理解：

- [特征](../../basic/trait.md) - 理解 trait 定义
- [泛型](../../basic/generic.md) - 理解泛型语法
- [异步编程](../async/async.md) - 理解 async/await

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add thiserror
cargo add async-trait
```

## 第一个例子

让我们看一个最简单的服务定义：

```rust,ignore
use async_trait::async_trait;

#[async_trait]
pub trait Service: Send + Sync {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn health_check(&self) -> bool;
}

pub struct MyService {
    name: String,
}

#[async_trait]
impl Service for MyService {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("启动服务：{}", self.name);
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("停止服务：{}", self.name);
        Ok(())
    }

    async fn health_check(&self) -> bool {
        true
    }
}
```

**发生了什么？**

- `Service` trait 定义服务接口
- `start()` / `stop()`: 生命周期管理
- `health_check()`: 健康检查

---

## 原理解析

### 1. 服务生命周期

```rust,ignore
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

pub struct ServiceManager {
    state: RwLock<ServiceState>,
    services: Vec<Box<dyn Service>>,
}

impl ServiceManager {
    pub async fn start_all(&self) -> Result<(), Error> {
        *self.state.write().await = ServiceState::Starting;
        
        for service in &self.services {
            service.start().await?;
        }
        
        *self.state.write().await = ServiceState::Running;
        Ok(())
    }
}
```

**状态转换**：
```
Stopped → Starting → Running → Stopping → Stopped
```

### 2. 依赖注入模式

```rust,ignore
// 定义依赖 trait
#[async_trait]
pub trait Database: Send + Sync {
    async fn query(&self, sql: &str) -> Result<Vec<Row>>;
}

// 服务持有依赖
pub struct UserService<DB: Database> {
    db: DB,
}

impl<DB: Database> UserService<DB> {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
    
    pub async fn get_user(&self, id: i32) -> Result<User> {
        self.db.query("SELECT * FROM users WHERE id = $1").await
    }
}

// 组合服务
pub struct App {
    user_service: UserService<PgDatabase>,
}
```

**优势**：
- 服务解耦
- 易于测试（可注入 Mock）
- 依赖显式声明

### 3. 健康检查模式

```rust,ignore
pub struct HealthStatus {
    pub service: String,
    pub healthy: bool,
    pub message: String,
}

#[async_trait]
pub trait HealthCheck: Send + Sync {
    async fn health(&self) -> HealthStatus;
}

impl HealthCheck for App {
    async fn health(&self) -> HealthStatus {
        let db_healthy = self.user_service.db.ping().await.is_ok();
        
        HealthStatus {
            service: "app".to_string(),
            healthy: db_healthy,
            message: if db_healthy { 
                "OK".to_string() 
            } else { 
                "Database connection failed".to_string() 
            },
        }
    }
}
```

### 4. 错误处理策略

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("启动失败：{0}")]
    StartupFailed(#[from] std::io::Error),
    
    #[error("依赖缺失：{0}")]
    DependencyMissing(String),
    
    #[error("健康检查失败：{0}")]
    HealthCheckFailed(String),
}

#[async_trait]
impl Service for MyService {
    async fn start(&self) -> Result<(), ServiceError> {
        // 具体实现
        Ok(())
    }
}
```

---

## 常见错误

### 错误 1: 循环依赖

```rust,ignore
// ❌ 错误：A 依赖 B，B 依赖 A
struct ServiceA { b: ServiceB }
struct ServiceB { a: ServiceA }

// ✅ 正确：使用 trait 或事件解耦
trait EventHandler {
    fn handle(&self, event: Event);
}

struct ServiceA<E: EventHandler> { handler: E }
struct ServiceB { /* 不持有 ServiceA */ }
```

### 错误 2: 忘记清理资源

```rust,ignore
// ❌ 错误：stop() 为空
async fn stop(&self) -> Result<(), Error> {
    // 忘记关闭数据库连接
}

// ✅ 正确：清理所有资源
async fn stop(&self) -> Result<(), Error> {
    self.db.close().await?;
    self.cache.clear().await;
    Ok(())
}
```

### 错误 3: 阻塞异步服务

```rust,ignore
// ❌ 错误：在异步服务中同步阻塞
async fn process(&self) {
    std::thread::sleep(Duration::from_secs(1));  // 阻塞
}

// ✅ 正确：使用异步等待
async fn process(&self) {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

---

## 动手练习

### 练习 1: 实现日志服务

创建服务框架：

```rust,ignore
#[async_trait]
pub trait Logger: Send + Sync {
    async fn log(&self, message: &str);
    async fn flush(&self);
}

// TODO: 实现 ConsoleLogger 和 FileLogger
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
pub struct ConsoleLogger;

#[async_trait]
impl Logger for ConsoleLogger {
    async fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
    
    async fn flush(&self) {
        // 控制台不需要 flush
    }
}

pub struct FileLogger {
    file: tokio::fs::File,
}

#[async_trait]
impl Logger for FileLogger {
    async fn log(&self, message: &str) {
        use tokio::io::AsyncWriteExt;
        let mut file = self.file.try_clone().await.unwrap();
        file.write_all(message.as_bytes()).await.unwrap();
    }
    
    async fn flush(&self) {
        // 刷新文件缓冲区
    }
}
```

**解析**: 不同日志实现可以互换，符合依赖注入原则。
</details>

---

## 故障排查

### Q: 何时使用服务框架？

**A**: 当应用满足以下条件时：
- 多个组件需要协调
- 需要统一的生命周期管理
- 依赖关系复杂
- 需要可测试性

### Q: 服务框架的性能开销？

**A**: 
- Trait 对象有轻微动态分发开销
- 依赖注入增加间接层
- 但对于业务逻辑，开销可忽略

### Q: 如何测试服务？

**A**: 使用 Mock 实现：
```rust,ignore
struct MockDatabase;

#[async_trait]
impl Database for MockDatabase {
    async fn query(&self, _sql: &str) -> Result<Vec<Row>> {
        Ok(vec![])  // 返回测试数据
    }
}
```

---

## 小结

**核心要点**：

1. **服务接口**: 定义统一的 start/stop/health 方法
2. **依赖注入**: 通过 trait 解耦服务
3. **生命周期**: 管理服务的启动和停止
4. **健康检查**: 提供服务状态监控

**关键术语**：

- **Service**: 服务
- **Dependency Injection**: 依赖注入
- **Lifecycle**: 生命周期
- **Health Check**: 健康检查
- **Trait Object**: 特征对象
- **Mock**: 模拟

**下一步**：

- 学习 [依赖注入](../../awesome/dependency_injection.md)
- 探索 [插件系统](../../awesome/plugin.md)
- 理解 [错误处理](../error-handling/error-handling.md)

---

## 术语表

| English            | 中文       |
| ------------------ | ---------- |
| Service            | 服务       |
| Dependency         | 依赖       |
| Injection          | 注入       |
| Lifecycle          | 生命周期   |
| Health Check       | 健康检查   |
| Trait Object       | 特征对象   |
| Mock               | 模拟       |

---

完整示例：`src/advance/tools/services_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. 服务生命周期的三个阶段是什么？

2. 依赖注入的作用是什么？

3. 如何测试使用依赖注入的服务？

<details>
<summary>点击查看答案与解析</summary>

1. 启动 (Start) → 运行 (Running) → 停止 (Stop)
2. 解耦服务，使组件可替换、可测试
3. 注入 Mock 依赖，验证服务行为

**关键理解**: 好的服务框架让复杂应用变得简单。
</details>

## 继续学习

- 下一步：[依赖注入](../../awesome/dependency_injection.md)
- 进阶：[插件系统](../../awesome/plugin.md)
- 回顾：[特征对象](../../basic/trait-objects.md)

> 💡 **记住**：好的服务框架让复杂应用变得简单。定义清晰的接口，注入依赖，管理生命周期！
