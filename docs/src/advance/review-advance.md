# 阶段复习：高级进阶

## 开篇故事

想象你学完了所有驾驶技巧——高速公路、夜间驾驶、雨雪天气、紧急避让。现在你需要一次综合路考，把所有技能串联起来。阶段复习就是你的"高级路考"——把分散的高级知识整合成完整的生产能力。

---

## 本章适合谁

如果你已经完成了高级进阶部分（异步、数据库、Web、系统编程等），现在想检验自己的学习成果，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 综合运用异步编程、数据库、Web 框架知识
2. 设计包含服务层、数据层、API 层的完整应用
3. 识别和修复高级 Rust 编译错误
4. 理解生产级 Rust 项目的架构模式

---

## 前置要求

完成以下章节：
- [异步编程](async/async.md)
- [数据库操作](database/database.md)
- [Web 开发](web/axum.md)
- [数据处理](data/json.md)
- [系统编程](system/process.md)
- [测试与模拟](testing/test.md)
- [宏编程](testing/macros.md)

---

## 第一个例子

回顾异步 + 数据库 + Web 的整合模式：

```rust,ignore
use axum::{Json, Router, routing::get};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ])
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_users));
    
    println!("Server running on http://localhost:3000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app
    ).await.unwrap();
}
```

这个例子整合了：异步运行时 (Tokio) + Web 框架 (Axum) + 序列化 (Serde)。

---

## 原理解析

### 高级知识整合图

```
异步编程 ──→ 数据库操作 ──→ Web 开发
    ↓            ↓            ↓
Tokio 运行时   SQLx/Diesel  Axum/Hyper
    ↓            ↓            ↓
    └────────────┴────────────┘
                 ↓
          生产级服务架构
                 ↓
    ┌────────────┴────────────┐
    ↓                         ↓
数据处理                  系统编程
(JSON/CSV/Rkyv)        (进程/文件/内存)
    ↓                         ↓
    └────────────┬────────────┘
                 ↓
          测试与模拟
        (Mock/RSpec)
```

每个模块都可以独立使用，但组合起来就是完整的生产应用。

---

## 复习范围

异步编程、数据库操作、Web 开发、数据处理、系统编程、测试与模拟、宏编程

---

## 综合练习：构建一个简单的 REST API 服务

### 练习 1：定义数据模型

```rust,ignore
// TODO: 定义 Task 结构体
// 字段：id (u64), title (String), completed (bool), created_at (String)
// 派生：Serialize, Deserialize, Debug, Clone

// TODO: 定义 TaskStore 结构体
// 字段：tasks (HashMap<u64, Task>), next_id (u64)
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
    created_at: String,
}

#[derive(Debug)]
struct TaskStore {
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TaskStore {
    fn new() -> Self {
        TaskStore {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
}
```
</details>

### 练习 2：实现 CRUD 操作

```rust,ignore
// TODO: 实现 TaskStore 的方法
// - create_task(&mut self, title: String) -> Task
// - get_task(&self, id: u64) -> Option<&Task>
// - update_task(&mut self, id: u64, title: String) -> Result<Task, String>
// - delete_task(&mut self, id: u64) -> Result<Task, String>
// - list_tasks(&self) -> Vec<&Task>
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
impl TaskStore {
    fn create_task(&mut self, title: String) -> Task {
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.tasks.insert(task.id, task.clone());
        self.next_id += 1;
        task
    }

    fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn update_task(&mut self, id: u64, title: String) -> Result<Task, String> {
        let task = self.tasks.get_mut(&id)
            .ok_or_else(|| format!("Task {} not found", id))?;
        task.title = title;
        Ok(task.clone())
    }

    fn delete_task(&mut self, id: u64) -> Result<Task, String> {
        self.tasks.remove(&id)
            .ok_or_else(|| format!("Task {} not found", id))
    }

    fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}
```
</details>

### 练习 3：异步任务处理

```rust,ignore
// TODO: 使用 tokio 实现异步任务保存
// - save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>>
// - load_from_file(&mut self, path: &str) -> Result<(), Box<dyn Error>>
// 使用 tokio::fs 进行异步文件操作
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use tokio::fs;
use std::error::Error;

impl TaskStore {
    async fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, json).await?;
        Ok(())
    }

    async fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(path).await?;
        self.tasks = serde_json::from_str(&content)?;
        Ok(())
    }
}
```
</details>

---

## 知识检查

### 问题 1：Future 惰性

```rust,ignore
async fn fetch_data() -> String {
    println!("Fetching...");
    "data".to_string()
}

fn main() {
    let future = fetch_data();
    println!("Created future");
    // 没有 .await
}
```

会打印什么？

<details>
<summary>点击查看答案</summary>

只打印 **"Created future"**。

Future 是惰性的，不调用 `.await` 或 `block_on` 就不会执行。`fetch_data()` 只是创建了 Future 对象，没有执行函数体。
</details>

### 问题 2：Arc vs Rc

为什么多线程中使用 `Rc<T>` 会编译失败？

<details>
<summary>点击查看答案</summary>

`Rc<T>` 不是线程安全的（引用计数操作不是原子的）。

多线程中应该使用 `Arc<T>`（Atomic Reference Counting），它的引用计数操作是原子的，保证线程安全。
</details>

### 问题 3：mpsc vs oneshot

何时使用 `mpsc` 通道，何时使用 `oneshot` 通道？

<details>
<summary>点击查看答案</summary>

- **mpsc**: 多发送端，单接收端。用于持续通信（如任务间消息传递）。
- **oneshot**: 单次通信。用于请求-响应模式（如等待一个结果）。
</details>

### 问题 4：宏卫生

```rust,ignore
macro_rules! create_var {
    () => {
        let x = 42;
    };
}

fn main() {
    create_var!();
    println!("{}", x);  // 能编译吗？
}
```

<details>
<summary>点击查看答案</summary>

**不能编译**。Rust 宏是卫生的（hygienic），宏内部创建的变量在宏外部不可见。

这是 Rust 宏的安全特性，防止宏意外覆盖外部变量。
</details>

### 问题 5：serde 自定义序列化

如何为字段自定义序列化行为？

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    
    #[serde(default = "default_age")]
    age: u8,
    
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
```
</details>

---

## 常见错误回顾

| 错误 | 原因 | 修复 |
|------|------|------|
| `future cannot be sent between threads` | Future 包含非 Send 类型 | 使用 `Send` 安全的类型，检查闭包捕获 |
| `borrowed value does not live long enough` | 生命周期不足 | 添加生命周期标注或改变数据结构 |
| `cannot move out of shared reference` | 试图移动借用的值 | 使用 `.clone()` 或引用 |
| `no method named 'await'` | 不在 async 上下文中 | 在 `#[tokio::main]` 或 `async fn` 中使用 |
| `expected struct, found enum` | 类型不匹配 | 检查 serde 派生和字段类型 |

---

## 小结

**核心要点**：

1. **异步编程**是生产级 Rust 的基础
2. **数据库 + Web** 是最常见的应用组合
3. **错误处理**必须贯穿整个应用
4. **测试**保证代码质量
5. **复习是巩固知识的关键**

**关键术语**：

- **Future**: 异步计算
- **Executor**: 执行器 (Tokio)
- **ORM**: 对象关系映射
- **REST API**: RESTful 接口
- **Mock**: 模拟对象

---

## 术语表

| English | 中文 |
|---------|------|
| Future | 异步计算 |
| Executor | 执行器 |
| ORM | 对象关系映射 |
| REST API | RESTful 接口 |
| Mock | 模拟对象 |
| Serialization | 序列化 |
| Deserialization | 反序列化 |

---

## 继续学习

- 下一步：[精选实战](../awesome/awesome-overview.md)
- 挑战：[项目实战](../projects/)
- 回顾：[高级进阶概览](advance-overview.md)

> 💡 **记住**：复习是学习的重要部分。不要急于前进，确保每个概念都理解了！

> 💡 **记住**：高级概念需要更多实践。不要只看代码，动手写、运行、调试！
