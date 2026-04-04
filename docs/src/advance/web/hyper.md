# Hyper HTTP 库

## 开篇故事

想象你要建一家餐厅。Axum 是全套服务（前台、服务员、厨房），而 Hyper 只是厨房——它处理 HTTP 协议的核心部分，让你能构建自己的 Web 框架。Hyper 是 Rust 生态中许多 Web 框架的基础。

---

## 本章适合谁

如果你想深入理解 HTTP 协议底层，或想构建自己的 Web 框架，本章适合你。Hyper 是低级 HTTP 库，提供最大的灵活性。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 HTTP 请求和响应
2. 创建 Hyper 服务器
3. 处理请求路由
4. 处理请求体和响应体
5. 实现自定义服务

---

## 前置要求

- Tokio 异步运行时 - 异步基础
- [HTTP 基础](axum.md) - HTTP 概念（可选）

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde_json
cargo add tokio --features full
cargo add hyper --features full
```

## 第一个例子

最简单的 Hyper 服务器：

```rust,ignore
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

async fn handle_request(
    req: Request<Incoming>
) -> Result<Response<String>, hyper::Error> {
    Ok(Response::new("Hello, World!".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    
    println!("服务器运行在 http://{}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            
            http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
        });
    }
}
```

> 💡 **注意**：此代码需要服务端运行。请使用 `cargo run --bin hyper_sample` 进行完整测试，并使用 HTTP 客户端访问。

**完整示例**: [hyper_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/hyper_sample.rs)

---

## 原理解析

### Hyper 特性

**Hyper 是一个 HTTP 库**：

- ✅ 低级别 HTTP 实现
- ✅ 高性能
- ✅ 异步支持
- ✅ 可构建 Web 框架

### HTTP 请求和响应

**Request 结构**：

```rust,ignore
use hyper::{Request, Method};

let req = Request::builder()
    .method(Method::GET)
    .uri("/hello")
    .body(())
    .unwrap();
```

**Response 结构**：

```rust,ignore
use hyper::{Response, StatusCode};

let resp = Response::builder()
    .status(StatusCode::OK)
    .body("Hello, World!")
    .unwrap();
```

### 服务函数

**service_fn 处理请求**：

```rust,ignore
use hyper::service::service_fn;

async fn handle(
    req: Request<Incoming>
) -> Result<Response<String>, hyper::Error> {
    Ok(Response::new("Hello!".to_string()))
}

// 使用
service_fn(handle)
```

### 路由

**手动路由**：

```rust,ignore
async fn router(
    req: Request<Incoming>
) -> Result<Response<String>, hyper::Error> {
    match req.uri().path() {
        "/hello" => Ok(Response::new("Hello!".to_string())),
        "/echo" => Ok(Response::new("Echo!".to_string())),
        _ => Ok(Response::builder()
            .status(404)
            .body("Not Found".to_string())
            .unwrap()),
    }
}
```

### 请求体处理

**读取请求体**：

```rust,ignore
use hyper::body::Bytes;
use http_body_util::BodyExt;

async fn echo(
    req: Request<Incoming>
) -> Result<Response<String>, hyper::Error> {
    // 收集整个请求体
    let whole_body = req.collect().await?.to_bytes();
    
    Ok(Response::new(format!(
        "Echo: {}",
        String::from_utf8_lossy(&whole_body)
    )))
}
```

### 响应体

**使用 Full 响应体**：

```rust,ignore
use http_body_util::Full;
use hyper::body::Bytes;

let body = Full::new(Bytes::from("Hello, World!"));
let response = Response::new(body);
```

**使用 Empty 响应体**：

```rust,ignore
use http_body_util::Empty;

let body = Empty::<Bytes>::new();
let response = Response::new(body);
```

### 完整服务器示例

```rust,ignore
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use tokio::net::TcpListener;

async fn handle(
    req: Request<Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match req.uri().path() {
        "/hello" => Ok(Response::new(Full::new(Bytes::from("Hello!")))),
        "/echo" => {
            let body = req.collect().await?.to_bytes();
            Ok(Response::new(Full::new(body)))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    
    println!("服务器运行在 http://{}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let io = hyper_util::rt::TokioIo::new(stream);
        
        tokio::task::spawn(async move {
            http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
        });
    }
}
```

> 💡 **注意**：此代码需要服务端运行。请使用 `cargo run` 启动服务器，并使用 HTTP 客户端访问进行测试。

---

## 常见错误

### 错误 1: 忘记使用 Tokio

```rust,ignore
fn main() {  // ❌ 忘记 #[tokio::main]
    let listener = TcpListener::bind("127.0.0.1:3000");
    // ...
}
```

**错误信息**:
```
error[E0308]: mismatched types
```

**修复方法**:
```rust,ignore
#[tokio::main]  // ✅ 添加异步运行时
async fn main() {
    // ...
}
```

### 错误 2: 类型不匹配

```rust,ignore
// ❌ 错误的响应体类型
Response::new("Hello")  // 期望 Body 类型
```

**修复方法**:
```rust,ignore
Response::new(Full::new(Bytes::from("Hello")))  // ✅ 正确的类型
```

### 错误 3: 忘记收集请求体

```rust,ignore
async fn handler(req: Request<Incoming>) {
    let body = req.body();  // ❌ 这是 Incoming 类型，不是数据
}
```

**修复方法**:
```rust,ignore
async fn handler(req: Request<Incoming>) {
    let body = req.collect().await?.to_bytes();  // ✅ 收集并转换为 Bytes
}
```

---

## 动手练习

### 练习 1: 创建简单服务器

```rust,ignore
use hyper::server::conn::http1;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // TODO: 绑定 3000 端口
    // TODO: 接受连接
    // TODO: 处理请求
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

loop {
    let (stream, _) = listener.accept().await.unwrap();
    let io = hyper_util::rt::TokioIo::new(stream);
    
    tokio::task::spawn(async move {
        http1::Builder::new()
            .serve_connection(io, service_fn(handle))
            .await
    });
}
```

> 💡 **注意**：此代码需要服务端运行。请使用 `cargo run` 启动服务器，并使用 HTTP 客户端访问进行测试。

</details>

### 练习 2: 实现路由

```rust,ignore
async fn handle(req: Request<Incoming>) {
    // TODO: 根据路径路由
    // "/" → 返回 "Home"
    // "/about" → 返回 "About"
    // 其他 → 返回 404
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
match req.uri().path() {
    "/" => Response::new(Full::new(Bytes::from("Home"))),
    "/about" => Response::new(Full::new(Bytes::from("About"))),
    _ => Response::builder()
        .status(404)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap(),
}
```
</details>

### 练习 3: 实现 Echo 服务

```rust,ignore
async fn echo(req: Request<Incoming>) {
    // TODO: 读取请求体
    // TODO: 返回请求体内容
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let body = req.collect().await?.to_bytes();
Ok(Response::new(Full::new(body)))
```
</details>

---

## 故障排查 (FAQ)

### Q: Hyper 和 Axum 有什么区别？

**A**: 
- **Hyper**: 低级 HTTP 库，最大灵活性
- **Axum**: 基于 Hyper 的高级框架，更易用
- **推荐**: Axum（除非需要底层控制）

### Q: 如何处理 JSON？

**A**: 
```rust,ignore
use serde_json;

let json = serde_json::to_string(&data)?;
Response::new(Full::new(Bytes::from(json)))
```

### Q: 如何添加中间件？

**A**: 
```rust,ignore
use tower::ServiceBuilder;

let service = ServiceBuilder::new()
    .layer(LoggingLayer)
    .service(service_fn(handle));
```

---

## 知识扩展

### HTTPS 支持

```rust,ignore
use tokio_rustls::TlsAcceptor;

let acceptor = TlsAcceptor::from(config);
let stream = acceptor.accept(stream).await?;
```

### WebSocket 支持

```rust,ignore
use tokio_tungstenite::WebSocketStream;

// 升级 HTTP 连接到 WebSocket
let ws_stream = tokio_tungstenite::accept_async(stream).await?;
```

### 连接池

```rust,ignore
use hyper_util::client::legacy::Client;

let client = Client::builder(hyper_util::rt::TokioExecutor::new())
    .build_http();

let resp = client.get(uri).await?;
```

---

## 小结

**核心要点**：

1. **Hyper**: 低级 HTTP 库
2. **Request/Response**: HTTP 请求和响应
3. **service_fn**: 请求处理函数
4. **路由**: 手动匹配路径
5. **Body**: 请求体和响应体处理
6. **异步**: 使用 Tokio 运行时

**关键术语**：

- **HTTP**: 超文本传输协议
- **Request**: HTTP 请求
- **Response**: HTTP 响应
- **Body**: 请求/响应体
- **Service**: 服务处理函数

---

## 术语表

| English | 中文 |
| ------- | ---- |
| HTTP | 超文本传输协议 |
| Request | 请求 |
| Response | 响应 |
| Body | 体/主体 |
| Service | 服务 |
| Route | 路由 |

---

## 知识检查

**快速测验**（答案在下方）：

1. Hyper 和 Tokio 是什么关系？

2. `Service` trait 的作用是什么？

3. 什么时候应该直接使用 Hyper 而不是 Axum？

<details>
<summary>点击查看答案与解析</summary>

1. Hyper 是基于 Tokio 的 HTTP 库
2. `Service` 是处理请求/响应的抽象
3. 需要极致性能控制、自定义 HTTP 行为时

**关键理解**: Hyper 是底层 HTTP 库，Axum 是高级框架。
</details>

## 继续学习

**前一章**: [Axum Web 框架](axum.md)  
**下一章**: JSON 序列化

**相关章节**:
- [Axum Web 框架](axum.md)
- Tokio 异步运行时
- JSON 序列化

**返回**: 高级进阶

---

**完整示例**: [hyper_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/hyper_sample.rs)
