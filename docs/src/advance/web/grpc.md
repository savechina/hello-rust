# gRPC 服务

## 开篇故事

想象你在开发一个微服务系统。服务 A 需要调用服务 B 的函数，如果直接调用，两个服务必须部署在同一台机器上，使用相同的编程语言。但现实是：服务 A 用 Rust 编写，服务 B 用 Go 编写，它们运行在不同的服务器上。

gRPC 就像是一个"通用翻译器"——你定义服务接口（使用 Protocol Buffers），gRPC 自动生成客户端和服务端代码，让不同语言、不同机器的服务可以像本地调用一样通信。

---

## 本章适合谁

如果你想学习：
- 如何使用 gRPC 构建跨服务通信
- tonic 框架在 Rust 中的使用
- 如何定义和使用 Protocol Buffers

本章适合你。gRPC 是现代微服务架构的核心技术。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 gRPC 的核心概念（服务定义、Protocol Buffers、流式通信）
2. 使用 tonic 创建 gRPC 服务端
3. 使用 tonic 创建 gRPC 客户端
4. 实现 Unary、Server Streaming、Client Streaming、Bidirectional Streaming
5. 使用 clap 解析命令行参数

---

## 前置要求

- [异步编程](../advance/async/async.md) - async/await 基础
- [Tokio](../advance/async/tokio.md) - Tokio 异步运行时
- 理解微服务基本概念

---

### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tonic --features full
cargo add prost
cargo add clap --features derive
cargo add tokio --features full
cargo add anyhow
```

---

## 第一个例子

最简单的 gRPC Hello 服务：

**服务端** (`grpc_hello_server.rs`):

```rust,ignore
use awesome::services::tonic_hello_server;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "127.0.0.1", long)]
    host: String,
    #[arg(default_value = "9001", short, long)]
    port: u32,
}

fn main() {
    let opts = Args::parse();
    println!("启动 gRPC 服务器: {}:{}", opts.host, opts.port);
    tonic_hello_server::hello_server(&opts.host, opts.port);
}
```

> 💡 **注意**：此代码需要服务端和客户端配合运行。请使用 `cargo run --bin grpc_hello_server` 进行完整测试。

**客户端** (`grpc_hello_client.rs`):

```rust,ignore
use awesome::services::tonic_hello_client;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(default_value = "http://127.0.0.1:9001", long)]
    url: String,
}

fn main() {
    let opts = Args::parse();
    println!("连接到 gRPC 服务器: {}", opts.url);
    tonic_hello_client::hello_client(opts.url);
}
```

> 💡 **注意**：此代码需要服务端和客户端配合运行。请使用 `cargo run --bin grpc_hello_client` 进行完整测试。

**运行方式**:

```bash
# 先启动服务器
cargo run --bin grpc_hello_server

# 再启动客户端
cargo run --bin grpc_hello_client
```

完整示例：
- [grpc_hello_server.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_server.rs)
- [grpc_hello_client.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_client.rs)

---

## 原理解析

### gRPC 架构概览

```
┌─────────────────────────────────────────────────────────────────────┐
│                          gRPC 架构                                  │
│                                                                      │
│   ┌──────────────┐         ┌──────────────┐         ┌──────────────┐│
│   │   Client     │         │   Protocol   │         │   Server     ││
│   │  (客户端)     │────────→│   Buffers    │←────────│  (服务端)     ││
│   │              │  HTTP/2  │  (序列化)     │  HTTP/2  │              ││
│   └──────────────┘         └──────────────┘         └──────────────┘│
│                                                                      │
│   通信模式:                                                            │
│   1. Unary RPC         - 单次请求/响应                                 │
│   2. Server Streaming  - 一次请求，多次响应                            │
│   3. Client Streaming  - 多次请求，一次响应                            │
│   4. Bidirectional     - 双向流式通信                                  │
└─────────────────────────────────────────────────────────────────────┘
```

### gRPC 四种通信模式

**1. Unary RPC（单次请求/响应）**

```
Client → Request → Server
Client ← Response ← Server
```

**2. Server Streaming（服务端流式）**

```
Client → Request → Server
Client ← Response1 ← Server
Client ← Response2 ← Server
Client ← Response3 ← Server
```

**3. Client Streaming（客户端流式）**

```
Client → Request1 → Server
Client → Request2 → Server
Client → Request3 → Server
Client ← Response ← Server
```

**4. Bidirectional Streaming（双向流式）**

```
Client → Request1 → Server → Response1 → Client
Client → Request2 → Server → Response2 → Client
Client → Request3 → Server → Response3 → Client
```

### Protocol Buffers 基础

```protobuf
// 定义服务
service Greeter {
    rpc SayHello (HelloRequest) returns (HelloReply);
    rpc SayHelloStream (HelloRequest) returns (stream HelloReply);
}

// 定义消息
message HelloRequest {
    string name = 1;
}

message HelloReply {
    string message = 1;
}
```

### tonic 服务端实现

```rust,ignore
use tonic::{transport::Server, Request, Response, Status};
use tonic::codegen::tokio_stream;

pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = GreeterService;

    println!("gRPC 服务器启动在 {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

> 💡 **注意**：此代码需要服务端和客户端配合运行。请使用 `cargo run --bin greeter_server` 进行完整测试。

### tonic 客户端实现

```rust,ignore
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "World".into(),
    });

    let response = client.say_hello(request).await?;

    println!("收到响应: {:?}", response.into_inner());

    Ok(())
}
```

> 💡 **注意**：此代码需要服务端配合运行。请先启动服务器，再运行客户端进行完整测试。

### 生产级 gRPC 服务架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                    生产级 gRPC 服务架构                               │
│                                                                      │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐           │
│  │   Consul     │    │   gRPC       │    │   Tracing    │           │
│  │  (服务发现)   │←──→│   Server     │←──→│  (链路追踪)   │           │
│  └──────────────┘    └──────────────┘    └──────────────┘           │
│         │                   │                   │                    │
│         │                   │                   │                    │
│  ┌──────┴───────────────────┴───────────────────┴──────┐            │
│  │              Application Framework                  │            │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │            │
│  │  │   Config    │  │  Lifecycle  │  │   Health    │  │            │
│  │  │  (配置管理)  │  │  (生命周期)  │  │  (健康检查)  │  │            │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │            │
│  └─────────────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 常见错误

### 错误 1: 忘记启动服务器

```rust,ignore
// ❌ 错误：客户端连接时服务器未启动
let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;
// 连接失败！

// ✅ 正确：先启动服务器，再连接客户端
// 终端 1: cargo run --bin grpc_hello_server
// 终端 2: cargo run --bin grpc_hello_client
```

### 错误 2: 端口冲突

```rust,ignore
// ❌ 错误：端口已被占用
let addr = "127.0.0.1:9001".parse()?;
// 如果 9001 端口已被其他服务占用，会失败

// ✅ 正确：使用可配置端口
#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = "9001", short, long)]
    port: u32,
}
```

### 错误 3: 未处理连接错误

```rust,ignore
// ❌ 错误：直接 unwrap
let mut client = GreeterClient::connect("http://127.0.0.1:50051").await.unwrap();

// ✅ 正确：使用 ? 传播错误
let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;
```

---

## 动手练习

### 练习 1: 添加新的 gRPC 方法

在 Greeter 服务中添加 `SayHelloMultiple` 方法，接受一个名字列表并返回多个问候：

```rust,ignore
// TODO: 在 .proto 文件中定义
// rpc SayHelloMultiple (NamesRequest) returns (stream HelloReply);

// TODO: 实现服务端逻辑
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
async fn say_hello_multiple(
    &self,
    request: Request<NamesRequest>,
) -> Result<Response<Self::SayHelloMultipleStream>, Status> {
    let names = request.into_inner().names;
    let output = async_stream::stream! {
        for name in names {
            yield Ok(HelloReply {
                message: format!("Hello, {}!", name),
            });
        }
    };
    Ok(Response::new(Box::pin(output)))
}
```

</details>

---

## 小结

**核心要点**：

1. **gRPC** 是高性能、跨语言的 RPC 框架
2. **Protocol Buffers** 是 gRPC 的接口定义语言
3. **tonic** 是 Rust 的 gRPC 实现
4. **四种通信模式**: Unary、Server Streaming、Client Streaming、Bidirectional
5. **clap** 用于解析命令行参数

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| gRPC | gRPC 远程过程调用 | 高性能 RPC 框架 |
| Protocol Buffers | 协议缓冲区 | 接口定义语言 |
| tonic | tonic 框架 | Rust 的 gRPC 实现 |
| Unary RPC | 单次请求/响应 | 最简单的通信模式 |
| Streaming | 流式通信 | 多次请求/响应 |
| Service Discovery | 服务发现 | 动态查找服务地址 |

**下一步**：

- 学习 [Unix Domain Socket](uds.md) - 本地进程间通信
- 了解 [服务框架](services.md) - 生产级服务架构
- 探索 [依赖注入](dependency_injection.md) - 服务容器模式

---

## 术语表

| English | 中文 |
|---------|------|
| gRPC | gRPC 远程过程调用 |
| Protocol Buffers | 协议缓冲区 |
| Service | 服务 |
| Method | 方法 |
| Request | 请求 |
| Response | 响应 |
| Streaming | 流式通信 |
| Unary | 单次请求/响应 |
| Bidirectional | 双向流式 |

完整示例：
- [grpc_hello_server.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_server.rs)
- [grpc_hello_client.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_client.rs)
- [greeter_server.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/greeter_server.rs)

---

## 继续学习

- 上一步：[消息队列](../awesome/mq.md) - 异步通信
- 下一步：[Unix Domain Socket](uds.md) - 本地 IPC
- 相关：[服务框架](../awesome/services.md) - 生产级服务架构

> 💡 **记住**：gRPC 的核心是"定义接口，自动生成代码"。使用 Protocol Buffers 定义服务，tonic 自动生成类型安全的客户端和服务端代码！
