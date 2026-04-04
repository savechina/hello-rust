# Axum Web 框架

## 开篇故事

想象你开了一家餐厅。你需要：前台接待（路由）、服务员（处理器）、厨房（业务逻辑）、收银台（响应）。Axum 就像这个餐厅的完整管理系统——它帮你组织所有组件，让顾客（请求）得到高效服务。

---

## 本章适合谁

如果你想用 Rust 构建 Web 服务（REST API、Web 应用），本章适合你。Axum 是 Tokio 团队开发的 Web 框架，以类型安全、高性能、易用性著称。

---

## 你会学到什么

完成本章后，你可以：

1. 创建 Axum Web 应用
2. 定义路由和处理器
3. 处理 JSON 请求和响应
4. 实现优雅的服务器关闭
5. 使用中间件和错误处理

---

## 前置要求

- Tokio 异步运行时 - 异步基础
- Serde 序列化 - JSON 处理
- 结构体 - 结构体定义

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo add axum
```

## 第一个例子

最简单的 Axum Web 服务器：

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 创建路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    
    // 监听端口
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("服务器运行在 http://localhost:3000");
    
    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
```

**完整示例**: [axum_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/axum_sample.rs)

---

## 原理解析

### Axum 核心概念

**Axum 是一个 Web 框架**：

- ✅ 基于 Tokio 异步运行时
- ✅ 类型安全的路由
- ✅ 内置 JSON 支持
- ✅ 中间件支持
- ✅ 优雅关闭

### 创建应用

**Router 定义路由**：

```rust
use axum::Router;

let app = Router::new()
    .route("/", get(root_handler))
    .route("/users", get(list_users))
    .route("/users/:id", get(get_user));
```

### 路由处理器

**简单处理器**：

```rust
use axum::response::IntoResponse;

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

**带参数的处理器**：

```rust
use axum::extract::Path;

async fn get_user(Path(user_id): Path<u32>) -> String {
    format!("获取用户 {}", user_id)
}
```

### JSON 响应

**定义数据结构**：

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}
```

**返回 JSON**：

```rust
use axum::Json;

async fn get_user() -> Json<User> {
    Json(User {
        id: 1337,
        username: "Wee".to_string(),
    })
}
```

### JSON 请求

**接收 JSON 数据**：

```rust
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> impl IntoResponse {
    // 处理创建用户逻辑
    let user = User {
        id: 1,
        username: payload.username,
    };
    
    (StatusCode::CREATED, Json(user))
}
```

### 优雅关闭

**实现优雅关闭**：

```rust
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("收到关闭信号，优雅关闭中...");
}

// 使用
let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
```

### 完整示例

**多路由应用**：

```rust
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("服务器运行在 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, username: "Alice".to_string() },
        User { id: 2, username: "Bob".to_string() },
    ])
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        username: payload.username,
    })
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        username: "User".to_string(),
    })
}
```

---

## 常见错误

### 错误 1: 忘记使用 #[tokio::main]

```rust
fn main() {  // ❌ 忘记 #[tokio::main]
    let app = Router::new();
    // ...
}
```

**错误信息**:
```
error[E0308]: mismatched types
```

**修复方法**:
```rust
#[tokio::main]  // ✅ 添加异步运行时
async fn main() {
    // ...
}
```

### 错误 2: 路由路径不匹配

```rust
// 定义路由
.route("/users/:id", get(get_user))

// 访问 /users/ 或 /users/abc
// ❌ 404 Not Found
```

**修复方法**:
- 确保 URL 与定义匹配
- `:id` 必须是数字（如果定义为 `Path<u32>`）

### 错误 3: JSON 解析失败

```rust
// 发送错误的 JSON
POST /users
{"user_name": "Alice"}  // ❌ 字段名错误

// 期望的 JSON
{"username": "Alice"}  // ✅
```

**错误信息**:
```
422 Unprocessable Entity
missing field `username`
```

---

## 动手练习

### 练习 1: 创建简单路由

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // TODO: 创建 Router
    // TODO: 添加 "/" 路由，返回 "Hello!"
    // TODO: 监听 3000 端口
}
```

<details>
<summary>点击查看答案</summary>

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello!" }));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```
</details>

### 练习 2: 实现用户 API

```rust
#[derive(Serialize, Deserialize)]
struct User {
    // TODO: 定义字段 (id, username)
}

// TODO: 实现 list_users 处理器
// TODO: 实现 create_user 处理器
```

<details>
<summary>点击查看答案</summary>

```rust
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, username: "Alice".to_string() },
    ])
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        username: payload.username,
    })
}
```
</details>

### 练习 3: 添加路径参数

```rust
use axum::extract::Path;

// TODO: 实现 get_user 处理器
// 接收用户 ID 参数
// 返回用户信息
```

<details>
<summary>点击查看答案</summary>

```rust
use axum::extract::Path;

async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
    Json(User {
        id: user_id,
        username: format!("User{}", user_id),
    })
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Axum 和 Actix-web 有什么区别？

**A**: 
- **Axum**: Tokio 团队开发，类型安全，更现代
- **Actix-web**: 性能最优，生态成熟
- **推荐**: Axum（类型安全更好）

### Q: 如何处理 CORS？

**A**: 
```rust
use tower_http::cors::{CorsLayer, Any};

let app = Router::new()
    .route("/", get(handler))
    .layer(CorsLayer::new().allow_origin(Any));
```

### Q: 如何添加中间件？

**A**: 
```rust
use axum::middleware;

let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(logging_middleware));
```

---

## 知识扩展

### 状态共享

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
}

let state = AppState {
    users: Arc::new(RwLock::new(vec![])),
};

let app = Router::new()
    .route("/users", get(list_users))
    .with_state(state);
```

### 错误处理

```rust
use axum::http::StatusCode;

async fn handler() -> Result<Json<User>, StatusCode> {
    let user = get_user_from_db().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(user))
}
```

### 请求体验证

```rust
use axum::extract::rejection::JsonRejection;

async fn handler(
    result: Result<Json<CreateUser>, JsonRejection>
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => create_user(payload).await,
        Err(rejection) => {
            (StatusCode::BAD_REQUEST, rejection.body_text())
        }
    }
}
```

---

## 小结

**核心要点**：

1. **Axum**: 类型安全的 Web 框架
2. **Router**: 定义路由
3. **Handler**: 处理请求的函数
4. **Json**: 自动序列化/反序列化
5. **Path**: 提取路径参数
6. **优雅关闭**: 处理关闭信号

**关键术语**：

- **Router (路由器)**: 路由管理
- **Handler (处理器)**: 请求处理函数
- **Middleware (中间件)**: 请求/响应拦截器
- **Extractor (提取器)**: 从请求提取数据
- **Graceful Shutdown (优雅关闭)**: 优雅处理关闭

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Router | 路由器 |
| Handler | 处理器 |
| Middleware | 中间件 |
| Extractor | 提取器 |
| Graceful Shutdown | 优雅关闭 |
| Route | 路由 |

---

## 知识检查

**快速测验**（答案在下方）：

1. Axum 和 Hyper 是什么关系？

2. 路由参数如何提取？

3. 中间件在 Axum 中如何实现？

<details>
<summary>点击查看答案与解析</summary>

1. Axum 构建在 Hyper 之上，提供更高级的 API
2. 使用 `Path<T>` 提取器，T 需要实现 `Deserialize`
3. 使用 `tower::Service` 或 `axum::middleware`

**关键理解**: Axum 是 Tokio 团队开发的 Web 框架。
</details>

## 继续学习

**前一章**: Tokio 异步运行时  
**下一章**: [HTTP 库](hyper.md)

**相关章节**:
- Tokio 异步运行时
- [HTTP 库](hyper.md)
- 序列化

**返回**: 高级进阶

---

**完整示例**: [axum_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/axum_sample.rs)
