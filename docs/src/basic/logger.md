# 日志记录 (Logger)

## 开篇故事

想象你在驾驶一辆汽车，仪表盘告诉你车速、油量、发动机状态。没有这些信息，你就像在盲开。Rust 程序的**日志**就是仪表盘 - 它告诉你程序正在发生什么，帮助你诊断问题。

---

## 本章适合谁

如果你已经能写基础 Rust 代码，现在想知道如何让程序"开口说话"（输出运行信息），本章适合你。日志是调试和监控的关键工具。

---

## 你会学到什么

1. 使用 env_logger 配置日志
2. 不同日志级别（info, debug, error, trace）
3. 自定义日志格式
4. 日志与随机数生成
5. 实际应用中的日志模式

---

## 前置要求

学习本章前，你需要理解：

- [变量表达式](expression.md) - 变量绑定
- [函数基础](functions.md) - 函数定义

---

## 第一个例子

```rust
// src/basic/logger_sample.rs

use env_logger;
use log::{info, debug, trace};
use rand::random;

pub fn logger_print() {
    // 初始化日志器，设置级别为 Debug
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let n = random::<i32>();
    info!("logger is info, random n: {}", n);

    let m = random::<char>();
    trace!("logger trace is {}", m);

    let x = 32;
    debug!("this is debug info. {}", x);
}
```

**输出示例**：
```
[INFO] logger is info, random n: 42
[TRACE] logger trace is a
[DEBUG] this is debug info. 32
[INFO] add_one result is 33
```

---

## 原理解析

### 1. 日志级别

Rust 日志有 5 个级别（从低到高）：

```rust
trace!("最详细的调试信息");  // TRACE - 最详细
debug!("调试信息");          // DEBUG - 开发时使用
info!("一般信息");           // INFO - 用户可见
warn!("警告");               // WARN - 潜在问题
error!("错误");              // ERROR - 严重影响
```

### 2. 初始化日志器

```rust
fn main() {
    // 方式 1：简单初始化（使用环境变量控制级别）
    env_logger::init();
    
    // 方式 2：自定义配置
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)  // 设置级别
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();
}
```

### 3. 环境变量控制

运行程序时设置 `RUST_LOG` 环境变量：

```bash
# 只显示错误
RUST_LOG=error cargo run

# 显示调试信息
RUST_LOG=debug cargo run

# 对不同模块设置不同级别
RUST_LOG=my_crate=debug,other_crate=info cargo run
```

---

## 常见错误

### 错误 1: 忘记初始化

```rust
fn main() {
    info!("This won't show up!");  // ❌ 没有初始化
}
```

**修复**：必须先初始化

```rust
fn main() {
    env_logger::init();  // ✅
    info!("Now it works!");
}
```

### 错误 2: 忘记添加依赖

```toml
# Cargo.toml
[dependencies]
log = "0.4"
env_logger = "0.10"
```

### 错误 3: 日志级别设置错误

```rust
env_logger::Builder::new()
    .filter(None, log::LevelFilter::Error)  // ❌ 只显示错误
    .init();

debug!("This debug won't show");  // 被过滤了
```

**修复**：根据需求选择级别

```rust
.filter(None, log::LevelFilter::Debug)  // ✅ 显示 debug 及以上
```

---

## 动手练习

### 练习 1: 设置日志级别

```rust
// TODO: 设置日志级别为 Warn，观察输出变化
env_logger::Builder::new()
    .filter(None, /* TODO: 这里填什么？ */)
    .init();
```

<details>
<summary>点击查看答案</summary>

```rust
.filter(None, log::LevelFilter::Warn)
```

**结果**：只有 `warn!` 和 `error!` 会显示，`info!` 和`debug!` 被过滤。
</details>

### 练习 2: 格式化日志

```rust
// TODO: 使用 format! 宏创建自定义日志消息
let x = 42;
info!(/* TODO: 自定义格式 */);
```

<details>
<summary>点击查看答案</summary>

```rust
info!("计算结果：{}", x);
```
</details>

---

## 故障排查 (FAQ)

### Q: 日志没有输出怎么办？

**A**: 检查三点：
1. 是否调用了 `env_logger::init()`
2. `RUST_LOG` 环境变量设置
3. 日志级别是否正确

```bash
# 临时设置
RUST_LOG=debug cargo run
```

### Q: 如何把日志输出到文件？

**A**: 使用 `env_logger` 的 `write_style` 配置：

```rust
use std::fs::File;

let file = File::create("app.log").unwrap();
env_logger::Builder::new()
    .target(env_logger::Target::Pipe(Box::new(file)))
    .init();
```

### Q: 生产环境应该用什么级别？

**A**: 推荐：
- **开发**: `debug`
- **测试**: `info`
- **生产**: `warn` 或 `error`

---

## 小结

**核心要点**：

1. **5 个日志级别**: trace → debug → info → warn → error
2. **必须初始化**: `env_logger::init()` 或 `Builder::new().init()`
3. **环境变量控制**: `RUST_LOG=debug cargo run`
4. **灵活配置**: 可为不同模块设置不同级别
5. **性能考虑**: 生产环境使用较高日志级别（减少输出）

**术语**：

- **Log level (日志级别)**: 日志的严重等级
- **Filter (过滤)**: 根据级别筛选日志
- **Formatter (格式化器)**: 日志输出格式

**下一步**：

- 继续：[追踪 (Tracking)](tracing.md) - 异步日志
- 相关：[错误处理](error-handling.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Logger | 日志器 |
| Log level | 日志级别 |
| Filter | 过滤器 |
| Format | 格式化 |

---

**完整源码**：[`src/basic/logger_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/logger_sample.rs)

---

> 💡 **提示**：好的日志就像飞机的黑匣子 - 平时看不见，出问题时能救命！
