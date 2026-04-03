# 进程管理与 IPC

## 开篇故事

想象你是一位指挥家，面前有一整个交响乐团。你需要协调每个乐手的演奏时机、控制乐章的开始与结束、确保小提琴和大提琴之间的配合默契。在操作系统中，进程管理就像这位指挥家——你需要启动新的进程（乐手入场）、协调它们之间的通信（乐器合奏）、处理它们的退出（乐章结束）。

进程间通信（IPC, Inter-Process Communication）则是让不同进程能够"对话"的机制。就像乐团成员通过眼神交流和指挥棒信号协调演奏，进程通过管道、信号、共享内存等方式交换数据。掌握进程管理和 IPC，是构建复杂系统、实现服务解耦的关键技能。

---

## 本章适合谁

如果你已经掌握了 Rust 基础，现在想学习：
- 如何在 Rust 中启动和管理子进程
- 进程间通信的各种机制（管道、信号、Unix Domain Socket）
- 如何构建 parent-child 架构的应用程序
- 如何实现优雅的进程控制和关闭

本章适合你。进程管理是构建后台服务、任务调度器、分布式系统的核心技能。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `std::process::Command` 启动和配置子进程
2. 通过管道（pipe）实现父子进程间的双向通信
3. 获取进程 ID（PID）并实现进程控制（启动、停止、状态检查）
4. 发送和处理 Unix 信号（SIGTERM、SIGKILL 等）
5. 使用 Unix Domain Socket 进行本地 IPC
6. 实现优雅关闭（graceful shutdown）模式
7. 避免常见的进程管理错误

---

## 前置要求

学习本章前，你需要理解：

- [线程基础](../basic/threads.md) - 并发执行的基本概念
- [错误处理](../basic/error.md) - `Result` 和错误传播
- [I/O 操作](../basic/file.md) - 标准输入输出、文件读写
- [所有权](../basic/ownership.md) - 特别是资源的创建和释放

---

## 第一个例子

让我们看一个简单的进程管理示例：

```rust
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

fn process_getpid_sample() -> io::Result<()> {
    // 获取当前进程 ID
    let current_pid = process::id();
    println!("PID: {}", current_pid);
    
    // 立即刷新到 stdout
    io::stdout().flush()?;
    
    println!(
        "Main application running with PID {}. Will run for 60 seconds...",
        current_pid
    );
    
    // 模拟长时间运行的进程
    thread::sleep(Duration::from_secs(60));
    
    println!("Main application finished naturally.");
    Ok(())
}
```

**发生了什么？**

1. `process::id()` 获取当前进程的唯一标识符（PID）
2. `io::stdout().flush()` 确保输出立即显示，对 IPC 很重要
3. 进程模拟长时间运行任务，最后正常退出

---

## 原理解析

### 进程与 IPC 架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                    进程管理与 IPC 架构                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐         管道 (Pipe)          ┌─────────────┐  │
│  │   父进程    │◄────────────────────────────►│   子进程    │  │
│  │  (Parent)   │    stdin/stdout/stderr      │   (Child)   │  │
│  └──────┬──────┘                              └──────┬──────┘  │
│         │                                            │          │
│         │           Unix Domain Socket               │          │
│         │◄──────────────────────────────────────────►│          │
│         │               /tmp/hello.socket            │          │
│         │                                            │          │
│         │               信号 (Signals)               │          │
│         │    SIGTERM ───────────────────────────────►│          │
│         │    SIGKILL ───────────────────────────────►│          │
│         │                                            │          │
│  ┌──────┴──────┐                              ┌──────┴──────┐  │
│  │   PID File  │                              │   PID File  │  │
│  │  app.pid    │                              │  app.pid    │  │
│  └─────────────┘                              └─────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 核心概念

**1. 进程标识（PID）**

```rust
use std::process;

// 获取当前进程 ID
let pid = process::id();
println!("Current PID: {}", pid);
```

每个进程在系统中都有唯一的 PID，用于识别和管理进程。

**2. 进程创建（Command）**

```rust
use std::process::{Command, Stdio};

// 创建子进程
let mut child = Command::new("./stdio_child")
    .stdin(Stdio::piped())   // 重定向 stdin
    .stdout(Stdio::piped())  // 重定向 stdout
    .spawn()?;               // 启动进程
```

`Command` 用于配置和启动子进程，`Stdio::piped()` 创建管道用于通信。

**3. 管道通信（Pipe IPC）**

```
管道通信流程:

┌─────────────┐                    ┌─────────────┐
│   父进程    │                    │   子进程    │
│             │    ┌──────┐       │             │
│  writeln!() │───►│ 管道 │──────►│  stdin      │
│  (child_    │    └──────┘       │  (lines())  │
│   stdin)    │                   │             │
│             │                   │  stdout     │
│  BufReader  │◄───┌──────┐◄──────│  (writeln!) │
│  (read_line)│    │ 管道 │       │             │
└─────────────┘    └──────┘       └─────────────┘

        数据流: 父 ──► 子 ──► 父 (双向)
```

**4. Unix 信号（Signals）**

```
信号处理流程:

     用户/系统
         │
         │ SIGTERM (优雅关闭)
         ▼
┌─────────────────┐
│    父进程       │
│   (控制进程)    │
└────────┬────────┘
         │
         │ 1. 发送 SIGTERM
         ▼
┌─────────────────┐
│    子进程       │
│   (工作进程)    │
│                 │
│  等待 5 秒      │
│  检查是否退出   │
│                 │
│  仍在运行?      │
│  ├─ Yes ──► SIGKILL (强制)
│  └─ No ───► 清理完成
└─────────────────┘
```

### 进程间通信方式对比

| 通信方式 | 适用场景 | 优点 | 缺点 |
|---------|---------|------|------|
| **管道 (Pipe)** | 父子进程单向/双向通信 | 简单易用，标准支持 | 仅限于相关进程 |
| **Unix Domain Socket** | 同机进程间通信 | 双向，可靠性高 | 需要文件系统 |
| **信号 (Signal)** | 进程控制（停止、重启） | 异步通知 | 信息量有限 |
| **PID File** | 进程状态管理 | 持久化状态 | 需要文件清理 |
| **共享内存** | 高频大数据交换 | 速度快 | 复杂，需同步 |

---

## 常见错误

### 错误 1: 未刷新输出导致子进程读取不到数据

```rust
// ❌ 错误：输出未刷新，子进程可能读取不到
writeln!(child_stdin, "{}", msg)?;
// 子进程立即读取，可能为空

// ✅ 正确：立即刷新缓冲区
writeln!(child_stdin, "{}", msg)?;
child_stdin.flush()?;  // 关键！
```

**问题**：操作系统会缓冲 stdout/stdin，不刷新时数据可能还在缓冲区。

---

### 错误 2: 未关闭 stdin 导致子进程一直等待

```rust
// ❌ 错误：子进程永远等待更多输入
for msg in messages {
    writeln!(child_stdin, "{}", msg)?;
}
child.wait()?; // 子进程阻塞在 read_line

// ✅ 正确：显式关闭 stdin 表示输入结束
drop(child_stdin); // 或 child_stdin.flush() 后关闭
child.wait()?;
```

**问题**：子进程通过 `lines()` 读取，stdin 未关闭时会一直阻塞等待。

---

### 错误 3: 未处理孤儿 PID 文件

```rust
// ❌ 错误：PID 文件残留导致启动失败
fn start() {
    if Path::new("app.pid").exists() {
        panic!("Already running!"); // 但进程可能已崩溃
    }
    // ...
}

// ✅ 正确：验证 PID 是否有效
fn start() {
    if let Some(pid) = read_pid() {
        if is_process_running(pid) {
            panic!("Already running with PID {}", pid);
        } else {
            fs::remove_file("app.pid")?; // 清理残留
        }
    }
    // ...
}
```

---

### 错误 4: 信号处理不当导致僵尸进程

```rust
// ❌ 危险：只发送信号，不等待子进程
kill(pid, Signal::SIGTERM)?;
// 子进程可能正在清理，变成僵尸进程

// ✅ 正确：等待进程真正退出
kill(pid, Signal::SIGTERM)?;
thread::sleep(Duration::from_secs(5));

// 检查进程是否还在运行
match kill(nix_pid, Signal::SIGCONT) {
    Ok(_) => {
        // 仍在运行，强制终止
        kill(nix_pid, Signal::SIGKILL)?;
    }
    Err(ESRCH) => {
        // 进程已退出
        println!("Gracefully shutdown");
    }
}
```

---

## 动手练习

### 练习 1: 理解父子进程通信

下面的代码有什么问题？

```rust
fn main() -> io::Result<()> {
    let mut child = Command::new("./child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    
    writeln!(stdin, "hello")?;
    // 尝试读取响应...
    
    child.wait()?;
    Ok(())
}
```

<details>
<summary>点击查看答案与解析</summary>

**答案**：缺少 `flush()` 和可能的 `drop(stdin)`

**解析**:
1. `writeln!` 后必须调用 `stdin.flush()`，否则数据在缓冲区
2. 如果子进程等待 EOF（输入结束），需要 `drop(stdin)` 关闭管道
3. `stdout` 没有使用 `BufReader` 包装，无法方便地按行读取

**修复方案**:
```rust
fn main() -> io::Result<()> {
    let mut child = Command::new("./child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);
    
    writeln!(stdin, "hello")?;
    stdin.flush()?;  // ✅ 立即刷新
    
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Got: {}", response);
    
    drop(stdin);     // ✅ 关闭 stdin
    child.wait()?;
    Ok(())
}
```

</details>

---

### 练习 2: 实现双向通信

补全代码，实现父进程发送消息、子进程处理后返回的模式：

```rust
// 父进程
fn main() -> io::Result<()> {
    let messages = vec!["hello", "world", "rust"];
    
    let mut child = Command::new("./stdio_child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(child_stdout);
    
    for msg in messages {
        // TODO: 发送消息到子进程
        // TODO: 从子进程读取响应
        // TODO: 打印响应
    }
    
    // TODO: 关闭 stdin 并等待子进程
    
    Ok(())
}
```

<details>
<summary>点击查看答案</summary>

```rust
for msg in messages {
    // 发送消息
    writeln!(child_stdin, "{}", msg)?;
    child_stdin.flush()?;
    
    // 读取响应
    let mut response = String::new();
    reader.read_line(&mut response)?;
    print!("Parent got: {}", response);
}

// 关闭 stdin
drop(child_stdin);

// 等待子进程结束
child.wait()?;
```

</details>

---

### 练习 3: 信号处理流程

预测以下代码的执行流程：

```rust
#[cfg(target_family = "unix")]
fn graceful_shutdown(pid: u32, timeout: u64) {
    let nix_pid = Pid::from_raw(pid as i32);
    
    // 1. 发送 SIGTERM
    kill(nix_pid, Signal::SIGTERM).unwrap();
    println!("Sent SIGTERM");
    
    // 2. 等待
    thread::sleep(Duration::from_secs(timeout));
    
    // 3. 检查是否还在运行
    match kill(nix_pid, Signal::SIGCONT) {
        Ok(_) => {
            println!("Still running, sending SIGKILL...");
            kill(nix_pid, Signal::SIGKILL).unwrap();
        }
        Err(_) => {
            println!("Process exited gracefully");
        }
    }
}
```

**问题**：
1. 如果进程在 5 秒内正常退出，输出什么？
2. 如果进程无视 SIGTERM，输出什么？
3. `SIGCONT` 信号的作用是什么？

<details>
<summary>点击查看解析</summary>

**答案**:

1. **正常退出**: 输出 "Sent SIGTERM" → "Process exited gracefully"
   - 进程收到 SIGTERM 后退出
   - `kill(SIGCONT)` 返回 `ESRCH` 错误（进程不存在）

2. **无视 SIGTERM**: 输出 "Sent SIGTERM" → "Still running, sending SIGKILL..."
   - 5 秒后进程仍在运行
   - `kill(SIGCONT)` 成功（进程存在）
   - 发送 SIGKILL 强制终止

3. **SIGCONT 的作用**: 用于检查进程是否存在
   - 向已停止的进程发送继续信号
   - 如果进程存在且能接收信号，返回 Ok
   - 如果进程已不存在，返回 `ESRCH` 错误
   - 是一种非破坏性的进程存在性检查方法

</details>

---

## 实际应用

### 应用场景 1: Parent-Child IPC 处理器

```rust
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn parent_child_communication() -> io::Result<()> {
    let messages = vec!["hello", "world", "rust", "done"];

    // 启动子进程
    let mut child = Command::new("./stdio_child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // 获取管道句柄
    let mut child_stdin = child.stdin.take().expect("Failed to open child stdin");
    let child_stdout = child.stdout.take().expect("Failed to open child stdout");
    let mut reader = BufReader::new(child_stdout);

    // 双向通信循环
    for msg in messages {
        // 发送消息
        writeln!(child_stdin, "{}", msg)?;
        child_stdin.flush()?;

        // 接收响应
        let mut response = String::new();
        reader.read_line(&mut response)?;
        print!("Parent got: {}", response);
        io::stdout().flush()?;
    }

    // 关闭输入管道，通知子进程输入结束
    drop(child_stdin);

    // 等待子进程完成
    child.wait()?;
    
    Ok(())
}
```

**子进程示例** (`stdio_child.rs`):

```rust
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut stdout = io::stdout();

    // 从 stdin 读取，处理后写入 stdout
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

---

### 应用场景 2: PID 文件管理器

```rust
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const PID_FILE: &str = "app.pid";

/// 写入 PID 文件
fn write_pid() -> io::Result<u32> {
    let pid = std::process::id();
    File::create(PID_FILE)?.write_all(pid.to_string().as_bytes())?;
    Ok(pid)
}

/// 读取 PID 文件
fn read_pid() -> io::Result<Option<u32>> {
    if !Path::new(PID_FILE).exists() {
        return Ok(None);
    }
    let pid_str = fs::read_to_string(PID_FILE)?;
    let pid = pid_str.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid PID format")
    })?;
    Ok(Some(pid))
}

/// 删除 PID 文件
fn remove_pid() -> io::Result<()> {
    if Path::new(PID_FILE).exists() {
        fs::remove_file(PID_FILE)?;
    }
    Ok(())
}
```

---

### 应用场景 3: Unix 信号处理器

```rust
#[cfg(target_family = "unix")]
use nix::sys::signal::{kill, Signal};
#[cfg(target_family = "unix")]
use nix::unistd::Pid;
use std::thread;
use std::time::Duration;

/// 优雅关闭进程
#[cfg(target_family = "unix")]
fn graceful_shutdown_process(pid: u32, timeout_secs: u64) -> io::Result<()> {
    let nix_pid = Pid::from_raw(pid as i32);
    
    println!("Attempting graceful shutdown for PID {}...", pid);
    
    // 1. 发送 SIGTERM 请求优雅关闭
    match kill(nix_pid, Signal::SIGTERM) {
        Ok(_) => {
            println!("Sent SIGTERM, waiting {} seconds...", timeout_secs);
            thread::sleep(Duration::from_secs(timeout_secs));
            
            // 2. 检查进程是否已退出
            match kill(nix_pid, Signal::SIGCONT) {
                Ok(_) => {
                    // 进程仍在运行，强制终止
                    println!("PID {} still running, sending SIGKILL...", pid);
                    kill(nix_pid, Signal::SIGKILL)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                }
                Err(nix::Error::ESRCH) => {
                    // 进程已正常退出
                    println!("PID {} exited gracefully.", pid);
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
                }
            }
        }
        Err(nix::Error::ESRCH) => {
            println!("PID {} was not running.", pid);
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    }
    
    Ok(())
}
```

---

### 应用场景 4: 服务启动控制器

```rust
use clap::{Parser, Subcommand};
use sysinfo::{Pid as SysPid, System};

#[derive(Parser)]
#[command(name = "app-controller")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动服务
    Start,
    /// 停止服务
    Stop,
    /// 查看状态
    Status,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start => start_service()?,
        Commands::Stop => stop_service()?,
        Commands::Status => check_status()?,
    }
    
    Ok(())
}

fn start_service() -> io::Result<()> {
    // 检查是否已在运行
    if let Some(pid) = read_pid()? {
        if is_process_running(pid) {
            println!("Service already running with PID {}", pid);
            return Ok(());
        }
        // 清理残留的 PID 文件
        remove_pid()?;
    }
    
    // 启动服务并记录 PID
    let pid = std::process::id();
    write_pid()?;
    println!("Service started with PID {}", pid);
    
    // 运行服务主循环...
    
    Ok(())
}

fn is_process_running(pid: u32) -> bool {
    let mut system = System::new_all();
    system.refresh_all();
    system.process(SysPid::from_u32(pid)).is_some()
}
```

---

## 故障排查 (FAQ)

### Q: 如何确保子进程一定能收到 stdin 的数据？

**A**: 遵循三步原则：

1. **写入后立即刷新**：
   ```rust
   writeln!(stdin, "{}", msg)?;
   stdin.flush()?;  // 关键！
   ```

2. **使用 BufReader 读取**：
   ```rust
   let reader = BufReader::new(stdout);
   reader.read_line(&mut buf)?;
   ```

3. **结束时关闭 stdin**：
   ```rust
   drop(stdin);  // 发送 EOF 信号
   ```

---

### Q: PID 文件的作用是什么？如何防止重复启动？

**A**: PID 文件用于进程状态持久化：

```
启动流程:
┌─────────────┐
│   启动命令   │
└──────┬──────┘
       ▼
┌─────────────┐    Yes    ┌─────────────┐
│ PID文件存在? │─────────►│ 读取PID     │
└─────────────┘           │ 检查进程    │
       │                  │ 是否在运行  │
       No                 └──────┬──────┘
       │                        │
       ▼                        ▼
┌─────────────┐           ┌─────────────┐
│ 创建PID文件  │           │ 进程在运行?  │
│ 启动服务    │           └──────┬──────┘
└─────────────┘                  │
                          Yes    │    No
                          ┌──────┴──────┐
                          ▼             ▼
                    ┌─────────┐   ┌─────────┐
                    │ 退出    │   │ 删除文件 │
                    │ 已运行  │   │ 重新启动 │
                    └─────────┘   └─────────┘
```

---

### Q: SIGTERM 和 SIGKILL 有什么区别？

**A**: 

| 信号 | 作用 | 能否捕获 | 使用场景 |
|-----|------|---------|---------|
| **SIGTERM** | 请求优雅关闭 | 可以捕获 | 首选，给进程清理机会 |
| **SIGKILL** | 强制立即终止 | 无法捕获 | 最后手段，立即结束 |

**最佳实践**：
1. 先发送 SIGTERM，等待一段时间
2. 如果进程仍在运行，发送 SIGKILL
3. 最后清理 PID 文件

---

### Q: 如何处理 Windows 上的进程控制？

**A**: Windows 不支持 Unix 信号，需要不同方法：

```rust
#[cfg(windows)]
fn stop_process_windows(pid: u32) -> io::Result<()> {
    use std::process::Command;
    
    // 使用 taskkill 命令
    Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/T", "/F"])
        .status()?;
    
    Ok(())
}

#[cfg(unix)]
fn stop_process_unix(pid: u32) -> io::Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    
    kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;
    Ok(())
}
```

---

## 知识扩展 (选学)

### Unix Domain Socket IPC

对于更复杂的 IPC 需求，可以使用 Unix Domain Socket：

```rust
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// 服务器端
async fn uds_server() -> io::Result<()> {
    let listener = UnixListener::bind("/tmp/hello.socket")?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            socket.write_all(&buf[..n]).await.unwrap();
        });
    }
}

// 客户端
async fn uds_client() -> io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/hello.socket").await?;
    stream.write_all(b"Hello from client").await?;
    
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    
    Ok(())
}
```

### 进程状态监控

```rust
use sysinfo::{System, Pid, Process};

fn monitor_process(pid: u32) {
    let mut system = System::new_all();
    system.refresh_all();
    
    if let Some(process) = system.process(Pid::from_u32(pid)) {
        println!("PID: {:?}", process.pid());
        println!("Name: {:?}", process.name());
        println!("CPU: {:.1}%", process.cpu_usage());
        println!("Memory: {} KB", process.memory());
        println!("Run time: {:?}", process.run_time());
    }
}
```

---

## 小结

**核心要点**：

1. **进程创建**：使用 `Command` 配置并启动子进程，通过 `Stdio::piped()` 建立管道
2. **PID 管理**：使用 PID 文件防止重复启动，注意清理残留文件
3. **管道通信**：父进程通过 `stdin` 写入，从 `stdout` 读取；记得 `flush()` 和 `drop()`
4. **信号处理**：SIGTERM 请求优雅关闭，SIGKILL 强制终止；遵循"优雅→强制"流程
5. **IPC 选择**：管道适合父子通信，UDS 适合同机任意进程，信号适合控制

**关键术语**：

- **PID (Process ID)**: 进程唯一标识符
- **Pipe**: 管道，用于进程间单向数据传输
- **Signal**: 信号，异步通知机制
- **Graceful Shutdown**: 优雅关闭，给进程清理资源的机会
- **PID File**: 记录进程 ID 的文件，用于状态管理
- **IPC (Inter-Process Communication)**: 进程间通信

**下一步**：

- 学习 [Tokio 异步运行时](tokio.md) - 结合异步实现高效 IPC
- 理解 [Unix Domain Socket 示例](../../src/bin/uds_parent.rs) - 完整 IPC 示例
- 探索 [服务框架](../awesome/services.md) - 生产级进程管理

---

## 术语表

| English | 中文 |
|---------|------|
| Process | 进程 |
| PID (Process ID) | 进程标识符 |
| Pipe | 管道 |
| Signal | 信号 |
| Fork | 派生/创建 |
| Spawn | 启动（进程） |
| IPC | 进程间通信 |
| Graceful Shutdown | 优雅关闭 |
| Zombie Process | 僵尸进程 |
| Orphan Process | 孤儿进程 |
| STDIN | 标准输入 |
| STDOUT | 标准输出 |
| STDERR | 标准错误 |

完整示例：[src/advance/process_sample.rs](../../../src/advance/process_sample.rs)

IPC 示例：[src/bin/stdio_parent.rs](../../../src/bin/stdio_parent.rs)、[src/bin/stdio_child.rs](../../../src/bin/stdio_child.rs)

进程控制示例：[src/bin/app_sys_ctl.rs](../../../src/bin/app_sys_ctl.rs)、[src/bin/app_nix_ctl.rs](../../../src/bin/app_nix_ctl.rs)

---

## 继续学习

- 下一步：[Tokio 异步编程](tokio.md)
- 进阶：[gRPC 服务通信](../awesome/services.md)
- 回顾：[线程基础](../basic/threads.md)

> 💡 **记住**：进程管理就像指挥交响乐团——每个进程是乐手，IPC 是配合信号，优雅关闭是完美的乐章结束。掌握这些，你就能构建复杂的分布式系统！

---

## 进程通信模型可视化

### 1. 管道通信模型

```
┌─────────────────────────────────────────────────────────────┐
│                     Parent-Child IPC                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   父进程                          子进程                      │
│  ┌──────────┐                   ┌──────────┐                │
│  │          │  write ────────►  │  stdin   │                │
│  │  main()  │      (管道)       │          │                │
│  │          │                   │  process │                │
│  │          │  read  ◄────────  │          │                │
│  │          │      (管道)       │  stdout  │                │
│  └──────────┘                   └──────────┘                │
│        │                               │                     │
│        │ 1. writeln!(stdin, ...)       │                     │
│        │ 2. flush()                    │                     │
│        │                               │ 3. read_line()      │
│        │                               │ 4. process          │
│        │ 5. read_line()                │ 5. writeln!(stdout) │
│        │                               │ 6. flush()          │
│        ▼                               ▼                     │
│  ┌──────────────────────────────────────────┐               │
│  │           循环直到 "done"                 │               │
│  └──────────────────────────────────────────┘               │
│        │                                                     │
│        │ 7. drop(stdin) ──► EOF                             │
│        │ 8. wait()                                          │
│        ▼                                                     │
│  ┌──────────────────────────────────────────┐               │
│  │           子进程退出                      │               │
│  └──────────────────────────────────────────┘               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 2. PID 文件管理流程

```
启动流程:
┌──────────┐
│ 启动请求  │
└────┬─────┘
     ▼
┌─────────────────┐    存在    ┌──────────────┐
│ 读取 PID 文件    │───────────►│ 进程在运行?   │
│                 │            └──────┬───────┘
└─────────────────┘                   │
     │ 不存在                         │
     ▼                                 ▼
┌─────────────────┐            ┌──────────┐
│ 创建 PID 文件    │            │ Yes      │
│ 写入当前 PID     │            │ 退出     │
│                 │            │ 已在运行  │
└─────────────────┘            └──────────┘
     │                              │
     ▼                              ▼
┌─────────────────┐            ┌──────────┐
│ 启动服务主循环   │            │ No       │
│                 │            │ 删除文件  │
│ 运行中...       │            │ 继续启动  │
│                 │            └──────────┘
└─────────────────┘
     │
     ▼ 收到停止信号
┌─────────────────┐
│ 清理资源        │
│ 删除 PID 文件   │
│ 退出进程        │
└─────────────────┘
```

### 3. 信号处理时序

```
时间轴 ───────────────────────────────────────────────►

   T0          T+1s        T+5s         T+5.1s
    │            │            │             │
    ▼            ▼            ▼             ▼
┌───────┐   ┌────────┐   ┌────────┐   ┌─────────┐
│SIGTERM│   │        │   │SIGCONT │   │SIGKILL  │
│发送   │   │ 等待中  │   │检查    │   │(如需要) │
└───────┘   └────────┘   └────────┘   └─────────┘
    │            │            │             │
    │            │            │             │
    ▼            ▼            ▼             ▼
进程状态:
┌───────┐   ┌────────┐   ┌────────┐   ┌─────────┐
│Running│   │Cleaning│   │Exited  │   │Killed   │
│       │   │up      │   │Graceful│   │Forceful │
└───────┘   └────────┘   └────────┘   └─────────┘

情况 A: 进程正常响应 SIGTERM
├───────●──────────────────●──────────────►
      T0                  T+2s
                    (提前退出)

情况 B: 进程忽略 SIGTERM
├───────●──────────────────●───────●──────►
      T0                  T+5s    T+5.1s
                    (检查仍在)   (强制终止)
```

---

## 知识检查

### 检查点 1 🟢 (基础概念)

以下代码的输出顺序是什么？

```rust
fn main() -> io::Result<()> {
    let mut child = Command::new("echo")
        .arg("Hello")
        .stdout(Stdio::piped())
        .spawn()?;
    
    println!("A");
    let status = child.wait()?;
    println!("B");
    println!("Exit code: {:?}", status.code());
    Ok(())
}
```

A) Hello → A → B → Exit code  
B) A → B → Exit code → Hello  
C) A → Hello → B → Exit code  
D) 不确定

<details>
<summary>答案与解析</summary>

**答案**: C) A → Hello → B → Exit code

**解析**: 
- `spawn()` 立即返回，打印 "A"
- 子进程在后台运行，输出 "Hello"
- `wait()` 阻塞等待子进程完成
- 子进程完成后，打印 "B" 和退出码

注意：实际输出顺序可能因缓冲区而略有不同，但逻辑顺序是 A → (子进程输出) → B

</details>

---

### 检查点 2 🟡 (管道通信)

父子进程通过管道通信时，以下哪个操作是必须的？

A) 父进程必须使用 `BufReader` 包装 stdout  
B) 子进程必须使用 `println!` 输出  
C) 父进程写入后必须调用 `flush()`  
D) 必须使用 `tokio` 进行异步通信  

<details>
<summary>答案与解析</summary>

**答案**: C) 父进程写入后必须调用 `flush()`

**解析**:
- A: `BufReader` 推荐但不是必须的，可以直接读取
- B: 子进程可以使用任何写入 stdout 的方式
- C: **正确** - 必须 `flush()` 确保数据立即发送到管道
- D: 标准库的同步 API 完全可以实现 IPC

**关键代码**:
```rust
writeln!(child_stdin, "{}", msg)?;
child_stdin.flush()?;  // 必须！
```

</details>

---

### 检查点 3 🔴 (信号与进程控制)

以下关于 Unix 信号的描述，哪个是**错误**的？

A) SIGTERM 可以被进程捕获并处理  
B) SIGKILL 会立即终止进程，无法捕获  
C) SIGCONT 用于请求进程继续执行，也可用于检查进程是否存在  
D) 发送 SIGTERM 后不需要等待，进程会立即退出  

<details>
<summary>答案与解析</summary>

**答案**: D) 发送 SIGTERM 后不需要等待，进程会立即退出

**解析**:
- A: 正确，SIGTERM 是请求关闭，进程可以捕获并执行清理
- B: 正确，SIGKILL 是强制终止，无法捕获或忽略
- C: 正确，SIGCONT 用于继续已停止进程，发送给运行中进程可检查存在性
- D: **错误** - SIGTERM 只是请求，进程可能忽略或需要时间清理

**正确做法**:
```rust
kill(pid, Signal::SIGTERM)?;
thread::sleep(Duration::from_secs(5)); // 等待
// 检查进程是否已退出
match kill(pid, Signal::SIGCONT) {
    Ok(_) => { /* 仍在运行，发送 SIGKILL */ }
    Err(ESRCH) => { /* 已退出 */ }
}
```

</details>

---

## 💡 小知识：为什么需要进程间通信？

**问题来源**：
在单体应用中，所有功能在一个进程内：

```rust
// 单体应用 - 所有模块耦合
fn main() {
    let data = load_data();      // 数据加载
    let result = process(data);  // 数据处理  
    let output = render(result); // 结果渲染
    save(output);                // 保存结果
}
```

**问题**：
- 一个模块崩溃，整个程序崩溃
- 无法独立升级某个模块
- 无法利用多核 CPU

**解决方案** - 进程架构：

```
┌─────────────────────────────────────────────────┐
│                 Master Process                   │
│  (协调者，轻量，负责启动和管理 Worker)            │
└───────────────┬─────────────────────────────────┘
                │ spawn
    ┌───────────┼───────────┐
    ▼           ▼           ▼
┌───────┐   ┌───────┐   ┌───────┐
│Worker1│   │Worker2│   │Worker3│
│Load   │   │Process│   │Render │
│       │   │       │   │       │
└───┬───┘   └───┬───┘   └───┬───┘
    │           │           │
    └───────────┴───────────┘
                │ IPC / Channel
                ▼
          ┌───────────┐
          │  Save     │
          │  Process  │
          └───────────┘

优势：
- 单个 Worker 崩溃，Master 可以重启它
- 每个 Worker 可以独立升级
- Worker 可以分布在不同机器上
```

**真实案例**：
- **Nginx**: Master + Worker 进程模型
- **PostgreSQL**: Postmaster + Backend 进程
- **Chrome**: Browser 进程 + 每个标签页独立进程

---

## 🌟 工业界应用：微服务进程管理

**场景**：某电商平台的服务启动器

```rust
// 服务管理器 - 管理多个微服务进程
struct ServiceManager {
    services: HashMap<String, Child>,
    pid_dir: PathBuf,
}

impl ServiceManager {
    fn start_service(&mut self, name: &str, cmd: &str) -> io::Result<u32> {
        // 检查是否已在运行
        if let Some(pid) = self.read_pid(name)? {
            if self.is_running(pid) {
                println!("{} already running", name);
                return Ok(pid);
            }
        }
        
        // 启动服务
        let child = Command::new(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        let pid = child.id();
        self.write_pid(name, pid)?;
        self.services.insert(name.to_string(), child);
        
        println!("Started {} with PID {}", name, pid);
        Ok(pid)
    }
    
    fn stop_all(&mut self) {
        for (name, mut child) in &mut self.services {
            println!("Stopping {}...", name);
            
            // 尝试优雅关闭
            #[cfg(unix)]
            if let Some(pid) = child.id() {
                let _ = self.graceful_shutdown(pid, 5);
            }
            
            // 确保进程结束
            let _ = child.wait();
            let _ = self.remove_pid(name);
        }
    }
}
```

**结果**：
- 服务启动时间：**从 30s 降到 5s**（并行启动）
- 故障隔离：**单个服务崩溃不影响其他服务**
- 升级便捷：**可以独立升级和重启单个服务**

---

## 🧪 动手试试：实现一个 Mini Systemd

**练习**：实现一个简单的服务管理器

```rust
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::io;

struct MiniSystemd {
    services: HashMap<String, Child>,
}

impl MiniSystemd {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }
    
    // TODO: 启动服务
    fn start(&mut self, name: &str, command: &str) -> io::Result<u32> {
        // 1. 使用 Command 启动进程
        // 2. 获取 PID
        // 3. 保存到 services HashMap
        // 4. 返回 PID
        todo!()
    }
    
    // TODO: 停止服务
    fn stop(&mut self, name: &str) -> io::Result<()> {
        // 1. 从 HashMap 获取 Child
        // 2. 尝试 wait() 或 kill()
        // 3. 从 HashMap 移除
        todo!()
    }
    
    // TODO: 查看状态
    fn status(&self, name: &str) -> Option<u32> {
        // 返回服务的 PID（如果存在）
        todo!()
    }
}

fn main() -> io::Result<()> {
    let mut systemd = MiniSystemd::new();
    
    // 启动一个长时间运行的进程
    let pid = systemd.start("worker", "./long_running_process")?;
    println!("Started worker with PID {}", pid);
    
    // 查看状态
    if let Some(pid) = systemd.status("worker") {
        println!("Worker is running with PID {}", pid);
    }
    
    // 5 秒后停止
    std::thread::sleep(std::time::Duration::from_secs(5));
    systemd.stop("worker")?;
    println!("Worker stopped");
    
    Ok(())
}
```

<details>
<summary>点击查看完整实现</summary>

```rust
impl MiniSystemd {
    fn start(&mut self, name: &str, command: &str) -> io::Result<u32> {
        let child = Command::new(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        let pid = child.id();
        self.services.insert(name.to_string(), child);
        
        Ok(pid)
    }
    
    fn stop(&mut self, name: &str) -> io::Result<()> {
        if let Some(mut child) = self.services.remove(name) {
            // 尝试优雅关闭（发送 SIGTERM）
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;
                
                if let Some(pid) = child.id() {
                    let _ = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
                }
            }
            
            // 等待进程结束
            child.wait()?;
        }
        
        Ok(())
    }
    
    fn status(&self, name: &str) -> Option<u32> {
        self.services.get(name).map(|c| c.id())
    }
}
```

</details>

---

完整示例：[src/advance/process_sample.rs](../../../src/advance/process_sample.rs)

IPC 完整示例：[src/bin/stdio_parent.rs](../../../src/bin/stdio_parent.rs)、[src/bin/stdio_child.rs](../../../src/bin/stdio_child.rs)

进程控制示例：[src/bin/app_sys_ctl.rs](../../../src/bin/app_sys_ctl.rs)、[src/bin/app_nix_ctl.rs](../../../src/bin/app_nix_ctl.rs)
