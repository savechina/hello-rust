# CLI 开发最佳实践

## 开篇故事

想象你在设计一个瑞士军刀。如果每个工具都混在一起，用户会很难找到需要的功能。但如果每个工具都有清晰的标签、合理的位置，用户就能快速找到并使用。

CLI（命令行界面）工具就是程序的"瑞士军刀"。好的 CLI 工具应该：参数清晰、帮助文档完整、错误信息友好、支持子命令组织功能。在 Rust 生态中，`clap` 是最流行的 CLI 参数解析库，它让构建专业的 CLI 工具变得简单。

---

## 本章适合谁

如果你想学习：
- 如何使用 clap 构建专业的 CLI 工具
- 如何设计子命令结构
- CLI 项目的最佳实践

本章适合你。CLI 开发是 Rust 最常见的应用场景之一。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 clap 解析命令行参数
2. 设计子命令结构组织复杂功能
3. 实现 `--help` 和 `--version` 支持
4. 处理 CLI 错误并输出友好信息
5. 设计符合 Unix 哲学的 CLI 工具

---

## 前置要求

- [结构体](../../basic/struct.md) - 结构体定义
- [枚举](../../basic/enums.md) - 枚举和模式匹配
- [错误处理](../tools/error-handling.md) - 错误处理基础

---

### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add clap --features derive
cargo add anyhow
```

---

## 第一个例子

最简单的 CLI 工具：

```rust,ignore
use clap::Parser;

/// 简单的问候程序
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 要问候的人名
    #[arg(short, long)]
    name: String,

    /// 问候次数
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
```

**运行效果**:

```bash
$ cargo run -- --name World --count 3
Hello, World!
Hello, World!
Hello, World!

$ cargo run -- --help
简单的问候程序

Usage: cli-example [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    要问候的人名
  -c, --count <COUNT>  问候次数 [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

完整示例：[basic.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/basic.rs)

---

## 原理解析

### CLI 架构概览

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CLI 工具架构                                  │
│                                                                      │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐           │
│   │   clap       │    │   Business   │    │   Output     │           │
│   │  (参数解析)   │───→│   Logic      │───→│  (输出/错误)  │           │
│   │              │    │  (业务逻辑)   │    │              │           │
│   └──────────────┘    └──────────────┘    └──────────────┘           │
│         │                   │                   │                    │
│   ┌─────┴───────────────────┴───────────────────┴─────┐             │
│   │              Error Handling (anyhow)               │             │
│   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐│             │
│   │  │   Context   │  │   Backtrace │  │   Exit Code ││             │
│   │  │  (上下文)    │  │  (调用栈)    │  │  (退出码)    ││             │
│   │  └─────────────┘  └─────────────┘  └─────────────┘│             │
│   └───────────────────────────────────────────────────┘             │
└─────────────────────────────────────────────────────────────────────┘
```

### clap 核心概念

**1. 派生宏（Derive Macros）**

```rust,ignore
use clap::Parser;

#[derive(Parser)]
#[command(name = "my-app")]
#[command(about = "A great CLI tool")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}
```

**2. 子命令（Subcommands）**

```rust,ignore
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新项
    Add { name: String },
    /// 删除项
    Remove { id: u32 },
    /// 列出所有项
    List,
}
```

**3. 嵌套子命令**

```rust,ignore
use clap::{Parser, Subcommand, Args};

#[derive(Subcommand)]
enum CalcCommands {
    /// 加法
    Add { a: i32, b: i32 },
    /// 减法
    Sub { a: i32, b: i32 },
    /// 乘法（使用 Args 结构体）
    Mul(Mul),
}

#[derive(Args)]
struct Mul {
    /// 第一个数
    a: i32,
    /// 第二个数
    b: i32,
}
```

### 完整 CLI 项目结构

```
my-cli-app/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI 入口，参数解析
│   ├── commands/         # 子命令实现
│   │   ├── mod.rs
│   │   ├── add.rs
│   │   ├── remove.rs
│   │   └── list.rs
│   ├── config.rs         # 配置管理
│   └── error.rs          # 错误类型定义
└── tests/
    └── cli_tests.rs      # CLI 集成测试
```

### 完整示例：计算器 CLI

```rust,ignore
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author="ren", version, about="高级计算工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 问候某人
    Hello { name: String },
    /// 执行计算
    Calc {
        #[command(subcommand)]
        operation: CalcCommands,
    },
}

#[derive(Subcommand)]
enum CalcCommands {
    /// 加法
    Add { a: i32, b: i32 },
    /// 减法
    Sub { a: i32, b: i32 },
    /// 乘法
    Mul(Mul),
    /// 除法
    Div(Div),
}

#[derive(Args)]
struct Mul {
    /// 第一个数
    a: i32,
    /// 第二个数
    b: i32,
}

#[derive(Args)]
struct Div {
    /// 第一个数
    a: i32,
    /// 第二个数
    b: i32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello { name } => {
            println!("hello to {}", name)
        }
        Commands::Calc { operation } => {
            execute_calc_command(operation);
        }
    }
}

fn execute_calc_command(operation: &CalcCommands) {
    match operation {
        CalcCommands::Add { a, b } => {
            println!("{} + {} = {}", a, b, a + b);
        }
        CalcCommands::Sub { a, b } => {
            println!("{} - {} = {}", a, b, a - b);
        }
        CalcCommands::Mul(Mul { a, b }) => {
            println!("{} * {} = {}", a, b, a * b);
        }
        CalcCommands::Div(Div { a, b }) => {
            if *b == 0 {
                eprintln!("错误：除数不能为 0");
                std::process::exit(1);
            }
            println!("{} / {} = {}", a, b, a / b);
        }
    }
}
```

**运行效果**:

```bash
$ cargo run -- calc add 5 3
5 + 3 = 8

$ cargo run -- calc mul 4 7
4 * 7 = 28

$ cargo run -- calc div 10 0
错误：除数不能为 0

$ cargo run -- --help
高级计算工具

Usage: calc-cli <COMMAND>

Commands:
  hello  问候某人
  calc   执行计算
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

完整示例：[advance.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/advance.rs)

### 错误处理最佳实践

```rust,ignore
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match run(&cli) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("错误: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run(cli: &Cli) -> Result<()> {
    // 业务逻辑
    let config = std::fs::read_to_string(&cli.config)
        .with_context(|| format!("无法读取配置文件: {}", cli.config))?;
    
    // ...
    Ok(())
}
```

### 日志集成

```rust,ignore
use tracing_subscriber;

fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .init();
    
    let cli = Cli::parse();
    
    if cli.verbose {
        tracing::info!("详细模式已启用");
    }
    
    // ...
    Ok(())
}
```

---

## CLI 设计原则

### Unix 哲学

1. **每个程序只做一件事**
2. **程序之间通过文本流通信**
3. **快速原型，使用文本流**

### 好的 CLI 特征

| 特征 | 说明 | 示例 |
|------|------|------|
| **清晰的帮助** | `--help` 输出完整用法 | clap 自动生成 |
| **版本信息** | `--version` 显示版本 | clap 自动生成 |
| **合理的退出码** | 0=成功，1=错误 | `std::process::exit(1)` |
| **友好的错误** | 错误信息包含上下文 | anyhow 的 context |
| **支持管道** | 从 stdin 读取，向 stdout 写入 | `cat file \| my-cli` |

---

## 常见错误

### 错误 1: 未提供默认值

```rust,ignore
// ❌ 错误：必需参数，但用户可能不知道
#[arg(short, long)]
output: String,

// ✅ 正确：提供默认值
#[arg(short, long, default_value = "output.txt")]
output: String,
```

### 错误 2: 错误信息不友好

```rust,ignore
// ❌ 错误：直接 unwrap
let file = std::fs::read_to_string(path).unwrap();

// ✅ 正确：提供上下文
let file = std::fs::read_to_string(path)
    .with_context(|| format!("无法读取文件: {}", path))?;
```

### 错误 3: 未处理 SIGINT/SIGTERM

```rust,ignore
// ❌ 错误：无法优雅退出
loop {
    // 永远运行，无法停止
}

// ✅ 正确：监听信号
use tokio::signal;

signal::ctrl_c().await?;
println!("收到 Ctrl+C，正在关闭...");
```

---

## 动手练习

### 练习 1: 添加新的子命令

在计算器 CLI 中添加 `power` 子命令，计算 a 的 b 次方：

```rust,ignore
// TODO: 在 CalcCommands 枚举中添加 Power 变体
// Power { a: i32, b: i32 }

// TODO: 在 execute_calc_command 中处理 Power
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Subcommand)]
enum CalcCommands {
    // ... 其他命令
    /// 幂运算
    Power {
        /// 底数
        a: i32,
        /// 指数
        b: u32,
    },
}

fn execute_calc_command(operation: &CalcCommands) {
    match operation {
        // ... 其他处理
        CalcCommands::Power { a, b } => {
            println!("{} ^ {} = {}", a, b, a.pow(*b));
        }
    }
}
```

</details>

---

## 小结

**核心要点**：

1. **clap** 是 Rust 最流行的 CLI 参数解析库
2. **派生宏** 让参数定义变得简单
3. **子命令** 帮助组织复杂功能
4. **错误处理** 使用 anyhow 提供友好上下文
5. **Unix 哲学** 指导 CLI 设计

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| CLI | 命令行界面 | 文本交互界面 |
| Subcommand | 子命令 | 嵌套命令 |
| Argument | 参数 | 命令行输入 |
| Flag | 标志 | 布尔参数（如 --verbose） |
| Option | 选项 | 带值的参数（如 --name World） |

**下一步**：

- 学习 [进程管理](../system/process.md) - 进程控制
- 了解 [错误处理](../tools/error-handling.md) - 错误处理最佳实践
- 探索 [日志记录](../../basic/logger.md) - 日志集成

---

## 术语表

| English | 中文 |
|---------|------|
| CLI | 命令行界面 |
| Subcommand | 子命令 |
| Argument | 参数 |
| Flag | 标志 |
| Option | 选项 |
| Parse | 解析 |
| Derive | 派生 |
| Context | 上下文 |
| Exit Code | 退出码 |

完整示例：
- [basic.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/basic.rs)
- [advance.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/advance.rs)
- [app_sys_ctl.rs](https://github.com/savechina/hello-rust/blob/main/src/bin/app_sys_ctl.rs)

---

## 继续学习

- 上一步：[Stdio IPC](stdio-ipc.md) - 基于管道的进程间通信
- 下一步：[进程管理](../system/process.md) - 进程控制基础
- 相关：[错误处理](../tools/error-handling.md) - 错误处理最佳实践

> 💡 **记住**：好的 CLI 工具应该像好的 API 一样设计——清晰、一致、友好。使用 clap 的派生宏，让参数定义变得简单！
