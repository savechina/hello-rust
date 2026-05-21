# Web 开发

Rust 生态中有多种 Web 开发方案，从高层框架到底层 HTTP 库，满足不同场景需求。

## 为什么用 Rust 做 Web 开发？

- **性能卓越**：零成本抽象，无 GC 停顿，请求延迟极低
- **内存安全**：编译时消除空指针、数据竞争等常见 Web 漏洞
- **类型安全**：强类型系统让 API 设计更可靠
- **异步原生**：tokio 运行时提供高并发处理能力

## 本章节内容

| 主题 | 说明 |
|------|------|
| [Axum Web 框架](./axum.md) | 基于 Tower 生态的现代化 Web 框架，路由、中间件、错误处理一应俱全 |
| [Hyper HTTP 库](./hyper.md) | Rust 底层 HTTP 实现，适合需要精细控制 HTTP 行为的场景 |
| [Ollama AI 集成](./ollama.md) | 在 Web 服务中集成本地 AI 模型，实现智能 API |
| [gRPC 服务](./grpc.md) | 基于 Tonic 的 gRPC 远程过程调用，适合微服务通信 |

## 快速示例：Axum Hello World

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust Web!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 下一步

- 学习 [Axum](./axum.md) 构建 RESTful API
- 了解 [Hyper](./hyper.md) 底层 HTTP 机制
- 探索 [gRPC](./grpc.md) 微服务通信
