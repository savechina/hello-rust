# Mio 异步 I/O 库

## 开篇故事

想象你是一位邮局管理员，负责处理成千上万封信件。传统方式是依次处理每封信（**阻塞 I/O**）：拿起一封信 → 读取 → 回复 → 放下 → 拿下一封。如果某封信需要等待回复，你就干等着，什么也做不了。

更聪明的做法是同时观察所有信箱，哪个有新信件就处理哪个（**非阻塞 I/O**）。这就是 Mio 的核心思想——它提供低级的异步 I/O 原语，是 Tokio 等高级运行时的基石。

---

## 本章适合谁

如果你想深入理解 Rust 异步 I/O 的底层实现原理，或者需要构建高性能的网络服务，本章适合你。Mio 是 Tokio、Hyper 等库的核心依赖。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Mio 的设计理念：为什么它是"底层"的
2. 使用 Poll、Token、Event 实现事件循环
3. 构建非阻塞 TCP 服务器
4. 理解 epoll/kqueue 等系统调用抽象
5. 区分 Mio 与 Tokio 的使用场景

---

## 前置要求

- [异步编程](async.md) - async/await 基础
- [TCP 网络编程](../web/hyper.md) - 网络基础
- [所有权](../../basic/ownership.md) - 所有权和借用

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add mio --features os-poll,net
```

## 第一个例子

最简单的 Mio TCP 服务器：

```rust
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::io::{self, Read};

const SERVER: Token = Token(0);

fn main() -> io::Result<()> {
    // 创建 Poll（事件轮询器）
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    // 绑定并监听端口
    let addr = "127.0.0.1:9000".parse()?;
    let mut server = TcpListener::bind(addr)?;

    // 注册服务器到 Poll
    poll.registry().register(
        &mut server,
        SERVER,
        Interest::READABLE,
    )?;

    println!("服务器启动在 127.0.0.1:9000");

    // 事件循环
    loop {
        // 阻塞等待事件
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // 接受新连接
                    let (mut stream, _) = server.accept()?;
                    println!("新连接！");
                }
                _ => unreachable!(),
            }
        }
    }
}
```

**发生了什么？**

- `Poll::new()` - 创建事件轮询器
- `register()` - 注册 I/O 源到 Poll
- `poll()` - 阻塞等待事件
- `Events` - 存储就绪事件的集合

---

## 原理解析

### 1. Mio 事件循环架构

```
┌─────────────────────────────────────────────────────────┐
│                    Mio 事件循环                          │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│  Poll (事件轮询器)                                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │  TcpListener│  │  TcpStream  │  │  UdpSocket  │     │
│  │  Token(0)   │  │  Token(1)   │  │  Token(2)   │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
└─────────┼────────────────┼────────────────┼─────────────┘
          │                │                │
          ▼                ▼                ▼
┌─────────────────────────────────────────────────────────┐
│  系统调用 (epoll/kqueue/IOCP)                            │
│  - Linux:   epoll_wait()                                │
│  - macOS:   kqueue()                                    │
│  - Windows: IOCP                                        │
└─────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────┐
│  Events (就绪事件集合)                                   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Event { token: Token(0), readable: true }       │   │
│  │ Event { token: Token(1), writable: true }       │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────┐
│  Handler (事件处理器)                                    │
│  - 根据 Token 分发事件                                   │
│  - 处理可读/可写事件                                     │
└─────────────────────────────────────────────────────────┘
```

### 2. 核心概念详解

**Poll (事件轮询器)**:
```rust
let mut poll = Poll::new()?;
```
- 封装操作系统的 I/O 多路复用机制
- Linux 下使用 `epoll`
- macOS 下使用 `kqueue`
- Windows 下使用 `IOCP`

**Token (标识符)**:
```rust
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);
```
- 用户自定义的整数标识
- 用于区分不同的 I/O 源
- 必须唯一

**Event (事件)**:
```rust
for event in events.iter() {
    if event.is_readable() {
        // 处理可读事件
    }
    if event.is_writable() {
        // 处理可写事件
    }
}
```

**Interest (关注事件)**:
```rust
Interest::READABLE   // 关注可读事件
Interest::WRITABLE   // 关注可写事件
Interest::READABLE | Interest::WRITABLE  // 同时关注
```

### 3. 完整 TCP Echo 服务器

```rust
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::{self, Read, Write};

const SERVER: Token = Token(0);

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:9000".parse()?;
    let mut server = TcpListener::bind(addr)?;

    poll.registry().register(
        &mut server,
        SERVER,
        Interest::READABLE,
    )?;

    // 存储客户端连接
    let mut clients: HashMap<Token, TcpStream> = HashMap::new();
    let mut next_token = 1;

    println!("Echo 服务器启动在 127.0.0.1:9000");

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // 接受新连接
                    loop {
                        match server.accept() {
                            Ok((mut stream, _)) => {
                                let token = Token(next_token);
                                next_token += 1;

                                poll.registry().register(
                                    &mut stream,
                                    token,
                                    Interest::READABLE,
                                )?;

                                clients.insert(token, stream);
                                println!("新客户端连接：{}", token.0);
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                eprintln!("接受连接失败：{}", e);
                                break;
                            }
                        }
                    }
                }
                token => {
                    // 处理客户端数据
                    if let Some(mut stream) = clients.remove(&token) {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(0) => {
                                // 连接关闭
                                println!("客户端断开：{}", token.0);
                                poll.registry().deregister(&mut stream)?;
                            }
                            Ok(n) => {
                                // Echo 回数据
                                let data = &buf[..n];
                                stream.write_all(data)?;
                                println!("Echo {} 字节给客户端 {}", n, token.0);

                                // 重新注册
                                poll.registry().register(
                                    &mut stream,
                                    token,
                                    Interest::READABLE,
                                )?;
                                clients.insert(token, stream);
                            }
                            Err(e) => {
                                eprintln!("读取失败：{}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### 4. Mio vs Tokio 对比

| 特性         | Mio                    | Tokio                  |
| ------------ | ---------------------- | ---------------------- |
| **抽象级别**   | 低级（接近系统调用）   | 高级（异步运行时）       |
| **编程模型**   | 手动事件循环           | async/await            |
| **任务调度**   | 无                     | 工作窃取调度器           |
| **适用场景**   | 自定义运行时、极致性能 | 大多数异步应用           |
| **学习曲线**   | 陡峭                   | 平缓                   |
| **代码复杂度** | 高（手动管理状态）     | 低（编译器管理状态）     |

**何时使用 Mio？**
- 构建自定义异步运行时
- 需要极致性能控制
- 学习异步 I/O 底层原理

**何时使用 Tokio？**
- 构建普通网络服务
- 需要丰富的生态系统
- 快速开发

---

## 常见错误

### 错误 1: 忘记注册事件源

```rust
// ❌ 错误：未注册就等待事件
let mut stream = TcpStream::connect(addr)?;
poll.poll(&mut events, None)?;  // 永远不会收到事件

// ✅ 正确：先注册
let mut stream = TcpStream::connect(addr)?;
poll.registry().register(
    &mut stream,
    Token(1),
    Interest::READABLE,
)?;
```

### 错误 2: Token 冲突

```rust
// ❌ 错误：两个源使用相同 Token
poll.registry().register(&mut server, Token(0), Interest::READABLE)?;
poll.registry().register(&mut client, Token(0), Interest::READABLE)?;  // 冲突！

// ✅ 正确：使用唯一 Token
poll.registry().register(&mut server, Token(0), Interest::READABLE)?;
poll.registry().register(&mut client, Token(1), Interest::READABLE)?;
```

### 错误 3: 阻塞操作

```rust
// ❌ 错误：在事件循环中阻塞
loop {
    poll.poll(&mut events, None)?;
    for event in events.iter() {
        std::thread::sleep(Duration::from_secs(1));  // 阻塞整个事件循环！
    }
}

// ✅ 正确：非阻塞操作
loop {
    poll.poll(&mut events, None)?;
    for event in events.iter() {
        // 快速处理事件
        handle_event(event);
    }
}
```

---

## 动手练习

### 练习 1: 实现简单的 TCP 服务器

创建一个监听 9000 端口的服务器，接受连接并打印客户端地址：

```rust
// TODO: 实现服务器
// 1. 创建 Poll
// 2. 绑定 TcpListener
// 3. 注册到 Poll
// 4. 事件循环接受连接
```

<details>
<summary>点击查看答案</summary>

```rust
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::io;

const SERVER: Token = Token(0);

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:9000".parse()?;
    let mut server = TcpListener::bind(addr)?;

    poll.registry().register(
        &mut server,
        SERVER,
        Interest::READABLE,
    )?;

    println!("服务器启动");

    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            if event.token() == SERVER {
                if let Ok((stream, addr)) = server.accept() {
                    println!("新连接：{}", addr);
                }
            }
        }
    }
}
```
</details>

---

## 故障排查

### Q: Mio 和 epoll 是什么关系？

**A**: Mio 是跨平台抽象层：
- Linux → epoll
- macOS → kqueue
- Windows → IOCP

### Q: 为什么 Mio 代码这么复杂？

**A**: Mio 是低级库，需要手动管理状态。大多数情况下应该使用 Tokio。

### Q: 如何处理大量连接？

**A**: 使用 `HashMap<Token, TcpStream>` 存储连接，用递增 Token 标识。

---

## 小结

**核心要点**：

1. **Poll**: 事件轮询器，封装系统调用
2. **Token**: 唯一标识 I/O 源
3. **Event**: 就绪事件
4. **Interest**: 关注的事件类型
5. **事件循环**: 持续轮询和处理事件

**关键术语**：

- **Event Loop**: 事件循环
- **Non-blocking I/O**: 非阻塞 I/O
- **epoll/kqueue/IOCP**: 系统调用
- **Token**: 标识符
- **Registry**: 注册表

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| Event Loop        | 事件循环   |
| Poll              | 轮询器     |
| Token             | 标识符     |
| Non-blocking I/O  | 非阻塞 I/O |
| epoll             | Linux 事件 |
| kqueue            | macOS 事件 |

---

完整示例：[src/advance/mio_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/mio_sample.rs)

---

## 知识检查

**快速测验**（答案在下方）：

1. Mio 和 Tokio 是什么关系？

2. `Interest::READABLE` 表示什么？

3. 为什么事件循环中不能有阻塞操作？

<details>
<summary>点击查看答案与解析</summary>

1. Mio 是底层 I/O 抽象层，Tokio 在其上构建运行时
2. 关注源的可读事件（有数据到达）
3. 阻塞操作会阻止事件循环处理其他就绪事件

**关键理解**: Mio 提供原始 I/O 原语，Tokio 提供高级异步抽象。
</details>

## 继续学习

- 下一步：[并行计算](rayon.md)
- 进阶：[Tokio 运行时](tokio.md)
- 回顾：[异步编程](async.md)

> 💡 **记住**：Mio 是异步 I/O 的基石。理解它，你就能理解所有异步运行时！
