# 服务框架与微服务架构

## 开篇故事

想象一下，你正在经营一家大型连锁餐厅。最初只有一家店，你自己就能管理所有的事情——采购、烹饪、服务、收银。但随着业务扩大，你开了十家店、一百家店，你不可能亲自管理每一家店的每个环节。

于是你设计了这样的架构：
- **中央厨房**（服务注册中心）统一协调所有分店的食材供应
- **各分店**（微服务）专注于自己的业务：有的专做寿司、有的专做披萨、有的专做甜品
- **服务员**（gRPC 客户端）知道如何找到对应的分店下单
- **经理**（服务框架）负责监控每个分店的运营状态，在出现问题时及时调配资源

在软件世界里，这就是微服务架构——将大型应用拆分为多个独立部署、独立扩展的小型服务。`awesome` crate 中的服务框架就是一套生产级的"餐厅管理系统"，它包含了服务发现（Consul）、远程调用（gRPC）、依赖注入（DI）、生命周期管理等核心组件。

---

## 本章适合谁

如果你已经掌握了 Rust 基础和异步编程，现在希望构建：
- 生产级的微服务应用
- 支持服务注册与发现的服务集群
- 基于 gRPC 的高性能 RPC 服务
- 具备依赖注入和插件机制的可扩展系统

本章适合你。微服务架构是现代分布式系统的主流选择，而这些知识将帮助你构建企业级应用。

---

## 你会学到什么

完成本章后，你可以：

1. 理解依赖注入（DI）的三种实现方式：具体类型注入、动态 trait 注入（Arc）、动态 trait 注入（Box）
2. 使用 `inventory` crate 实现编译时插件注册
3. 使用 Consul 进行服务注册、发现和配置管理
4. 使用 Tonic 编写 gRPC 服务端和客户端
5. 实现自定义服务生命周期框架，支持优雅启动和关闭
6. 构建支持流式数据传输的 gRPC 服务
7. 理解微服务架构中的健康检查和服务治理

---

## 前置要求

学习本章前，你需要理解：

- Tokio 异步运行时 - 特别是 `spawn`、`await`、通道的使用
- [所有权与生命周期](../basic/ownership.md) - 特别是 `Arc`、trait 对象的理解
- 泛型与 trait - 理解泛型约束和 trait 对象
- [线程与并发](../basic/threads.md) - 了解并发基本概念

本章涉及的 crate：
- `tonic` - gRPC 框架
- `consul` / 自定义 HTTP 客户端 - 服务注册与发现
- `inventory` - 编译时插件注册
- `async-trait` - 异步 trait
- `serde` - 配置序列化

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full
cargo add axum
cargo add async-trait
```

## 第一个例子

让我们从最简单的依赖注入开始：

```rust,ignore
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

// 定义 Repository Trait
trait UserRepository: Any + Send + Sync {
    fn get_user(&self, id: u32) -> String;
}

// 实现具体 Repository
struct InMemoryUserRepository;
impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: u32) -> String {
        format!("User {} from InMemoryRepo", id)
    }
}

// Service 依赖 Repository
struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(repo: R) -> Self {
        Self { repo }
    }
    fn greet_user(&self, id: u32) -> String {
        let user = self.repo.get_user(id);
        format!("Hello, {}!", user)
    }
}

fn main() {
    // 创建 DI 容器
    let mut services = HashMap::<TypeId, Arc<dyn Any + Send + Sync>>::new();
    
    // 注册 Repository
    services.insert(
        TypeId::of::<InMemoryUserRepository>(),
        Arc::new(InMemoryUserRepository),
    );
    
    // 注册 Service（注入依赖）
    services.insert(
        TypeId::of::<UserService<InMemoryUserRepository>>(),
        Arc::new(UserService::new(InMemoryUserRepository)),
    );
    
    // 从容器解析并调用
    let service = services
        .get(&TypeId::of::<UserService<InMemoryUserRepository>>())
        .unwrap()
        .clone()
        .downcast::<UserService<InMemoryUserRepository>>()
        .unwrap();
    
    println!("{}", service.greet_user(42));
}
```

完整示例：concrete_injection_sample.rs

---

## 原理解析

### 1. 依赖注入的三种模式

**模式一：具体类型注入（Concrete Injection）**

```rust,ignore
// 使用泛型参数，编译时确定类型
struct UserService<R: UserRepository> {
    repo: R,
}
```

优点：零运行时开销，编译时类型安全  
缺点：无法运行时切换实现，容器需要为每种组合存储单独的类型

完整示例：concrete_injection_sample.rs

---

**模式二：动态 Trait 注入（Arc 版本）**

```rust,ignore
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Service 使用 Arc<dyn Trait> 存储依赖
struct BusinessService {
    logger: Arc<dyn LoggerService>,
    database: Arc<dyn DatabaseService>,
}

// ServiceContainer 支持 trait 对象注册
struct ServiceContainer {
    services: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
    factories: Mutex<HashMap<TypeId, Box<dyn Fn(&ServiceContainer) -> Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceContainer {
    fn register_trait<T>(&self, service: Arc<T>)
    where
        T: ?Sized + Any + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<Arc<T>>();
        let service: Arc<dyn Any + Send + Sync> = Arc::new(service);
        self.services.lock().unwrap().insert(type_id, service);
    }
    
    fn resolve_trait<T>(&self) -> Option<Arc<T>>
    where
        T: ?Sized + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<Arc<T>>();
        self.services.lock().unwrap()
            .get(&type_id)
            .and_then(|s| s.clone().downcast::<Arc<T>>().ok())
            .map(|arc| (*arc).clone())
    }
}
```

优点：支持运行时切换实现，trait 对象共享所有权  
缺点：需要处理复杂的类型擦除和 downcast

完整示例：dynmaic_injection_arc_sample.rs

---

**模式三：动态 Trait 注入（Box 版本）**

与 Arc 版本类似，但使用 `Box<dyn Any>` 存储：

```rust,ignore
struct ServiceContainer {
    services: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl ServiceContainer {
    fn register<T: Sized + Any + Send + Sync + 'static>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        let service: Box<dyn Any + Send + Sync> = Box::new(service);
        self.services.lock().unwrap().insert(type_id, service);
    }
}
```

完整示例：dynmaic_injection_box_sample.rs

---

### 2. 编译时插件注册（Inventory）

```rust,ignore
use inventory::submit;

// 定义插件 trait
trait InventoryOp: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32);
}

// 插件注册结构
#[derive(Clone, Copy)]
struct InventoryPlugin {
    name: &'static str,
    handler: &'static dyn InventoryOp,
}

// 收集所有插件
inventory::collect!(InventoryPlugin);

// 注册插件
inventory::submit! {
    InventoryPlugin {
        name: "add",
        handler: &AddItem,
    }
}

// 运行时获取所有插件
fn main() {
    for plugin in inventory::iter::<InventoryPlugin> {
        println!("Loaded plugin: {}", plugin.name);
    }
}
```

完整示例：inventory_sample.rs

---

### 3. Consul 服务注册与发现

**服务注册（使用 rs_consul crate）：**

```rust,ignore
use rs_consul::{Config, Consul, RegisterEntityPayload, RegisterEntityService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        address: "http://127.0.0.1:8500".to_string(),
        ..Default::default()
    };
    let consul = Consul::new(config);
    
    // 注册服务
    let payload = RegisterEntityPayload {
        Node: "node-1".to_string(),
        Address: "192.168.1.100".to_string(),
        Service: Some(RegisterEntityService {
            Service: "my-service".to_string(),
            Port: Some(8080),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    consul.register_entity(&payload).await?;
    Ok(())
}
```

完整示例：consul_sample.rs

---

**自定义 Consul 客户端（生产环境）：**

```rust,ignore
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AgentServiceRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub check: Option<AgentServiceCheck>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CatalogServiceNode {
    pub service_name: String,
    pub service_address: String,
    pub service_port: u16,
}

pub struct ConsulClient {
    http_client: Client,
    consul_api_base_url: Url,
}

impl ConsulClient {
    pub async fn register_service(&self, registration: &AgentServiceRegistration) -> Result<()> {
        let url = self.consul_api_base_url.join("agent/service/register")?;
        self.http_client
            .put(url)
            .json(registration)
            .send()
            .await?;
        Ok(())
    }
    
    pub async fn discover_service(&self, service_name: &str) -> Result<Vec<CatalogServiceNode>> {
        let url = self.consul_api_base_url
            .join(&format!("catalog/service/{}", service_name))?;
        let nodes: Vec<CatalogServiceNode> = self.http_client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(nodes)
    }
}
```

完整代码：framework/registry.rs

---

### 4. gRPC 服务（Tonic）

**基础服务端：**

```rust,ignore
mod helloworld {
    tonic::include_proto!("helloworld");
}

use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    
    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
```

完整示例：tonic_hello_server.rs

---

**基础客户端：**

```rust,ignore
use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response.into_inner().message);
    Ok(())
}
```

完整示例：tonic_hello_client.rs

---

**流式 gRPC 服务：**

```rust,ignore
type WatchStream = Pin<Box<dyn Stream<Item = Result<Item, Status>> + Send>>;

#[tonic::async_trait]
impl Inventory for StoreInventory {
    async fn watch(
        &self,
        request: Request<ItemIdentifier>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let (tx, rx) = mpsc::unbounded_channel();
        let inventory = self.inventory.clone();
        let id = request.into_inner();
        
        // 后台任务监控库存变化
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                
                let map = inventory.lock().await;
                if let Some(item) = map.get(&id.sku) {
                    if tx.send(Ok(item.clone())).is_err() {
                        break;
                    }
                }
            }
        });
        
        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}
```

完整示例：tonic_store_server.rs  
客户端：tonic_store_client.rs

---

### 5. 服务生命周期框架

**架构概览：**

```
┌─────────────────────────────────────────────────────────────┐
│                 ApplicationFramework<S>                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  RunnableService                      │  │
│  │   ┌─────────┐   ┌─────────┐   ┌─────────────────┐    │  │
│  │   │ gRPC    │   │ HTTP    │   │ Health Check    │    │  │
│  │   │ Server  │   │ Server  │   │ Endpoint        │    │  │
│  │   └────┬────┘   └────┬────┘   └─────────────────┘    │  │
│  │        └─────────────┴─────────────┘                  │  │
│  │                      │                                │  │
│  │              Consul Registration                      │  │
│  └───────────────────────────────────────────────────────┘  │
│                           │                                  │
│                    ┌──────┴──────┐                          │
│                    │   oneshot   │ ← 优雅关闭信号            │
│                    │   channel   │                          │
│                    └─────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

**RunnableService trait：**

```rust,ignore
use async_trait::async_trait;
use tokio::sync::{oneshot, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    Initializing,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed(String),
}

#[async_trait]
pub trait RunnableService: Send + Sync + 'static {
    type Config: ServiceConfig;
    
    fn new(config: Self::Config, instance_id: String, status_arc: Arc<RwLock<ServiceStatus>>) -> Self;
    fn instance_id(&self) -> &str;
    fn get_status(&self) -> Arc<RwLock<ServiceStatus>>;
    
    // 核心逻辑：接收关闭信号，运行到结束
    async fn start_service_logic(
        &self,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), FrameworkError>;
}
```

**应用框架实现：**

```rust,ignore
pub struct ApplicationFramework<S: RunnableService> {
    service_instance: Arc<S>,
    status_receiver: Arc<RwLock<ServiceStatus>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    service_handle: Option<JoinHandle<Result<(), FrameworkError>>>,
}

impl<S: RunnableService> ApplicationFramework<S> {
    pub fn new(config: S::Config) -> Result<Self, FrameworkError> {
        let instance_id = format!("{}-{}", config.base_config().service_id_prefix, Uuid::new_v4());
        let status_arc = Arc::new(RwLock::new(ServiceStatus::Initializing));
        let (tx_shutdown, rx_shutdown) = oneshot::channel();
        
        let service_instance = Arc::new(S::new(config, instance_id, Arc::clone(&status_arc)));
        let service_for_task = Arc::clone(&service_instance);
        
        // Spawn 服务主逻辑
        let service_handle = tokio::spawn(async move {
            // 启动 Consul 注册
            // 启动 gRPC 和 HTTP 服务器
            // 等待关闭信号
            service_for_task.start_service_logic(rx_shutdown).await
        });
        
        Ok(Self { /* ... */ })
    }
    
    pub async fn stop(&mut self) -> Result<(), FrameworkError> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()); // 发送关闭信号
        }
        // 等待服务完全停止
        if let Some(handle) = self.service_handle.take() {
            handle.await?;
        }
        Ok(())
    }
}
```

完整代码：
- framework/lifecycle.rs
- framework/config.rs
- framework/error.rs

---

**生产级服务实现（GreeterService）：**

```rust,ignore
#[async_trait]
impl RunnableService for GreeterApplicationService {
    type Config = GreeterServiceConfig;
    
    async fn start_service_logic(
        &self,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), FrameworkError> {
        // 1. 构建 gRPC 服务
        let greeter_service = GreeterServer::new(MyGreeter::default());
        let grpc_server = tonic::transport::Server::builder()
            .layer(TraceLayer::new_for_grpc())
            .add_service(greeter_service)
            .serve(grpc_addr);
        
        // 2. 启动 HTTP 健康检查
        let health_app = axum::Router::new()
            .route("/health", axum::routing::get(|| async { StatusCode::OK }));
        let health_server = axum::serve(listener, health_app);
        
        // 3. 注册到 Consul
        self.consul_client.register_service(&registration_payload).await?;
        
        // 4. 并发运行，等待关闭信号
        tokio::select! {
            _ = grpc_server => {},
            _ = health_server => {},
            _ = shutdown_rx => {},
        }
        
        // 5. 从 Consul 注销
        self.consul_client.deregister_service(&self.instance_id).await?;
        Ok(())
    }
}
```

完整示例：greeter_service.rs  
服务消费者：greeter_consume.rs

---

## 常见错误

### 错误 1：在 async 上下文中使用 std::sync::Mutex

```rust,ignore
// ❌ 错误：在 async 函数中使用阻塞锁
async fn bad_example(data: Arc<std::sync::Mutex<i32>>) {
    let mut guard = data.lock().unwrap(); // 阻塞整个线程！
    some_async_op().await; // 锁被持有跨越 await 点
    *guard += 1;
}

// ✅ 正确：使用 tokio::sync::Mutex
async fn good_example(data: Arc<tokio::sync::Mutex<i32>>) {
    let mut guard = data.lock().await; // 异步锁
    some_async_op().await; // 锁自动释放和重新获取
    *guard += 1;
}
```

---

### 错误 2：服务发现时没有处理空列表

```rust,ignore
// ❌ 危险：没有检查服务列表为空
let nodes = consul_client.discover_service("my-service").await?;
let node = &nodes[0]; // panic if empty!

// ✅ 安全：检查空列表
let nodes = consul_client.discover_service("my-service").await?;
if nodes.is_empty() {
    tokio::time::sleep(Duration::from_secs(5)).await;
    return Ok(()); // 或者重试
}
let node = &nodes[index % nodes.len()]; // 轮询
```

---

### 错误 3：忘记在关闭时从 Consul 注销服务

```rust,ignore
// ❌ 错误：服务关闭后仍残留在 Consul
async fn run_server() {
    consul.register(&service).await.unwrap();
    server.await; // 服务停止后没有注销
}

// ✅ 正确：使用 drop guard 或 select
async fn run_server() {
    consul.register(&service).await.unwrap();
    tokio::select! {
        _ = server => {},
        _ = shutdown_rx => {},
    }
    consul.deregister(&service.id).await.unwrap(); // 确保注销
}
```

---

### 错误 4：gRPC 流没有正确处理客户端断开

```rust,ignore
// ❌ 问题：客户端断开后继续发送
while let Some(item) = rx.recv().await {
    tx.send(Ok(item)).await?; // 可能无限阻塞
}

// ✅ 正确：使用 try_send 或检查发送结果
tokio::spawn(async move {
    loop {
        if let Some(item) = map.get(&id.sku) {
            if tx.send(Ok(item.clone())).is_err() {
                return; // 客户端断开，停止任务
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
});
```

---

## 动手练习

### 练习 1：实现服务注册

补全以下代码，实现服务注册到 Consul：

```rust,ignore
async fn register_service(
    consul: &ConsulClient,
    name: &str,
    address: &str,
    port: u16,
) -> Result<()> {
    let registration = AgentServiceRegistration {
        id: Some(format!("{}-{}", name, Uuid::new_v4())),
        name: name.to_string(),
        address: Some(address.to_string()),
        port: Some(port),
        // TODO: 添加健康检查
        check: Some(AgentServiceCheck {
            http: Some(format!("http://{}:{}/health", address, port + 1)),
            interval: Some("10s".to_string()),
            timeout: Some("1s".to_string()),
            ..Default::default()
        }),
    };
    
    // TODO: 调用 consul.register_service
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
async fn register_service(
    consul: &ConsulClient,
    name: &str,
    address: &str,
    port: u16,
) -> Result<()> {
    let registration = AgentServiceRegistration {
        id: Some(format!("{}-{}", name, Uuid::new_v4())),
        name: name.to_string(),
        address: Some(address.to_string()),
        port: Some(port),
        check: Some(AgentServiceCheck {
            http: Some(format!("http://{}:{}/health", address, port + 1)),
            interval: Some("10s".to_string()),
            timeout: Some("1s".to_string()),
            deregister_critical_service_after: Some("1m".to_string()),
            ..Default::default()
        }),
    };
    
    consul.register_service(&registration).await?;
    Ok(())
}
```

</details>

---

### 练习 2：实现轮询负载均衡

补全以下代码，实现基于 Consul 的轮询服务发现：

```rust,ignore
pub struct LoadBalancer {
    consul: ConsulClient,
    service_name: String,
    index: AtomicUsize,
}

impl LoadBalancer {
    pub async fn next(&self) -> Result<CatalogServiceNode> {
        // TODO: 获取服务列表并返回下一个节点
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use std::sync::atomic::{AtomicUsize, Ordering};

impl LoadBalancer {
    pub async fn next(&self) -> Result<CatalogServiceNode> {
        let nodes = self.consul.discover_service(&self.service_name).await?;
        if nodes.is_empty() {
            return Err(anyhow!("No available service instances"));
        }
        
        let index = self.index.fetch_add(1, Ordering::Relaxed);
        let node = &nodes[index % nodes.len()];
        Ok(node.clone())
    }
}
```

</details>

---

### 练习 3：实现优雅关闭

补全以下代码，在收到 Ctrl+C 信号时优雅关闭服务：

```rust,ignore
#[tokio::main]
async fn main() -> Result<()> {
    let framework = ApplicationFramework::<MyService>::new(config)?;
    
    // TODO: 监听 Ctrl+C 信号并调用 framework.stop()
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut framework = ApplicationFramework::<MyService>::new(config)?;
    
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal...");
            framework.stop().await?;
        }
        status = watch_service_status(&framework) => {
            println!("Service exited with status: {:?}", status);
        }
    }
    
    Ok(())
}
```

</details>

---

## 实际应用

### 应用场景 1：微服务架构的完整链路

```rust,ignore
// 1. 服务提供者：GreeterService
pub struct GreeterApplicationService {
    config: GreeterServiceConfig,
    consul_client: ConsulClient,
    // ...
}

// 2. 服务注册（启动时）
async fn start(&self) -> Result<()> {
    // 注册到 Consul
    self.consul_client.register_service(&AgentServiceRegistration {
        name: "greeter-service".to_string(),
        address: Some(self.config.service_ip.clone()),
        port: Some(self.config.grpc_port),
        check: Some(health_check),
        ..Default::default()
    }).await?;
    
    // 启动 gRPC 服务
    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve(addr)
        .await?;
}

// 3. 服务消费者：GreeterClient
async fn call_greeter(consul: &ConsulClient) -> Result<String> {
    // 服务发现
    let nodes = consul.discover_service("greeter-service").await?;
    let node = &nodes[0];
    
    // 创建连接
    let addr = format!("http://{}:{}", node.service_address, node.service_port);
    let mut client = GreeterClient::connect(addr).await?;
    
    // 调用 RPC
    let response = client.say_hello(HelloRequest { name: "World".into() }).await?;
    Ok(response.into_inner().message)
}
```

完整示例：greeter_service.rs / greeter_consume.rs

---

### 应用场景 2：带缓存的服务发现

```rust,ignore
pub struct CachedServiceDiscovery {
    consul: ConsulClient,
    cache: Arc<RwLock<HashMap<String, (Vec<CatalogServiceNode>, Instant)>>>,
    ttl: Duration,
}

impl CachedServiceDiscovery {
    pub async fn discover(&self, service_name: &str) -> Result<Vec<CatalogServiceNode>> {
        // 检查缓存
        if let Some((nodes, timestamp)) = self.cache.read().await.get(service_name) {
            if timestamp.elapsed() < self.ttl {
                return Ok(nodes.clone());
            }
        }
        
        // 从 Consul 获取
        let nodes = self.consul.discover_service(service_name).await?;
        
        // 更新缓存
        self.cache.write().await.insert(
            service_name.to_string(),
            (nodes.clone(), Instant::now()),
        );
        
        Ok(nodes)
    }
}
```

---

### 应用场景 3：插件化架构

```rust,ignore
// 使用 inventory 实现编译时插件注册
inventory::collect!(Plugin);

pub struct Plugin {
    name: &'static str,
    handler: fn(&Context) -> Result<()>,
}

// 各插件自行注册
inventory::submit! {
    Plugin { name: "logger", handler: logger_plugin }
}

inventory::submit! {
    Plugin { name: "metrics", handler: metrics_plugin }
}

// 主程序加载所有插件
fn main() {
    let ctx = Context::new();
    for plugin in inventory::iter::<Plugin> {
        println!("Loading plugin: {}", plugin.name);
        (plugin.handler)(&ctx).unwrap();
    }
}
```

完整示例：inventory_sample.rs

---

## 故障排查 (FAQ)

### Q: Consul 服务注册成功但发现为空？

**A**: 检查以下几点：
1. 服务名称是否完全匹配（大小写敏感）
2. Consul 的健康检查是否通过（失败的服务会被过滤）
3. 是否等待了足够的时间（服务注册有延迟）

```rust,ignore
// 调试技巧：打印所有已注册服务
let services = consul.get_all_registered_service_names(None).await?;
println!("Registered services: {:?}", services);
```

---

### Q: gRPC 客户端连接失败？

**A**: 常见原因：
1. **地址格式**：必须使用 `http://` 前缀
2. **端口错误**：检查服务端的监听端口
3. **TLS 配置**：如果使用 TLS，需要配置证书

```rust,ignore
// ✅ 正确
GreeterClient::connect("http://127.0.0.1:50051").await?;

// ❌ 错误：缺少协议前缀
GreeterClient::connect("127.0.0.1:50051").await?;
```

---

### Q: 依赖注入时 downcast 失败？

**A**: `TypeId` 是编译期确定的，确保注册和解析时使用完全相同的类型：

```rust,ignore
// ✅ 一致：注册和解析使用相同类型
container.register_trait::<dyn LoggerService>(Arc::new(ConsoleLogger));
let logger = container.resolve_trait::<dyn LoggerService>();

// ❌ 不一致：类型不同
container.register::<ConsoleLogger>(ConsoleLogger); // TypeId::of::<ConsoleLogger>()
let service = container.resolve::<dyn LoggerService>(); // TypeId::of::<dyn LoggerService>()
```

---

### Q: 服务框架启动后立即退出？

**A**: 确保你的 `start_service_logic` 是"长期运行"的：

```rust,ignore
async fn start_service_logic(&self, shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
    // ❌ 错误：立即返回
    tokio::spawn(async { /* ... */ });
    Ok(()) // 任务还在运行，但函数已返回
    
    // ✅ 正确：等待信号
    let server_future = tokio::spawn(server);
    tokio::select! {
        _ = server_future => {},
        _ = shutdown_rx => {},
    }
    Ok(())
}
```

---

## 知识扩展 (选学)

### Tonic 中间件

使用 Tower 中间件添加通用功能：

```rust,ignore
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use tower_http::trace::TraceLayer;

let layer = ServiceBuilder::new()
    .layer(TraceLayer::new_for_grpc())
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    .into_inner();

Server::builder()
    .layer(layer)
    .add_service(GreeterServer::new(MyGreeter::default()))
    .serve(addr)
    .await?;
```

---

### gRPC 反射服务

添加反射支持，方便调试：

```rust,ignore
let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
    .build_v1()?;

Server::builder()
    .add_service(InventoryServer::new(inventory))
    .add_service(reflection_service)
    .serve(addr)
    .await?;
```

完整示例：tonic_store_server.rs

---

### 配置热更新

```rust,ignore
pub struct ConfigWatcher {
    path: PathBuf,
    current: Arc<RwLock<BaseServiceConfig>>,
}

impl ConfigWatcher {
    pub async fn watch(&self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            
            let content = tokio::fs::read_to_string(&self.path).await?;
            let new_config: BaseServiceConfig = serde_json::from_str(&content)?;
            
            *self.current.write().await = new_config;
            println!("Config updated");
        }
    }
}
```

---

## 小结

**核心要点：**

1. **依赖注入**有三种模式：具体类型注入（零开销）、动态 Arc 注入（灵活共享）、动态 Box 注入（独占所有权）
2. **Service Container** 使用 `TypeId` 进行类型安全的运行时服务查找
3. **Consul** 提供服务注册、发现、健康检查和配置管理
4. **Tonic** 是 Rust 生态中功能完整的 gRPC 框架，支持 unary、streaming、拦截器等
5. **ApplicationFramework** 通过 `RunnableService` trait 统一管理服务生命周期
6. **优雅关闭**使用 `oneshot::channel` 传递信号，确保资源正确释放

**关键术语：**

| English | 中文 |
|---------|------|
| Dependency Injection (DI) | 依赖注入 |
| Service Registry | 服务注册中心 |
| Service Discovery | 服务发现 |
| Health Check | 健康检查 |
| gRPC | gRPC 远程过程调用 |
| Unary RPC | 一元 RPC |
| Streaming RPC | 流式 RPC |
| Graceful Shutdown | 优雅关闭 |
| Circuit Breaker | 熔断器 |
| Load Balancing | 负载均衡 |

**下一步：**

- 探索 数据库访问 - 与 SQLx 和 Diesel 集成
- 学习 Tokio - 深入理解异步运行时
- 查看 Axum - 构建 HTTP REST API

---

## 术语表

| English | 中文 |
|---------|------|
| Dependency Injection | 依赖注入 |
| Service Container | 服务容器 |
| TypeId | 类型标识 |
| Trait Object | Trait 对象 |
| Service Registry | 服务注册表 |
| Service Discovery | 服务发现 |
| Consul | Consul 服务治理工具 |
| gRPC | Google RPC 框架 |
| Tonic | Rust gRPC 实现 |
| Protobuf | Protocol Buffers 序列化 |
| Unary | 一元调用 |
| Streaming | 流式传输 |
| RunnableService | 可运行服务 trait |
| ApplicationFramework | 应用框架 |
| Graceful Shutdown | 优雅关闭 |
| Health Check | 健康检查 |
| Round-robin | 轮询算法 |
| Inventory | 编译时插件注册 |

---

## 知识检查点

### 检查点 1 🟢 (DI 基础)

以下代码的输出是什么？

```rust,ignore
use std::any::TypeId;

trait Service: Any + Send + Sync {}
struct Logger;
impl Service for Logger {}

fn main() {
    let id1 = TypeId::of::<Logger>();
    let id2 = TypeId::of::<dyn Service>();
    println!("{}", id1 == id2);
}
```

A) true  
B) false  
C) 编译错误  
D) 运行时 panic

<details>
<summary>答案与解析</summary>

**答案**: B) false

**解析**:
- `TypeId::of::<Logger>()` 是具体类型的 TypeId
- `TypeId::of::<dyn Service>()` 是 trait 对象的 TypeId
- 两者在编译期生成不同的标识符
- 这是依赖注入容器必须使用一致类型的原因

</details>

---

### 检查点 2 🟡 (Consul 服务发现)

以下代码有什么问题？

```rust,ignore
async fn call_service(consul: &ConsulClient) -> Result<String> {
    let nodes = consul.discover_service("my-service").await?;
    let node = &nodes[0]; // 第 0 行
    let addr = format!("http://{}:{}", node.service_address, node.service_port);
    // ...
}
```

A) 没有问题  
B) 如果服务列表为空会 panic  
C) 应该使用轮询而不是固定取第一个  
D) B 和 C 都正确

<details>
<summary>答案与解析</summary>

**答案**: D) B 和 C 都正确

**解析**:
1. **空列表问题**：如果 Consul 返回空列表，`nodes[0]` 会导致 panic，应该使用 `nodes.get(0)` 或检查 `is_empty()`
2. **负载均衡**：生产环境应该使用轮询（round-robin）或随机选择，而不是固定取第一个，以实现负载分散

**修复方案**:
```rust,ignore
if nodes.is_empty() {
    return Err(anyhow!("No service available"));
}
let idx = rand::random::<usize>() % nodes.len();
let node = &nodes[idx];
```

</details>

---

### 检查点 3 🔴 (生命周期与优雅关闭)

以下代码为什么可能导致资源泄漏？

```rust,ignore
#[async_trait]
impl RunnableService for MyService {
    async fn start_service_logic(
        &self,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), FrameworkError> {
        // 注册到 Consul
        self.consul.register_service(&self.registration).await?;
        
        // 启动服务器
        self.server.serve(addr).await?;
        
        // 从 Consul 注销
        self.consul.deregister_service(&self.id).await?;
        Ok(())
    }
}
```

A) Consul 注册应该在构造函数中完成  
B) 如果服务器 panic，不会执行注销代码  
C) 缺少对 shutdown_rx 的处理  
D) B 和 C 都正确

<details>
<summary>答案与解析</summary>

**答案**: D) B 和 C 都正确

**解析**:
1. **panic 处理**：如果 `self.server.serve()` panic，注销代码不会执行，导致"僵尸服务"残留在 Consul
2. **缺少 shutdown 处理**：没有响应 `shutdown_rx` 信号，无法接受优雅关闭指令

**修复方案**:
```rust,ignore
async fn start_service_logic(&self, shutdown_rx: oneshot::Receiver<()>) -> Result<(), FrameworkError> {
    self.consul.register_service(&self.registration).await?;
    
    tokio::select! {
        result = self.server.serve(addr) => {
            result?;
        }
        _ = shutdown_rx => {
            info!("Shutdown signal received");
        }
    }
    
    // 确保注销被执行（即使前面的代码 panic，这里不会执行，需要额外处理）
    self.consul.deregister_service(&self.id).await?;
    Ok(())
}
```

</details>

---

## 扩展阅读

### 官方资源

- [Tonic 官方文档](https://github.com/hyperium/tonic)
- [Consul 官方文档](https://www.consul.io/docs)
- [rs_consul crate](https://docs.rs/rs-consul/)
- [inventory crate](https://docs.rs/inventory/)

### 相关项目

- [Tower](https://github.com/tower-rs/tower) - 服务中间件抽象
- [Hyper](https://hyper.rs/) - HTTP 实现，Tonic 底层使用
- [Prost](https://github.com/tokio-rs/prost) - Protocol Buffers 实现
- [Arc-swap](https://docs.rs/arc-swap/) - 零拷贝配置热更新

### 进阶主题

- **Service Mesh**: Istio、Linkerd 等服务网格技术
- **gRPC 流控**: 背压（backpressure）和流控策略
- **分布式追踪**: OpenTelemetry、Jaeger 集成
- **服务熔断**: 使用 `tower::limit` 实现熔断和限流

---

## 继续学习

- 下一步：Tokio 异步运行时
- 进阶：Axum Web 框架
- 相关：数据库访问

> 💡 **记住**：微服务架构的核心是解耦和自治——每个服务独立部署、独立扩展，通过标准协议（gRPC/HTTP）通信。服务框架让这些理念在 Rust 中变得可实现！
