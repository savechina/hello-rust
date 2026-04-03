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

---

## 过程宏

过程宏是一种更强大的宏类型，它们在编译时操作 Rust 代码的抽象语法树（AST）。与声明宏不同，过程宏可以：
- 更灵活地转换代码结构
- 实现自定义 derive 宏
- 创建属性宏（如 `#[log]`）
- 实现函数式宏

### 本项目的 #[log] 属性宏

本项目在 `crates/macros/src/lib.rs` 中实现了一个简单但实用的属性宏 `#[log]`，它会自动为函数添加执行跟踪日志。

#### 源码位置

- 实现代码：`crates/macros/src/lib.rs`
- 使用示例：`src/advance/macros_sample.rs`

---

### #[log] 宏的功能

`#[log]` 宏会在函数执行前后自动添加日志输出：

```text
--> enter {函数名}()
... 函数实际执行内容 ...
<-- exit {函数名}()
```

---

### 实现原理

过程宏接收 Rust 源代码作为输入，通过解析、转换、生成新的代码。`#[log]` 宏的工作流程：

```
┌─────────────────────────────────────────┐
│  输入: 带有 #[log] 属性的函数            │
│  #[log]                                 │
│  fn say_hello(name: &str) {             │
│      println!("Hello, {}!", name);      │
│  }                                      │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│  解析阶段                                │
│  - 使用 syn 解析 TokenStream            │
│  - 提取函数签名、代码块                  │
│  - 保存函数名、参数、返回类型            │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│  生成阶段                                │
│  - 使用 quote! 生成新代码               │
│  - 在函数体前后添加 println!            │
│  - 用闭包包装原函数体                    │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│  输出: 转换后的函数                      │
│  fn say_hello(name: &str) {             │
│      println!("--> enter say_hello()"); │
│      let result = (|| {                 │
│          println!("Hello, {}!", name);  │
│      })();                              │
│      println!("<-- exit say_hello()");  │
│      result                             │
│  }                                      │
└─────────────────────────────────────────┘
```

---

### 代码展开对比

#### 展开前（原始代码）

```rust
#[log]
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}
```

#### 展开后（编译器看到的代码）

```rust
fn say_hello(name: &str) {
    println!("--> enter say_hello()");
    let result = (|| {
        println!("Hello, {}!", name);
    })();
    println!("<-- exit say_hello()");
    result
}
```

**关键技巧**：
- 使用闭包 `(|| #fn_block)()` 包装原函数体，确保返回值正确处理
- `stringify!(#fn_name)` 将函数名转为字符串，用于日志输出
- 保留原函数的可见性、参数和返回类型

---

### 实现代码详解

查看 `crates/macros/src/lib.rs` 的核心实现：

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// 定义属性宏 #[log]
#[proc_macro_attribute]
pub fn log(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // 提取函数的各个部分
    let fn_name = &input_fn.sig.ident;        // 函数名
    let fn_block = &input_fn.block;           // 函数体
    let fn_inputs = &input_fn.sig.inputs;     // 参数列表
    let fn_output = &input_fn.sig.output;     // 返回类型
    let fn_vis = &input_fn.vis;               // 可见性（pub 等）

    // 生成展开后的代码
    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            println!("--> enter {}()", stringify!(#fn_name));
            let result = (|| #fn_block)();
            println!("<-- exit {}()");
            result
        }
    };

    TokenStream::from(expanded)
}
```

**关键依赖**：
- `proc_macro` - Rust 编译器提供的宏 API
- `syn` - 解析 Rust 代码为 AST
- `quote` - 将 AST 转回代码

---

### 使用示例

#### 示例 1：简单函数日志

```rust
use macros::log;

#[log]
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    say_hello("World");
}
```

**输出**：
```text
--> enter say_hello()
Hello, World!
<-- exit say_hello()
```

#### 示例 2：带返回值的函数

```rust
#[log]
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = calculate_sum(5, 3);
    println!("Result: {}", result);
}
```

**输出**：
```text
--> enter calculate_sum()
<-- exit calculate_sum()
Result: 8
```

注意：返回值被闭包正确捕获和返回。

---

### 知识检查点

#### ✅ 检查点 1

**问题**：过程宏和声明宏的主要区别是什么？

**答案**：过程宏操作 AST（抽象语法树），可以更灵活地转换代码结构；声明宏使用模式匹配和替换，能力受限。

---

#### ✅ 检查点 2

**问题**：为什么 `#[log]` 宏使用闭包 `(|| #fn_block)()` 包装函数体？

**答案**：闭包确保函数体的返回值被正确捕获。如果函数有返回值，闭包会捕获它，然后在宏生成的代码中返回。

---

#### ✅ 检查点 3

**问题**：`stringify!(#fn_name)` 的作用是什么？

**答案**：将标识符转换为字符串字面量。例如 `stringify!(say_hello)` 会生成 `"say_hello"` 字符串，用于日志输出。

---

### 扩展阅读

#### 官方文档
- [Rust 官方宏教程](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [proc_macro crate 文档](https://doc.rust-lang.org/proc_macro/index.html)
- [syn crate 文档](https://docs.rs/syn/latest/syn/)
- [quote crate 文档](https://docs.rs/quote/latest/quote/)

#### 进阶主题
- 自定义 derive 宏（如 `#[derive(Debug)]`）
- 函数式过程宏（如 `sql!(SELECT * FROM users)`）
- 宏的错误处理和诊断信息

#### 本项目相关
- 查看 `crates/macros/src/lib.rs` 完整实现
- 运行 `src/advance/macros_sample.rs` 中的测试
- 使用 `cargo expand` 工具查看宏展开结果（需安装 `cargo-expand`）

---

### 实践建议

#### 安装 cargo-expand

查看宏展开的实际代码：

```bash
cargo install cargo-expand
cargo expand --test test_macros_proc_log
```

这会显示宏展开后的完整代码，帮助你理解过程宏的工作原理。

#### 调试宏开发

开发过程宏时，可以使用 `eprintln!` 调试：

```rust
#[proc_macro_attribute]
pub fn log(_attr: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("Input: {}", input);  // 调试输出到 stderr
    // ... 宏逻辑 ...
}
```

编译时会输出宏接收到的原始代码。

---

### 小结

过程宏是 Rust 元编程的重要工具：

1. **三种类型**：属性宏、derive 宏、函数式宏
2. **工作原理**：接收代码 → 解析 AST → 转换 → 生成新代码
3. **本项目示例**：`#[log]` 自动添加函数执行日志
4. **关键依赖**：`syn`（解析）、`quote`（生成）、`proc_macro`（编译器接口）

通过理解和实践过程宏，你可以：
- 创建自定义属性和 derive 宏
- 减少重复代码
- 实现编译时代码生成
- 提升开发效率

下一步：尝试扩展 `#[log]` 宏，添加参数打印或执行时间测量功能。