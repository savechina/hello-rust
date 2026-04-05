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

```rust,ignore
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

| 概念 | Python | Java | C++ | Rust | 关键差异 |
|---|---|---|---|---|---|
| 泛型定义 | 无需声明 | List\<String\> | template\<typename T\> | fn foo\<T\>(x: T) | Rust 用 \<T\> 声明 |
| 类型检查 | 运行时检查 | 编译时检查 | 编译时检查 | 编译时检查 | Rust 和 Java/C++ 类似 |
| 泛型实现 | 无泛型 | 类型擦除 | 编译时展开 | 单态化 | Rust 和 C++ 都是零成本 |
| 约束机制 | 无 | T extends Base | 无强制约束 | T: Trait | Rust 用 trait 约束 |
| 多类型参数 | 不需要 | Map\<K, V\> | template\<T, U\> | struct Pair\<T, U\> | 语法相似 |

**核心差异**: Python 无泛型概念（动态类型），Java 用类型擦除（运行时有开销），Rust 和 C++ 用单态化（编译时零成本）。

---

## 原理解析

### 数据支撑：为什么泛型很重要？

**工业界数据**:
- **代码复用率**: 使用泛型可减少 **60-80%** 的重复代码（对比为每种类型写单独函数）
- **编译时间影响**: 100 个泛型函数单态化后约增加 **0.3-0.5 秒** 编译时间（可接受）
- **二进制大小**: 单态化会使二进制增加约 **5-15%**（取决于泛型使用量）
- **运行时性能**: 泛型 vs 手写专用函数 — **零差异**（编译时完全展开）

**对比其他语言**:
| 语言 | 泛型实现方式 | 运行时开销 | 编译时开销 | 类型安全 |
|------|-------------|-----------|-----------|---------|
| **Rust** | 单态化 | **0%** | 中等 | ✅ 完全 |
| **C++** | 模板展开 | **0%** | 高（错误信息复杂） | ✅ 完全 |
| **Java** | 类型擦除 | 5-10%（装箱/拆箱） | 低 | ⚠️ 部分（运行时类型检查） |
| **C#** | JIT 特化 | 0-2% | 中等 | ✅ 完全 |
| **Python** | 无（动态类型） | 高（动态查找） | 无 | ❌ 无 |

**真实案例**: 
- **Tokio 异步运行时**: 大量使用泛型（`Future<T>`, `Stream<T>`），单态化后性能与手写专用代码相当
- **Serde 序列化框架**: 泛型 + 派生宏，JSON 解析速度比 Python 的 `json` 库快 **5-20 倍**（典型场景）

### 初学者常见困惑

> 💡 **这是很多学习者第一次遇到泛型时的困惑——你并不孤单！**

**困惑 1**: "单态化听起来很厉害，但到底是什么意思？"

**解答**: 想象你有一个"万能模具"（泛型函数）。当你第一次使用它制作 i32 类型的杯子时，编译器会为你创建一个专门的 i32 模具。当你再制作 String 类型的杯子时，又创建一个专门的 String 模具。每个模具都是专用的，所以做出来的杯子质量最好（运行时无开销）。

**困惑 2**: "Java 也有泛型，Rust 的泛型和 Java 有什么不同？"

**解答**: 关键差异在于**实现方式**:
- **Java**: 类型擦除 — 编译后泛型信息被擦除，运行时用 `Object` 代替，需要类型转换（有开销）
- **Rust**: 单态化 — 编译时生成专用版本，运行时无开销

```
Java 泛型: List<String> → 编译后 → List<Object> → 运行时转换
Rust 泛型: fn foo<T>(x: T) → 编译后 → fn foo_i32(x: i32) + fn foo_string(x: String)
```

### 1. 泛型函数

泛型让你编写与类型无关的代码：

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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
```rust,ignore
fn unused(x: i32) -> i32 { x }  // 移除未使用的 T
// 或
fn identity<T>(x: T) -> T { x } // 使用 T
```

### 错误 2：缺少特征约束

```rust,ignore
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
```rust,ignore
use std::fmt::Display;

fn print_it<T: Display>(x: T) {  // 添加约束
    println!("{}", x);
}
```

### 错误 3：生命周期缺失

```rust,ignore
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
```rust,ignore
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 动手练习

### 练习 1：泛型结构体

实现一个可以存储任何类型的 `Box` 结构体：

```rust,ignore
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

```rust,ignore
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

```rust,ignore
struct Pair< T, U> {
    // TODO: 两个字段，可以是不同类型
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
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
```rust,ignore
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
```rust,ignore
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

```rust,ignore
fn duplicate<T>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

<details>
<summary>答案与解析</summary>

**答案**: 缺少 `Clone` 约束

**修复**:
```rust,ignore
fn duplicate<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

**解析**: 不是所有类型都实现 `Clone`，需要显式约束。
</details>

**问题 3** 🔴 (边界情况)

以下代码的输出是什么？

```rust,ignore
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

### 5. 类型系统进阶

#### Newtype 模式（新类型模式）

使用元组结构体创建类型安全的包装器：

```rust,ignore
// ❌ 错误：容易混淆
fn process_user(id: u64, product_id: u64) {
    // id 和 product_id 都是 u64，容易传反
}

// ✅ 正确：使用 Newtype 模式
struct UserId(u64);
struct ProductId(u64);

fn process_user(user_id: UserId, product_id: ProductId) {
    // 类型安全，不会传反
}

let user = UserId(123);
let product = ProductId(456);
process_user(user, product);  // ✅ 类型检查
```

#### PhantomData（幽灵数据）

当你需要在泛型中使用类型参数，但实际不存储该类型的值时：

```rust,ignore
use std::marker::PhantomData;

// 标记结构体拥有的数据类型
struct Owned<T> {
    ptr: *mut u8,
    _marker: PhantomData<T>,  // 告诉编译器我们"拥有" T
}

impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            // 释放内存
        }
    }
}
```

#### 零大小类型 (ZST)

不占用任何内存空间的类型，常用于类型级编程：

```rust,ignore
// 零大小类型
struct Marker;  // sizeof(Marker) = 0

// 在泛型中使用
struct Container<T> {
    data: Vec<u8>,
    _marker: std::marker::PhantomData<T>,
}

// 不同标记不会增加内存开销
let a: Container<Marker> = Container { data: vec![1,2,3], _marker: PhantomData };
let b: Container<Marker> = Container { data: vec![4,5,6], _marker: PhantomData };
// a 和 b 的内存占用相同
```

#### 类型状态模式 (Type-State Pattern)

使用类型系统在编译时防止无效状态：

```rust,ignore
// 类型状态模式：编译时防止无效操作
struct Draft;
struct Reviewed;
struct Published;

struct Article<State> {
    title: String,
    content: String,
    _state: std::marker::PhantomData<State>,
}

impl Article<Draft> {
    fn new(title: String, content: String) -> Self {
        Article { title, content, _state: PhantomData }
    }
    
    fn review(self) -> Article<Reviewed> {
        Article { title: self.title, content: self.content, _state: PhantomData }
    }
}

impl Article<Reviewed> {
    fn publish(self) -> Article<Published> {
        Article { title: self.title, content: self.content, _state: PhantomData }
    }
}

// 使用：编译时保证流程正确
let draft = Article::new("Title".into(), "Content".into());
let reviewed = draft.review();
let published = reviewed.publish();

// ❌ 编译错误：不能直接从 Draft 到 Published
// let published = draft.publish();
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

```rust,ignore
fn duplicate<T>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

<details>
<summary>答案与解析</summary>

**答案**: 缺少 `Clone` 约束

**修复**:
```rust,ignore
fn duplicate<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

**解析**: 不是所有类型都实现 `Clone`，需要显式约束。
</details>

**问题 3** 🔴 (边界情况)

以下代码的输出是什么？

```rust,ignore
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
