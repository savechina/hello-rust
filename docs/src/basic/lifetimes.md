# 生命周期

## 开篇故事

想象你在图书馆借书。每本书都有一个借阅期限标签，告诉你这本书必须在什么时候归还。如果借阅期限到了，你就不能再使用这本书。Rust 的生命周期就像是这些借阅期限标签——它们告诉编译器**引用的有效范围**，确保你永远不会使用一个已经"过期"的引用。

生命周期是 Rust 最独特的特性之一。它确保内存安全，无需垃圾回收。理解生命周期是掌握 Rust 借用检查器的关键。

---

## 本章适合谁

如果你已经理解了所有权和借用，但在编译时遇到类似"borrowed value does not live long enough"的错误，本章适合你。生命周期标注听起来复杂，但一旦理解，就能轻松解决这类错误。

---

## 你会学到什么

完成本章后，你可以：

1. 理解生命周期的概念和作用
2. 识别需要生命周期标注的场景
3. 使用 `'a` 语法标注生命周期
4. 理解生命周期省略规则
5. 解决常见的生命周期错误

---

## 前置要求

学习本章前，你需要理解：

- [所有权](ownership.md) - 理解所有权转移和借用
- [结构体](struct.md) - 理解结构体定义
- [泛型](generic.md) - 理解泛型语法

---

## 第一个例子

让我们看一个最简单的生命周期示例：

```rust
fn main() {
    let r;                // ---------+-- 'a (外层作用域)
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

**发生了什么？**

变量 `x` 的生命周期是 `'b`（内层花括号），引用 `r` 的生命周期是 `'a`（整个 `main` 函数）。

第 5 行 `r = &x;` 试图将短生命周期的引用赋给长生命周期的变量，这会导致**悬垂引用**错误。

**编译器输出**:
```
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
```

---

## 原理解析

### 生命周期关系图

```
'a: [=======================================]  (外层作用域)
      'b: [==================]               (内层作用域)
      
'a 包含 'b → 'b: 'a (b 的生命周期是 a 的子集)

函数签名:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
         │         │          │          │
         │         │          │          └── 返回值生命周期 = 较短的那个
         │         │          └── y 活至少 'a
         │         └── x 活至少 'a
         └── 声明生命周期参数
```

### 1. 什么是生命周期？

生命周期是**引用的有效作用域**。每个引用都有生命周期，编译器自动推断，通常无需标注。

```rust
{
    let x = 5;            // 'a 开始
    let r = &x;           // r 的生命周期依赖于 x
    println!("{}", r);
}                         // 'a 结束，x 和 r 都失效
```

**关键点**：
- 生命周期是作用域的名称
- 编译器确保引用不会比它指向的数据活得更久
- 防止悬垂引用（dangling reference）

### 2. 生命周期标注语法

当编译器无法自动推断时，需要手动标注：

```rust
// 泛型生命周期参数 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**语法解析**：
- `<'a>`: 声明生命周期参数
- `&'a str`: 引用活至少 `'a` 这么久
- `-> &'a str`: 返回值也活至少 `'a` 这么久

**含义**：返回值的生命周期与两个参数中较短的那个相同。

### 3. 结构体中的生命周期

当结构体持有引用时，必须标注生命周期：

```rust
// ❌ 错误：缺少生命周期
struct Excerpt {
    part: &str,
}

// ✅ 正确：标注生命周期
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("摘录：{}", excerpt.part);
}
```

**规则**：
- 结构体有引用字段 → 需要生命周期参数
- 生命周期参数放在结构体名后：`struct Name<'a>`
- 每个引用字段使用该生命周期：`field: &'a Type`

### 4. 生命周期省略规则

编译器使用三条规则自动推断生命周期，无需标注：

**规则 1**: 每个引用参数获得独立的生命周期
```rust
fn first_word(s: &str) -> &str {
    // 实际被推断为：
    // fn first_word<'a>(s: &'a str) -> &'a str
}
```

**规则 2**: 如果只有一个输入生命周期，它被赋给所有输出生命周期
```rust
fn longest(x: &str, y: &str) -> &str {
    // 无法推断，因为有两个输入生命周期
    // 必须手动标注
}
```

**规则 3**: 如果有 `&self` 或 `&mut self`，`self` 的生命周期被赋给所有输出生命周期
```rust
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // self 的生命周期赋给返回值
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### 5. 'static 生命周期

`'static` 是特殊的生命周期，表示**整个程序运行期间都有效**：

```rust
// 字符串字面量是 &'static str
let s: &'static str = "I live forever!";

// 函数返回 'static
fn get_static() -> &'static str {
    "also lives forever"
}
```

**常见场景**：
- 字符串字面量
- 全局变量
- 单例

---

## 常见错误

### 错误 1: 悬垂引用

```rust
// ❌ 错误：返回局部变量的引用
fn dangling_reference() -> &i32 {
    let x = 5;
    &x  // x 在函数结束时被丢弃
}

// ✅ 正确：返回拥有的值
fn no_dangling() -> i32 {
    let x = 5;
    x  // 返回值，不是引用
}
```

**编译器输出**:
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:31
  |
1 | fn dangling_reference() -> &i32 {
  |                               ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```

### 错误 2: 生命周期不匹配

```rust
// ❌ 错误：返回值生命周期不明确
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ✅ 正确：标注生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 错误 3: 结构体缺少生命周期

```rust
// ❌ 错误
struct Excerpt {
    part: &str,
}

// ✅ 正确
struct Excerpt<'a> {
    part: &'a str,
}
```

**编译器输出**:
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     part: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Excerpt<'a> {
2 ~     part: &'a str,
```

### 错误 4: 过度标注生命周期

```rust
// ❌ 不必要：编译器可以推断
fn print(s: &str) {
    println!("{}", s);
}

// ✅ 更好：让编译器处理
fn print(s: &str) {
    println!("{}", s);
}
```

**规则**：只在编译器要求时标注生命周期。

---

## 动手练习

### 练习 1: 添加生命周期标注

为以下函数添加生命周期标注使其编译通过：

```rust
// TODO: 添加生命周期参数
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let result = longest(&s1, &s2);
    println!("最长的字符串是：{}", result);
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**解析**: 返回值依赖于两个参数，需要明确标注生命周期 `'a`。
</details>

### 练习 2: 结构体生命周期

定义一个持有字符串切片引用的结构体：

```rust
// TODO: 定义 Excerpt 结构体，包含 part: &str 字段

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("摘录：{}", excerpt.part);
}
```

<details>
<summary>点击查看答案</summary>

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

**解析**: 结构体有引用字段，需要生命周期参数 `'a`。
</details>

### 练习 3: 方法中的生命周期

为以下方法添加生命周期：

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // TODO: 添加生命周期标注
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
impl<'a> Excerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

**解析**: 根据省略规则 3，`&self` 的生命周期自动赋给返回值，无需额外标注。
</details>

### 练习 4: 理解 'static

解释以下代码为什么可以工作：

```rust
fn get_greeting() -> &'static str {
    "Hello, world!"
}

fn main() {
    let greeting = get_greeting();
    println!("{}", greeting);
}
```

<details>
<summary>点击查看答案</summary>

**答案**: 字符串字面量 `"Hello, world!"` 的类型是 `&'static str`，它存储在程序的二进制文件中，整个程序运行期间都有效。

**解析**: `'static` 表示引用在整个程序生命周期内有效，字符串字面量是典型例子。
</details>

---

## 故障排查

### Q: 什么时候需要标注生命周期？

**A**: 当编译器无法自动推断时。常见场景：
- 函数有多个引用参数且返回引用
- 结构体包含引用字段
- impl 块中的方法返回引用且依赖多个参数

### Q: 生命周期参数名必须是 `'a` 吗？

**A**: 不必须。可以使用任何有效标识符：
```rust
fn longest<'lifetime>(x: &'lifetime str, y: &'lifetime str) -> &'lifetime str
```
但约定使用短名称：`'a`, `'b`, `'r` 等。

### Q: 一个函数可以有多个生命周期参数吗？

**A**: 可以：
```rust
fn select_first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // 返回值生命周期与 x 相同
}
```

### Q: 生命周期和泛型有什么区别？

**A**: 
- 泛型：参数的**类型**
- 生命周期：引用的**作用域**

```rust
fn example<T, 'a>(value: T, reference: &'a T) {
    // T 是类型参数
    // 'a 是生命周期参数
}
```

---

## 知识扩展 (选学)

### 生命周期子类型

生命周期可以有约束关系：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,  // 'b 至少和 'a 一样长
{
    x
}
```

**含义**：`'b: 'a` 表示 `'b` 的生命周期包含 `'a`。

### 静态生命周期限制

`'static` 不意味着"永远"，而是"程序运行期间"：

```rust
// 可以存储在静态内存
const GREETING: &'static str = "Hello!";

// 但不能持有对动态数据的 'static 引用
fn create_string() -> &'static String {
    let s = String::from("hello");
    &s  // ❌ 错误：s 在函数结束时被丢弃
}
```

### 生命周期与闭包

闭包中的生命周期通常自动推断：

```rust
let x = 5;
let closure = |y| y + x;  // x 的生命周期被捕获
```

---

## 小结

**核心要点**：

1. **生命周期**: 引用的有效作用域，防止悬垂引用
2. **标注语法**: `&'a Type` 表示引用活至少 `'a` 这么久
3. **省略规则**: 编译器自动推断常见模式
4. **结构体**: 有引用字段必须标注生命周期
5. **'static**: 整个程序运行期间有效

**关键术语**：

- **Lifetime**: 生命周期，引用的作用域
- **Dangling Reference**: 悬垂引用，指向已释放数据的引用
- **Lifetime Annotation**: 生命周期标注，`'a` 语法
- **Lifetime Elision**: 生命周期省略，编译器自动推断
- **'static**: 特殊生命周期，程序运行期间有效

**下一步**：

- 学习 [闭包](closure.md)
- 理解 [特征对象](trait-objects.md)
- 探索 高级生命周期

---

## 术语表

| English    | 中文       |
| ---------- | ---------- |
| Lifetime   | 生命周期   |
| Reference  | 引用       |
| Borrow     | 借用       |
| Scope      | 作用域     |
| Dangling   | 悬垂的     |
| Static     | 静态的     |
| Elision    | 省略       |

---

完整示例：`src/basic/ownership_sample.rs` - 生命周期和借用  
相关示例：`src/basic/generic_sample.rs` - 泛型中的生命周期标注

---

## 知识检查

**快速测验**（答案在下方）：

1. 这段代码能编译通过吗？
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

2. `'static` 生命周期意味着什么？

3. 什么时候需要标注生命周期？

<details>
<summary>点击查看答案与解析</summary>

1. ❌ 不能 - 需要生命周期标注 `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
2. 数据在整个程序运行期间都有效（如字符串字面量）
3. 当编译器无法自动推断时（多个引用参数且返回引用）

**关键理解**: 生命周期标注告诉编译器引用的有效范围。
</details>

## 继续学习

- 下一步：[闭包](closure.md) - 捕获环境变量的匿名函数
- 进阶：[泛型](generic.md) - 生命周期与泛型结合
- 回顾：[所有权](ownership.md) - 生命周期基础

> 💡 **记住**：生命周期是 Rust 的安全保障。标注生命周期不是负担，而是编译器帮助你避免错误的工具！
