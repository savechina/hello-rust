# 进程管理

## 开篇故事

想象你要运行一个外部程序。传统方式是：手动启动 → 等待完成 → 获取结果。process 库就像是：程序管家——帮你启动、管理、监控外部进程。

---

## 本章适合谁

如果你需要在 Rust 程序中运行外部命令、管理子进程，本章适合你。进程管理是系统编程的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 获取当前进程 ID
2. 启动子进程
3. 管理进程生命周期
4. 捕获进程输出
5. 处理进程错误

---

## 前置要求

- 错误处理 - 错误处理基础
- [文件与目录操作](directory.md) - 文件路径基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add nix --features process,signal
```

## 第一个例子

获取当前进程 ID：

```rust
use std::process;

fn main() {
    // 获取当前进程 ID
    let current_pid = process::id();
    
    println!("当前进程 ID: {}", current_pid);
}
```

**完整示例**: [process_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/process_sample.rs)

---

## 原理解析

### process 特性

**std::process 是进程管理库**：

- ✅ 获取进程信息
- ✅ 启动子进程
- ✅ 管理进程生命周期
- ✅ 捕获输出

### 获取进程 ID

**使用 process::id()**：

```rust
use std::process;

let pid = process::id();
println!("进程 ID: {}", pid);
```

### 启动子进程

**使用 Command**：

```rust
use std::process::Command;

let output = Command::new("ls")
    .arg("-l")
    .output()
    .expect("Failed to execute command");

println!("输出：{}", String::from_utf8_lossy(&output.stdout));
```

### 捕获输出

**使用 output()**：

```rust
use std::process::Command;

let output = Command::new("echo")
    .arg("Hello from Rust!")
    .output()
    .expect("Failed");

println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
```

### 进程状态

**检查进程状态**：

```rust
use std::process::Command;

let mut child = Command::new("sleep")
    .arg("5")
    .spawn()
    .expect("Failed to spawn");

// 等待进程完成
let status = child.wait().expect("Failed to wait");

println!("进程退出码：{}", status.code().unwrap_or(-1));
```

---

## 常见错误

### 错误 1: 命令不存在

```rust
let output = Command::new("nonexistent_command")
    .output();  // ❌ 命令不存在
```

**错误信息**:
```
No such file or directory (os error 2)
```

**修复方法**:
```rust
let output = Command::new("ls")  // ✅ 使用存在的命令
    .output();
```

### 错误 2: 忘记处理错误

```rust
let output = Command::new("ls").output();
println!("{}", output.stdout);  // ❌ output 是 Result
```

**错误信息**:
```
no field `stdout` on type `Result<Output, Error>`
```

**修复方法**:
```rust
let output = Command::new("ls").output()?;  // ✅ 使用 ? 处理错误
```

### 错误 3: 忘记 flush

```rust
println!("PID: {}", pid);
// ❌ 如果没有 flush，输出可能不会立即显示
```

**修复方法**:
```rust
use std::io::Write;

println!("PID: {}", pid);
std::io::stdout().flush()?;  // ✅ 立即刷新
```

---

## 动手练习

### 练习 1: 获取进程信息

```rust
use std::process;

fn main() {
    // TODO: 获取当前进程 ID
    // TODO: 打印 PID
}
```

<details>
<summary>点击查看答案</summary>

```rust
let pid = process::id();
println!("当前进程 ID: {}", pid);
```
</details>

### 练习 2: 运行外部命令

```rust
use std::process::Command;

fn main() {
    // TODO: 运行 "pwd" 命令
    // TODO: 打印输出
}
```

<details>
<summary>点击查看答案</summary>

```rust
let output = Command::new("pwd")
    .output()
    .expect("Failed");

println!("当前目录：{}", String::from_utf8_lossy(&output.stdout));
```
</details>

### 练习 3: 带参数的命令

```rust
use std::process::Command;

fn main() {
    // TODO: 运行 "ls -la" 命令
    // TODO: 打印输出
}
```

<details>
<summary>点击查看答案</summary>

```rust
let output = Command::new("ls")
    .arg("-la")
    .output()
    .expect("Failed");

println!("{}", String::from_utf8_lossy(&output.stdout));
```
</details>

---

## 故障排查 (FAQ)

### Q: spawn() 和 output() 有什么区别？

**A**: 
- **output()**: 等待进程完成，返回输出
- **spawn()**: 立即返回，异步管理进程

### Q: 如何处理大输出？

**A**: 
```rust
let mut child = Command::new("cat")
    .arg("large_file.txt")
    .spawn()?;

// 逐行读取，避免内存溢出
let stdout = child.stdout.take().unwrap();
for line in std::io::BufReader::new(stdout).lines() {
    println!("{}", line?);
}
```

### Q: 如何设置超时？

**A**: 
```rust
use std::time::Duration;

let mut child = Command::new("sleep")
    .arg("10")
    .spawn()?;

// 设置 5 秒超时
child.wait_timeout(Duration::from_secs(5))?;
```

---

## 知识扩展

### 环境变量

```rust
use std::process::Command;
use std::env;

let output = Command::new("echo")
    .env("MY_VAR", "Hello")
    .arg("$MY_VAR")
    .output()?;
```

### 管道

```rust
use std::process::{Command, Stdio};

let child = Command::new("cat")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

// 写入 stdin
child.stdin.unwrap().write_all(b"Hello")?;
```

### 退出码

```rust
let status = child.wait()?;

if status.success() {
    println!("成功");
} else {
    println!("失败，退出码：{}", status.code().unwrap_or(-1));
}
```

---

## 小结

**核心要点**：

1. **process::id()**: 获取进程 ID
2. **Command**: 启动子进程
3. **output()**: 同步执行
4. **spawn()**: 异步执行
5. **wait()**: 等待进程完成

**关键术语**：

- **Process**: 进程
- **PID**: 进程 ID
- **Command**: 命令
- **Spawn**: 派生

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Process | 进程 |
| PID | 进程 ID |
| Command | 命令 |
| Spawn | 派生 |
| Output | 输出 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `Command::spawn()` 和 `Command::output()` 有什么区别？

2. 如何向子进程发送数据？

3. 子进程退出后如何获取退出码？

<details>
<summary>点击查看答案与解析</summary>

1. `spawn()` 返回正在运行的子进程，`output()` 等待完成并返回输出
2. 使用 `stdin(Stdio::piped())` 获取 stdin 句柄
3. `Child::wait()` 返回 `ExitStatus`，使用 `.code()` 获取退出码

**关键理解**: `spawn()` 适合长时间运行的进程，`output()` 适合一次性命令。
</details>

## 继续学习

**前一章**: Ollama AI 集成  
**下一章**: [系统信息](sysinfo.md)

**相关章节**:
- Ollama AI 集成
- [系统信息](sysinfo.md)
- 错误处理

**返回**: 高级进阶

---

**完整示例**: [process_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/process_sample.rs)
