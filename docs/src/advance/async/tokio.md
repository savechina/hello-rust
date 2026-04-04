# Tokio 异步运行时

## 开篇故事

想象你在经营一家快递公司。如果只有一个快递员（单线程），他必须按顺序送每个包裹：送完 A 才能送 B，送完 B 才能送 C。这很慢。

如果你雇了多个快递员（多线程），他们可以同时送包裹——但协调成本很高：谁送哪个？怎么避免重复？

Tokio 就像一位超级调度员：它管理一个快递员团队，当某个快递员等待客户开门时（I/O 阻塞），立刻安排他去送下一个包裹。这样每个快递员都在高效工作，不会浪费时间等待。这就是 Tokio 的核心思想：**在等待 I/O 时做其他事情**。

---

## 本章适合谁

如果你想编写高并发网络服务，或者理解 Rust 异步编程的实际应用，本章适合你。Tokio 是 Rust 生态的事实标准异步运行时。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Tokio 运行时的核心组件
2. 使用 `#[tokio::main]` 启动异步程序
3. 使用 `tokio::spawn` 创建异步任务
4. 使用 `mpsc` 通道在任务间传递消息
5. 使用 `RwLock` 安全共享状态
6. 使用 `spawn_blocking` 运行阻塞代码
7. 理解 oneshot 通道的单次通信模式

---

## 前置要求

- [异步编程](async.md) - async/await 基础
- [线程与并发](../../basic/threads.md) - 并发概念

---

## 第一个例子

最简单的 Tokio 服务器：

```rust,ignore
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器启动在 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    return; // 客户端断开
                }
                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}
```

> 💡 **注意**：此代码需要客户端配合运行。请使用 `cargo run --bin echo_server` 和客户端程序进行完整测试。

**发生了什么？**

- `#[tokio::main]` - 启动 Tokio 运行时
- `TcpListener::bind` - 绑定端口
- `tokio::spawn` - 为每个连接创建异步任务
- 服务器可以**并发**处理多个客户端

---

## 原理解析

### 1. Tokio 运行时架构

```
Tokio 运行时
├── Reactor (I/O 多路复用)
│   ├── epoll (Linux)
│   ├── kqueue (macOS)
│   └── IOCP (Windows)
├── Scheduler (任务调度)
│   ├── 工作窃取 (work-stealing)
│   └── 多线程调度
└── Timer (定时器)
    └── 时间轮算法
```

### 2. 异步任务创建

```rust
// 方式 1: #[tokio::main]
#[tokio::main]
async fn main() {
    println!("主函数");
}

// 方式 2: 手动创建运行时
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("手动运行时");
    });
}

// 方式 3: tokio::spawn
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("子任务");
    });
    handle.await.unwrap();
}
```

### 3. 通道通信 (mpsc)

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 创建异步通道（缓冲区大小 100）
    let (tx, mut rx) = mpsc::channel(100);

    // 发送端任务
    let task = tokio::spawn(async move {
        let val = String::from("hello from tokio");
        tx.send(val).await.unwrap();
        drop(tx); // 重要！关闭发送端
    });

    // 接收端
    if let Some(received) = rx.recv().await {
        println!("收到：{}", received);
    }

    task.await.unwrap();
}
```

**mpsc 特点**：
- **m**ultiple **p**roducer, **s**ingle **c**onsumer
- 多发送端，单接收端
- 异步非阻塞发送

### 4. 多任务并发

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    // 创建 50 个并发任务
    for i in 0..50 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let val = format!("task {}", i);
            tx_clone.send(val).await.unwrap();
        });
    }

    // 关闭发送端
    drop(tx);

    // 接收所有消息
    while let Some(received) = rx.recv().await {
        println!("收到：{}", received);
    }
}
```

### 5. 共享状态 (RwLock)

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(0));

    // 多个读任务
    let mut read_tasks = Vec::new();
    for _ in 0..5 {
        let data_clone = data.clone();
        read_tasks.push(tokio::spawn(async move {
            let read_guard = data_clone.read().await;
            println!("读取：{}", *read_guard);
        }));
    }

    // 一个写任务
    let data_clone = data.clone();
    let write_task = tokio::spawn(async move {
        let mut write_guard = data_clone.write().await;
        *write_guard += 1;
        println!("写入：{}", *write_guard);
    });

    // 等待所有任务
    for task in read_tasks {
        task.await.unwrap();
    }
    write_task.await.unwrap();
}
```

**RwLock 特点**：
- 允许多个读或一个写
- 读锁不互斥，写锁独占
- 适合读多写少场景

### 6. 阻塞代码 (spawn_blocking)

```rust
use parking_lot::RwLock;

#[tokio::main]
async fn main() {
    let data = Arc::new(parking_lot::RwLock::new(0));

    // 在阻塞线程池中运行
    let result = tokio::task::spawn_blocking({
        let data = data.clone();
        move || {
            let mut write_guard = data.write();
            *write_guard += 1;
            *write_guard
        }
    }).await.unwrap();

    println!("结果：{}", result);
}
```

**为什么需要 spawn_blocking？**
- Tokio 是异步运行时，不适合 CPU 密集型或阻塞操作
- `spawn_blocking` 将任务移到专用阻塞线程池
- 避免阻塞异步运行时的主线程

### 7. Oneshot 通道

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("hello").unwrap();
    });

    let result = rx.await.unwrap();
    println!("收到：{}", result);
}
```

**Oneshot 特点**：
- 只能发送一次
- 适合请求-响应模式
- 比 mpsc 更轻量

---

## 常见错误

### 错误 1: 忘记 drop 发送端

```rust
// ❌ 错误：接收端会永远等待
let (tx, mut rx) = mpsc::channel(100);
tokio::spawn(async move {
    tx.send("hello").await.unwrap();
    // 忘记 drop(tx)
});

while let Some(msg) = rx.recv().await {
    println!("{}", msg);
} // 这里会死锁！

// ✅ 正确：关闭发送端
let (tx, mut rx) = mpsc::channel(100);
tokio::spawn(async move {
    tx.send("hello").await.unwrap();
    drop(tx); // 重要！
});
```

### 错误 2: 在异步上下文中使用阻塞操作

```rust
// ❌ 错误：阻塞异步运行时
#[tokio::main]
async fn main() {
    std::thread::sleep(Duration::from_secs(1)); // 阻塞！
}

// ✅ 正确：使用异步等待
#[tokio::main]
async fn main() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 错误 3: 共享随机数生成器

```rust
// ❌ 错误：thread_rng 不是 Send
tokio::spawn(async move {
    let mut rng = rand::thread_rng(); // 编译错误！
});

// ✅ 正确：每个任务创建自己的 rng
tokio::spawn(async move {
    let mut rng = StdRng::from_entropy();
    let num = rng.gen_range(0..100);
});
```

---

## 动手练习

### 练习 1: 实现 Echo 服务器

创建一个 TCP 服务器，回显客户端发送的消息：

```rust
// TODO: 实现 echo_server 函数
// 监听 127.0.0.1:8080
// 为每个连接创建异步任务
// 读取数据并原样返回
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    return;
                }
                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}
```

> 💡 **注意**：此代码需要客户端配合运行。请使用 `cargo run` 启动服务器后，再使用客户端程序连接测试。

</details>

---

## 故障排查

### Q: Tokio 和 async-std 有什么区别？

**A**: 
- **Tokio**: 功能更全，生态更大，性能更好
- **async-std**: 标准库风格，更轻量
- 推荐：生产环境用 Tokio

### Q: 如何选择通道类型？

**A**:
- `mpsc` - 多发送端，单接收端
- `oneshot` - 单次通信
- `broadcast` - 多发送端，多接收端
- `watch` - 单发送端，多接收端，只保留最新值

### Q: RwLock 和 Mutex 如何选择？

**A**:
- `RwLock` - 读多写少
- `Mutex` - 读写均衡或写多
- Tokio 的 `RwLock` 是异步版本

---

## 知识扩展（选学）

### Tokio 性能调优

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

**运行时配置**：
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 4 个工作线程
}
```

---

## 小结

**核心要点**：

1. **#[tokio::main]**: 启动异步运行时
2. **tokio::spawn**: 创建异步任务
3. **mpsc**: 多发送端通道
4. **RwLock**: 读写锁共享状态
5. **spawn_blocking**: 运行阻塞代码

**关键术语**：

- **Runtime**: 运行时
- **Executor**: 执行器
- **Reactor**: I/O 反应器
- **Work-stealing**: 工作窃取
- **Channel**: 通道

---

## 术语表

| English       | 中文       |
| ------------- | ---------- |
| Runtime       | 运行时     |
| Async Task    | 异步任务   |
| Channel       | 通道       |
| Work-stealing | 工作窃取   |
| Blocking      | 阻塞       |
| Spawn         | 生成任务   |

---

完整示例：[src/advance/tokio_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/tokio_sample.rs)

---

## 知识检查

**快速测验**（答案在下方）：

1. `mpsc` 通道的 `m`、`p`、`s`、`c` 分别代表什么？

2. 为什么发送端需要 `drop(tx)`？

3. `RwLock` 和 `Mutex` 如何选择？

<details>
<summary>点击查看答案与解析</summary>

1. **m**ultiple **p**roducer, **s**ingle **c**onsumer
2. 不 drop 的话接收端会永远等待（认为还有更多消息）
3. 读多写少用 `RwLock`，读写均衡用 `Mutex`

**关键理解**: Tokio 的通道是异步的，与 std::sync::mpsc 不同。
</details>

## 继续学习

- 下一步：[Futures 异步编程](futures.md)
- 进阶：[数据库操作](../database/database.md)
- 回顾：[异步编程](async.md)

> 💡 **记住**：Tokio 是 Rust 异步编程的基石。掌握它，你就能构建高并发服务！
