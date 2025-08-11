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

```rust
use std::time::{Duration, Instant, SystemTime};

pub fn time_sample() {
    //Instant
    let now: Instant = Instant::now();
    println!("Instant: {:?}!", now); // 输出当前时间

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

    //SystemTime
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    //since 1970-01-01 00:00:00 UTC was seconds ago
    println!("Seconds since the epoch: {}", since_the_epoch.as_secs());

    sleep(Duration::from_millis(5000));

    //返回now与当前时间的差值,单位为秒
    let elapsed = now.elapsed().unwrap();
    println!("Seconds since the elapsed: {}", elapsed.as_secs());
}

```


Rust 提供了 `chrono` 库来处理日期和时间。 你可以使用 `chrono::prelude::NaiveDate` 和 `use chrono::prelude::NaiveDateTime` 来表示日期和时间，并且可以使用 `chrono::Duration` 来表示时间差。


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



