# 宏

## 声明宏

### 一个简单的声明宏(Macro)， Say Hello

一个简单的声明宏(Macro),`say_hello`,用于打印一条消息。

```rust
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn declare_macros_hello_sample() {
    say_hello!(); // 调用宏
}

```

### 带参数的声明宏

带参数的声明宏(Macro),`make_vec`,用于创建一个动态数组，并初始化值，与标准库的`vec!` 很像吧。

```rust

macro_rules! make_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

fn declare_macros_make_sample() {
    let v = make_vec!(1, 2, 3, 4);
    println!("{:?}", v); // [1, 2, 3, 4]
}

```