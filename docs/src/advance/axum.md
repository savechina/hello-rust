# Axum Web 框架

## 开篇故事

想象你走进一家繁忙的餐厅。当你坐下后，服务员过来记录你的点餐（**提取请求**），然后将订单传递给厨房（**路由分发**），厨师根据订单准备食物（**处理请求**），最后服务员将菜品端给你（**返回响应**）。在这个过程中，可能还有领班监督服务质量（**中间件**），确保每个环节顺畅进行。

Axum 就是这样一个"餐厅"——它是一套基于 Tokio 的 Web 框架，专注于人体工学和模块化设计，让你能用最少的代码构建高性能的 HTTP 服务。

---

## 本章适合谁

如果你已经掌握了 Rust 异步编程基础，想要构建 RESTful API 或 Web 服务，本章适合你。Axum 是 Rust 生态中最流行的 Web 框架之一，与 Tower、Hyper 深度集成，适合构建生产级应用。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 Router 构建路由系统，处理不同的 HTTP 方法和路径
2. 编写异步 Handler 函数，处理请求并返回响应
3. 使用 Extractor 从请求中提取数据（JSON、查询参数等）
4. 实现中间件，在请求处理前后执行逻辑
5. 集成 Tower 服务，利用生态系统中的组件

---

## 前置要求

学习本章前，你需要理解：

- 异步编程基础
- Rust 所有权和借用规则
- 结构体和泛型基础
- HTTP 协议基本概念

---

## 第一个例子

让我们从一个最简单的 Axum 服务开始：

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

完整示例：src/advance/axum_sample.rs

---

## 原理解析

### Axum 架构概览

HTTP 请求流程：

```
Client → TcpListener → Router → Middleware → Handler → Response
```

核心组件详解：

#### 1. 路由 (Router)

Router 负责将 HTTP 请求分发到对应的处理器：

```rust
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo));
```

#### 2. 处理器 (Handler)

处理器是处理请求的异步函数：

```rust
async fn root() -> &'static str {
    "Hello, World!"
}
```

#### 3. 提取器 (Extractor)

提取器从请求中提取数据：

```rust
async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(user))
}
```

---

## 常见错误

### 错误 1: JSON 提取失败

确保客户端发送 Content-Type: application/json

### 错误 2: 路由顺序导致 404

具体路由应在通配符之前定义

---

## 知识检查

**问题 1**: Router::new().route("/users", get(list_users).post(create_user)) 支持哪些 HTTP 方法？

答案: GET 和 POST

**问题 2**: 如何从路径 /users/42 提取 ID？

答案: 使用 Path(id): Path<u64> 提取器

---

## 小结

核心要点：Router、Handler、Extractor、Middleware、Tower集成

完整示例：src/advance/axum_sample.rs
