# 常见问题 FAQ

本页面收集 Rust 学习过程中最常见的问题。如果你有其他问题，欢迎[提交 Issue](https://github.com/savechina/hello-rust/issues)。

<!-- FAQPage JSON-LD Schema for GEO (Generative Engine Optimization) -->
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "Rust 适合用来做什么？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Rust 适合系统编程、Web 后端（高性能 API 服务）、命令行工具、嵌入式开发、WebAssembly 和区块链智能合约开发。"
      }
    },
    {
      "@type": "Question",
      "name": "Rust 和 C/C++ 有什么区别？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Rust 在编译时保证内存安全，消除空指针和数据竞争；内置 Cargo 包管理器；学习曲线较陡峭但编译器错误信息友好。"
      }
    },
    {
      "@type": "Question",
      "name": "Rust 难学吗？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Rust 的学习曲线比 Python、JavaScript 更陡峭，主要原因是所有权系统、借用检查器和强类型系统。但掌握核心概念后，编译器会成为最好的老师。"
      }
    },
    {
      "@type": "Question",
      "name": "什么是 Rust 的所有权（Ownership）？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "所有权是 Rust 管理内存的核心机制：每个值有一个所有者；值在任何时刻只能有一个所有者；当所有者离开作用域时，值会被丢弃。"
      }
    },
    {
      "@type": "Question",
      "name": "什么时候需要用异步编程？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "当需要处理大量并发 I/O 操作时：HTTP 服务器、数据库并发查询、文件并发读写、网络爬虫等。CPU 密集型任务更适合用线程或 Rayon。"
      }
    },
    {
      "@type": "Question",
      "name": "panic! 和 Result 有什么区别？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "panic! 用于不可恢复的错误，程序立即终止；Result 用于可恢复的错误，调用者决定如何处理。"
      }
    },
    {
      "@type": "Question",
      "name": "Cargo 是什么？",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Cargo 是 Rust 的包管理器和构建工具，支持创建项目、构建、运行、测试、生成文档和发布到 crates.io。"
      }
    }
  ]
}
</script>

---

## 入门相关

### Rust 适合用来做什么？

Rust 是一门系统级编程语言，适合以下场景：

- **系统编程**：操作系统、文件系统、网络协议栈
- **Web 后端**：高性能 API 服务（Axum、Actix）
- **命令行工具**：快速、跨平台的 CLI 应用
- **嵌入式开发**：无 GC、低内存占用的嵌入式系统
- **WebAssembly**：编译为 WASM 在浏览器中运行
- **区块链**：Solana 等区块链的智能合约开发

### Rust 和 C/C++ 有什么区别？

| 特性 | Rust | C/C++ |
|------|------|-------|
| 内存安全 | 编译时保证（所有权系统） | 手动管理，容易出错 |
| 数据竞争 | 编译时消除 | 运行时可能出现 |
| 空指针 | 无空指针（使用 Option） | 常见错误来源 |
| 包管理 | Cargo（内置） | 无统一方案 |
| 学习曲线 | 较陡峭 | 中等 |

### Rust 难学吗？

Rust 的学习曲线确实比 Python、JavaScript 等语言更陡峭，主要原因是：

1. **所有权系统**：这是 Rust 独有的概念，需要转变思维方式
2. **借用检查器**：编译器会严格检查引用的生命周期
3. **类型系统**：强类型 + 泛型 + Trait 系统

但一旦掌握了核心概念，你会发现 Rust 的编译器是最好的"老师"——它会给出清晰的错误信息和修复建议。

**建议学习路径**：
1. 先阅读本教程的[基础部分](../basic/basic-overview.md)
2. 动手写代码，不要只看不练
3. 遇到编译错误时，仔细阅读编译器的建议

---

## 所有权与借用

### 什么是所有权（Ownership）？

所有权是 Rust 管理内存的核心机制，有三条基本规则：

1. 每个值都有一个"所有者"（变量）
2. 值在任何时刻只能有一个所有者
3. 当所有者离开作用域时，值会被丢弃（Drop）

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权转移给 s2，s1 不再有效
// println!("{}", s1); // 编译错误！
println!("{}", s2); // OK
```

### 什么是借用（Borrowing）？

借用允许你临时访问数据而不获取所有权：

```rust
let s = String::from("hello");
let len = calculate_length(&s); // 借用 s，不转移所有权
println!("{} 的长度是 {}", s, len); // s 仍然有效

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 为什么不能同时有多个可变引用？

这是为了防止**数据竞争**（data race）。Rust 的借用规则保证：

- 同一时刻，要么有多个不可变引用（`&T`）
- 要么只有一个可变引用（`&mut T`）
- 不能同时存在

---

## 异步编程

### 什么时候需要用异步编程？

当你需要处理大量并发 I/O 操作时：

- HTTP 服务器处理多个请求
- 数据库并发查询
- 文件并发读写
- 网络爬虫

对于 CPU 密集型任务，使用线程（`std::thread`）或 Rayon 并行计算更合适。

### `async`/`await` 和线程有什么区别？

| 特性 | 异步任务 (async) | 线程 (thread) |
|------|-----------------|---------------|
| 开销 | 极小（KB 级别） | 较大（MB 级别） |
| 并发数 | 数十万 | 数百到数千 |
| 适合场景 | I/O 密集型 | CPU 密集型 |
| 调度 | 用户态（tokio） | 操作系统 |

---

## 错误处理

### `panic!` 和 `Result` 有什么区别？

- **`panic!`**：不可恢复的错误，程序立即终止
- **`Result`**：可恢复的错误，调用者决定如何处理

```rust
// panic! - 程序崩溃
let x = vec![1, 2, 3];
let y = x[10]; // panic!

// Result - 优雅处理
let file = std::fs::File::open("config.toml");
match file {
    Ok(f) => println!("打开成功"),
    Err(e) => println!("打开失败: {}", e),
}
```

### `?` 运算符是什么？

`?` 是错误传播的语法糖，等价于：

```rust
// 使用 ?
let f = File::open("config.toml")?;

// 等价于
let f = match File::open("config.toml") {
    Ok(file) => file,
    Err(e) => return Err(e.into()),
};
```

---

## 工具链

### Cargo 是什么？

Cargo 是 Rust 的包管理器和构建工具，功能包括：

- 创建项目：`cargo new my-project`
- 构建项目：`cargo build`
- 运行项目：`cargo run`
- 运行测试：`cargo test`
- 生成文档：`cargo doc`
- 发布到 crates.io：`cargo publish`

### 如何添加第三方库？

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

然后运行 `cargo build` 自动下载和编译依赖。

---

## 下一步

- 回到[教程首页](../about-hello.md)
- 查看[学习路径](../learning_path.md)
- 在[GitHub](https://github.com/savechina/hello-rust)上报告问题
