# Unix Domain Socket (UDS)

## 开篇故事

想象你在一家大公司工作。如果两个部门需要通信，传统方式是打电话（网络 Socket）——即使他们在同一栋楼里。但更高效的方式是直接走到对方办公室（Unix Domain Socket）——因为他们在同一台机器上，无需经过外部网络。

Unix Domain Socket（UDS）是同一台机器上进程间通信（IPC）的高效方式。它使用文件系统路径作为地址，比 TCP/IP 快得多，因为数据不需要经过网络协议栈。

---

## 本章适合谁

如果你想学习：
- 如何在同一台机器上实现高效的进程间通信
- Unix Domain Socket 的工作原理
- 如何设计自定义二进制协议

本章适合你。UDS 是本地 IPC 的首选方案。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Unix Domain Socket 的核心概念
2. 使用 `std::os::unix::net` 创建 UDS 服务端
3. 使用 `std::os::unix::net` 创建 UDS 客户端
4. 实现自定义二进制协议（长度前缀 + 负载）
5. 使用父进程编排多个子进程

---

## 前置要求

- [进程管理](../advance/system/process.md) - 进程创建和管理
- [字节处理](../advance/system/bytes.md) - 字节操作基础

---

### 依赖安装

UDS 使用 Rust 标准库，无需额外依赖：

```rust
use std::os::unix::net::{UnixListener, UnixStream};
```

---

## 第一个例子

最简单的 UDS 服务端：

```rust,ignore
use std::fs;
use std::os::unix::net::UnixListener;
use std::io;

fn main() -> io::Result<()> {
    let socket_path = "/tmp/hello.socket";
    
    // 清理已存在的 socket 文件
    let _ = fs::remove_file(socket_path);
    
    // 创建并绑定 socket
    let listener = UnixListener::bind(socket_path)?;
    println!("UDS 服务端启动在 {}", socket_path);
    
    // 接受连接
    match listener.accept() {
        Ok((stream, _addr)) => {
            println!("客户端已连接");
            // 处理客户端请求...
        }
        Err(e) => eprintln!("接受连接失败: {}", e),
    }
    
    // 清理
    fs::remove_file(socket_path)?;
    Ok(())
}
```

> 💡 **注意**：此代码需要服务端和客户端配合运行。请使用 `cargo run --bin uds_server` 和 `cargo run --bin uds_client` 进行完整测试。

完整示例：
- [uds_server.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_server.rs)
- [uds_client.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_client.rs)
- [uds_parent.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_parent.rs)

---

## 原理解析

### UDS 架构概览

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Unix Domain Socket 架构                           │
│                                                                      │
│   ┌──────────────┐         ┌──────────────┐         ┌──────────────┐│
│   │   Parent     │         │   Server     │         │   Client     ││
│   │  (编排进程)   │────────→│  (UDS 监听)   │←────────│  (UDS 连接)   ││
│   │              │  spawn   │              │  accept  │              ││
│   └──────────────┘         └──────┬───────┘         └──────────────┘│
│                                   │                                  │
│   通信协议:                                                                │
│   ┌─────────────────────────────────────────────────────────────────┐ │
│   │ 4 字节大端长度前缀 + UTF-8 负载                                   │ │
│   │ ┌─────────────┐ ┌───────────────────────────────────────────┐   │ │
│   │ │ Length (4B) │ │ Payload (UTF-8 string)                    │   │ │
│   │ │ 0x00000005  │ │ "hello"                                   │   │ │
│   │ └─────────────┘ └───────────────────────────────────────────┘   │ │
│   └─────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│   Socket 路径: /tmp/hello.socket                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 自定义二进制协议

**协议格式**：

```
┌─────────────┬──────────────────────────────────────────┐
│ Length (4B) │ Payload (UTF-8 string)                   │
│ 大端序       │ 可变长度                                  │
└─────────────┴──────────────────────────────────────────┘
```

**为什么使用长度前缀？**
- 接收方知道要读取多少字节
- 避免粘包问题
- 支持可变长度消息

### UDS 服务端实现

```rust,ignore
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    loop {
        // 1. 读取长度前缀（4 字节大端序）
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes) as usize;

        // 2. 读取负载
        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;
        let message = String::from_utf8(payload).expect("Invalid UTF-8");

        // 3. 处理消息（反转字符串）
        let response = if message == "done" {
            String::from("ok")
        } else {
            message.chars().rev().collect()
        };

        // 4. 发送响应
        let resp_payload = response.as_bytes();
        stream.write_all(&u32::to_be_bytes(resp_payload.len() as u32))?;
        stream.write_all(resp_payload)?;
        stream.flush()?;

        if message == "done" {
            break;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket_path = "/tmp/hello.socket";
    
    // 清理已存在的 socket 文件
    let _ = fs::remove_file(socket_path);
    
    // 创建并绑定 socket
    let listener = UnixListener::bind(socket_path)?;
    println!("服务端启动在 {}", socket_path);
    
    // 接受连接并处理
    match listener.accept() {
        Ok((stream, _addr)) => handle_client(stream)?,
        Err(e) => eprintln!("接受连接失败: {}", e),
    }
    
    // 清理
    fs::remove_file(socket_path)?;
    Ok(())
}
```

> 💡 **注意**：此代码需要服务端配合运行。请使用 `cargo run --bin uds_server` 进行完整测试。

### UDS 客户端实现

```rust,ignore
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> io::Result<()> {
    let socket_path = "/tmp/hello.socket";
    let messages = vec!["hello", "world", "rust", "done"];

    // 连接到服务端
    let mut stream = UnixStream::connect(socket_path)?;

    for msg in messages {
        // 1. 发送：长度前缀 + 负载
        let payload = msg.as_bytes();
        stream.write_all(&u32::to_be_bytes(payload.len() as u32))?;
        stream.write_all(payload)?;
        stream.flush()?;

        // 2. 接收：长度前缀 + 响应
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes) as usize;
        let mut resp_payload = vec![0u8; length];
        stream.read_exact(&mut resp_payload)?;
        let resp_str = String::from_utf8(resp_payload).expect("Invalid UTF-8");
        
        println!("收到响应: {}", resp_str);
    }

    Ok(())
}
```

> 💡 **注意**：此代码需要服务端配合运行。请使用 `cargo run --bin uds_client` 进行完整测试。

### 父进程编排

```rust,ignore
use std::fs;
use std::io;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let socket_path = "/tmp/hello.socket";
    
    // 确保 socket 文件干净
    let _ = fs::remove_file(socket_path);
    
    // 启动服务端
    let mut server = Command::new("./uds_server")
        .stdout(Stdio::inherit())
        .spawn()?;
    
    // 等待服务端绑定
    thread::sleep(Duration::from_millis(500));
    
    // 启动客户端
    let mut client = Command::new("./uds_client")
        .stdout(Stdio::inherit())
        .spawn()?;
    
    // 等待完成
    client.wait()?;
    server.wait()?;
    
    // 清理
    let _ = fs::remove_file(socket_path);
    
    Ok(())
}
```

> 💡 **注意**：此代码需要服务端和客户端配合运行。请使用 `cargo run --bin uds_parent` 进行完整测试。

### UDS vs TCP 对比

| 特性 | UDS | TCP |
|------|-----|-----|
| 通信范围 | 同一台机器 | 跨网络 |
| 性能 | 更快（无需网络协议栈） | 较慢 |
| 地址格式 | 文件系统路径 | IP:Port |
| 安全性 | 文件权限控制 | 防火墙/ACL |
| 适用场景 | 本地 IPC | 远程通信 |

---

## 常见错误

### 错误 1: 未清理已存在的 socket 文件

```rust
// ❌ 错误：直接绑定，可能失败
let listener = UnixListener::bind("/tmp/hello.socket")?;

// ✅ 正确：先清理
let _ = fs::remove_file("/tmp/hello.socket");
let listener = UnixListener::bind("/tmp/hello.socket")?;
```

### 错误 2: 未处理 UTF-8 转换错误

```rust
// ❌ 错误：直接 unwrap
let message = String::from_utf8(payload).unwrap();

// ✅ 正确：处理错误
let message = match String::from_utf8(payload) {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Invalid UTF-8: {}", e);
        return;
    }
};
```

### 错误 3: 未等待服务端绑定

```rust
// ❌ 错误：立即启动客户端
let mut server = Command::new("./uds_server").spawn()?;
let mut client = Command::new("./uds_client").spawn()?; // 可能连接失败！

// ✅ 正确：等待服务端就绪
let mut server = Command::new("./uds_server").spawn()?;
thread::sleep(Duration::from_millis(500)); // 或使用更复杂的就绪检测
let mut client = Command::new("./uds_client").spawn()?;
```

---

## 动手练习

### 练习 1: 修改协议为 JSON

将当前的字符串反转协议改为 JSON 格式：

```rust
// TODO: 定义请求/响应结构体
// #[derive(Serialize, Deserialize)]
// struct Request { action: String, data: String }
// struct Response { result: String }

// TODO: 修改 handle_client 使用 JSON 序列化/反序列化
```

<details>
<summary>点击查看答案</summary>

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Request { action: String, data: String }

#[derive(Serialize, Deserialize)]
struct Response { result: String }

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    loop {
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes) as usize;
        
        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;
        
        let request: Request = serde_json::from_slice(&payload).expect("Invalid JSON");
        
        let response = Response {
            result: request.data.chars().rev().collect(),
        };
        
        let resp_payload = serde_json::to_vec(&response).expect("Serialization failed");
        stream.write_all(&u32::to_be_bytes(resp_payload.len() as u32))?;
        stream.write_all(&resp_payload)?;
        stream.flush()?;
    }
    Ok(())
}
```

</details>

---

## 小结

**核心要点**：

1. **UDS** 是同一台机器上进程间通信的高效方式
2. **自定义协议** 使用长度前缀避免粘包问题
3. **父进程编排** 使用 `Command` 启动和管理子进程
4. **清理 socket 文件** 是必要的，否则绑定会失败

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| Unix Domain Socket | Unix 域套接字 | 本地 IPC 机制 |
| Length Prefix | 长度前缀 | 避免粘包问题 |
| Parent Process | 父进程 | 编排子进程的进程 |
| Child Process | 子进程 | 被父进程启动的进程 |
| Big-Endian | 大端序 | 网络字节序 |

**下一步**：

- 学习 [Stdio IPC](stdio-ipc.md) - 基于管道的进程间通信
- 了解 [gRPC 服务](grpc.md) - 跨网络 RPC 通信
- 探索 [进程管理](../advance/system/process.md) - 进程控制基础

---

## 术语表

| English | 中文 |
|---------|------|
| Unix Domain Socket | Unix 域套接字 |
| IPC | 进程间通信 |
| Length Prefix | 长度前缀 |
| Payload | 负载/消息体 |
| Big-Endian | 大端序 |
| Parent Process | 父进程 |
| Child Process | 子进程 |
| Bind | 绑定 |
| Accept | 接受连接 |
| Connect | 连接 |

完整示例：
- [uds_server.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_server.rs)
- [uds_client.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_client.rs)
- [uds_parent.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_parent.rs)

---

## 继续学习

- 上一步：[gRPC 服务](grpc.md) - 跨网络 RPC 通信
- 下一步：[Stdio IPC](stdio-ipc.md) - 基于管道的进程间通信
- 相关：[进程管理](../advance/system/process.md) - 进程控制基础

> 💡 **记住**：UDS 比 TCP 快得多，因为数据不需要经过网络协议栈。在同一台机器上的进程间通信，优先选择 UDS！
