# 函数

## 开篇故事

想象你在组装乐高积木。每次需要搭建一个小房子时，你都要重新看说明书、找积木、一块块拼接——这既耗时又容易出错。但如果有一个"房子制作器"机器，你只需放入积木，按下按钮，房子就出来了！这就是函数的核心思想：**将重复的逻辑封装起来，随时调用**。

在 Rust 中，函数是代码的基本构建块。通过函数，你可以将复杂的问题分解为小的、可管理的部分，让代码更易读、可复用。

---

## 本章适合谁

如果你已经学完了变量和表达式，现在想学习如何组织代码、避免重复，本章适合你。函数是编程的基础，无论你是什么水平的开发者，都会频繁使用函数。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `fn` 关键字定义函数
2. 理解参数和返回值的语法
3. 区分表达式和语句
4. 理解所有权的转移和借用
5. 使用元组返回多个值

---

## 前置要求

学习本章前，你需要理解：

- [变量与表达式](expression.md) - 变量绑定和基础表达式
- [基础数据类型](datatype.md) - 基础类型如 i32, bool

---

## 第一个例子

让我们看一个最简单的函数定义：

```rust
fn main() {
    let result = add(3, 5);
    println!("3 + 5 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 隐式返回
}
```

**发生了什么？**

第 6 行定义了 `add` 函数：
- `fn`: 函数声明关键字
- `add`: 函数名称
- `(a: i32, b: i32)`: 两个参数，类型都是 `i32`
- `-> i32`: 返回值类型
- `a + b`: 函数体，**没有分号**表示返回值

第 2 行调用函数：`add(3, 5)` 返回 `8`。

---

## 原理解析

### 1. 函数定义语法

```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // 函数体
    expression  // 返回值（无分号）
}
```

**组成部分**：
- `fn`: 声明函数的关键字
- 函数名：使用 `snake_case` 命名（小写 + 下划线）
- 参数：`name: Type` 格式，每个参数必须标注类型
- 返回值：`-> Type`，如果无返回值可省略（相当于返回 `()`）
- 函数体：花括号包围的代码块

### 2. 参数与所有权

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ❌ 错误：s 已移动
    
    let x = 5;
    makes_copy(x);
    println!("{}", x); // ✅ 可以：i32 被复制
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string 在这里被丢弃
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // some_integer 是 Copy trait，离开作用域不丢弃
}
```

**关键点**：
- 传递所有权给函数：参数获得值的所有权
- `Copy` 类型（如 `i32`）：自动复制，原变量仍可用
- `Drop` 类型（如 `String`）：所有权转移，原变量不可用

### 3. 返回值与隐式返回

```rust
// ✅ 隐式返回（推荐）
fn add(a: i32, b: i32) -> i32 {
    a + b  // 无分号
}

// ✅ 显式返回
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // 有分号，使用 return 关键字
}

// ❌ 错误：有分号，返回 ()
fn add_wrong(a: i32, b: i32) -> i32 {
    a + b;  // 分号使这成为语句，返回 ()
}
```

**规则**：
- 最后一行表达式无分号 → 返回值
- 使用 `return` 关键字 → 提前返回
- 有分号的表达式 → 语句，不返回值

### 4. 使用元组返回多个值

```rust
fn main() {
    let (sum, product) = calculate(3, 5);
    println!("和：{}, 积：{}", sum, product);
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    let sum = a + b;
    let product = a * b;
    (sum, product)  // 返回元组
}
```

**关键点**：
- 返回类型：`(Type1, Type2)`
- 返回值：用括号包围多个值
- 解构：使用 `let (a, b) = tuple` 获取各个值

### 5. 函数参数模式

```rust
// 不可变参数（默认）
fn print_value(x: i32) {
    println!("{}", x);
    // x = x + 1; // ❌ 错误：不能修改
}

// 可变参数
fn modify_value(mut x: i32) {
    x = x + 1;  // ✅ 可以修改
    println!("{}", x);
}

// 借用参数（不获取所有权）
fn print_string(s: &String) {
    println!("{}", s);
    // s 仍归调用者所有
}

// 忽略参数
fn unused_param(_x: i32) {
    println!("不使用 x");
}
```

---

## 常见错误

### 错误 1: 忘记返回类型

```rust
// ❌ 错误：缺少返回类型
fn add(a: i32, b: i32) {
    a + b
}

// ✅ 正确
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**编译器输出**:
```
error[E0308]: mismatched types
  --> src/main.rs:4:5
   |
1  | fn add(a: i32, b: i32) {
   |                       - help: add a return type: `-> i32`
...
4  |     a + b
   |     ^^^^^ expected `()`, found `i32`
```

### 错误 2: 返回值加分号

```rust
// ❌ 错误：返回值有分号
fn add(a: i32, b: i32) -> i32 {
    a + b;  // 分号使这成为语句
}

// ✅ 正确
fn add(a: i32, b: i32) -> i32 {
    a + b  // 无分号
}
```

**编译器输出**:
```
error[E0308]: mismatched types
 --> src/main.rs:3:5
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |                             --- expected `i32` because of return type
2 |     a + b;
  |          - help: remove this semicolon to return this value
3 | }
  |     ^ expected `i32`, found `()`
```

### 错误 3: 所有权转移后使用

```rust
// ❌ 错误：所有权转移后使用变量
fn main() {
    let s = String::from("hello");
    print_string(s);
    println!("{}", s); // ❌ s 已移动
}

fn print_string(s: String) {
    println!("{}", s);
}

// ✅ 正确：使用借用
fn main() {
    let s = String::from("hello");
    print_string(&s);
    println!("{}", s); // ✅ s 仍可用
}

fn print_string(s: &String) {
    println!("{}", s);
}
```

### 错误 4: 参数类型不匹配

```rust
// ❌ 错误：类型不匹配
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3.0, 5.0); // f64 不是 i32
}

// ✅ 正确：使用正确类型
fn main() {
    let result = add(3, 5); // i32
}
```

---

## 动手练习

### 练习 1: 基础函数

定义一个函数 `greet`，接受名字参数并打印问候语：

```rust
// TODO: 定义 greet 函数

fn main() {
    greet("Alice");  // 应打印 "Hello, Alice!"
    greet("Bob");    // 应打印 "Hello, Bob!"
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
    greet("Bob");
}
```

**解析**: 使用 `&str` 作为参数类型，可以接受字符串字面量。
</details>

### 练习 2: 返回值

定义一个函数计算圆的面积：

```rust
// TODO: 定义 circle_area 函数，接受半径，返回面积

fn main() {
    let area = circle_area(5.0);
    println!("半径为 5 的圆面积：{}", area);
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn main() {
    let area = circle_area(5.0);
    println!("半径为 5 的圆面积：{}", area);
}
```

**解析**: 使用 `f64` 处理浮点数，`PI` 常量在 `std::f64::consts` 中。
</details>

### 练习 3: 多返回值

定义一个函数同时返回商和余数：

```rust
// TODO: 定义 div_mod 函数，返回 (商，余数)

fn main() {
    let (quotient, remainder) = div_mod(17, 5);
    println!("17 / 5 = {} 余 {}", quotient, remainder);
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn div_mod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, remainder) = div_mod(17, 5);
    println!("17 / 5 = {} 余 {}", quotient, remainder);
}
```

**解析**: 使用元组 `(i32, i32)` 返回两个值，`/` 是除法，`%` 是取余。
</details>

### 练习 4: 所有权与借用

完成以下代码，使 `s` 在调用后仍可用：

```rust
fn main() {
    let mut s = String::from("hello");
    
    // TODO: 修改 print_and_add 函数，使 s 在调用后仍可用
    print_and_add(&mut s);
    println!("修改后：{}", s);
}

fn print_and_add(s: String) {
    println!("{}", s);
    // s.push_str(" world"); // 如果取消注释，需要可变借用
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn main() {
    let mut s = String::from("hello");
    print_and_add(&mut s);
    println!("修改后：{}", s);
}

fn print_and_add(s: &mut String) {
    println!("{}", s);
    s.push_str(" world");
}
```

**解析**: 使用 `&mut String` 可变借用，函数可以修改但不获取所有权。
</details>

---

## 故障排查

### Q: 什么时候使用 `return` 关键字？

**A**: 通常不需要。Rust 函数隐式返回最后一个表达式。仅在以下情况使用：
- 提前返回（在 `if` 语句中）
- 从深层嵌套中返回
- 使代码更清晰

### Q: 参数类型应该用 `&str` 还是 `&String`？

**A**: 优先使用 `&str`，因为：
- 可以接受 `&String` 和字符串字面量
- 更灵活、更通用
- 除非需要 `String` 的方法，否则不需要 `&String`

### Q: 函数可以返回引用吗？

**A**: 可以，但需要生命周期标注：

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

### Q: 如何定义不接受参数的函数？

**A**: 使用空括号 `()`：

```rust
fn say_hello() {
    println!("Hello!");
}
```

---

## 知识扩展 (选学)

### 函数指针

函数也可以作为参数传递：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate<F>(a: i32, b: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(a, b)
}

fn main() {
    let result = calculate(3, 5, add);
    println!("结果：{}", result);
}
```

### 泛型函数

函数可以接受泛型参数：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 闭包简介

闭包是匿名函数，可以捕获环境变量：

```rust
let x = 5;
let add_x = |y| y + x;
println!("{}", add_x(3));  // 打印 8
```

---

## 小结

**核心要点**：

1. **函数定义**: `fn name(params) -> ReturnType { body }`
2. **参数类型**: 必须标注类型，使用 `name: Type` 格式
3. **返回值**: 最后一个表达式（无分号）或使用 `return`
4. **所有权**: 参数默认获取所有权，使用 `&` 借用
5. **元组返回**: 使用 `(a, b)` 返回多个值

**关键术语**：

- **Function**: 函数，可重用的代码块
- **Parameter**: 参数，函数的输入
- **Return Type**: 返回类型，函数的输出类型
- **Ownership**: 所有权，值的归属
- **Borrowing**: 借用，临时使用值

**下一步**：

- 学习 控制流
- 理解 [所有权](ownership.md)
- 探索 [闭包](closure.md)

---

## 术语表

| English    | 中文   |
| ---------- | ------ |
| Function   | 函数   |
| Parameter  | 参数   |
| Argument   | 实参   |
| Return     | 返回   |
| Ownership  | 所有权 |
| Borrow     | 借用   |
| Tuple      | 元组   |

---

完整示例：`src/basic/expression_sample.rs` - 函数定义和调用  
相关示例：`src/basic/generic_sample.rs` - 泛型函数示例

---

## 知识检查

**快速测验**（答案在下方）：

1. 这段代码能编译通过吗？为什么？
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}
```

2. 函数参数默认是可变还是不可变？

3. 如何返回多个值？

<details>
<summary>点击查看答案与解析</summary>

1. ❌ 不能 - 返回值有分号，返回 `()` 而不是 `i32`
2. 不可变 - 需要使用 `mut` 关键字
3. 使用元组：`fn foo() -> (i32, String) { (1, "hello".to_string()) }`

**关键理解**: Rust 函数返回值是最后一个表达式（无分号）。
</details>

## 继续学习

- 下一步：[基础数据类型](datatype.md) - 深入理解 Rust 类型系统
- 进阶：[泛型](generic.md) - 编写通用函数
- 回顾：[变量与表达式](expression.md) - 基础语法

> 💡 **记住**：函数是代码的基石。好的函数应该短小、专注、可复用！
