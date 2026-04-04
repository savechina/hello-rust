# 泛型 (Generics)

## 开篇故事

想象你有一个模具，可以用它制作不同材质的杯子——玻璃杯、陶瓷杯、塑料杯。模具本身不关心材质，它定义了杯子的**形状**，材质由使用者决定。Rust 的**泛型**就是这样——它让你编写与具体类型无关的代码，让编译器在实际使用时生成特定类型的版本。

---

## 本章适合谁

如果你已经理解了结构体和特征，现在想编写可复用的通用代码，本章适合你。泛型是高级 Rust 程序员的必备技能。

---

## 你会学到什么

完成本章后，你可以：

1. 定义泛型函数和结构体
2. 理解单态化 (monomorphization) 过程
3. 使用特征约束限制泛型类型
4. 使用 `where` 子句简化复杂约束
5. 区分泛型与特征对象的使用场景

---

## 前置要求

学习本章前，你需要理解：

- [结构体](struct.md) - 自定义类型的定义
- [特征](trait.md) - 行为的抽象
- [函数](functions.md) - Rust 函数语法

---

## 第一个例子

```rust
// 泛型函数 - 可以处理任何类型
fn identity<T>(x: T) -> T {
    x  // 返回输入的值
}

fn main() {
    let num = identity(5);           // T 推断为 i32
    let text = identity(String::from("hello")); // T 推断为 String
    let flag = identity(true);       // T 推断为 bool
    
    println!("{} {} {}", num, text, flag);
}
```

**关键点**：
- `<T>` 是**类型参数** - 调用时确定实际类型
- 一个函数，多种用法 - 代码复用
- 编译器生成 specialized 版本 - 零成本抽象

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念       | Python               | Java                 | C++                    | Rust                       | 关键差异                  |
| ---------- | -------------------- | -------------------- | ---------------------- | -------------------------- | ------------------------- |
| 泛型定义   | 无需声明（动态类型） | `List<String>`       | `template<typename T>` | `fn foo<T>(x: T)`          | Rust 用 `<T>` 声明参数    |
| 类型检查   | 运行时检查           | 编译时检查           | 编译时检查             | 编译时检查                 | Rust 和 Java/C++ 类似     |
| 泛型实现   | 无泛型（鸭子类型）   | 类型擦除             | 编译时展开             | 单态化（编译特化）         | Rust 和 C++ 都是零成本    |
| 约束机制   | 无                   | `<T extends Base>`   | 无强制约束             | `T: Trait`                 | Rust 用 trait 约束        |
| 多类型参数 | 不需要               | `Map<K, V>`          | `template<T, U>`       | `struct Pair<T, U>`        | 语法相似                  |

**核心差异**: Python 无泛型概念（动态类型），Java 用类型擦除（运行时有开销），Rust 和 C++ 用单态化（编译时零成本）。

---

## 原理解析

### 1. 泛型函数

泛型让你编写与类型无关的代码：

```rust
// 没有泛型：需要为每种类型写一个函数
fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn max_string(a: String, b: String) -> String {
    // 重复代码...
}

// 使用泛型：一个函数处理所有类型
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### 2. 单态化 (Monomorphization)

**重要概念**：
```
编译时：
泛型代码<T>  +  具体类型 (i32, String)
     │
     ├── 特化为 max_i32(a: i32, b: i32) → i32
     └── 特化为 max_String(a: String, b: String) → String
     ↓
运行时：
max_i32(a, b)     ← 专用版本（无泛型开销）
max_String(a, b)  ← 专用版本（无泛型开销）
```

**什么是单态化**：
Rust 在编译时将泛型代码**特化**为具体类型的版本。这意味着：
- ✅ 运行时无性能损失
- ✅ 类型安全在编译时检查
- ❌ 代码量可能增加（每个类型一个版本）

### 3. 泛型结构体

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };           // Point<i32>
    let p2 = Point { x: 1.0, y: 2.0 };       // Point<f64>
    
    // 错误：x 和 y 必须是同一类型
    // let p3 = Point { x: 5, y: 1.0 };  // ❌
}
```

---

## 常见错误

### 错误 1：类型参数未使用

```rust
fn unused<T>(x: i32) -> i32 {
    x  // T 未使用
}
```

**编译器输出**：
```
warning: type parameter `T` goes unused
 --> src/main.rs:1:12
  |
1 | fn unused<T>(x: i32) -> i32 {
  |            ^ help: remove the unused type parameter
```

**修复方法**：
```rust
fn unused(x: i32) -> i32 { x }  // 移除未使用的 T
// 或
fn identity<T>(x: T) -> T { x } // 使用 T
```

### 错误 2：缺少特征约束

```rust
fn print_it<T>(x: T) {
    println!("{}", x);  // ❌ T 可能不能打印
}
```

**编译器输出**：
```
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:20
  |
3 |     println!("{}", x);
  |                    ^ `T` cannot be formatted with the default formatter
```

**修复方法**：
```rust
use std::fmt::Display;

fn print_it<T: Display>(x: T) {  // 添加约束
    println!("{}", x);
}
```

### 错误 3：生命周期缺失

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**编译器输出**：
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:30
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |            ----     ----        ^ expected named lifetime parameter
```

**修复方法**：
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 动手练习

### 练习 1：泛型结构体

实现一个可以存储任何类型的 `Box` 结构体：

```rust
struct Box<T> {
    // TODO: 定义字段
}

impl<T> Box<T> {
    // TODO: 实现 new 和 get 方法
}

fn main() {
    let num_box = Box::new(42);
    let text_box = Box::new(String::from("hello"));
}
```

<details>
<summary>点击查看答案</summary>

```rust
struct Box<T> {
    inner: T,
}

impl<T> Box<T> {
    fn new(value: T) -> Self {
        Box { inner: value }
    }
    
    fn get(&self) -> &T {
        &self.inner
    }
}
```

**解析**：泛型结构体允许存储任何类型，但一旦创建，类型就固定了。
</details>

### 练习 2：多类型参数

实现一个可以容纳两种类型的 `Pair` 结构体：

```rust
struct Pair< T, U> {
    // TODO: 两个字段，可以是不同类型
}
```

<details>
<summary>点击查看答案</summary>

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}
```

**解析**：泛型可以有多个类型参数，适合元组、键值对等场景。
</details>

---

## 故障排查 (FAQ)

### Q: 泛型和动态特征对象有什么区别？

**A**: 
```rust
// 泛型：编译时确定类型，零开销
fn gen<T: Trait>(x: T) { }  // 单态化

// 特征对象：运行时确定类型，有开销
fn dyn_obj(x: &dyn Trait) { }  // 虚表查找
```

**选择**：
- 需要性能 → 用泛型
- 需要在集合中存储不同类型 → 用特征对象

### Q: 何时使用 `where` 子句？

**A**: 当约束复杂时：
```rust
// 约束太多时
fn complex<T: Clone + Display + Debug + Default>() { }

// 用 where 更清晰
fn complex<T>()
where
    T: Clone + Display + Debug + Default,
{ }
```

---

## 延伸阅读

学习完本章，你可能还想了解：

- [`const_generics`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) - 编译时常量作为泛型参数
- [`associated_types`](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html) - 特征中的类型占位符
- [`phantom_data`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) - 编译时类型标记

**选择建议**：
- 需要类型级别优化 → const generics
- 设计复杂特征 → associated types
- 高级技巧 → PhantomData

---

## 知识检查

**问题 1** 🟢 (基础概念)

以下哪个说法正确？

A) 泛型在运行时确定类型  
B) 泛型会导致运行时性能损失  
C) 泛型在编译时单态化  
D) 泛型不能有多个类型参数

<details>
<summary>答案与解析</summary>

**答案**: C) 泛型在编译时单态化

**解析**: Rust 的泛型在编译时将类型参数替换为具体类型，生成专门的代码版本。
</details>

**问题 2** 🟡 (特征约束)

这段代码有什么问题？

```rust
fn duplicate<T>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

<details>
<summary>答案与解析</summary>

**答案**: 缺少 `Clone` 约束

**修复**:
```rust
fn duplicate<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

**解析**: 不是所有类型都实现 `Clone`，需要显式约束。
</details>

**问题 3** 🔴 (边界情况)

以下代码的输出是什么？

```rust
fn process<T: Default>() -> T {
    T::default()
}

fn main() {
    let x: i32 = process();
    println!("{}", x);
}
```

<details>
<summary>答案与解析</summary>

**答案**: `0`

**解析**: `i32::default()` 返回 `0`。这是一个空值初始化模式。
</details>

---

## 小结

**核心要点**：

1. **泛型允许类型参数化** - 编写与具体类型无关的代码
2. **单态化产生专用版本** - 编译时替换，运行时零开销
3. **特征约束限制可用类型** - `T: Trait` 语法
4. **where 子句简化复杂约束** - 多行约束更清晰
5. **生命周期与泛型配合** - 确保引用有效性

**术语**：

- **Generic (泛型)**: 类型或函数的模板
- **Type Parameter (类型参数)**: 占位符类型 T
- **Monomorphization (单态化)**: 编译时特化过程
- **Trait Bound (特征约束)**: 限制泛型类型的约束

**下一步**：

- 继续：[闭包](closure.md) - 可执行的泛型代码
- 相关：[特征对象](trait-objects.md) - 动态类型的替代方案

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Generic | 泛型 |
| Type Parameter | 类型参数 |
| Monomorphization | 单态化 |
| Trait Bound | 特征约束 |
| Where Clause | where 子句 |

---

完整示例：`src/basic/generic_sample.rs`

---

> 💡 **提示**：泛型是 Rust 零成本抽象的核心 - 编译时的工作绝不留给运行时！

---

## 单态化可视化

### 1. 编译时单态化

```
编译前 (泛型代码):
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

编译时 (单态化):
         ↓ 为 i32 生成专用版本
         ↓ 为 String 生成专用版本
         ↓

编译后 (专用代码):
fn max_i32(a: i32, b: i32) -> i32 { ... }    // 类型 T = i32
fn max_string(a: String, b: String) -> String { ... }  // 类型 T = String
```

**关键点**:
- 编译时生成专用版本
- 运行时无性能损失
- 代码量可能增加

### 2. 特征约束图

```
T: Display + Clone

T 必须实现:
+-----------------+
|     Display     |
+-----------------+
|     Clone       |
+-----------------+
      ↑
    必须同时实现
```

### 3. where 子句可视化

```
fn process<T, U>()
where
    T: Display + Clone,
    U: Into<T>

约束关系:
U ───Into──→ T ───Display/Clone──→ 
```

### 4. 泛型实例化

```
泛型函数：
fn identity<T>(x: T) -> T { x }

调用时:
identity(5)      → T = i32
identity("hi")   → T = &str
identity(true)   → T = bool

编译器生成:
fn identity_i32(x: i32) -> i32 { x }
fn identity_str(x: &str) -> &str { x }
fn identity_bool(x: bool) -> bool { x }
```

---

## 知识检查

**问题 1** 🟢 (基础概念)

以下哪个说法正确？

A) 泛型在运行时确定类型  
B) 泛型会导致运行时性能损失  
C) 泛型在编译时单态化  
D) 泛型不能有多个类型参数

<details>
<summary>答案与解析</summary>

**答案**: C) 泛型在编译时单态化

**解析**: Rust 的泛型在编译时将类型参数替换为具体类型，生成 specialised 代码版本。
</details>

**问题 2** 🟡 (特征约束)

这段代码有什么问题？

```rust
fn duplicate<T>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

<details>
<summary>答案与解析</summary>

**答案**: 缺少 `Clone` 约束

**修复**:
```rust
fn duplicate<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

**解析**: 不是所有类型都实现 `Clone`，需要显式约束。
</details>

**问题 3** 🔴 (边界情况)

以下代码的输出是什么？

```rust
fn process<T: Default>() -> T {
    T::default()
}

fn main() {
    let x: i32 = process();
    println!("{}", x);
}
```

<details>
<summary>答案与解析</summary>

**答案**: `0`

**解析**: `i32::default()` 返回 `0`。这是一个空值初始化模式。
</details>


---

## 继续学习

**前一章**: [特征](trait.md)  
**下一章**: [闭包](closure.md)

**相关章节**:
- [特征](trait.md) - 特征约束
- [模块系统](module.md) - 组织代码

**返回**: [基础入门](basic-overview.md)
