# 基础数据类型

Rust 支持常见的基本类型:
* 整型: i8, i16, i32, i64, i128, isize
* 无符号整型: u8, u16, u32, u64, u128, usize
* 浮点数: f32, f64
* 布尔值: bool
* 字符: char

需要注意的是, Rust 中, "xxx" 是一个字符串字面量切片, 类型为 &str, 是在编译时就固定不可变的。 而 String 是一个字符串类型, 编译时动态分配, 可变长度。


## 整型

Rust 支持多种整数类型: i8, i16, i32, i64, i128, isize 和 u8, u16, u32, u64, u128, usize。 其中 isize 和 usize 是指针大小的整数类型。


## 浮点数


## 字符串

在 Rust 中，字符串类型是 `str`，它是不可变的字符串字面量切片，类型为 &str, 是在编译时就固定不可变的。 而 String 是一个字符串类型, 编译时动态分配, 可变长度。


## 日期时间

Rust `std::time` 库提供了处理时间的功能。 你可以使用 `SystemTime` 来获取当前时间，并且可以使用 `UNIX_EPOCH` 来表示1970-01-01 00:00:00 UTC的时间戳。

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
> 什么是Unix epoch?
> Unix epoch 是一个固定的时间点，即 1970年1月1日 00:00:00 UTC。
>
> 什么是Unixtime?
> Unix time 是 Unix epoch 的时间戳，（也称为 POSIX time 或 epoch time）,它是一个自从 Unix epoch 开始经过的秒数。Unixtime 是一个非常常用的时间戳格式，在许多编程语言和系统中都广泛使用。
>
> Unix Time 的作用和优点:
> * 简化时间存储和计算: Unix time 是一个简单的整数，非常适合在计算机内部存储和进行时间比较、计算等操作。这比处理复杂的日期、月份、年份、时区和闰年规则要简单得多。
> * 跨平台兼容性: 几乎所有主流的操作系统、编程语言和数据库都支持 Unix time，使其成为在不同系统之间传递时间信息时的通用标准。
> * 精确性和一致性: Unix time 避免了时区、夏令时等问题，确保了时间表示的精确性和一致性。它通常以 32 位或 64 位整数存储，能够表示非常长的时间范围。


Rust 提供了 `chrono` 库来处理日期和时间。 你可以使用 `chrono::prelude::NaiveDate` 和 `use chrono::prelude::NaiveDateTime` 来表示日期和时间，并且可以使用 `chrono::Duration` 来表示时间差。

添加 `chrono` 库到你的项目中，你可以使用以下命令来安装：
```bash

cargo add chrono
```

或者修改 `Cargo.toml` 文件中的依赖项：


```toml
[dependencies]
chrono = { version = "0.4.39", features = ["serde"] }

```

如何获取当前日期和时间呢？ 你可以使用 `chrono::Local.now()` 来获取当前的本地日期和时间。 你也可以使用 `chrono::Utc.now()` 来获取当前的 UTC 日期和时间。

获取本地当前日期和时间：
```rust,editable
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Local};
fn main() {
    let now = Local.now();
    println!("Current date and time: {}", now);
}
```

获取 UTC当前日期和时间：
```rust
extern crate chrono;
use chrono::{DateTime, Utc};
fn main() {
    let now = Utc.now();
    println!("Current date and time: {}", now);
}
```
