# 基础数据类型

## 开篇故事

想象你在整理一个工具箱。你会把螺丝刀、锤子、尺子放在不同的格子里，因为每种工具用途不同。Rust 的数据类型就像这些格子——它们告诉编译器每个值应该如何存储和使用。

---

## 本章适合谁

如果你已经学完了变量与表达式，现在想了解 Rust 有哪些数据类型以及如何使用它们，本章适合你。数据类型是编程的基础，理解它们对编写正确的代码至关重要。

---

## 你会学到什么

完成本章后，你可以：

1. 区分标量类型和复合类型
2. 使用整数、浮点数、布尔值和字符类型
3. 理解 String 和 &str 的区别
4. 使用元组、数组、Vec 和 HashMap
5. 进行类型转换和类型推断

---

## 前置要求

- [变量与表达式](expression.md) - 变量绑定和可变性

---

## 第一个例子

最简单的数据类型声明：

```rust
fn main() {
    // 整数类型
    let age: u8 = 25;
    
    // 浮点数类型
    let price: f64 = 19.99;
    
    // 布尔类型
    let is_active: bool = true;
    
    // 字符类型
    let initial: char = 'R';
    
    // 字符串类型
    let message: String = String::from("Hello, Rust!");
    
    println!("Age: {}, Price: {}, Active: {}, Initial: {}", 
             age, price, is_active, initial);
    println!("Message: {}", message);
}
```

**完整示例**: [datatype_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs)

---

## 原理解析

### 标量类型 (Scalar Types)

Rust 有四种基本标量类型：

**1. 整数类型**

| 长度 | 有符号 | 无符号 |
|------|--------|--------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| 架构相关 | `isize` | `usize` |

```rust
let x: i32 = 42;      // 32 位有符号整数
let y: u64 = 1000;    // 64 位无符号整数
let z: isize = -5;    // 取决于架构
```

**2. 浮点类型**

```rust
let pi: f32 = 3.14;   // 32 位浮点数
let e: f64 = 2.718;   // 64 位浮点数 (默认)
```

**3. 布尔类型**

```rust
let is_true: bool = true;
let is_false: bool = false;
```

**4. 字符类型**

```rust
let letter: char = 'A';           // Unicode 字符
let emoji: char = '🦀';           // 支持 emoji
let chinese: char = '中';          // 支持中文
```

### 复合类型 (Compound Types)

**1. 元组 (Tuple)**

```rust
let tuple: (i32, f64, char) = (42, 3.14, 'A');

// 访问元组元素
let (x, y, z) = tuple;
println!("x: {}, y: {}, z: {}", x, y, z);

// 或使用索引
println!("First: {}", tuple.0);
```

**2. 数组 (Array)**

```rust
let array: [i32; 5] = [1, 2, 3, 4, 5];

// 访问数组元素
println!("First: {}", array[0]);
println!("Length: {}", array.len());

// 创建重复元素的数组
let repeated = [0; 5];  // [0, 0, 0, 0, 0]
```

**3. 字符串 (String)**

```rust
// String - 可增长的字符串
let mut greeting = String::from("Hello");
greeting.push_str(", world!");

// &str - 字符串切片
let slice: &str = &greeting[0..5];  // "Hello"
```

**4. Vec (动态数组)**

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 或使用宏
let numbers = vec![1, 2, 3];
```

**5. HashMap (哈希表)**

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);

// 访问
let alice_score = scores.get("Alice");
```

### 类型转换

**1. 隐式转换 (类型推断)**

```rust
let x = 42;  // 编译器推断为 i32
let y = 3.14; // 编译器推断为 f64
```

**2. 显式转换**

```rust
// 数值转换
let x: i32 = 42;
let y: f64 = x as f64;  // 42.0

// 字符串转换
let num_str = "42";
let num: i32 = num_str.parse().unwrap();
```

---

## 常见错误

### 错误 1: 整数溢出

```rust,ignore
let x: u8 = 255;
let y = x + 1;  // ❌ 编译错误：溢出
```

**错误信息**:
```
error: this arithmetic operation will overflow
```

**修复方法**:
```rust
// 使用更大的类型
let x: u16 = 255;
let y = x + 1;  // ✅ 256

// 或使用检查过的运算
let y = x.checked_add(1).unwrap();
```

### 错误 2: 类型不匹配

```rust,ignore
let x: i32 = 42;
let y: f64 = x;  // ❌ 编译错误：类型不匹配
```

**修复方法**:
```rust,ignore
let y: f64 = x as f64;  // ✅ 显式转换
```

### 错误 3: 数组越界

```rust,ignore
let array = [1, 2, 3];
let x = array[5];  // ❌ 运行时 panic
```

**修复方法**:
```rust,ignore
// 使用 get 方法安全访问
match array.get(5) {
    Some(value) => println!("{}", value),
    None => println!("Index out of bounds"),
}
```

---

## 动手练习

### 练习 1: 创建个人信息

创建一个程序，存储并打印个人信息：

```rust
fn main() {
    // TODO: 定义以下变量
    // - 姓名 (String)
    // - 年龄 (u8)
    // - 身高 (f32)
    // - 是否学生 (bool)
    
    // TODO: 打印所有信息
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn main() {
    let name = String::from("张三");
    let age: u8 = 25;
    let height: f32 = 1.75;
    let is_student: bool = false;
    
    println!("姓名：{}, 年龄：{}, 身高：{}, 学生：{}", 
             name, age, height, is_student);
}
```
</details>

### 练习 2: 使用集合类型

创建一个程序，管理学生成绩：

```rust
use std::collections::HashMap;

fn main() {
    // TODO: 创建 HashMap 存储学生成绩
    // TODO: 添加 3 个学生
    // TODO: 计算平均分
}
```

<details>
<summary>点击查看答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    let total: i32 = scores.values().sum();
    let avg = total as f64 / scores.len() as f64;
    
    println!("平均分：{}", avg);
}
```
</details>

### 练习 3: 类型转换实践

```rust
fn main() {
    let num_str = "42";
    
    // TODO: 将字符串转换为 i32
    // TODO: 将 i32 转换为 f64
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn main() {
    let num_str = "42";
    let num: i32 = num_str.parse().unwrap();
    let num_f64: f64 = num as f64;
    
    println!("i32: {}, f64: {}", num, num_f64);
}
```
</details>

---

## 故障排查 (FAQ)

### Q: String 和 &str 有什么区别？

**A**: 
- **String**: 可增长、可变的字符串，存储在堆上
- **&str**: 不可变的字符串切片，通常指向字符串字面量

```rust
let s1: String = String::from("hello");  // 可修改
let s2: &str = "world";                   // 不可修改
```

### Q: 什么时候使用 Vec 而不是数组？

**A**: 
- **数组**: 长度固定，类型 `[T; N]`
- **Vec**: 长度可变，类型 `Vec<T>`

```rust
let array: [i32; 5] = [1, 2, 3, 4, 5];  // 固定长度
let vec: Vec<i32> = vec![1, 2, 3];       // 可变长度
```

### Q: 如何选择整数类型？

**A**: 
- 一般情况：使用 `i32` (最常用)
- 需要大数：使用 `i64` 或 `i128`
- 索引/计数：使用 `usize`
- 节省内存：使用 `i8` 或 `i16`

---

## 知识扩展

### 日期与时间

Rust 标准库提供基础时间类型，第三方库 `chrono` 提供完整的日期时间处理：

**1. 标准库时间类型**

```rust
use std::time::{Instant, Duration, SystemTime};

// Instant - 用于测量时间间隔（如性能测试）
let start = Instant::now();
// ... 执行一些代码 ...
let elapsed = start.elapsed();
println!("耗时：{:?}", elapsed);

// Duration - 时间间隔
let one_second = Duration::from_secs(1);
let one_millisecond = Duration::from_millis(1);

// SystemTime - 系统时钟（可获取当前时间）
let now = SystemTime::now();
let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
println!("Unix 时间戳：{} 秒", since_epoch.as_secs());
```

**2. chrono 库日期时间**

```rust,ignore
use chrono::{Utc, Local, DateTime, NaiveDateTime};

// 获取当前 UTC 时间
let now_utc: DateTime<Utc> = Utc::now();
println!("UTC 时间：{}", now_utc);

// 获取本地时间
let now_local: DateTime<Local> = Local::now();
println!("本地时间：{}", now_local);

// 创建指定时间
let specific = Utc.with_ymd_and_hms(2024, 10, 26, 12, 30, 0).unwrap();
println!("指定时间：{}", specific);

// Unix 时间戳转换
let timestamp = 1700000000;
let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
println!("时间戳 {} 对应：{}", timestamp, datetime);

// 格式化输出
println!("{}", now_utc.format("%Y-%m-%d %H:%M:%S"));
println!("{}", now_utc.format("%Y年%m月%d日"));

// 解析字符串
let parsed = "2024-10-26 12:30:00"
    .parse::<NaiveDateTime>()
    .unwrap();
println!("解析结果：{}", parsed);
```

**3. 时间类型对比**

| 类型 | 用途 | 是否含时区 | 使用场景 |
|------|------|-----------|---------|
| `Instant` | 测量时间间隔 | 否 | 性能测试、超时检测 |
| `SystemTime` | 系统时钟 | 否 | 文件时间戳、日志时间 |
| `DateTime<Utc>` | UTC 时间 | 是 (UTC) | 服务器时间、数据库存储 |
| `DateTime<Local>` | 本地时间 | 是 (本地) | 用户界面显示 |
| `NaiveDateTime` | 无时区时间 | 否 | 内部计算、解析中间值 |

**4. Unix 时间戳**

Unix 时间戳是从 1970 年 1 月 1 日 00:00:00 UTC 开始经过的秒数：

```rust,ignore
use chrono::{DateTime, Utc};

// DateTime → Unix 时间戳
let now = Utc::now();
let timestamp = now.timestamp();
println!("Unix 时间戳：{}", timestamp);

// Unix 时间戳 → DateTime
let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
println!("对应时间：{}", datetime);
```

### BigDecimal 高精度计算

```rust,ignore
use bigdecimal::BigDecimal;

let a = BigDecimal::from(10);
let b = BigDecimal::from(3);
let result = &a / &b;

println!("{}", result);  // 3.333...
```

### chrono 日期时间

```rust,ignore
use chrono::{Utc, DateTime};

let now: DateTime<Utc> = Utc::now();
println!("当前时间：{}", now);

// 格式化
println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
```

### 集合类型对比

| 类型 | 特点 | 使用场景 |
|------|------|---------|
| 数组 | 固定长度，栈上 | 已知大小的数据 |
| Vec | 可变长度，堆上 | 动态数据集合 |
| HashMap | 键值对，哈希 | 快速查找 |
| BTreeMap | 键值对，有序 | 有序数据 |

---

## 小结

**核心要点**:

1. **标量类型**: i32, f64, bool, char - 单个值
2. **复合类型**: 元组、数组、Vec、HashMap - 多个值
3. **String vs &str**: 可变 vs 不可变
4. **类型转换**: 使用 `as` 进行显式转换
5. **集合选择**: 根据需求选择合适的数据结构

**关键术语**:

- **Scalar Type (标量类型)**: 单个值的类型
- **Compound Type (复合类型)**: 多个值的类型
- **Type Inference (类型推断)**: 编译器自动推断类型
- **Type Conversion (类型转换)**: 显式或隐式的类型转换

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Scalar Type | 标量类型 |
| Compound Type | 复合类型 |
| Type Inference | 类型推断 |
| Type Conversion | 类型转换 |
| String Slice | 字符串切片 |
| Vector | 向量 |
| Hash Map | 哈希表 |

---

## 延伸阅读

学习完数据类型后，你可能还想了解：

- [数值类型转换](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) - `as` 关键字
- [字符串类型深入](https://doc.rust-lang.org/book/ch08-02-strings.html) - `String` vs `&str`
- 数组 vs Vec - 选择指南

**选择建议**:
- 想理解内存 → 继续学习 [所有权](ownership.md)
- 想练习 → 跳到 [结构体](struct.md)

## 继续学习

**前一章**: [变量与表达式](expression.md)  
**下一章**: [了解所有权](ownership.md)

**相关章节**:
- [变量与表达式](expression.md)
- [结构体](struct.md)
- [枚举](enums.md)

**返回**: [基础入门](basic-overview.md)

---

**完整示例**: [datatype_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs)

---

## 📚 扩展阅读

**完整示例代码**:
- [datatype_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs)

**相关章节**:
- [变量与表达式](expression.md)
- [结构体](struct.md)
- [枚举](enums.md)

**外部资源**:
- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Serde Documentation](https://serde.rs/)
- [Chrono Documentation](https://docs.rs/chrono/)

---

## 🎓 知识检查题库

**问题 1** 🟢 (基础)

以下哪个是复合类型？

A) i32  
B) bool  
C) Vec<String>  
D) char

<details>
<summary>点击查看答案</summary>

**答案**: C) Vec<String>

**解析**: Vec 是动态数组，可以存储多个值，属于复合类型。i32、bool、char 都是标量类型。
</details>

**问题 2** 🟡 (中等)

以下代码会编译通过吗？

```rust,ignore
let mut names = Vec::new();
names.push("Alice");
names.push(42);
```

<details>
<summary>点击查看答案</summary>

**答案**: 不会编译通过

**解析**: Vec 是类型安全的，所有元素必须是同一类型。这里试图混合 String 和 i32。

**修复**:
```rust
let mut names: Vec<&str> = Vec::new();
names.push("Alice");
// names.push(42);  // 需要改为字符串
```
</details>

**问题 3** 🔴 (困难)

HashMap 的底层实现是什么？

A) 红黑树  
B) 哈希表  
C) 链表  
D) 堆

<details>
<summary>点击查看答案</summary>

**答案**: B) 哈希表

**解析**: HashMap 使用哈希表实现，提供 O(1) 平均时间复杂度的插入和查找。BTreeMap 使用红黑树。
</details>

---

## ✅ 本章完成检查点

- [ ] 理解标量类型和复合类型的区别
- [ ] 能够使用 String 和 &str
- [ ] 能够创建和使用 Vec
- [ ] 能够使用 HashMap 存储键值对
- [ ] 能够进行类型转换
- [ ] 完成 3 个动手练习
- [ ] 通过知识检查题库

---

**完整示例**: [datatype_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs)
