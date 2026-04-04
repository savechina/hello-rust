# 宏编程

## 开篇故事

想象你经常写重复的代码。传统方式是：复制粘贴 → 修改 → 容易出错。宏就像是：告诉编译器"按这个模板生成代码"，它自动完成。Rust 宏是强大的元编程工具。

---

## 本章适合谁

如果你想减少代码重复、创建 DSL（领域特定语言），本章适合你。宏是 Rust 元编程的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 理解宏的概念
2. 使用声明宏 (macro_rules!)
3. 使用过程宏 (proc-macro)
4. 创建自定义宏
5. 理解宏卫生

---

## 前置要求

- 函数 - 函数基础
- 特征 - trait 基础
- 模块 - 模块基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add macros --path crates/macros
```

## 第一个例子

最简单的宏使用：

```rust,ignore
// 声明宏
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn main() {
    say_hello!();  // 调用宏
}
```

**完整示例**: [macros_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/macros_sample.rs)

---

## 原理解析

### 宏展开过程

**声明宏展开过程**：

```
源代码:
let v = make_vec!(1, 2, 3);
         │
         ▼
宏匹配阶段:
macro_rules! make_vec {
    ( $( $x:expr ),* ) => { ... }
}
         │
         ▼ 匹配: $(1, 2, 3)
         │
         ▼ 展开:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
         │
         ▼
编译后代码:
let v = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
```

**过程宏展开过程**：

```
源代码:
#[derive(Serialize, Deserialize)]
struct User { name: String }
         │
         ▼
编译时 (proc-macro):
1. 解析输入 TokenStream
2. 修改/生成 AST
3. 输出新 TokenStream
         │
         ▼
展开后代码:
struct User { name: String }
impl Serialize for User { ... }
impl Deserialize for User { ... }
```

### 宏的类型

**Rust 有两种宏**：

- ✅ **声明宏**: macro_rules!
- ✅ **过程宏**: 自定义派生、属性、函数宏

### 声明宏

**使用 macro_rules!**：

```rust,ignore
macro_rules! make_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

fn main() {
    let v = make_vec!(1, 2, 3, 4);
    println!("{:?}", v);  // [1, 2, 3, 4]
}
```

### 宏匹配模式

**匹配不同参数**：

```rust,ignore
macro_rules! print_value {
    ( $val:expr ) => {
        println!("The value is: {}", $val);
    };
    
    ( $name:ident => $val:expr ) => {
        println!("{}: {}", stringify!($name), $val);
    };
}

fn main() {
    print_value!(42);
    print_value!(name => "Alice");
}
```

### 过程宏

**使用过程宏**：

```rust,ignore
// 在 crates/macros/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 修改函数，添加日志
    // ...
}

// 使用
#[log]
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}
```

### 宏卫生

**宏卫生 (Hygiene)**：

```rust,ignore
macro_rules! create_x {
    () => {
        let x = 42;
    };
}

fn main() {
    create_x!();
    // println!("{}", x);  // ❌ x 在宏作用域外不可见
}
```

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到宏编程时的困惑——你并不孤单！**

### 困惑 1: "宏和函数有什么区别？为什么需要宏？"

**解答**: 宏在**编译时**展开生成代码，函数在**运行时**执行：

```
函数:
fn add(a: i32, b: i32) -> i32 { a + b }
调用: add(1, 2) → 运行时执行 → 返回 3

宏:
macro_rules! add {
    ($a:expr, $b:expr) => { $a + $b };
}
调用: add!(1, 2) → 编译时展开为 1 + 2 → 编译后代码直接是 3
```

**关键区别**:
- **函数**: 运行时调用，有调用开销
- **宏**: 编译时展开，零运行时开销，可以生成任意代码

**为什么需要宏**:
- 减少重复代码（如 `vec![1, 2, 3]` 生成多个 `push` 调用）
- 实现 DSL（领域特定语言，如 `println!`）
- 编译时计算和代码生成

### 困惑 2: "宏卫生 (Hygiene) 是什么？为什么宏里定义的变量外面不能用？"

**解答**: 宏卫生是 Rust 宏的安全特性——宏内部定义的标识符不会和外部冲突：

```rust
macro_rules! create_x {
    () => {
        let x = 42;  // 这个 x 在宏的"卫生作用域"内
    };
}

fn main() {
    create_x!();
    // println!("{}", x);  // ❌ 编译错误！x 不可见
}
```

**为什么这样设计**: 防止宏意外覆盖外部变量：

```rust
// 如果没有卫生机制:
macro_rules! bad_macro {
    () => { let x = 42; };
}

fn main() {
    let x = 10;
    bad_macro!();  // 如果宏能覆盖外部 x，这里 x 就变成 42 了！
    println!("{}", x);  // 预期 10，实际 42 — 严重 bug！
}
```

### 困惑 3: "什么时候用声明宏，什么时候用过程宏？"

**解答**:

| 场景                   | 推荐宏类型 | 原因               |
| ---------------------- | ---------- | ------------------ |
| 简单代码生成           | 声明宏     | 简单、无需额外 crate |
| 需要解析复杂语法       | 过程宏     | 可以访问 AST       |
| 自定义 `#[derive]`     | 过程宏     | 只能由过程宏实现   |
| 自定义属性 `#[attr]`   | 过程宏     | 只能由过程宏实现   |
| 需要外部 crate 依赖    | 过程宏     | 声明宏无法导入外部 |

**简单判断**: 
- 如果只是重复代码模式 → 声明宏
- 如果需要修改结构体/枚举的定义 → 过程宏

### 困惑 4: "`$()` 语法到底是什么意思？"

**解答**: `$()` 是宏的**重复模式**，类似正则表达式的分组：

```rust
macro_rules! make_vec {
    ( $( $x:expr ),* ) => {
        // $()* 表示"重复零次或多次"
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*  // 对每个 $x 执行一次
            temp_vec
        }
    };
}

// 调用:
make_vec!(1, 2, 3)
// 展开为:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

**常用重复模式**:
- `$(...)*` — 零次或多次
- `$(...)+` — 一次或多次
- `$(...)?` — 零次或一次
- 分隔符可以是 `,`、`;`、或任意符号

---

## 常见错误

### 错误 1: 宏作用域

```rust,ignore
mod utils {
    macro_rules! say_hello {
        () => { println!("Hello!"); };
    }
}

fn main() {
    say_hello!();  // ❌ 宏在模块外不可见
}
```

**修复方法**:
```rust,ignore
mod utils {
    #[macro_export]
    macro_rules! say_hello {
        () => { println!("Hello!"); };
    }
}

fn main() {
    say_hello!();  // ✅ 使用 #[macro_export]
}
```

### 错误 2: 参数不匹配

```rust,ignore
macro_rules! print_value {
    ( $val:expr ) => {
        println!("Value: {}", $val);
    };
}

print_value!(1, 2, 3);  // ❌ 期望 1 个参数，得到 3 个
```

**修复方法**:
```rust,ignore
macro_rules! print_values {
    ( $( $val:expr ),* ) => {
        $( println!("Value: {}", $val); )*
    };
}

print_values!(1, 2, 3);  // ✅ 多个参数
```

### 错误 3: 过程宏错误

```rust,ignore
#[log]  // ❌ 忘记导入宏
fn say_hello() {}
```

**修复方法**:
```rust,ignore
use macros::log;  // ✅ 导入宏

#[log]
fn say_hello() {}
```

---

## 动手练习

### 练习 1: 创建简单宏

```rust,ignore
// TODO: 创建 say_hi 宏
// 打印 "Hi from macro!"

fn main() {
    say_hi!();  // 调用宏
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
macro_rules! say_hi {
    () => {
        println!("Hi from macro!");
    };
}
```
</details>

### 练习 2: 带参数的宏

```rust,ignore
// TODO: 创建 greet 宏
// 接受一个名字参数
// 打印 "Hello, {name}!"

fn main() {
    greet!("Alice");
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
macro_rules! greet {
    ( $name:expr ) => {
        println!("Hello, {}!", $name);
    };
}
```
</details>

### 练习 3: 可变参数宏

```rust,ignore
// TODO: 创建 sum 宏
// 接受多个数字参数
// 打印总和

fn main() {
    sum!(1, 2, 3, 4, 5);  // 应该打印 15
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        let mut sum = 0;
        $( sum += $x; )*
        println!("Sum: {}", sum);
    };
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 宏和函数有什么区别？

**A**: 
- **宏**: 编译时展开，生成代码
- **函数**: 运行时调用，执行代码
- **宏**: 更灵活，可以生成任意代码

### Q: 什么时候使用宏？

**A**: 
- 减少代码重复
- 创建 DSL
- 元编程需求
- **避免**: 简单逻辑用函数

### Q: 宏会影响编译时间吗？

**A**: 
- 会略微增加编译时间
- 但通常可接受
- 复杂宏影响更大

---

## 知识扩展

### 高级宏技巧

```rust,ignore
macro_rules! impl_trait_for_nums {
    ( $($t:ty),* ) => {
        $(
            impl MyTrait for $t {
                fn do_something(&self) {
                    println!("Doing for {}", self);
                }
            }
        )*
    };
}

impl_trait_for_nums!(i32, i64, u32, u64);
```

### 过程宏类型

```rust,ignore
// 属性宏
#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
}

// 派生宏
#[proc_macro_derive(MyDerive)]
pub fn my_derive(item: TokenStream) -> TokenStream {
    // ...
}

// 函数宏
#[proc_macro]
pub fn my_macro(item: TokenStream) -> TokenStream {
    // ...
}
```

### 宏调试

```rust,ignore
macro_rules! debug {
    ( $val:expr ) => {
        println!("{} = {:?}", stringify!($val), $val);
    };
}

debug!(some_variable);
// 输出：some_variable = 42
```

---

## 小结

**核心要点**：

1. **macro_rules!**: 声明宏
2. **proc-macro**: 过程宏
5. **卫生**: 宏作用域隔离
6. **元编程**: 编译时生成代码

**关键术语**：

- **Macro**: 宏
- **macro_rules!**: 声明宏
- **proc-macro**: 过程宏
- **Hygiene**: 卫生
- **DSL**: 领域特定语言

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Macro | 宏 |
| macro_rules! | 声明宏 |
| Procedural Macro | 过程宏 |
| Hygiene | 卫生 |
| DSL | 领域特定语言 |

---

## 知识检查

**快速测验**（答案在下方）：

1. 声明宏和过程宏有什么区别？

2. `macro_rules!` 中的 `$()` 语法是什么？

3. 过程宏需要什么类型的 crate？

<details>
<summary>点击查看答案与解析</summary>

1. 声明宏 = 模式匹配替换，过程宏 = Rust 代码操作 AST
2. 重复匹配：`$(...),*` 匹配逗号分隔的零或多个项
3. `proc-macro = true` 的 crate 类型

**关键理解**: 声明宏适合简单代码生成，过程宏适合复杂转换。
</details>

## 继续学习

**前一章**: [派生宏](getset.md)  
**下一章**: [类型别名](typealias.md)

**相关章节**:
- [派生宏](getset.md)
- 特征
- 模块

**返回**: 高级进阶

---

**完整示例**: [macros_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/macros_sample.rs)
