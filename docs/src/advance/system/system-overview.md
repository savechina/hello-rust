# 系统编程

Rust 的系统编程能力是其核心优势之一：无 GC、直接操作系统 API、内存安全保证。

## 为什么用 Rust 做系统编程？

- **无运行时开销**：没有 GC，没有虚拟机，直接编译为机器码
- **内存安全**：所有权系统在编译时消除悬垂指针、缓冲区溢出
- **零成本抽象**：高级抽象不带来运行时性能损失
- **跨平台**：支持 Linux、macOS、Windows 等主流操作系统

## 本章节内容

| 主题 | 说明 |
|------|------|
| [文件与目录操作](./directory.md) | 文件系统遍历、权限管理、路径处理 |
| [临时文件](./tempfile.md) | 安全的临时文件创建与自动清理 |
| [内存映射](./memmap.md) | 大文件高效读写，mmap 系统调用 |
| [环境变量](./dotenv.md) | 配置管理，.env 文件加载 |
| [字节处理](./bytes.md) | 高效的字节缓冲区操作 |
| [Cow 类型](./cow.md) | Clone-on-Write 优化，减少不必要的复制 |
| [进程管理](./process.md) | 子进程创建、信号处理、进程间通信 |
| [系统信息](./sysinfo.md) | CPU、内存、进程等系统信息获取 |
| [资源嵌入](./includedir.md) | 编译时将文件嵌入二进制文件 |
| [Unix Domain Socket](./uds.md) | 高效的本地进程间通信 |
| [Stdio IPC](./stdio-ipc.md) | 标准输入输出进程间通信 |
| [CLI 开发](./cli.md) | 命令行工具开发最佳实践 |
| [Rust 消除的问题](./rust-eliminates.md) | Rust 编译时消除的常见系统编程错误 |
| [原子类型](../atomic-types.md) | 无锁并发编程 |

## 快速示例：文件读取

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("config.toml")?;
    println!("配置文件内容:\n{}", content);
    Ok(())
}
```

## 下一步

- 从 [文件与目录操作](./directory.md) 开始
- 探索 [内存映射](./memmap.md) 高性能文件处理
- 学习 [进程管理](./process.md) 和多进程编程
