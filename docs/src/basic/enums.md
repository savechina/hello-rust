# 枚举

## 开篇故事

想象你在设计一个交通信号系统。信号灯只有三种状态：红、黄、绿。你不会希望有人随意设置成"紫色"或"蓝色"。Rust 的**枚举**就是一种确保值只能是预定义选项之一的机制 - 它让不可能的状态无法表示。

---

## 本章适合谁

如果你已经学完了结构体，想学习如何表示"有限选项"的数据，本章适合你。枚举是 Rust 类型系统的重要组成部分，与模式匹配配合使用非常强大。

---

## 你会学到什么

完成本章后，你可以：

1. 定义枚举类型
2. 使用 match 进行模式匹配
3. 理解 Option 和 Result 枚举
4. 为枚举实现方法
5. 使用带数据的枚举变体

---

## 前置要求

- [结构体基础](struct.md) - 复合数据类型
- [所有权](ownership.md) - 移动和借用

---

## 第一个例子

定义一个交通灯枚举：

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;
    
    match light {
        TrafficLight::Red => println("停车！"),
        TrafficLight::Yellow => println("准备"),
        TrafficLight::Green => println("行驶！"),
    }
}
```

**关键概念**：

- `enum` - 定义枚举类型
- `TrafficLight::Red` - 枚举变体
- `match` - 模式匹配

---

## 原理解析

### 1. 枚举定义

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

每个变体（variant）都是该类型的一个可能值。

### 2. 带数据的变体

```rust
enum Message {
    Quit,  // 无数据
    Move { x: i32, y: i32 },  // 命名字段
    Write(String),  // 单个值
    ChangeColor(i32, i32, i32),  // 多个值
}
```

### 3. 模式匹配

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入：{}", text),
        Message::ChangeColor(r, g, b) => println!("颜色：{}, {}, {}", r, g, b),
    }
}
```

### 4. Option 枚举

Rust 用枚举处理空值：

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_char = Some('a');
let absent: Option<i32> = None;
```

### 5. Result 枚举

错误处理：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为 0"))
    } else {
        Ok(a / b)
    }
}
```

### 6. 代数数据类型 (ADT)

**什么是 ADT？**

代数数据类型（Algebraic Data Types）源自函数式编程，它用数学方式描述数据结构：

- **Product Types（积类型）**: 所有字段同时存在 → 就是 `struct`
- **Sum Types（和类型）**: 每次只有一个变体有效 → 就是 `enum`

```rust
// Product Type: 结构体包含所有字段
struct Point { x: i32, y: i32 }  // x 和 y 同时存在

// Sum Type: 枚举只能选一个
enum Option<T> { Some(T), None }  // 要么 Some，要么 None
```

**多语言枚举对比**

不同语言对枚举的支持差异很大：

C 语言（只有名称，无数据）:
```c
enum Color { RED, GREEN, BLUE };  // 只能是整数常量
```

Java 枚举（单例对象）:
```java
enum Color { RED, GREEN, BLUE }  // 本质是类的实例
```

Go 语言（常数 + iota）:
```go
type Color int
const (
    RED Color = iota  // 0
    GREEN              // 1
    BLUE               // 2
)
```

Swift 枚举（类似 Rust）:
```swift
enum Result<T, E> {
    case success(T)
    case failure(E)
}
```

Rust 枚举（完整 ADT）:
```rust
enum Message {
    Quit,                              // 无数据
    Move { x: i32, y: i32 },          // 命名字段
    Write(String),                     // 携带值
    ChangeColor(i32, i32, i32),        // 多个值
}
```

**为什么 ADT 强大？**

1. **类型安全**: 编译器保证你处理了所有可能的情况
2. **数据携带**: 变体可以携带任意类型的数据，不只是整数
3. **模式匹配**: `match` 表达式可以解构并提取数据

Rust 的枚举不是简单的整数常量，而是真正的代数数据类型。这让 `Option` 和 `Result` 这样的类型成为可能，从根本上避免了空指针错误。

---

## 常见错误

### 错误 1: match 不完整

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn describe(c: Color) {
    match c {
        Color::Red => println!("红色"),
        Color::Green => println("绿色"),
        // ❌ 缺少 Blue!
    }
}
```

**编译器输出**:
```
error[E0004]: non-exhaustive patterns: `Color::Blue` not covered
```

**修复**: 处理所有变体

```rust
match c {
    Color::Red => println!("红色"),
    Color::Green => println("绿色"),
    Color::Blue => println("蓝色"),
}
```

### 错误 2: 忘记解包数据

```rust
let msg = Message::Write(String::from("hello"));

match msg {
    Message::Write => println("有数据"),  // ❌ 没有解包
    _ => {}
}
```

**修复**:

```rust
match msg {
    Message::Write(text) => println("数据：{}", text),  // ✅
    _ => {}
}
```

### 错误 3: if let 类型错误

```rust
let some_value = Some(5);

if let Some(x) = some_value {
    println!("值是 {}", x);  // ✅ 正确
}

// ❌ 错误用法
if some_value == Some(5) {  // 需要 PartialEq trait
```

---

## 动手练习

### 练习 1: 定义枚举

定义一个表示一周七天的枚举：

```rust
// TODO: 定义 WeekDay 枚举，包含周一到周日
```

<details>
<summary>点击查看答案</summary>

```rust
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
```
</details>

### 练习 2: 实现方法

为 WeekDay 实现 is_weekend 方法：

```rust
impl WeekDay {
    fn is_weekend(&self) -> bool {
        // TODO: 周六或周日返回 true
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn is_weekend(&self) -> bool {
    matches!(self, WeekDay::Saturday | WeekDay::Sunday)
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 枚举和结构体有什么区别？

**A**: 
- **结构体**: 所有字段同时存在
- **枚举**: 每次只有一个变体有效

### Q: 什么时候用枚举，什么时候用布尔值？

**A**: 
- 2 个选项 → `bool`
- 3+ 个明确选项 → `enum`
- 可能需要更多选项 → `enum`

### Q: Option 和 null 有什么区别？

**A**: 
- **null**: 任何类型都可以是空，容易导致空指针错误
- **Option**: 显式标记可能为空，编译器强制检查

---

## 小结

**核心要点**：

1. **枚举定义有限选项**: 值只能是预定义的变体之一
2. **match 强制穷尽**: 必须处理所有情况
3. **Option 替代 null**: 安全的空值处理
4. **Result 处理错误**: 函数式错误处理
5. **变体可以带数据**: 灵活的类型表示

**术语**：

- **Enum (枚举)**: 有限选项的类型
- **Variant (变体)**: 枚举的一个可能值
- **Pattern matching (模式匹配)**: 根据变体执行不同代码

**下一步**：

- 继续：[特征](trait.md)
- 相关：[结构体](struct.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Enum | 枚举 |
| Variant | 变体 |
| Pattern matching | 模式匹配 |

---

完整示例：`src/basic/traits_sample.rs`

---

> 💡 **提示**：枚举让不可能的状态无法表示，这是 Rust 类型系统的美妙之处！

---

## 项目实例

虽然项目中 enums.md 示例较少，但 trait 实现和错误处理中大量使用枚举：

### Result 枚举（实际使用）

```rust
// 数据库操作中的 Result 使用
// src/advance/sqlx_sample.rs

use sqlx::{Result, Error};

async fn query_data() -> Result<Vec<User>> {
    let users = sqlx::query("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;  // 自动转换错误
    
    Ok(users)
}
```

### Option 枚举（实际使用）

```rust
// 查找用户，可能不存在
fn find_user(id: i64) -> Option<User> {
    // 可能返回 Some(user) 或 None
}

// 使用 match 处理
match find_user(1) {
    Some(user) => println!("找到用户：{}", user.name),
    None => println("用户不存在"),
}
```


---

## 延伸阅读

学习完枚举后，你可能还想了解：

- [Option 类型深入](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) - 为什么 Rust 没有 null
- [模式匹配高级用法](https://doc.rust-lang.org/book/ch18-00-patterns.html) - 高级模式
- [Newtype 模式](newtype-pattern.md) - 类型安全封装

**选择建议**:
- 想学习多态 → 继续学习 [特征](trait.md)
- 想练习 → 跳到 [泛型](generic.md)

## 继续学习

**前一章**: [结构体](struct.md)  
**下一章**: [特征 (Traits)](trait.md)

**相关章节**:
- [结构体](struct.md)
- [结构体字段](struct-fields.md)
- [结构体方法](struct-methods.md)

**返回**: [基础入门](basic-overview.md)
