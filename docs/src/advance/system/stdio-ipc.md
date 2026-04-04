# Stdio IPC (标准输入输出进程间通信)

## 开篇故事

想象你在教一个小孩学习。你问他问题（写入 stdin），他回答你（读取 stdout）。你们之间不需要复杂的协议，只需要简单的"你说一句，我说一句"。

Stdio IPC 就是这种简单的进程间通信方式——父进程通过管道（pipe）向子进程的标准输入写入数据，从子进程的标准输出读取响应。它比 Unix Domain Socket 更简单，适合轻量级的父子进程通信。

---

## 本章适合谁

如果你想学习：
- 如何使用标准输入输出实现进程间通信
- 父进程如何启动和管理子进程
- 管道（pipe）的工作原理

本章适合你。Stdio IPC 是最简单的本地 IPC 方案。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Stdio IPC 的核心概念
2. 使用 `std::process::Command` 启动子进程
3. 使用 `Stdio::piped()` 创建管道
4. 实现父进程与子进程的双向通信
5. 处理子进程的生命周期

---

## 前置要求

- [进程管理](../advance/system/process.md) - 进程创建和管理
- [字节处理](../advance/system/bytes.md) - 字节操作基础

---

### 依赖安装

Stdio IPC 使用 Rust 标准库，无需额外依赖：

```rust,ignore
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write};
```

---

## 第一个例子

最简单的 Stdio IPC 示例：

**子进程** (`stdio_child.rs`):

```rust,ignore
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut stdout = io::stdout();

    // 从 stdin 读取行
    for line in handle.lines() {
        let input = line?;
        if input == "done" {
            break;
        }
        // 处理：转换为大写
        writeln!(stdout, "Received: {}", input.to_uppercase())?;
        stdout.flush()?;
    }

    Ok(())
}
```

> 💡 **注意**：此代码需要父进程配合运行。请使用 `cargo run --bin stdio_parent` 进行完整测试。

**父进程** (`stdio_parent.rs`):

```rust,ignore
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let messages = vec!["hello", "world", "rust", "done"];

    // 启动子进程
    let mut child = Command::new("./stdio_child")
        .stdin(Stdio::piped())  // 管道用于父进程写入
        .stdout(Stdio::piped()) // 管道用于父进程读取
        .spawn()?;

    // 获取子进程的 stdin 和 stdout 句柄
    let mut child_stdin = child.stdin.take().expect("Failed to open child stdin");
    let child_stdout = child.stdout.take().expect("Failed to open child stdout");
    let mut reader = BufReader::new(child_stdout);

    // 与子进程通信
    for msg in messages {
        // 写入子进程的 stdin
        writeln!(child_stdin, "{}", msg)?;
        child_stdin.flush()?;

        // 从子进程的 stdout 读取
        let mut response = String::new();
        reader.read_line(&mut response)?;
        print!("父进程收到: {}", response);
        io::stdout().flush()?;
    }

    // 关闭 stdin 以通知子进程
    drop(child_stdin);

    // 等待子进程完成
    child.wait()?;

    Ok(())
}
```

> 💡 **注意**：此代码需要子进程配合运行。请使用 `cargo run --bin stdio_parent` 进行完整测试。

**运行方式**:

```bash
cargo run --bin stdio_parent
```

完整示例：
- [stdio_parent.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_parent.rs)
- [stdio_child.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_child.rs)

---

## 原理解析

### Stdio IPC 架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Stdio IPC 架构                                   │
│                                                                      │
│   ┌──────────────┐         ┌──────────────┐         ┌──────────────┐│
│   │   Parent     │         │    Pipe      │         │    Child     ││
│   │  (父进程)     │────────→│  (管道)       │────────→│  (子进程)     ││
│   │              │  stdin   │              │  stdin   │              ││
│   │              │←────────│              │←────────│              ││
│   │              │  stdout  │              │  stdout  │              ││
│   └──────────────┘         └──────────────┘         └──────────────┘│
│                                                                      │
│   通信协议: 行分隔文本（\n）                                           │
│   ┌─────────────────────────────────────────────────────────────────┐ │
│   │ "hello\n" → 子进程处理 → "Received: HELLO\n" ← 子进程响应       │ │
│   └─────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

### 管道工作原理

```
父进程写入 → stdin pipe → 子进程读取
父进程读取 ← stdout pipe ← 子进程写入

关键概念:
- Stdio::piped() 创建管道
- child.stdin.take() 获取写入句柄
- child.stdout.take() 获取读取句柄
- drop(child_stdin) 关闭管道，通知子进程
```

### 父进程实现详解

```rust,ignore
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let messages = vec!["hello", "world", "rust", "done"];

    // 1. 启动子进程
    let mut child = Command::new("./stdio_child")
        .stdin(Stdio::piped())   // 创建 stdin 管道
        .stdout(Stdio::piped())  // 创建 stdout 管道
        .spawn()?;

    // 2. 获取句柄
    let mut child_stdin = child.stdin.take().expect("Failed to open child stdin");
    let child_stdout = child.stdout.take().expect("Failed to open child stdout");
    let mut reader = BufReader::new(child_stdout);

    // 3. 通信循环
    for msg in messages {
        // 写入
        writeln!(child_stdin, "{}", msg)?;
        child_stdin.flush()?;

        // 读取
        let mut response = String::new();
        reader.read_line(&mut response)?;
        print!("父进程收到: {}", response);
        io::stdout().flush()?;
    }

    // 4. 关闭 stdin（通知子进程结束）
    drop(child_stdin);

    // 5. 等待子进程完成
    child.wait()?;

    Ok(())
}
```

> 💡 **注意**：此代码需要子进程配合运行。请使用 `cargo run --bin stdio_parent` 进行完整测试。

### 子进程实现详解

```rust,ignore
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // 1. 获取 stdin 和 stdout
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut stdout = io::stdout();

    // 2. 读取-处理-响应循环
    for line in handle.lines() {
        let input = line?;
        
        // 3. 检查结束条件
        if input == "done" {
            break;
        }
        
        // 4. 处理并响应
        writeln!(stdout, "Received: {}", input.to_uppercase())?;
        stdout.flush()?; // 重要：刷新缓冲区
    }

    Ok(())
}
```

> 💡 **注意**：此代码需要父进程配合运行。请使用 `cargo run --bin stdio_child` 进行完整测试。

### Stdio IPC vs UDS 对比

| 特性 | Stdio IPC | UDS |
|------|-----------|-----|
| 复杂度 | 简单 | 中等 |
| 协议 | 行分隔文本 | 自定义二进制 |
| 适用场景 | 父子进程 | 任意进程 |
| 性能 | 中等 | 高 |
| 跨平台 | ✅ 是 | ❌ 仅 Unix |

---

## 常见错误

### 错误 1: 忘记刷新缓冲区

```rust,ignore
// ❌ 错误：未刷新，子进程可能收不到数据
writeln!(child_stdin, "{}", msg)?;

// ✅ 正确：刷新缓冲区
writeln!(child_stdin, "{}", msg)?;
child_stdin.flush()?;
```

### 错误 2: 未关闭 stdin 导致子进程挂起

```rust,ignore
// ❌ 错误：子进程永远等待更多输入
child.wait()?; // 挂起！

// ✅ 正确：先关闭 stdin
drop(child_stdin);
child.wait()?;
```

### 错误 3: 未使用 BufReader

```rust,ignore
// ❌ 错误：直接读取，可能读到不完整的数据
let mut response = String::new();
child_stdout.read_to_string(&mut response)?;

// ✅ 正确：使用 BufReader 按行读取
let mut reader = BufReader::new(child_stdout);
let mut response = String::new();
reader.read_line(&mut response)?;
```

---

## 动手练习

### 练习 1: 实现 JSON 通信

将当前的行分隔协议改为 JSON 格式：

```rust,ignore
// TODO: 子进程接收 JSON 请求，返回 JSON 响应
// #[derive(Serialize, Deserialize)]
// struct Request { action: String, data: String }
// struct Response { result: String }
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
// 子进程
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Request { action: String, data: String }

#[derive(Serialize, Deserialize)]
struct Response { result: String }

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut stdout = io::stdout();

    for line in handle.lines() {
        let input = line?;
        if input == "done" { break; }
        
        let request: Request = serde_json::from_str(&input).expect("Invalid JSON");
        let response = Response {
            result: request.data.to_uppercase(),
        };
        
        writeln!(stdout, "{}", serde_json::to_string(&response).unwrap())?;
        stdout.flush()?;
    }
    Ok(())
}
```

</details>

---

## 小结

**核心要点**：

1. **Stdio IPC** 是最简单的进程间通信方式
2. **管道** 通过 `Stdio::piped()` 创建
3. **刷新缓冲区** 是必要的，否则数据不会发送
4. **关闭 stdin** 通知子进程结束

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| Stdio | 标准输入输出 | stdin/stdout/stderr |
| Pipe | 管道 | 进程间通信通道 |
| Parent Process | 父进程 | 启动子进程的进程 |
| Child Process | 子进程 | 被启动的进程 |
| Flush | 刷新 | 将缓冲区数据写入 |

**下一步**：

- 学习 [Unix Domain Socket](uds.md) - 更高效的本地 IPC
- 了解 [gRPC 服务](grpc.md) - 跨网络 RPC 通信
- 探索 [进程管理](../advance/system/process.md) - 进程控制基础

---

## 术语表

| English | 中文 |
|---------|------|
| Stdio | 标准输入输出 |
| Pipe | 管道 |
| Parent Process | 父进程 |
| Child Process | 子进程 |
| Flush | 刷新 |
| BufReader | 缓冲读取器 |
| Spawn | 启动（进程） |
| Wait | 等待（进程结束） |

完整示例：
- [stdio_parent.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_parent.rs)
- [stdio_child.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_child.rs)

---

## 继续学习

- 上一步：[Unix Domain Socket](uds.md) - 本地 IPC
- 下一步：[gRPC 服务](grpc.md) - 跨网络 RPC 通信
- 相关：[进程管理](../advance/system/process.md) - 进程控制基础

> 💡 **记住**：Stdio IPC 简单但强大。对于父子进程通信，优先选择 Stdio；对于任意进程间通信，选择 UDS 或 gRPC！
