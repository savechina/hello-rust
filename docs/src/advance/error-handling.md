# 错误处理

## 开篇故事

想象你在经营一家餐厅。同步错误处理就像每道菜做错时就立即停下来纠正——顾客等着上菜，整个流程都阻塞了。而在 Rust 的异步世界中，错误处理更加优雅：你可以继续处理其他订单，同时标记出错的菜品，稍后统一处理。这就是 Rust 错误处理的核心思想：**显式、安全、不 panic**。

在 Rust 中，错误分为两大类：可恢复的错误（使用 `Result<T, E>`）和不可恢复的错误（使用 `panic!`）。本章专注于可恢复的错误处理，这是生产代码中最常见的场景。

---

## 本章适合谁

如果你已经理解 Rust 的基础语法和所有权，现在想学习如何在实际项目中优雅地处理错误——特别是异步环境下的错误传播和转换，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `Result<T, E>` 和 `?` 操作符进行错误传播
2. 实现 `From` trait 自动转换错误类型
3. 使用 `Box<dyn Error>` 简化错误处理
4. 在异步函数中正确处理错误
5. 避免 `.unwrap()` 在生产代码中的滥用

---

## 前置要求

学习本章前，你需要理解：

- [所有权](../basic/ownership.md) - 理解所有权转移和借用
- [特征](../basic/trait.md) - 理解 trait 定义和实现
- [泛型](../basic/generic.md) - 理解泛型语法

---

## 第一个例子

让我们看一个最简单的错误处理示例：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // 如果失败，返回 io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(text) => println!("文件内容：{}", text),
        Err(e) => println!("读取失败：{}", e),
    }
}
```

**发生了什么？**

第 5 行 `File::open(path)?` 使用 `?` 操作符：如果打开成功返回 `File`，如果失败**立即返回** `io::Error`。这比 `match` 更简洁。

---

## 原理解析

### 1. Result<T, E> 枚举

```rust
pub enum Result<T, E> {
    Ok(T),   // 成功，包含值
    Err(E),  // 失败，包含错误
}
```

**常用方法**：

```rust
let result: Result<i32, &str> = Ok(42);

// 获取值（panic 如果 Err）
let value = result.unwrap();

// 获取值或默认值
let value = result.unwrap_or(0);

// 匹配处理
match result {
    Ok(v) => println!("成功：{}", v),
    Err(e) => println!("错误：{}", e),
}
```

### 2. ? 操作符

`?` 是错误传播的语法糖：

```rust
// 以下两种写法等价：

// 使用 ?
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 使用 match
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => return Err(e),
    }
}
```

**关键点**：
- `?` 只能在返回 `Result` 的函数中使用
- `?` 自动将错误类型转换为函数返回的错误类型

### 3. From trait 错误转换

实现 `From` trait 可以自动转换错误类型：

```rust
use std::fs::File;
use std::io::{self, Read, Write};

// 自定义错误类型
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    ParseError(String),
}

// 实现 From<io::Error>
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

// 现在可以混合使用不同错误
fn process_file(path: &str) -> Result<String, MyError> {
    let mut file = File::open(path)?;  // 自动转换为 MyError::Io
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### 4. Box<dyn Error> 简化错误

当有多种错误来源时，使用 `Box<dyn Error>` 避免定义复杂的枚举：

```rust
use std::error::Error;
use std::fs::File;
use std::num::ParseIntError;

// 返回统一的错误类型
fn read_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = File::open(path)
        .map_err(|e| e.to_string())?;
    
    let number: i32 = contents.parse()?;  // ParseIntError 自动转换
    
    Ok(number)
}
```

**优点**：
- 无需定义错误枚举
- 任何实现 `Error` trait 的类型都可以返回

**缺点**：
- 运行时动态分发（轻微性能开销）
- 调用者无法精确匹配错误类型

### 5. 异步环境中的错误处理

异步函数的错误处理与同步类似，但需要注意：

```rust
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

async fn read_file_async(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[tokio::main]
async fn main() {
    match read_file_async("config.json").await {
        Ok(text) => println!("配置：{}", text),
        Err(e) => eprintln!("加载配置失败：{}", e),
    }
}
```

**关键点**：
- `.await` 在 `?` 之前：`File::open(path).await?`
- 异步错误传播与同步相同

---

## 常见错误

### 错误 1: 在生产代码中使用 .unwrap()

```rust
// ❌ 错误：panic 如果文件不存在
let file = File::open("config.json").unwrap();

// ✅ 正确：优雅处理错误
let file = match File::open("config.json") {
    Ok(f) => f,
    Err(e) => {
        eprintln!("无法打开配置文件：{}", e);
        return Err(e);
    }
};

// 或使用 ?
let file = File::open("config.json")?;
```

**修复方法**：生产代码永远不要使用 `.unwrap()`，使用 `?` 或 `match`。

### 错误 2: 错误类型不匹配

```rust
use std::fs::File;
use std::num::ParseIntError;

// ❌ 错误：返回类型不匹配
fn read_number(path: &str) -> Result<i32, ParseIntError> {
    let contents = File::open(path)?;  // io::Error 无法转换为 ParseIntError
    contents.parse()
}

// ✅ 正确：使用 Box<dyn Error>
fn read_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let number: i32 = contents.parse()?;
    Ok(number)
}
```

### 错误 3: 忽略错误

```rust
// ❌ 错误：忽略错误
let _ = File::create("important.log");

// ✅ 正确：至少记录错误
if let Err(e) = File::create("important.log") {
    eprintln!("创建日志文件失败：{}", e);
}
```

### 错误 4: 过度使用 Box<dyn Error>

```rust
// ❌ 不必要：单一错误类型
fn simple_func() -> Result<i32, Box<dyn Error>> {
    Ok(42)
}

// ✅ 更好：使用具体类型
fn simple_func() -> Result<i32, std::io::Error> {
    Ok(42)
}
```

---

## 动手练习

### 练习 1: 实现 From trait

为以下自定义错误实现 `From<io::Error>`：

```rust
use std::io;

#[derive(Debug)]
enum AppError {
    DatabaseError(String),
    IoError(io::Error),
    ConfigError(String),
}

// TODO: 实现 From<io::Error> for AppError

fn read_config(path: &str) -> Result<String, AppError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}
```

<details>
<summary>点击查看答案</summary>

```rust
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

fn read_config(path: &str) -> Result<String, AppError> {
    let contents = std::fs::read_to_string(path)?;  // 自动转换
    Ok(contents)
}
```

**解析**: 实现 `From` 后，`?` 自动将 `io::Error` 转换为 `AppError::IoError`。
</details>

### 练习 2: 使用 Box<dyn Error>

重构以下函数使用 `Box<dyn Error>`：

```rust
use std::fs::File;
use std::num::ParseIntError;

#[derive(Debug)]
enum CombinedError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

fn read_id(path: &str) -> Result<i32, CombinedError> {
    let contents = File::open(path)
        .and_then(|mut f| {
            use std::io::Read;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        })?;
    
    let id: i32 = contents.trim().parse()?;
    Ok(id)
}
```

<details>
<summary>点击查看答案</summary>

```rust
use std::error::Error;

fn read_id(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    let id: i32 = contents.trim().parse()?;
    Ok(id)
}
```

**解析**: 使用 `Box<dyn Error>` 后，无需定义 `CombinedError` 枚举，代码更简洁。
</details>

### 练习 3: 异步错误处理

完成以下异步函数：

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// TODO: 实现异步读取并解析 JSON
async fn read_json_config(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // 提示：使用 tokio::fs::File 和 AsyncReadExt
}
```

<details>
<summary>点击查看答案</summary>

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn read_json_config(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
```

**解析**: 异步错误处理与同步相同，只需添加 `.await`。
</details>

---

## 故障排查

### Q: 什么时候使用 .unwrap()？

**A**: 仅在以下场景：
- 示例代码和原型
- 测试代码
- 确定不会失败的场景（如 `"hello".len()`）

生产代码使用 `?` 或适当的错误处理。

### Q: Box<dyn Error> 的性能开销有多大？

**A**: 动态分发有轻微开销（虚表查找），但对于 I/O 绑定操作可忽略。性能关键路径使用具体错误类型。

### Q: 如何处理多个可能的错误类型？

**A**: 三种方案：
1. 定义错误枚举 + 实现 `From`（推荐用于库）
2. 使用 `Box<dyn Error>`（推荐用于应用）
3. 使用 `anyhow::Result`（应用层最简洁）

### Q: async 函数中错误类型需要 Send + Sync 吗？

**A**: 如果 Future 需要跨线程发送，需要 `Box<dyn Error + Send + Sync>`。单线程异步不需要。

---

## 知识扩展 (选学)

### thiserror 库

[`thiserror`](https://docs.rs/thiserror) 简化错误类型定义：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("数据读取失败：{0}")]
    ReadFailed(#[from] std::io::Error),
    
    #[error("无效数据：{field}")]
    InvalidData { field: String },
}
```

### anyhow 库

[`anyhow`](https://docs.rs/anyhow) 简化应用层错误处理：

```rust
use anyhow::{Result, Context};

fn read_config() -> Result<String> {
    let contents = std::fs::read_to_string("config.json")
        .context("无法读取配置文件")?;
    Ok(contents)
}
```

---

## 小结

**核心要点**：

1. **Result<T, E>**: Rust 的错误处理类型，显式处理成功和失败
2. **? 操作符**: 简洁的错误传播语法
3. **From trait**: 自动转换错误类型
4. **Box<dyn Error>**: 简化多种错误的返回
5. **避免 .unwrap()**: 生产代码使用优雅的错误处理

**关键术语**：

- **panic**: 不可恢复的错误，程序终止
- **Result**: 可恢复错误的枚举类型
- **? operator**: 错误传播操作符
- **From trait**: 类型转换 trait
- **Box<dyn Error>**: 动态错误类型

**下一步**：

- 学习 [自定义错误类型](custom-errors.md)
- 实践 [异步错误处理最佳实践](async-error-patterns.md)
- 回顾 [Result 组合子](result-combinators.md)

---

## 术语表

| English          | 中文       |
| ---------------- | ---------- |
| Error Handling   | 错误处理   |
| Panic            | 恐慌       |
| Result           | 结果       |
| Propagate        | 传播       |
| Unwrap           | 解包       |
| From Trait       | From 特征  |
| Dynamic Dispatch | 动态分发   |
| Type Erasure     | 类型擦除   |

---

完整示例：[src/advance/error_handling_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/error_handling_sample.rs)

---

## 继续学习

- 下一步：[自定义错误类型](custom-errors.md)
- 进阶：[thiserror 和 anyhow](error-crates.md)
- 回顾：[Result 组合子](result-combinators.md)

> 💡 **记住**：好的错误处理让程序更健壮。永远不要忽略错误，显式处理每一个失败场景！
