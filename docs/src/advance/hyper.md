# Hyper HTTP 库

## 开篇故事

想象你正在构建一个微服务架构的电商系统。服务 A 需要调用服务 B 的 API 获取商品信息，服务 C 需要接收来自客户端的订单请求。这些场景都离不开 HTTP 通信。在 Rust 生态中，**Hyper** 就是这样一个底层的 HTTP 库——它提供极致的灵活性和性能，让你能够精确控制每一个 HTTP 字节。

---

## 本章适合谁

如果你已经学完了异步编程基础，现在想深入理解 HTTP 在 Rust 中的实现原理，本章适合你。Hyper 是 Tokio 生态系统的核心组件之一，也是许多高级 Web 框架（如 Axum）的底层依赖。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Hyper 的设计理念：为什么它如此"底层"
2. 使用 Hyper 构建简单的 HTTP 服务器
3. 处理 HTTP 请求和响应的 Body
4. 实现基本的路由逻辑

---

## 前置要求

学习本章前，你需要理解：

- async/await 基础
- Tokio 运行时概念
- 所有权和借用规则

---

## 第一个例子

最简单的 Hyper HTTP 服务器：

```rust
use hyper::{Request, Response, Body};
use hyper::service::{make_service_fn, service_fn};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}
```

完整示例：src/advance/hyper_sample.rs

---

## 原理解析

### HTTP 请求/响应流程

```
Client → TcpListener → http1::Builder → service_fn → Handler → Response
```

### 核心类型

- Request<Incoming>: HTTP 请求
- Response<Full<Bytes>>: HTTP 响应
- Body: 请求/响应正文

---

## 常见错误

### 错误 1: Body 只能消费一次

Incoming body 是流式类型，只能读取一次

### 错误 2: 忘记处理路由

需要根据 path 和 method 进行路由分发

---

## 知识检查

**问题 1**: Incoming 和 Full<Bytes> 的区别？

答案: Incoming 是流式请求 body，Full<Bytes> 是完整响应 body

**问题 2**: 如何设置响应状态码？

答案: 使用 Response::builder().status(StatusCode::NOT_FOUND)

---

## 小结

核心要点：底层 HTTP 库、Body 单次消费、Service 模式、Tokio集成

完整示例：src/advance/hyper_sample.rs
