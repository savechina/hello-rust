# 变量与表达式

## 开篇故事

想象你有一个工具箱，里面装着各种工具：螺丝刀、锤子、尺子。你给每个工具贴上标签，下一次需要时就知道去哪里找。Rust 中的**变量**就像这些贴标签的工具箱 - 它们帮你存储和管理程序中的数据。**表达式**则是你使用这些工具完成的工作。

---

## 本章适合谁

如果你是 Rust 初学者，想理解如何存储数据、进行计算和控制程序流程，本章适合你。这是所有编程的基础，即使你是第一次接触编程也能理解。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `let` 关键字声明和初始化变量
2. 理解不可变性（immutability）和可变性（mutability）的区别
3. 区分变量和常量
4. 使用表达式进行数值和位运算
5. 编写条件语句和循环表达式

---

## 前置要求

本章是 Rust 的第一章，不需要前置知识。如果你有任意编程基础（Python、JavaScript、Java 等）会更容易理解。

---

## 第一个例子

让我们从最简单的变量声明开始：

```rust
fn main() {
    let x = 5;  // 声明一个不可变变量
    let mut y = 10;  // 声明一个可变变量
    
    println!("x 的值是：{}", x);  // 输出：5
    println!("y 的值是：{}", y);  // 输出：10
    
    y = 15;  // ✅ 可以修改 y
    // x = 10;  // ❌ 编译错误！x 是不可变的
    
    println!("y 的新值是：{}", y);  // 输出：15
}
```

**关键概念**：

- `let` - 声明变量的关键字
- `mut` - 使变量可变（mutable 的缩写）
- 默认情况下，Rust 变量是**不可变**的

---

## 原理解析

### 1. 变量绑定 (Variable Binding)

在 Rust 中，声明变量称为"绑定" - 你将一个名称绑定到一个值上：

```rust
let apples = 5;  // 将名称 "apples" 绑定到值 5
```

**为什么叫绑定？**
- 不同于其他语言的"赋值"（assignment）
- Rust 的绑定是**一次性**的（除非使用 `mut`）
- 绑定后，名称和值关联在一起

### 2. 不可变性 (Immutability)

Rust 默认让变量不可变：

```rust
let x = 5;
x = 6; // ❌ 编译错误！
```

**为什么 Rust 要这样设计？**

1. **安全性**：防止意外的数据修改
2. **并发安全**：不可变数据可以安全地在线程间共享
3. **更容易推理**：知道值不会改变，代码更容易理解

**类比**：
> 就像你写在纸上的数字 - 写完后就不能改变。如果你想改，需要拿一张新纸重写。

### 3. 可变性 (Mutability)

当你需要修改变量时，使用 `mut`：

```rust
let mut counter = 0;
counter = 1;  // ✅ 可以修改
counter += 1; // ✅ 也可以这样累加
```

**注意**：只在需要修改时使用 `mut`，这是 Rust 的最佳实践。

### 4. 常量 (Constants)

常量是**永远不变**的值：

```rust
const MAX_USERS: u32 = 1000;
const PI: f64 = 3.14159;
```

**常量 vs 不可变变量**：

| 特征           | 不可变变量 (`let`) | 常量 (`const`)           |
| -------------- | ------------------ | ------------------------ |
| 关键字         | `let`              | `const`                  |
| 类型注解       | 可选               | **必须**                 |
| 编译时确定     | 否                 | **是**                   |
| 生命周期       | 作用域内           | 整个程序运行期间         |
| 内存地址       | 运行时分配         | 编译时嵌入代码           |
| 可以使用函数值 | ✅ 是               | ❌ 否（只能用字面量）      |

**何时使用常量**：

- 配置值（最大用户数、超时时间）
- 数学常数（π, e）
- 硬编码的字符串（应用名称、版本号）

### 5. 变量遮蔽 (Shadowing)

Rust 允许用相同的名称声明新变量 - 新变量会"遮蔽"旧变量：

```rust
let x = 5;
let x = x + 1;  // 新 x 遮蔽了旧 x
println!("{}", x);  // 输出：6

{
    let x = x * 2;  // 在这个作用域内，x 是 12
    println!("内部作用域的 x: {}", x);
}

println!("外部作用域的 x: {}", x);  // 输出：6
```

**遮蔽 vs 可变**：

```rust
// 使用 mut
let mut spaces = "   ";
spaces = spaces.len(); // ❌ 编译错误！类型不同

// 使用 shadowing
let spaces = "   ";
let spaces = spaces.len(); // ✅ 可以改变类型
```

**遮蔽的优势**：
- 可以改变类型
- 可以复用名称（代码更简洁）
- 在不同作用域有不同含义

---

## 常见错误

### 错误 1: 试图修改不可变变量

```rust
let x = 5;
x = 10; // ❌ 编译错误!
```

**编译器输出**:
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 10; // ❌ 编译错误!
  |     ^^^^^^^ cannot assign twice to immutable variable
```

**修复方法**：

1. **如果真的需要修改**，添加 `mut`：
   ```rust
   let mut x = 5;
   x = 10; // ✅ 现在可以了
   ```

2. **如果不需要修改**，接受不可变性：
   ```rust
   let x = 5;
   let y = 10;  // 创建新变量而不是修改
   ```

---

### 错误 2: 常量缺少类型注解

```rust
const MAX_SIZE = 100; // ❌ 编译错误!
```

**编译器输出**:
```
error[E0284]: type annotations needed
 --> src/main.rs:1:7
  |
1 | const MAX_SIZE = 100;
  |       ^^^^^^^^ consider giving `MAX_SIZE` a type
```

**修复方法**：

添加类型注解：
```rust
const MAX_SIZE: i32 = 100; // ✅
```

---

### 错误 3: 未使用变量的警告

```rust
fn main() {
    let unused = 5; // ⚠️ 警告：未使用的变量
}
```

**编译器输出**:
```
warning: unused variable: `unused`
 --> src/main.rs:2:9
  |
2 |     let unused = 5;
  |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
```

**修复方法**：

使用前缀下划线：
```rust
let _unused = 5; // ✅ 编译器知道你是故意的
```

---

## 动手练习

### 练习 1: 预测输出

不运行代码，预测下面代码的输出：

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("内部：{}", x);
    }
    println!("外部：{}", x);
}
```

<details>
<summary>点击查看答案</summary>

**输出**:
```
内部：12
外部：6
```

**解析**：
1. `x = 5` - 第一次绑定
2. `x = 5 + 1 = 6` - 遮蔽，新 x 是 6
3. 内部作用域：`x = 6 * 2 = 12` - 再次遮蔽
4. 内部作用域结束，内部 x 失效
5. 外部 x 仍然是 6

</details>

---

### 练习 2: 修复错误

下面的代码有编译错误，请修复：

```rust
fn main() {
    let counter = 0;
    counter = counter + 1;  // ❌ 错误
    println!("计数：{}", counter);
}
```

<details>
<summary>点击查看修复方法</summary>

**修复**：
```rust
fn main() {
    let mut counter = 0;  // 添加 mut 关键字
    counter = counter + 1;  // ✅ 现在可以修改了
    println!("计数：{}", counter);
}
```

</details>

---

### 练习 3: 使用常量

改写下面的代码，使用常量代替硬编码的值：

```rust
fn main() {
    let tax = 0.08 * 100.0;  // 税率 8%
    println!("税费：{}", tax);
}
```

<details>
<summary>点击查看参考实现</summary>

```rust
fn main() {
    const TAX_RATE: f64 = 0.08;
    let amount = 100.0;
    let tax = TAX_RATE * amount;
    println!("税费：{}", tax);
}
```

**好处**：
- 税率集中定义
- 易于修改
- 避免魔术数字

</details>

---

## 故障排查 (FAQ)

### Q: 什么时候应该使用 `mut`，什么时候不应该？

**A**: 遵循这个原则：

- **默认不使用** `mut` - 99% 的情况不需要
- **需要修改时使用** - 计数器、累加器、状态标志
- **可以遮蔽时优先遮蔽** - 转换类型或复用名称

示例：
```rust
// ✅ 好的实践
let config = load_config();  // 不需要修改
let mut total = 0;  // 需要累加
for item in items {
    total += item.value;
}

// ❌ 不好的实践
let mut data = fetch_data();  // 如果不需要修改，为什么要 mut？
data = transform(data); // 考虑使用遮蔽：let data = transform(data);
```

---

### Q: 常量为什么必须大写？

**A**: Rust 约定，不是强制要求。

- **大写**：`const MAX_VALUE` - 约定俗成的常量命名
- **小写**：`let max_value` - 变量命名

编译器不会强制，但违反约定会有警告。遵循约定让代码更易读。

---

### Q: 遮蔽会让代码混乱吗？

**A**: 有时会。遵循这个指南：

**好的遮蔽**：
```rust
let input = "  hello  ";
let input = input.trim();  // ✅ 明显是转换
```

**不好的遮蔽**：
```rust
let x = 5;
let x = "string";  // ❌ 类型变化太大，让人困惑
let x = x.len();   // ❌ x 现在是什么？
```

**规则**：如果遮蔽让代码更难理解，使用不同的名称。

---

## 知识扩展 (选学)

### 表达式 vs 语句

Rust 中表达式和语句有重要区别：

**语句 (Statement)**：
- 执行动作，不返回值
- 以分号 `;` 结尾
- 例如：`let x = 5;`

**表达式 (Expression)**：
- 计算并返回值
- 不以分号结尾
- 例如：`x + 1`

**示例**：
```rust
let x = 5;  // 语句
let y = {   // 块表达式
    let z = 10;
    z + 1  // 表达式（没有分号！）
};
println!("y = {}", y); // 输出：11
```

---

### 表达式返回值

在 Rust 中，**大多数结构都是表达式**：

```rust
// if 是表达式
let max = if a > b { a } else { b };

// match 是表达式
let description = match number {
    1 => "one",
    2 => "two",
    _ => "many",
};

// 循环也是表达式（返回 () 单元类型）
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // 返回值
    }
};
```

这使得 Rust 代码更简洁、更具表现力。

---

## 小结

**核心要点**：

1. **变量默认不可变** - 这是 Rust 的安全特性
2. **使用 `mut` 声明可变变量** - 只在需要时
3. **常量用 `const` 定义** - 必须标注类型，全大写命名
4. **遮蔽允许复用名称** - 可以改变类型，比 `mut` 更灵活
5. **Rust 大多数结构是表达式** - 返回值，不以分号结尾

**关键术语**：

- **Binding (绑定)**: 将名称关联到值
- **Immutable (不可变)**: 不能修改的值
- **Mutable (可变)**: 可以修改的值
- **Shadowing (遮蔽)**: 用相同名称声明新变量
- **Expression (表达式)**: 返回值的代码
- **Statement (语句)**: 不返回值的代码动作

**下一步**：

- 继续：[基础数据类型](datatype.md)
- 回顾：[总结与练习](basic-overview.md)

---

## 术语表

| English       | 中文       |
| ------------- | ---------- |
| Variable      | 变量       |
| Constant      | 常量       |
| Immutable     | 不可变     |
| Mutable       | 可变       |
| Binding       | 绑定       |
| Expression    | 表达式     |
| Statement     | 语句       |
| Shadowing     | 遮蔽       |
| Type annotation | 类型注解 |

---

完整示例：`src/basic/expression_sample.rs`

---

## 延伸阅读

学习完变量与表达式后，你可能还想了解：

- [`let` 关键字](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables) - 变量绑定深入
- [Rust 表达式语法](https://doc.rust-lang.org/reference/expressions.html) - 官方参考
- 常量 vs 变量 - 何时使用 `const`

**选择建议**:
- 初学者 → 继续学习 [数据类型](datatype.md)
- 有经验的程序员 → 跳到 [所有权](ownership.md)

## 继续学习

- 下一步：[数据类型](datatype.md)
- 相关：[函数基础](functions.md)
- 进阶：[所有权](ownership.md)

> 💡 **记住**：不可变性是 Rust 的默认设置 - 如果你不特别告诉它"这个要改变"，Rust 会让它保持不变。这是为了你的安全！

---

## 💡 小知识：为什么 Rust 变量默认不可变？

**历史教训**：
在 C/C++ 中，意外修改变量是常见 bug 来源：
```c
// C 语言示例
int calculate(int x) {
    x = x * 2;  // 😱 意外修改了参数
    return x;
}
```

**Rust 的设计哲学**：
> "如果你需要修改，请明确说出来"

这叫做**默认不可变** (immutable by default)，好处是：
1. **编译器能帮你发现错误** - 意外修改会报编译错误
2. **更容易推理代码** - 知道值不会变
3. **并发更安全** - 不可变数据可以在线程间安全共享

**试试这个**：
```rust
// 猜猜哪行会报错？
let x = 5;
x = 6;  // ❌ 编译错误！

let mut y = 5;
y = 6;  // ✅ 可以修改
```

---

## 🌟 工业界应用：配置管理

**场景**：Web 服务器配置

```rust
struct ServerConfig {
    port: u16,        // 监听端口
    host: String,     // 绑定地址
    max_connections: u32,  // 最大连接数
}

fn main() {
    // 配置在初始化后不应该改变
    let config = ServerConfig {
        port: 8080,
        host: String::from("127.0.0.1"),
        max_connections: 1000,
    };
    
    // 使用配置（只读）
    println!("服务器启动在 {}:{}", config.host, config.port);
}
```

**为什么不可变很重要**：
- 防止运行中意外修改配置
- 多个线程可以安全地读取同一配置
- 编译器保证配置不会被篡改

---

## 🧪 动手试试：不可变的好处

**练习**：下面哪个场景应该用 `mut`？

```rust
// A. 服务器端口配置
let port = 8080;

// B. 在线人数计数器
let user_count = 0;

// C. 数据库连接字符串
let db_url = "postgres://localhost/mydb";

// D. 购物车商品列表
let cart_items = Vec::new();
```

<details>
<summary>点击查看答案</summary>

**答案**：
- A. ❌ 不应该 mut - 端口配置不应该改变
- B. ✅ 应该 mut - 计数器需要累加
- C. ❌ 不应该 mut - 连接URL 不应该改变
- D. ✅ 应该 mut - 购物车需要添加/删除商品

**代码**：
```rust
let mut user_count = 0;
user_count += 1;  // ✅

let mut cart_items = Vec::new();
cart_items.push("商品");  // ✅
```

**原则**：
> 默认不加 `mut`，需要修改时再加

</details>

---

## 知识检查

**问题 1** 🟢 (基础概念)

以下代码的输出是什么？

```rust
let x = 5;
let y = x + 3;
println!("{}", y);
```

A) 编译错误  
B) 5  
C) 8  
D) 运行时错误

<details>
<summary>答案与解析</summary>

**答案**: C) 8

**解析**: i32 类型实现 Copy trait，赋值时复制值而非移动所有权。
</details>

**问题 2** 🟡 (移动语义)

这段代码有什么问题？

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```

<details>
<summary>答案与解析</summary>

**答案**: 编译错误 - `s1` 的所有权已移动给 `s2`

**修复**: 使用引用 `&s1` 或克隆 `s1.clone()`
</details>

**问题 3** 🔴 (边界情况)

以下哪个表达式会在编译时被优化？

```rust
const X: i32 = 2 + 3;
let y = 4 * 5;
static Z: i32 = 6 + 7;
```

<details>
<summary>答案与解析</summary>

**答案**: `const X` 和`let y` 都会在编译时计算

**解析**: const 强制编译时求值，字面量表达式也会被编译器优化。static 在运行时初始化。
</details>

