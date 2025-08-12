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


需要注意的是, Rust 中, "xxx" 是一个字符串字面量切片, 类型为 &str, 是在编译时就固定不可变的。 而 String 是一个字符串类型, 编译时动态分配, 可变长度。


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