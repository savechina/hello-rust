# Tokio 异步运行时

## 开篇故事

想象你经营一家餐厅。传统方式是：一个服务员服务一桌客人，客人点餐后服务员等待厨房做好，再端给客人。这很慢！现代方式是：服务员记下订单给厨房，然后去服务其他客人，菜好了会通知。Tokio 就像这个高效的服务员系统——它让你能同时处理成千上万个任务，而不会阻塞等待。

---

## 本章适合谁

如果你已经了解了 Rust 基础，现在想学习如何编写高性能的异步程序，本章适合你。Tokio 是 Rust 生态中最常用的异步运行时，是构建网络服务、并发系统的基石。

---

## 你会学到什么

完成本章后，你可以：

1. 理解异步编程的基本概念
2. 使用 Tokio 运行时执行异步任务
3. 创建 TCP 服务器和客户端
4. 使用 async/await 语法编写异步代码
9. 使用 spawn 并发执行任务
5. 理解 Future 和任务调度

---

## 前置要求

- [基础数据类型](../basic/datatype.md) - 基本类型
- [闭包](../basic/closure.md) - 闭包语法
- [特征](../basic/trait.md) - trait 基础

---

## 第一个例子

最简单的 Tokio 异步程序：

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 绑定监听地址
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听在 127.0.0.1:8080");

    loop {
        // 接受新连接
        let (socket, _) = listener.accept().await?;
        println!("新连接建立!");
        
        // 为每个连接 spawn 一个任务
        tokio::spawn(async move {
            // 处理连接...
        });
    }
}
```

**完整示例**: [tokio_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/tokio_sample.rs)

---

## 原理解析

### Tokio 运行时

**什么是运行时**：

Tokio 是一个异步运行时，它负责：
- 调度异步任务（tasks）
- 管理 I/O 资源（sockets, files）
- 提供定时器、间隔等工具

```rust
// 使用宏创建运行时
#[tokio::main]
async fn main() {
    // 异步代码在这里
}

// 或手动创建运行时
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // 异步代码
    });
}
```

### async/await 语法

**async 函数**：

```rust
// 同步函数
fn sync_hello() {
    println!("Hello");
}

// 异步函数
async fn async_hello() {
    println!("Hello");
}
```

**await 表达式**：

```rust
async fn fetch_data() -> String {
    // await 等待异步操作完成
    let data = read_file().await;
    data
}
```

### 任务生成 (spawn)

**spawn 创建并发任务**：

```rust
tokio::spawn(async move {
    // 这个任务独立运行
    println!("Hello from task!");
});
```

**任务特点**：
- 轻量级（比线程轻得多）
- 由 Tokio 调度
- 可以并发执行成千上万个

### TCP 服务器示例

**完整服务器代码**：

```rust
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
                    return; // 连接关闭
                }
                
                // 回显数据
                socket.write_all(&buf[..n]).await.unwrap();
            }
        });
    }
}
```

### TCP 客户端示例

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    // 发送数据
    stream.write_all(b"Hello, server!").await?;
    
    // 读取响应
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    
    println!("收到：{}", String::from_utf8_lossy(&buf[..n]));
    
    Ok(())
}
```

---

## 常见错误

### 错误 1: 在同步上下文中 await

```rust
fn main() {
    some_async_function().await;  // ❌ 编译错误
}
```

**错误信息**:
```
error[E0728]: `await` is only allowed inside `async` functions and blocks
```

**修复方法**:
```rust
#[tokio::main]
async fn main() {
    some_async_function().await;  // ✅
}
```

### 错误 2: 忘记 spawn

```rust
#[tokio::main]
async fn main() {
    // 这些会顺序执行，不是并发
    task1().await;
    task2().await;
    task3().await;
}
```

**修复方法**:
```rust
#[tokio::main]
async fn main() {
    // 并发执行
    let t1 = tokio::spawn(task1());
    let t2 = tokio::spawn(task2());
    let t3 = tokio::spawn(task3());
    
    // 等待所有任务完成
    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();
}
```

### 错误 3: 阻塞异步运行时

```rust
#[tokio::main]
async fn main() {
    // ❌ 这会阻塞整个运行时
    std::thread::sleep(Duration::from_secs(5));
}
```

**修复方法**:
```rust
#[tokio::main]
async fn main() {
    // ✅ 使用异步 sleep
    tokio::time::sleep(Duration::from_secs(5)).await;
}
```

---

## 动手练习

### 练习 1: 创建简单的异步函数

```rust
// TODO: 定义一个异步函数
// - 返回 String
// - 打印 "Hello from async!"
// - 返回 "Done"

#[tokio::main]
async fn main() {
    // TODO: 调用异步函数并打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
use tokio::main;

async fn async_task() -> String {
    println!("Hello from async!");
    "Done".to_string()
}

#[tokio::main]
async fn main() {
    let result = async_task().await;
    println!("Result: {}", result);
}
```
</details>

### 练习 2: 并发执行多个任务

```rust
async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }
async fn task3() -> i32 { 30 }

#[tokio::main]
async fn main() {
    // TODO: 并发执行 3 个任务
    // TODO: 计算总和
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
use tokio::main;

async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }
async fn task3() -> i32 { 30 }

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(task1());
    let t2 = tokio::spawn(task2());
    let t3 = tokio::spawn(task3());
    
    let sum = t1.await.unwrap() + t2.await.unwrap() + t3.await.unwrap();
    println!("总和：{}", sum);  // 60
}
```
</details>

### 练习 3: 实现回显服务器

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 绑定监听地址
    // TODO: 接受连接
    // TODO: 为每个连接 spawn 任务
    // TODO: 读取数据并回显
}
```

<details>
<summary>点击查看答案</summary>

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听在 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    return;
                }
                socket.write_all(&buf[..n]).await.unwrap();
            }
        });
    }
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Tokio 和 async-std 有什么区别？

**A**: 
- **Tokio**: 最成熟，生态最丰富，性能最优
- **async-std**: 标准库风格，API 更简洁
- **推荐**: Tokio（社区支持更好）

### Q: 什么时候使用 spawn？

**A**: 
- 需要并发执行多个任务时
- 任务可能运行很长时间
- 需要任务独立运行（即使主函数返回）

```rust
// 顺序执行
task1().await;
task2().await;

// 并发执行
let t1 = tokio::spawn(task1());
let t2 = tokio::spawn(task2());
```

### Q: spawn 和 tokio::task::spawn_local 有什么区别？

**A**: 
- **spawn**: 可以在任何线程运行
- **spawn_local**: 只能在当前线程运行（更轻量）

---

## 知识扩展

### 多运行时配置

```rust
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)  // 4 个工作线程
        .thread_name("my-worker")
        .build()
        .unwrap();
    
    rt.block_on(async {
        // 异步代码
    });
}
```

### 通道 (Channel)

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        tx.send("Hello").await.unwrap();
    });
    
    let msg = rx.recv().await.unwrap();
    println!("收到：{}", msg);
}
```

### 定时器

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(2)).await;
    println!("2 秒后执行");
}
```

---

## 小结

**核心要点**：

1. **Tokio 运行时**: 异步任务调度器
2. **async/await**: Rust 异步语法
3. **spawn**: 并发执行任务
4. **TCP 网络**: TcpListener, TcpStream
5. **非阻塞**: 使用 async sleep 而非 thread::sleep

**关键术语**：

- **Async (异步)**: 不阻塞执行的操作
- **Await (等待)**: 等待异步操作完成
- **Runtime (运行时)**: 执行异步代码的环境
- **Task (任务)**: 轻量级并发单元
- **Future (未来值)**: 表示可能未完成的计算结果

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Async | 异步 |
| Await | 等待 |
| Runtime | 运行时 |
| Task | 任务 |
| Future | 未来值 |
| Spawn | 生成任务 |
| Channel | 通道 |

---

## 继续学习

**前一章**: [并发异步编程](async.md)  
**下一章**: [数据库操作 (SQLx)](database.md)

**相关章节**:
- [并发异步编程](async.md)
- [Futures 异步编程](futures.md)
- [数据库操作](database.md)

**返回**: [高级进阶](advance-overview.md)

---

**完整示例**: [tokio_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/tokio_sample.rs)
