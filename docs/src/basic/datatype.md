# 基础数据类型

在 Rust 中，每一个值都有一个特定 数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。

记住，Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。

## 标量类型
标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。

Rust 支持常见的基本类型:
* 整型: i8, i16, i32, i64, i128, isize
* 无符号整型: u8, u16, u32, u64, u128, usize
* 浮点数: f32, f64
* 布尔值: bool
* 字符: char


### 整型

整型 是一个没有小数部分的数字。我们在Getting Started 部分已经使用过 u64 整数类型。该类型声明表明，它关联的值应该是一个占据 64 比特位的无符号整数（有符号整数类型以 i 开头而不是 u）。表格 1 展示了 Rust 内建的整数类型。我们可以使用其中的任一个来声明一个整数值的类型。

Rust 支持多种整数类型: i8, i16, i32, i64, i128, isize 和 u8, u16, u32, u64, u128, usize。 其中 isize 和 usize 是指针大小的整数类型。

表格 1 Rust 中的整型：

| 长度      | 有符号     | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| 架构相关    | `isize` | `usize` |

### 浮点数


```admonish 

什么是数据类型？

在计算机编程语言中，数据类型（Data Type）是用来定义变量或数据的属性，指定其存储方式、取值范围以及可以执行的操作。数据类型为编译器或解释器提供了如何处理数据的指令，包括：

- 内存分配：决定变量占用多少内存（如 32 位整数占 4 字节）。
- 操作支持：定义允许的操作（如整数支持加减，字符串支持拼接）。
- 值范围：限定数据的有效范围（如 i32 的范围是 -2³¹ 到 2³¹-1）。
- 类型安全：确保操作符合数据类型的规则，防止错误（如避免将字符串当作数字相加）。

数据类型是编程语言的基础，影响程序的性能、内存使用和代码可读性。在像 Rust 这样的强类型语言中，数据类型在编译时严格检查，确保类型安全。数据类型的简要历史数据类型的发展与编程语言的演进密切相关：

- 早期（1950s-1960s）：在汇编语言和早期高级语言（如 Fortran、ALGOL）中，数据类型较为简单，主要包括整数、浮点数和字符。例如，Fortran 引入了 INTEGER 和 REAL 类型，用于数值计算。
- 结构化编程（1970s）：随着 Pascal 和 C 的出现，数据类型变得更加丰富，支持结构体（struct）和数组等复合类型，允许更复杂的数据组织。
- 面向对象编程（1980s-1990s）：Smalltalk 和 C++ 引入了类和对象作为数据类型，结合数据和行为。类型系统开始支持继承和多态。
- 现代语言（2000s-至今）：语言如 Java、Python 和 Rust 引入了更高级的类型系统，包括泛型、动态类型、枚举和模式匹配。Rust 等语言强调类型安全和内存安全，引入所有权和生命周期等概念。
- 类型系统理论：类型理论（如 Hindley-Milner 类型推导）影响了现代语言（如 Haskell、TypeScript），支持更复杂的类型推导和静态检查。

数据类型的演进反映了编程语言对性能、灵活性和安全性的不断追求。数据类型的分类数据类型的分类方式因编程语言而异，但通常可以分为以下几大类：

1. 按数据结构分类

- 标量类型（Scalar Types）：
    - 表示单一值。
    - 示例：
        - 整数（如 Rust 的 i8, i32, u64）：存储整数值。
        - 浮点数（如 f32, f64）：存储小数。
        - 布尔值（bool）：表示 `true` 或 `false`。
        - 字符（char）：存储单个 Unicode 字符（如 Rust 的 `char`）。
    - 常见于所有语言，用于基本计算。
- 复合类型（Compound Types）：
    - 由多个值组成，封装更复杂的数据结构。
    - 示例：
        - 数组（Array）：固定长度的同类型元素集合（如 Rust 的 `[i32; 5]`）。
        - 元组（Tuple）：固定长度的异构类型集合（如 Rust 的 (`i32`, `f64`, `char`)）。
        - 结构体（Struct）：自定义类型，包含命名字段（如 Rust 的 `struct`）。
        - 枚举（Enum）：表示一组可能的值（如 Rust 的 `enum`）。
        - 字符串：如 Rust 的 String 和 &str，存储文本数据。
        - 集合类型：如向量（Rust 的 `Vec<T>`）、哈希表（`HashMap`）等。
    - 复合类型用于组织复杂数据，支持更高层次的抽象。

2. 按类型检查分类

- 静态类型（Static Typing）：
    - 类型在编译时确定，变量的类型不可更改。
    - 示例：Rust、C、Java。
    - 优点：编译时捕获类型错误，性能更高。
- 动态类型（Dynamic Typing）：
    - 类型在运行时确定，变量类型可动态变化。
    - 示例：Python、JavaScript。
    - 优点：灵活性高，开发速度快。
- 结构化类型 vs 名称类型：
    - 结构化类型（Structural Typing）：类型兼容性基于结构（如 TypeScript 的鸭子类型）。
    - 名称类型（Nominal Typing）：类型基于名称（如 Rust、Java）。

3. 按内存分配分类

- 基本/原生类型（Primitive Types）：
    - 由语言直接支持，存储在栈上，固定大小。
    - 示例：Rust 的 `i32`, `f64`, `bool`, `char`。
- 引用/堆类型（Reference/Heap Types）：
    - 数据存储在堆上，通过引用访问，动态分配内存。
    - 示例：Rust 的 `String`, `Vec<T>`, `Box<T>`。
    - 通常涉及指针或引用，需考虑内存管理。

4. 按功能分类

- 数值类型：用于计算，如整数、浮点数。
- 文本类型：如字符串（`String`, `&str`）或字符（`char`）。
- 逻辑类型：如布尔值（`bool`）。
- 集合类型：如数组、列表、字典。
- 时间类型：如 Rust 标准库的 `std::time::Duration` 或 `chrono::DateTime`，用于表示时间和日期。
- 函数类型：在支持函数式编程的语言中，函数本身也是一种类型（如 Rust 的闭包类型 Fn、FnMut）。
- 用户自定义类型：通过结构体、枚举或类定义的类型。

5. 其他特殊分类

- 泛型类型（Generic Types）：允许类型参数化（如 Rust 的 `Vec<T>`）。
- 空类型/可选类型：如 Rust 的 `Option<T>`，表示可能为空的值。
- 错误类型：如 Rust 的 `Result<T, E>`，用于错误处理。
- 动态类型（Any Types）：如 Rust 的 `Box<dyn Trait>`，支持运行时多态。

```

## 复合类型

复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

### 元组（tuple）


### 数组（array）

### 字符串

在 Rust 中，字符串类型是 `str`，它是不可变的字符串字面量切片，类型为 &str, 是在编译时就固定不可变的。 而 String 是一个字符串类型, 编译时动态分配, 可变长度。


### 日期时间

#### Rust 标准库时间

Rust 的标准库，`std::time` 库提供了基础的时间处理功能。
* `SystemTime` 是一个表示时间的结构体，它是一个不可变的时间戳。表示一个时间点。 你可以使用 `SystemTime` 来获取当前时间点，并且可以使用 `UNIX_EPOCH` 来获取自Unix epoch （1970-01-01 00:00:00 UTC）以来的时间戳（Unixtime）。
* `Instant` 是用于高精度单调时间测量（不考虑系统时钟调整），适合性能测试或计时。
* `Duration` 是表示一个时间间隔，以秒和纳秒为单位。时间加减：可与 SystemTime 或 Instant 结合使用


以下是使用 `SystemTime` 和 `Instant` 的示例，你可以直接复制到你的 Rust 项目中直接运行，查看输出结果：

```rust
use std::time::{Duration, Instant, SystemTime};
use std::thread::sleep;

/// Sample function to demonstrate time operations.
pub fn time_sample() {
    //Instant
    let now: Instant = Instant::now();
    println!("Instant: {:?}!", now); // 输出当前时间戳

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); // 输出经过的时间

    //Duration
    let timeout = Duration::from_secs(1);
    let start = Instant::now();

    sleep(Duration::from_millis(500));

    if start.elapsed() > timeout {
        println!("Timeout!");
    } else {
        println!("Operation completed within timeout.");
    }

    //SystemTime 获取当前时间戳
    let now = SystemTime::now();

    //fetch unixtime, return current time in seconds since Unix epoch.it since 1970-01-01 00:00:00 UTC was seconds ago
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    //unixtime
    println!("Seconds since the epoch: {}s", since_the_epoch.as_secs());

    //当前线程休眠1000ms
    sleep(Duration::from_millis(1000));

    //返回now与当前时间的差值,单位为秒
    let elapsed = now.elapsed().unwrap();
    println!("Seconds since the elapsed: {}s", elapsed.as_secs()); //打印输出 1s
}

///to call run the time_sample function
time_sample();

// to unit test the time_sample function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_sample() {
        time_sample();
    }
}

```


> [!TIP]
> **什么是Unix epoch?**
>
> Unix epoch 是一个固定的时间点，即 1970年1月1日 00:00:00 UTC。
>
> **什么是Unixtime?**
>
> Unix time 是 Unix epoch 的时间戳，（也称为 POSIX time 或 epoch time）,它是一个自从 Unix epoch 开始经过的秒数，它不考虑闰秒，以简化和标准化时间计算。通常表示为一个整数。简单来说，Unix epoch 是基准点，Unix time 是从这个基准点开始的秒数计数。Unixtime 是一个非常常用的时间戳格式，在许多编程语言和系统中都广泛使用。
>
> **Unix Time 的作用和优点:**
> * 简化时间存储和计算: Unix time 是一个简单的整数，非常适合在计算机内部存储和进行时间比较、计算等操作。这比处理复杂的日期、月份、年份、时区和闰年规则要简单得多。
> * 跨平台兼容性: 几乎所有主流的操作系统、编程语言和数据库都支持 Unix time，使其成为在不同系统之间传递时间信息时的通用标准。
> * 精确性和一致性: Unix time 避免了时区、夏令时等问题，确保了时间表示的精确性和一致性。它通常以 32 位或 64 位整数存储，能够表示非常长的时间范围。


####  Chrono 库日期时间

Rust 社区提供了 `chrono` 库来处理日期和时间。 你可以使用 `chrono::prelude::NaiveDate` 来表示日期和 `chrono::prelude::NaiveDateTime` 来表示日期时间，并且可以使用 `chrono::Duration` 来表示时间差。


* 依赖配置

添加 `chrono` 库到你的项目中，你可以使用以下命令来安装：
```bash

cargo add chrono
```

或者修改 `Cargo.toml` 文件中的依赖项：


```toml
[dependencies]
chrono = { version = "0.4.39", features = ["serde"] }

```
* 示例代码

如何获取当前日期和时间呢？ 你可以使用 `chrono::Local.now()` 来获取当前的本地日期和时间。 你也可以使用 `chrono::Utc.now()` 来获取当前的 UTC 日期和时间。


获取本地当前日期和时间：
```rust ignore

use chrono::prelude::*;
use chrono::{DateTime, Local};
fn main() {
    let now = Local.now();
    println!("Current date and time: {}", now);
}
```


获取 UTC当前日期和时间：
```rust ignore

use chrono::{DateTime, Utc};
fn main() {
    let now = Utc.now();
    println!("Current date and time: {}", now);
}
```

一个更完成样例代码，展示了如何使用 `chrono` 库来获取当前日期和时间，并将其格式化为字符串：

```rust ignore

fn date_sample() {
    // 使用 from_ymd_opt 创建 NaiveDate
    let date = NaiveDate::from_ymd_opt(2024, 10, 26).unwrap();
    println!("Date: {}", date);

    // 使用 from_hms_opt 创建 NaiveTime
    let time = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
    println!("Time: {}", time);

    // 使用 new 创建 NaiveDateTime
    let datetime = NaiveDateTime::new(date, time);
    println!("DateTime: {}", datetime);

    // 使用 with_ymd_and_hms 创建 DateTime<Utc>
    let utc_datetime = Utc.with_ymd_and_hms(2024, 10, 26, 12, 30, 0).unwrap();
    println!("UTC DateTime: {}", utc_datetime);

    // 使用 with_ymd_and_hms 创建 DateTime<Local>
    let local_datetime = Local.with_ymd_and_hms(2024, 10, 26, 12, 30, 0).unwrap();
    println!("Local DateTime: {}", local_datetime);

    // 获取当前 UTC 时间
    let now_utc = Utc::now();
    println!("Now (UTC): {}", now_utc);

    // 获取当前本地时间
    let now_local = Local::now();
    println!("Now (Local): {}", now_local);

    //日期格式化
    let now = Utc::now();

    // 常用格式
    println!("ISO 8601 / RFC 3339: {}", now.to_rfc3339()); // 推荐的格式
    println!(
        "Year-Month-Day Hour:Minute:Second: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "Day/Month/Year Hour:Minute:Second: {}",
        now.format("%d/%m/%Y %H:%M:%S")
    );
    println!("Month Day, Year: {}", now.format("%B %d, %Y"));
    println!("Weekday, Day Month Year: {}", now.format("%A, %d %B %Y"));

    // 自定义格式
    println!("Custom format: {}", now.format("%a %b %e %T %Y"));

    // 时间戳 (Unix timestamp)
    println!("Timestamp (seconds): {}", now.timestamp());
    println!("Timestamp (milliseconds): {}", now.timestamp_millis());

    //日期解析
    let datetime_str = "2024-10-26 12:30:00";
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
    println!("Parsed DateTime: {}", datetime);

    let date_str = "2024-10-26";
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    println!("Parsed Date: {}", date);

    let rfc3339_str = "2024-10-26T12:30:00Z";
    let rfc3339_datetime = DateTime::parse_from_rfc3339(rfc3339_str).unwrap();
    println!("Parsed RFC3339 DateTime: {}", rfc3339_datetime);

    //错误处理
    let invalid_date_str = "2024-13-26";
    let invalid_date = NaiveDate::parse_from_str(invalid_date_str, "%Y-%m-%d");
    match invalid_date {
        Ok(_) => println!("Parsed Date: {:?}", invalid_date),
        Err(e) => println!("Error parsing date: {}", e),
    }
}

```