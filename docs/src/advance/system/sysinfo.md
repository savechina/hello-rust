# 系统信息

## 开篇故事

想象你要检查电脑的健康状况。传统方式是：打开各种工具 → 查看 CPU → 查看内存 → 查看进程。sysinfo 库就像是：系统仪表盘——一个库获取所有系统信息。

---

## 本章适合谁

如果你需要在 Rust 程序中获取系统信息（CPU、内存、进程），本章适合你。sysinfo 是跨平台系统监控的标准库。

---

## 你会学到什么

完成本章后，你可以：

1. 获取系统信息
2. 获取内存使用情况
3. 获取进程列表
4. 监控特定进程
5. 获取 CPU 信息

---

## 前置要求

- 结构体 - 结构体基础
- 错误处理 - 错误处理基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add sysinfo
cargo add bigdecimal
```

## 第一个例子

获取系统信息：

```rust
use sysinfo::{System, SystemExt};

fn main() {
    // 创建系统实例
    let mut system = System::new_all();
    
    // 刷新所有信息
    system.refresh_all();
    
    // 获取可用内存
    println!("可用内存：{} MB", system.available_memory() / 1024 / 1024);
    
    // 获取系统信息
    println!("操作系统：{:?}", System::name());
    println!("系统版本：{:?}", System::os_version());
    println!("CPU 架构：{:?}", System::cpu_arch());
}
```

**完整示例**: [sysinfo_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/sysinfo_sample.rs)

---

## 原理解析

### sysinfo 特性

**sysinfo 是系统信息库**：

- ✅ 跨平台支持
- ✅ 获取系统信息
- ✅ 获取进程信息
- ✅ 实时监控

### 创建系统实例

**使用 System::new_all()**：

```rust
use sysinfo::{System, SystemExt};

let mut system = System::new_all();
```

**使用 System::new()**：

```rust
// 只创建，不刷新
let mut system = System::new();

// 手动刷新
system.refresh_memory();
system.refresh_processes();
```

### 获取系统信息

**获取操作系统信息**：

```rust
use sysinfo::System;

println!("操作系统：{:?}", System::name());
println!("系统版本：{:?}", System::os_version());
println!("CPU 架构：{:?}", System::cpu_arch());
println!("发行版：{:?}", System::distribution_id());
println!("内核版本：{:?}", System::kernel_version());
```

### 获取内存信息

**获取内存使用情况**：

```rust
use sysinfo::{System, SystemExt};

let mut system = System::new_all();

println!("总内存：{} MB", system.total_memory() / 1024 / 1024);
println!("可用内存：{} MB", system.available_memory() / 1024 / 1024);
println!("已用内存：{} MB", system.used_memory() / 1024 / 1024);
```

### 获取进程信息

**获取所有进程**：

```rust
use sysinfo::{System, SystemExt, Pid};

let mut system = System::new_all();

for (pid, process) in system.processes() {
    println!("进程 ID: {:?}", pid);
    println!("进程名：{}", process.name());
    println!("CPU 使用：{}%", process.cpu_usage());
    println!("内存：{} MB", process.memory() / 1024 / 1024);
}
```

### 监控特定进程

**获取特定进程**：

```rust
use sysinfo::{System, SystemExt, Pid, ProcessExt};

let mut system = System::new_all();
let pid = std::process::id();

if let Some(process) = system.process(Pid::from_u32(pid as usize)) {
    println!("当前进程：");
    println!("  名称：{}", process.name());
    println!("  CPU: {}%", process.cpu_usage());
    println!("  内存：{} MB", process.memory() / 1024 / 1024);
    println!("  启动时间：{}", process.start_time());
    println!("  运行时间：{:?}", process.run_time());
}
```

---

## 常见错误

### 错误 1: 忘记刷新

```rust
let mut system = System::new();
println!("{}", system.available_memory());
// ❌ 没有刷新，数据可能过时
```

**修复方法**:
```rust
let mut system = System::new_all();  // ✅ 创建时刷新
// 或
system.refresh_memory();  // ✅ 手动刷新
```

### 错误 2: PID 类型错误

```rust
let pid = std::process::id();  // u32
system.process(pid);  // ❌ 需要 Pid 类型
```

**修复方法**:
```rust
use sysinfo::Pid;

let pid = std::process::id();
system.process(Pid::from_u32(pid as usize));  // ✅ 转换类型
```

### 错误 3: 忘记导入 trait

```rust
use sysinfo::System;

let mut system = System::new_all();
system.refresh_all();  // ❌ 需要导入 SystemExt
```

**修复方法**:
```rust
use sysinfo::{System, SystemExt};  // ✅ 导入 trait
```

---

## 动手练习

### 练习 1: 获取系统信息

```rust
use sysinfo::{System, SystemExt};

fn main() {
    let mut system = System::new_all();
    
    // TODO: 打印操作系统名称
    // TODO: 打印系统版本
    // TODO: 打印 CPU 架构
}
```

<details>
<summary>点击查看答案</summary>

```rust
println!("操作系统：{:?}", System::name());
println!("系统版本：{:?}", System::os_version());
println!("CPU 架构：{:?}", System::cpu_arch());
```
</details>

### 练习 2: 获取内存信息

```rust
use sysinfo::{System, SystemExt};

fn main() {
    let mut system = System::new_all();
    
    // TODO: 打印总内存
    // TODO: 打印可用内存
    // TODO: 打印已用内存
}
```

<details>
<summary>点击查看答案</summary>

```rust
println!("总内存：{} MB", system.total_memory() / 1024 / 1024);
println!("可用内存：{} MB", system.available_memory() / 1024 / 1024);
println!("已用内存：{} MB", system.used_memory() / 1024 / 1024);
```
</details>

### 练习 3: 监控当前进程

```rust
use sysinfo::{System, SystemExt, Pid};

fn main() {
    let mut system = System::new_all();
    let pid = std::process::id();
    
    // TODO: 获取当前进程
    // TODO: 打印进程信息（名称、CPU、内存）
}
```

<details>
<summary>点击查看答案</summary>

```rust
if let Some(process) = system.process(Pid::from_u32(pid as usize)) {
    println!("名称：{}", process.name());
    println!("CPU: {}%", process.cpu_usage());
    println!("内存：{} MB", process.memory() / 1024 / 1024);
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 为什么获取的内存信息不准确？

**A**: 
- 需要调用 refresh_all() 或 refresh_memory()
- 数据是快照，会随时间变化

### Q: 如何持续监控？

**A**: 
```rust
loop {
    system.refresh_processes();
    println!("CPU: {}%", process.cpu_usage());
    std::thread::sleep(Duration::from_secs(1));
}
```

### Q: sysinfo 支持哪些平台？

**A**: 
- Linux
- macOS
- Windows
- FreeBSD

---

## 知识扩展

### CPU 信息

```rust
use sysinfo::{System, SystemExt};

let mut system = System::new_all();

println!("CPU 核心数：{}", system.cpus().len());

for cpu in system.cpus() {
    println!("CPU {}: {}%", cpu.name(), cpu.cpu_usage());
}
```

### 磁盘信息

```rust
use sysinfo::{System, SystemExt};

let mut system = System::new_all();

for disk in system.disks() {
    println!("磁盘：{:?}", disk.name());
    println!("总空间：{} GB", disk.total_space() / 1024 / 1024 / 1024);
    println!("可用空间：{} GB", disk.available_space() / 1024 / 1024 / 1024);
}
```

### 网络接口

```rust
use sysinfo::{System, SystemExt};

let mut system = System::new_all();

for (interface_name, network) in system.networks() {
    println!("接口：{}", interface_name);
    println!("接收：{} bytes", network.total_received());
    println!("发送：{} bytes", network.total_transmitted());
}
```

---

## 小结

**核心要点**：

1. **System**: 系统信息
2. **SystemExt**: 系统扩展 trait
5. **进程监控**: 获取进程信息
6. **跨平台**: 支持多平台

**关键术语**：

- **System**: 系统
- **Process**: 进程
- **PID**: 进程 ID
- **Memory**: 内存

---

## 术语表

| English | 中文 |
| ------- | ---- |
| System | 系统 |
| Process | 进程 |
| PID | 进程 ID |
| Memory | 内存 |
| CPU | 中央处理器 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `System::new_all()` 做了什么？

2. 如何获取特定进程的信息？

3. `refresh_all()` 和 `new_all()` 的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 创建 System 实例并刷新所有信息（CPU、内存、进程等）
2. 使用 `system.process(Pid::from_u32(pid))`
3. `new_all()` = 创建 + 刷新，`refresh_all()` = 仅刷新已有实例

**关键理解**: sysinfo 提供跨平台的系统和进程信息。
</details>

## 继续学习

**前一章**: [进程管理](process.md)  
**下一章**: [资源嵌入](includedir.md)

**相关章节**:
- [进程管理](process.md)
- [资源嵌入](includedir.md)
- [系统信息](sysinfo.md)

**返回**: 高级进阶

---

**完整示例**: [sysinfo_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/sysinfo_sample.rs)
