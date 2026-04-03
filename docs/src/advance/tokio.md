# Tokio 异步运行时

## 开篇故事

想象一下，你是一家餐厅的主厨。传统同步编程就像你一个人独自完成所有工作：先接待一位客人点餐，然后亲自去厨房做菜，等菜做好端给客人，才能接待下一位客人。当客人很多时，队伍会越来越长，效率极低。

而异步编程就像是组建了一个高效的厨房团队：
- **前台接待员**（async runtime）负责接待所有客人并记录订单
- **多位厨师**（tasks）同时工作，互不阻塞
- **传菜员**（channels）负责在厨师和前台之间传递信息
- **订单系统**（futures）记录每个菜品的进度，厨师完成一道菜后立即接手下一道

Tokio 就是这样的"厨房团队管理系统"——它是 Rust 生态中最流行的异步运行时，让你能同时处理成千上万个连接，而不会因为等待 IO 而阻塞程序执行。

---

## 本章适合谁

如果你已经学完了 Rust 基础（所有权、借用、生命周期），现在想掌握：
- 如何用异步方式编写高性能网络程序
- 如何处理并发任务而不使用线程池
- 如何理解和使用 async/await 语法

本章适合你。异步编程是构建高性能服务器、微服务和实时应用的必备技能。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是异步运行时以及 Tokio 的作用
2. 使用 `#[tokio::main]` 启动异步程序
3. 使用 `tokio::spawn` 创建并发任务
4. 使用 `mpsc` 和 `oneshot` 通道进行任务间通信
5. 编写异步 TCP 客户端和服务器
6. 使用 `RwLock` 进行异步并发数据访问
7. 理解 `spawn_blocking` 的使用场景
8. 避免常见的异步编程错误

---

## 前置要求

学习本章前，你需要理解：

- [所有权](../basic/ownership.md) - 特别是 `Arc` 智能指针的使用
- [线程](../basic/threads.md) - 了解并发基本概念
- [生命周期](../basic/lifetimes.md) - 理解引用有效性
- Cargo.toml 中添加 tokio 依赖（features = ["full"]）

---

## 第一个例子

让我们看一个最简单的 Tokio 程序：

```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
    
    // 创建一个异步任务
    let task = tokio::spawn(async {
        println!("Running in a separate task!");
        42
    });
    
    // 等待任务完成并获取结果
    let result = task.await.unwrap();
    println!("Task returned: {}", result);
}
```

**发生了什么？**

1. `#[tokio::main]` 宏将 `main` 函数包装在 Tokio 运行时中
2. `tokio::spawn` 创建一个新的异步任务，立即开始执行
3. `.await` 暂停当前任务，等待另一个任务完成
4. 两个任务可以并发执行，互不阻塞

---

## 原理解析

### Tokio 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                    Tokio Runtime                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                 Thread Pool (多线程)                   │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  │  │
│  │  │ Worker  │  │ Worker  │  │ Worker  │  │ Worker  │  │  │
│  │  │ Thread 1│  │ Thread 2│  │ Thread 3│  │ Thread N│  │  │
│  │  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘  │  │
│  │       │            │            │            │       │  │
│  │       └────────────┴────────────┴────────────┘       │  │
│  │                     │                                │  │
│  │              Task Queue                             │  │
│  │         ┌──────────────────┐                        │  │
│  │         │ Task 1 │ Task 2  │ Task 3 │ ...           │  │
│  │         └──────────────────┘                        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                 Async I/O (异步IO)                     │  │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────┐              │  │
│  │   │  TCP    │  │  Timer  │  │ Channel │              │  │
│  │   │ Listener│  │         │  │         │              │  │
│  │   └─────────┘  └─────────┘  └─────────┘              │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 核心组件

**1. Runtime（运行时）**

```rust
#[tokio::main]
async fn main() {
    // 这里已经有一个 Tokio runtime 在运行
    // 等价于:
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async { ... })
}
```

Tokio runtime 包含：
- **任务调度器**：管理并分发异步任务到工作线程
- **I/O 驱动器**：处理所有异步 IO 操作（网络、文件）
- **定时器**：管理 `sleep` 和 `timeout`

**2. Task（任务）**

```rust
// 创建新任务 - 立即返回一个 JoinHandle
let handle = tokio::spawn(async {
    // 异步代码块
    do_something().await;
    result
});

// 等待任务完成
let result = handle.await?;
```

任务特点：
- 轻量级（比线程小得多）
- 可以在任意线程上执行
- 任务切换由运行时管理，不是操作系统

**3. Channels（通道）**

**MPSC（多生产者单消费者）**：

```rust
// 创建通道，缓冲区大小为 100
let (tx, mut rx) = tokio::sync::mpsc::channel(100);

// 发送端可以 clone 给多个任务
let tx2 = tx.clone();

tokio::spawn(async move {
    tx.send("Hello").await.unwrap();
});

tokio::spawn(async move {
    tx2.send("World").await.unwrap();
});

// 接收端唯一
while let Some(msg) = rx.recv().await {
    println!("Got: {}", msg);
}
```

**Oneshot（一次性通道）**：

```rust
let (tx, rx) = tokio::sync::oneshot::channel();

tokio::spawn(async move {
    // 发送后通道关闭
    tx.send("Response").unwrap();
});

// 只能接收一次
let response = rx.await.unwrap();
```

**4. TCP 网络编程**

```rust
// 服务器
let listener = TcpListener::bind("127.0.0.1:8080").await?;

loop {
    let (mut socket, _) = listener.accept().await?;
    
    // 为每个连接 spawn 一个任务
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        socket.write_all(&buf[..n]).await.unwrap();
    });
}
```

### 异步执行流程

```
同步代码:                    异步代码:
┌─────┐                      ┌─────────────┐
│Step1│──→ wait IO ──→ wait  │Step1        │
└─────┘                      │  ┌────────┐ │
┌─────┐                      │  │.await  │─┼─→ runtime 调度其他任务
│Step2│                      │  │yield   │ │
└─────┘                      │  └────────┘ │
                             │Step2 (resume)│
                             │  ┌────────┐ │
                             │  │.await  │─┼─→ 可能再次 yield
                             │  └────────┘ │
                             │Step3        │
                             └─────────────┘
```

---

## 常见错误

### 错误 1: 在 async 函数中使用阻塞操作

```rust
// ❌ 错误：在异步任务中使用阻塞 IO
async fn bad_example() {
    let file = std::fs::File::open("data.txt").unwrap(); // 阻塞！
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // 阻塞！
}

// ✅ 正确：使用 tokio 的异步 IO
async fn good_example() -> tokio::io::Result<()> {
    let contents = tokio::fs::read_to_string("data.txt").await?;
    Ok(())
}

// ✅ 或者使用 spawn_blocking
async fn alternative() {
    let contents = tokio::task::spawn_blocking(|| {
        std::fs::read_to_string("data.txt").unwrap()
    }).await.unwrap();
}
```

**问题**：阻塞操作会阻塞整个线程，影响其他任务执行。

---

### 错误 2: 忘记 drop 发送端导致接收端永远等待

```rust
// ❌ 错误：接收端永远不会结束
async fn bad_mpsc() {
    let (tx, mut rx) = mpsc::channel(10);
    
    tx.send("hello").await.unwrap();
    // tx 没有 drop，rx 会永远等待更多消息
    
    while let Some(msg) = rx.recv().await {
        println!("{}", msg); // 打印一次后卡住
    }
}

// ✅ 正确：显式 drop 发送端
async fn good_mpsc() {
    let (tx, mut rx) = mpsc::channel(10);
    
    tx.send("hello").await.unwrap();
    drop(tx); // 告诉接收端：不会再有消息了
    
    while let Some(msg) = rx.recv().await {
        println!("{}", msg); // 正常结束
    }
}
```

---

### 错误 3: 跨 await 持有锁

```rust
// ❌ 危险：持有 std::sync::MutexGuard 跨越 await
async fn dangerous() {
    let data = std::sync::Mutex::new(0);
    let guard = data.lock().unwrap(); // 阻塞锁
    some_async_op().await; // ❌ 可能持有锁数秒！
    *guard += 1;
}

// ✅ 正确：使用 tokio::sync::Mutex
async fn safe() {
    let data = tokio::sync::Mutex::new(0);
    let mut guard = data.lock().await; // 异步锁
    some_async_op().await; // 锁自动释放，重新获取
    *guard += 1;
}

// ✅ 或者缩小锁的作用域
async fn alternative() {
    let data = std::sync::Mutex::new(0);
    {
        let guard = data.lock().unwrap();
        // 只在同步代码块中使用锁
        let value = *guard;
    } // 锁在这里释放
    some_async_op().await;
}
```

**问题**：`std::sync::Mutex` 的锁可能阻塞其他任务，应该用 `tokio::sync::Mutex`。

---

### 错误 4: 尝试在非 async 上下文中使用 await

```rust
fn sync_function() {
    let result = async_op().await; // ❌ 编译错误！
}

// ✅ 正确：使用 block_on
fn sync_function() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        async_op().await
    });
}

// ✅ 或者将函数改为 async
async fn async_function() {
    let result = async_op().await; // ✅
}
```

---

## 动手练习

### 练习 1: 修复通道示例

下面的代码有什么问题？

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    
    for i in 0..5 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(i).await.unwrap();
        });
    }
    
    while let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
    println!("Done!"); // 这行会执行吗？
}
```

<details>
<summary>点击查看答案与解析</summary>

**答案**：不会执行 "Done!"，因为主 `tx` 没有被 drop，接收端永远等待。

**修复方案**：
```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    
    for i in 0..5 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(i).await.unwrap();
        });
    }
    
    drop(tx); // ✅ 关键：drop 原始发送端
    
    while let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
    println!("Done!"); // ✅ 现在会执行
}
```

</details>

---

### 练习 2: 实现简单的 Echo 服务器

补全下面的代码，实现一个 Echo 服务器：

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        // TODO: spawn 一个新任务处理连接
        // TODO: 读取数据并原样写回
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break, // 连接关闭
                    Ok(n) => {
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("write error: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("read error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

### 练习 3: 理解并发执行

预测以下代码的输出顺序：

```rust
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("Task 1 started");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("Task 1 finished");
    });
    
    tokio::spawn(async {
        println!("Task 2 started");
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("Task 2 finished");
    });
    
    println!("Main continues");
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("Main finished");
}
```

<details>
<summary>点击查看解析</summary>

**可能的输出**（顺序可能不同）：
```
Main continues
Task 1 started
Task 2 started
Task 2 finished
Task 1 finished
Main finished
```

**关键点**：
- `spawn` 的任务立即开始执行，但不一定立即运行
- `await` 让出控制权，让其他任务有机会执行
- 任务 2 先完成（休眠时间更短）
- `main` 函数必须先等待，否则程序会立即退出

</details>

---

## 实际应用

### 应用场景 1: 高性能 Web 服务器

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        // 每个连接一个任务 - 轻松处理数万并发
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            
            match socket.read(&mut buf).await {
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buf[..n]);
                    println!("Request: {}", request);
                    
                    let response = "HTTP/1.1 200 OK\\r\\n\\r\\nHello!";
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
                Err(e) => eprintln!("Failed to read: {}", e),
            }
        });
    }
}
```

---

### 应用场景 2: 任务间协作（生产者-消费者）

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(format!("Message {}", i)).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        // 自动 drop tx
    });
    
    // 消费者任务
    let consumer = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Processing: {}", msg);
            // 模拟处理时间
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        println!("Consumer done");
    });
    
    // 等待两者完成
    let _ = tokio::join!(producer, consumer);
}
```

---

### 应用场景 3: 带超时的请求处理

```rust
use tokio::time::{timeout, Duration};

async fn fetch_data() -> Result<String, reqwest::Error> {
    // 模拟网络请求
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok("Data".to_string())
}

#[tokio::main]
async fn main() {
    // 3 秒超时
    match timeout(Duration::from_secs(3), fetch_data()).await {
        Ok(Ok(data)) => println!("Got: {}", data),
        Ok(Err(e)) => println!("Request failed: {}", e),
        Err(_) => println!("Request timeout!"),
    }
}
```

---

### 应用场景 4: 共享状态（RwLock）

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(0));
    
    let mut handles = vec![];
    
    // 10 个读任务
    for i in 0..10 {
        let data = data.clone();
        handles.push(tokio::spawn(async move {
            let read_guard = data.read().await;
            println!("Reader {}: value = {}", i, *read_guard);
        }));
    }
    
    // 1 个写任务
    let data = data.clone();
    handles.push(tokio::spawn(async move {
        let mut write_guard = data.write().await;
        *write_guard += 1;
        println!("Writer: updated to {}", *write_guard);
    }));
    
    // 等待所有任务
    for handle in handles {
        handle.await.unwrap();
    }
}
```

---

## 故障排查 (FAQ)

### Q: 什么时候用 tokio::sync::Mutex vs std::sync::Mutex？

**A**:

| 场景 | 推荐使用 | 原因 |
|-----|---------|------|
| 数据只在同步代码中使用 | `std::sync::Mutex` | 更快，无额外开销 |
| 锁需要跨越 await 点 | `tokio::sync::Mutex` | 避免阻塞其他任务 |
| 读多写少 | `tokio::sync::RwLock` | 允许多个并发读 |
| 高频短锁 | `parking_lot::Mutex` | 性能更好 |

---

### Q: spawn_blocking 什么时候用？

**A**: 当你需要执行真正的阻塞操作（如 CPU 密集型计算、同步 IO）时：

```rust
// CPU 密集型计算
let result = tokio::task::spawn_blocking(|| {
    heavy_computation() // 会阻塞线程
}).await.unwrap();

// 同步文件 IO
let data = tokio::task::spawn_blocking(|| {
    std::fs::read("file.txt").unwrap()
}).await.unwrap();
```

Tokio 有专门的线程池处理 `spawn_blocking` 任务，不会阻塞 async 任务。

---

### Q: 如何优雅关闭服务器？

**A**: 使用 `tokio::signal` 和 graceful shutdown 模式：

```rust
use tokio::signal;

#[tokio::main]
async fn main() {
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        shutdown_tx.send(()).unwrap();
    });
    
    // 你的服务器代码...
    
    // 等待关闭信号
    shutdown_rx.await.unwrap();
    println!("Shutting down gracefully...");
}
```

---

### Q: async fn 和 fn -> impl Future 有什么区别？

**A**: 本质上一样，但 `async fn` 更简洁：

```rust
// 这两种写法等价
async fn foo() -> i32 { 42 }

fn foo() -> impl Future<Output = i32> {
    async { 42 }
}

// async fn 自动处理生命周期
async fn bar(x: &i32) -> i32 { *x }
// 等价于:
fn bar(x: &i32) -> impl Future<Output = i32> + '_ {
    async move { *x }
}
```

---

## 知识扩展 (选学)

### Select 和 Join

**Join**：等待多个 future 全部完成

```rust
let (a, b) = tokio::join!(
    fetch_from_api1(),
    fetch_from_api2()
);
// a 和 b 都有结果后才继续
```

**Select**：等待多个 future 中任意一个完成

```rust
tokio::select! {
    result = fetch_from_api1() => {
        println!("API1 won: {:?}", result);
    }
    result = fetch_from_api2() => {
        println!("API2 won: {:?}", result);
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("Timeout!");
    }
}
```

---

### Stream 和 Sink

Tokio 提供异步版本的迭代器：

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    
    // StreamExt 提供了异步迭代方法
    while let Some(socket) = stream.next().await {
        println!("New connection");
    }
}
```

---

## 小结

**核心要点**：

1. **Tokio runtime** 是异步程序的引擎，管理任务调度和 IO
2. **`#[tokio::main]`** 是最简单的启动方式
3. **`tokio::spawn`** 创建并发任务，任务轻量且可扩展
4. **`.await`** 暂停当前任务，让出控制权给其他任务
5. **Channels**（`mpsc`, `oneshot`）用于任务间通信
6. **`tokio::sync::Mutex/RwLock`** 用于异步安全的共享状态
7. **`spawn_blocking`** 在专用线程池中执行阻塞操作

**关键术语**：

- **Runtime（运行时）**：管理异步任务执行的引擎
- **Task（任务）**：轻量级执行单元，比线程更高效
- **Future（未来）**：代表异步计算的结果
- **Await（等待）**：暂停并等待 Future 完成
- **Channel（通道）**：任务间消息传递机制
- **Spawn（派生）**：创建新任务

**下一步**：

- 学习 [Tonic](tonic.md) - 基于 Tokio 的 gRPC 框架
- 理解 [Axum](axum.md) - 基于 Tokio 的 Web 框架
- 探索 [Service 框架](../awesome/services.md) - 生产级服务架构

---

## 术语表

| English | 中文 |
|---------|------|
| Async Runtime | 异步运行时 |
| Task | 任务 |
| Spawn | 派生/创建 |
| Await | 等待 |
| Future | 异步未来 |
| Channel | 通道 |
| MPSC | 多生产者单消费者 |
| Oneshot | 一次性通道 |
| RwLock | 读写锁 |
| Non-blocking | 非阻塞 |
| Concurrent | 并发 |
| Event Loop | 事件循环 |

完整示例：[src/advance/tokio_sample.rs](../../src/advance/tokio_sample.rs)

---

## 继续学习

- 下一步：[Tonic gRPC 框架](tonic.md)
- 进阶：[Axum Web 框架](axum.md)
- 回顾：[线程基础](../basic/threads.md)

> 💡 **记住**：异步编程的核心是协作式多任务——任务自愿让出控制权，让其他任务有机会执行。Tokio 让这个模型变得简单高效！

---

## 知识检查点

### 检查点 1 🟢 (基础概念)

以下代码的输出顺序是什么？

```rust
#[tokio::main]
async fn main() {
    println!("A");
    
    tokio::spawn(async {
        println!("B");
    });
    
    println!("C");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("D");
}
```

A) A → B → C → D  
B) A → C → B → D  
C) A → C → D → B  
D) 不确定，每次运行可能不同

<details>
<summary>答案与解析</summary>

**答案**: D) 不确定

**解析**: 
- "A" 和 "C" 一定先打印（同步执行）
- "B" 在 spawn 的任务中，可能在任何时候执行
- "D" 在 sleep 之后
- 实际顺序取决于调度器，可能是 A → C → B → D 或 A → C → D → B
</details>

---

### 检查点 2 🟡 (通道使用)

以下代码会输出什么？

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap(); // ❓ 这里会发生什么
    
    while let Some(n) = rx.recv().await {
        println!("{}", n);
    }
}
```

A) 1 2  
B) 1  
C) 死锁（永远不会结束）  
D) 编译错误

<details>
<summary>答案与解析</summary>

**答案**: C) 死锁

**解析**:
- 通道缓冲区大小为 1，第一个 `send(1)` 成功
- 第二个 `send(2)` 会阻塞等待空间，但接收端还没开始接收
- 程序卡在 `tx.send(2).await`，形成死锁

**修复方案**:
```rust
let (tx, mut rx) = tokio::sync::mpsc::channel(2); // 增大缓冲区
// 或者
let tx_clone = tx.clone();
tokio::spawn(async move { tx_clone.send(2).await }); // 在不同任务发送
```
</details>

---

### 检查点 3 🔴 (生命周期理解)

以下代码为什么编译失败？如何修复？

```rust
async fn process_data(data: &str) {
    let handle = tokio::spawn(async {
        println!("{}", data); // ❌ 编译错误
    });
    handle.await.unwrap();
}
```

A) data 生命周期不够长  
B) spawn 要求 'static 闭包  
C) 应该用 String 而不是 &str  
D) 以上都是

<details>
<summary>答案与解析</summary>

**答案**: D) 以上都是

**解析**:
1. `tokio::spawn` 要求闭包是 `'static`（可能运行任意长时间）
2. `&str` 引用不能保证在整个任务生命周期有效
3. 解决方案是拥有数据的所有权：

```rust
async fn process_data(data: String) {
    let handle = tokio::spawn(async move {
        println!("{}", data); // ✅ 现在 data 是 move 进来的
    });
    handle.await.unwrap();
}
```

**关键规则**: `spawn` 的任务必须是 `'static`，不能借用外部数据。
</details>

---

## 扩展阅读

### 官方资源

- [Tokio 官方文档](https://tokio.rs/)
- [Tokio API 文档](https://docs.rs/tokio/)
- [Async Book](https://rust-lang.github.io/async-book/)

### 相关项目

- [Hyper](https://hyper.rs/) - 基于 Tokio 的 HTTP 库
- [Axum](https://github.com/tokio-rs/axum) - 基于 Tokio 的 Web 框架
- [Tonic](https://github.com/hyperium/tonic) - 基于 Tokio 的 gRPC 框架
- [Tower](https://github.com/tower-rs/tower) - 服务中间件抽象

### 进阶主题

- **Work-stealing scheduler**：Tokio 的工作窃取调度算法
- **io_uring**：Linux 下的高性能异步 IO（tokio-uring）
- **Tracing**：异步程序的性能分析和调试
