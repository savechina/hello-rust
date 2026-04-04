# 追踪 (Tracing)

## 开篇故事

想象你在玩一个复杂的桌游，每走一步都有人记录："玩家 A 从起点移动到第 5 格"。如果游戏出错了，你可以回放整个游戏过程找出问题。Rust 的**追踪** (tracing) 就是这样 - 它记录程序的每一步执行，帮助你理解异步代码的执行流程。

---

## 本章适合谁

如果你已经学习了基础日志，现在想深入理解异步程序的执行流程，本章适合你。追踪是现代 Rust 异步编程的必备工具。

---

## 你会学到什么

1. tracing 与 log 的区别
2. Span的概念和使用
3. 异步函数追踪
4. 自定义追踪事件
5. 性能影响分析

---

## 前置要求

- [日志记录](logger.md) - 基础日志概念
- 异步编程 - async/await 基础

---

## 第一个例子

```rust
// src/basic/tracing_sample.rs

use tracing::{debug, error, info, span, warn, Level};

// #[tracing::instrument] 自动为函数创建 span
#[tracing::instrument]
pub fn shave(yak: usize) -> Result<(), Box<dyn std::error::Error>> {
    debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
    
    if yak == 3 {
        warn!("could not locate yak!");
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!("yak shaved successfully");
    }
    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    // 创建 span 记录执行过程
    let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
    let _enter = span.enter();
    
    info!("shaving yaks");
    
    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        if shave(yak).is_ok() {
            yaks_shaved += 1;
        }
    }
    
    yaks_shaved
}
```

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念       | Python               | Java                   | C++                    | Rust                       | 关键差异                  |
| ---------- | -------------------- | ---------------------- | ---------------------- | -------------------------- | ------------------------- |
| 追踪框架   | 无标准               | 无标准                 | 无标准                 | `tracing` crate            | Rust 有专用追踪框架       |
| Span 概念  | 无                   | 无                     | 无                     | 有时间跨度的追踪           | Rust 独创 span 概念       |
| 异步支持   | 弱                   | 弱                     | 弱                     | 强（跨线程追踪）           | Rust 为异步设计           |
| 结构化日志 | `structlog` 库       | 需要库                 | 需要库                 | 原生支持                   | Rust 原生结构化           |
| 自动仪器化 | 无                   | 无                     | 无                     | `#[instrument]` 属性       | Rust 自动追踪函数         |

**核心差异**: Python/Java/C++ 无标准追踪框架，Rust 的 tracing 为异步而生，支持 span 和自动仪器化。

---

## 原理解析

### 1. Span vs Event

**Span** - 表示一段时间（函数执行、请求处理）：

```rust
use tracing::{span, Level};

let span = span!(Level::INFO, "my_span");
let _enter = span.enter();  // 进入 span

// 这段代码在 span 内执行
// span 结束时自动记录耗时
```

**Event** - 表示一个时间点的事件：

```rust
use tracing::info;

info!("用户登录");  // 事件，没有时间跨度
```

### 2. 自动仪器化

使用 `#[tracing::instrument]` 自动追踪函数：

```rust
#[tracing::instrument]
fn process_user(id: u64) -> User {
    // 自动创建 span，记录参数和返回值
}

// 输出类似：
// shave{id=42}: process_user: entering
// shave{id=42}: process_user: exiting
```

### 3. 自定义字段

```rust
// 在 span 中添加自定义字段
let span = span!(
    Level::INFO,
    "database_query",
    user_id = user.id,
    query_time = start_time.elapsed().as_millis()
);
```

### 4. 异步支持

```rust
#[tracing::instrument]
async fn fetch_data(url: &str) -> Result<Data, Error> {
    // async 函数也能自动追踪
    // span 会跟随任务在不同线程间移动
}
```

---

## 常见错误

### 错误 1: 忘记 enter

```rust
let span = span!(Level::INFO, "my_span");
// ❌ 没有调用 enter()，span 不会生效
```

**修复**：

```rust
let span = span!(Level::INFO, "my_span");
let _enter = span.enter();  // ✅
```

### 错误 2: 忘记添加依赖

```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"  # 用于输出
```

### 错误 3: 在同步代码中使用异步追踪

```rust
#[tokio::main]
async fn main() {
    // ❌ 忘记初始化 subscriber
    my_async_function().await;
}
```

**修复**：

```rust
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();  // ✅
    my_async_function().await;
}
```

---

## 动手练习

### 练习 1: 创建 span

```rust
// TODO: 为这个函数添加追踪
fn process_order(order_id: u64) -> Result<(), Error> {
    // TODO: 创建 span，记录 order_id
    let _span = /* TODO */;
    let _enter = _span.enter();
    
    // 处理订单...
    Ok(())
}
```

<details>
<summary>点击查看答案</summary>

```rust
let _span = span!(Level::INFO, "process_order", order_id = order_id);
```
</details>

---

## 故障排查 (FAQ)

### Q: tracing 和 log 有什么区别？

**A**:
- **log**: 简单的事件日志，没有时间概念
- **tracing**: 支持 span，可以追踪执行流程，特别适合异步代码

### Q: 性能影响大吗？

**A**: 
- 默认级别以上：几乎为零（<1%）
- 详细级别：可能有 5-10% 开销
- **生产建议**: 使用 `warn` 或 `error` 级别

### Q: 如何输出到文件？

**A**:

```rust
use tracing_subscriber::fmt;
use std::fs::File;

let file = File::create("app.log").unwrap();
fmt::fmt()
    .with_writer(file)
    .init();
```

---

## 小结

**核心要点**：

1. **Span vs Event**: Span 有时间跨度，Event 是时间点
2. **自动仪器化**: `#[tracing::instrument]` 减少样板代码
3. **异步友好**: span 可以跟随任务跨线程
4. **性能影响**: 生产环境使用较高级别
5. **调试利器**: 可视化程序执行流程

**术语**：

- **Span**: 表示一段时间的追踪
- **Event**: 时间点的事件
- **Instrument**: 自动添加追踪
- **Subscriber**: 处理追踪输出的组件

**下一步**：

- 相关：[日志记录](logger.md)
- 进阶：异步调试

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Span | 跨度 |
| Event | 事件 |
| Instrument | 仪器化 |
| Subscriber | 订阅器 |

---

**完整源码**：[`src/basic/tracing_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/tracing_sample.rs)

---

> 💡 **提示**：追踪让你像看慢动作回放一样理解异步代码！

---

## 知识检查

**快速测验**（答案在下方）：

1. `tracing` 和 `log` 的区别是什么？

2. span 的作用是什么？

3. 如何添加自定义字段到 span？

<details>
<summary>点击查看答案与解析</summary>

1. `tracing` 支持结构化日志和 span，`log` 是简单文本
2. span 表示一段时间内的操作，可嵌套
3. 在 `#[instrument]` 中添加参数或使用 `Span::current().record()`

**关键理解**: tracing 是现代 Rust 应用的首选日志框架。
</details>

## 延伸阅读

学习完追踪后，你可能还想了解：

- [tracing-subscriber](https://docs.rs/tracing-subscriber) - 日志订阅器配置
- [OpenTelemetry](https://docs.rs/opentelemetry) - 分布式追踪
- [tracing-tree](https://docs.rs/tracing-tree) - 树形日志输出

**选择建议**:
- 想学习可见性 → 继续学习 [可见性控制](visiable.md)
- 想复习基础 → 跳到 [基础入门](basic-overview.md)

## 继续学习

**前一章**: [日志记录](logger.md)  
**下一章**: [可见性控制](visiable.md)

**相关章节**:
- [日志记录](logger.md) - 基础日志
- [模块系统](module.md) - tracing 模块

**返回**: [基础入门](basic-overview.md)
