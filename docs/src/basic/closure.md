# 闭包 (Closures)

## 开篇故事

想象你有个魔法便签，可以记住周围的环境。比如你在厨房时，便签记住冰箱里有牛奶；你走到客厅，便签依然记得牛奶的事。Rust 的**闭包**就是这样——它能"捕获"周围环境的变量，带着这些记忆到任何地方执行。

---

## 本章适合谁

如果你已经学完函数基础，想了解更灵活的代码块传递方式，本章适合你。闭包是 Rust 函数式编程的核心。

---

## 你会学到什么

完成本章后，你可以：

1. 定义和使用闭包
2. 理解闭包捕获环境的方式（by ref, by mut ref, by value）
3. 区分 Fn、FnMut、FnOnce trait
4. 在迭代器中使用闭包
5. 使用 `move` 关键字转移所有权

---

## 前置要求

学习本章前，你需要理解：

- [所有权](ownership.md) - 借用和移动
- [特征](trait.md) - Fn trait 的基础
- [函数](functions.md) - 基本函数语法

---

## 第一个例子

```rust
fn main() {
    // 最简单的闭包
    let add_one = |x| x + 1;
    
    let result = add_one(5);
    println!("6 = {}", result);  // 输出：6 = 6
    
    // 带类型注解的闭包
    let add_two = |x: i32| -> i32 { x + 2 };
    println!("7 = {}", add_two(5));  // 输出：7 = 7
}
```

**关键点**：
- `|x|` 是参数列表
- 自动推断类型 - 更简洁
- 可以存储在变量中 - 一等公民

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念       | Python               | Java                     | C++                    | Rust                       | 关键差异                  |
| ---------- | -------------------- | ------------------------ | ---------------------- | -------------------------- | ------------------------- |
| 闭包语法   | `lambda x: x + 1`    | `x -> x + 1`             | `[](int x) { return x+1; }` | `|x| x + 1`            | Rust 用竖线包围参数       |
| 捕获环境   | 自动捕获（引用）     | 需声明 `final` 变量      | 显式 `[&]` 或 `[=]`    | 自动推断借用/移动          | Rust 编译器自动选择       |
| 类型标注   | 不需要               | 需明确类型               | 可选                   | 可选或推断                 | Rust 第一次调用后固定     |
| 修改环境   | `nonlocal x`         | 不支持                   | `[&]` 可修改           | 需要 `mut` 和 `FnMut`      | Rust 用 trait 区分        |
| 存储闭包   | 直接赋值             | 需要接口类型             | `std::function`        | `Box<dyn Fn>` 或泛型       | Rust 类型匿名             |

**核心差异**: Python 最灵活，Java 最受限，C++ 需显式指定捕获方式，Rust 编译器自动推断但用 trait 严格区分。

---

## 原理解析

### 1. 闭包语法

```rust
// 各种闭包形式
let add = |x, y| x + y;
let print = || println!("hello");
let square = |x: i32| -> i32 { x * x };
```

**与函数的区别**：
```rust
// 函数：必须指定类型
fn add_fn(x: i32, y: i32) -> i32 {
    x + y
}

// 闭包：类型可推断
let add_closure = |x, y| x + y;
```

### 2. 捕获环境

**三种捕获方式**：

```rust
fn main() {
    let x = 42;
    
    // 1. 不可变借用 (&x)
    let print_x = || println!("{}", x);
    
    // 2. 可变借用 (&mut x)
    let mut counter = 0;
    let mut increment = || counter += 1;
    
    // 3. 转移所有权 (move)
    let data = String::from("hello");
    let use_data = move || println!("{}", data);
}
```

### 3. Fn Trait 层次

闭包自动实现三个 trait,取决于如何捕获环境:

```rust
// 示例来自 src/basic/closure_sample.rs

// Fn: 只读捕获,可多次调用
let captured_value = 10;
let add_captured = |x: i32| x + captured_value;  // 实现 Fn
let result1 = add_captured(5);   // 输出: 15
let result2 = add_captured(20);  // 输出: 30 (多次调用)

// FnMut: 可变捕获,需 mut 声明
let mut mutable_value = 0;
let mut increment = |x: i32| {
    mutable_value += x;
    mutable_value
};
let incr1 = increment(5);   // 输出: 5
let incr2 = increment(10);  // 输出: 15

// FnOnce: 消耗所有权,仅一次调用
let owned_value = String::from("Owned");
let consume_string = move || owned_value;  // 实现 FnOnce
let consumed = consume_string;  // 第一次调用
// consume_string;  // ❌ 错误: 已被消耗
```

**Trait 定义**:

```rust
// 标准库 trait 定义 (简化版)
trait Fn {
    fn call(&self, args: Args) -> Output;  // 不可变借用 self
}

trait FnMut {
    fn call_mut(&mut self, args: Args) -> Output;  // 可变借用 self
}

trait FnOnce {
    fn call_once(self, args: Args) -> Output;  // 消耗 self
}
```

**关系图**:
```
Fn: &self  ──可以──> FnMut: &mut self ──可以──> FnOnce: self
     ^                                              ^
     |                                              |
     +────────────只能降级调用────────────────────────+

解释:
- 实现 Fn 的闭包 → 自动实现 FnMut 和 FnOnce
- 实现 FnMut 的闭包 → 自动实现 FnOnce
- 实现 FnOnce 的闭包 → 不一定实现 Fn 或 FnMut
```

### 4. 闭包作为函数参数

闭包可以作为参数传递,让函数接受"行为"而非仅数据:

```rust
// 示例来自 src/basic/closure_sample.rs

// 泛型函数接受闭包
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,  // Fn trait 约束
{
    f(value)
}

let double = |x| x * 2;
let result = apply(double, 10);
println!("Doubled Result: {}", result);  // 输出: 20

// 闭包返回不同类型
fn process_and_print<F>(f: F, value: i32)
where
    F: Fn(i32) -> String,
{
    let result_string = f(value);
    println!("{}", result_string);
}

let stringify = |num: i32| format!("Number: {}", num);
process_and_print(stringify, 42);  // 输出: Number: 42
```

**关键点**:
- `F: Fn(i32) -> i32` 是 trait 约束
- 闭包可以作为参数传递给泛型函数
- 函数可以接受不同行为的闭包

---

## 常见错误

### 错误 1：类型推断不一致

```rust
let closure = |x| x + 1;
let result1 = closure(5);      // ✅ 推断为 i32
let result2 = closure(5.0);    // ❌ 错误: 期望 i32 但找到 f64
```

**编译器输出**:
```
error[E0308]: mismatched types
  --> src/main.rs:3:24
   |
3 |     let result2 = closure(5.0);
   |                        ^^^ expected i32, found floating-point number
```

**修复方法**:
```rust
// 显式标注类型
let closure = |x: i32| x + 1;
```

**解析**: 闭包在第一次调用时推断类型,之后类型固定。

### 错误 2：可变借用冲突

```rust
let mut counter = 0;
let mut increment = || counter += 1;

println!("{}", counter);  // ❌ 错误: 与闭包的可变借用冲突
increment();
```

**编译器输出**:
```
error[E0502]: cannot borrow `counter` as immutable because it is also borrowed as mutable
  --> src/main.rs:5:20
   |
3 |     let mut increment = || {
4 |         counter += 1;
   |         ------- mutable borrow occurs here
5 |     println!("{}", counter);
   |                    ^^^^^^^ immutable borrow occurs here
```

**修复方法**:
```rust
let mut counter = 0;
let mut increment = || counter += 1;

increment();  // 先调用闭包,释放借用
println!("{}", counter);  // ✅ 借用已释放
```

**解析**: 闭包持有可变借用期间,不能有其他借用。

### 错误 3：move 后使用

```rust
let x = String::from("hello");
let closure = move || println!("{}", x);  // move 转移所有权
println!("{}", x);  // ❌ 错误: x 已被移动
```

**修复方法**:
```rust
let x = String::from("hello");
let closure = || println!("{}", &x);  // 借用,不转移
println!("{}", x);  // ✅ x 仍可用
```

**解析**: `move` 关键字强制转移所有权,捕获变量不再可用。

### 错误 4：FnOnce 被重复调用

```rust
let text = String::from("Hello");
let consume = move || text;  // 实现 FnOnce

consume();  // ✅ 第一次调用
consume();  // ❌ 错误: 闭包已被消耗
```

**编译器输出**:
```
error[E0382]: use of moved value: `consume`
  --> src/main.rs:6:5
   |
5 |     consume();
   |     ------- value moved here
6 |     consume();
   |     ^^^^^^^ value used here after move
```

**修复方法**:
```rust
// 方法 1: 不使用 move
let text = String::from("Hello");
let print = || println!("{}", &text);
print();
print();  // ✅ 可多次调用

// 方法 2: 克隆返回值
let text = String::from("Hello");
let print_clone = || {
    println!("{}", text);
    text.clone()
};
```

**解析**: `FnOnce` 闭包消耗捕获变量,只能调用一次。

---

## 动手练习

### 练习 1：闭包捕获环境

实现一个计算器闭包,捕获基础值并累加:

```rust
fn main() {
    let base = 10;
    
    // TODO: 定义 add_to_base 闭包,捕获 base
    // let add_to_base = ???;
    
    println!("{}", add_to_base(5));   // 应输出: 15
    println!("{}", add_to_base(20));  // 应输出: 30
}
```

<details>
<summary>点击查看答案</summary>

```rust
let base = 10;
let add_to_base = |x: i32| x + base;

println!("{}", add_to_base(5));   // 输出: 15
println!("{}", add_to_base(20));  // 输出: 30
```

**解析**: 闭包捕获 `base` 为不可变引用,实现 `Fn` trait,可多次调用。
</details>

### 练习 2：可变捕获闭包

实现一个累加器闭包,每次调用递增:

```rust
fn main() {
    let mut total = 0;
    
    // TODO: 定义 accumulate 闭包
    // let accumulate = ???;
    
    println!("{}", accumulate(5));   // 应输出: 5
    println!("{}", accumulate(10));  // 应输出: 15
    println!("{}", accumulate(3));   // 应输出: 18
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut total = 0;
let mut accumulate = |x: i32| {
    total += x;
    total
};

println!("{}", accumulate(5));   // 输出: 5
println!("{}", accumulate(10));  // 输出: 15
println!("{}", accumulate(3));   // 输出: 18
```

**解析**: 闭包可变借用 `total`,实现 `FnMut` trait,需要 `mut` 声明。
</details>

### 练习 3：闭包作为函数参数

实现 `apply_operation` 函数,接受不同操作的闭包:

```rust
fn apply_operation<F>(f: F, value: i32) -> i32
// TODO: 添加 trait 约束
{
    // TODO: 调用闭包
}

fn main() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    
    println!("{}", apply_operation(double, 10));  // 应输出: 20
    println!("{}", apply_operation(triple, 10));  // 应输出: 30
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn apply_operation<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    
    println!("{}", apply_operation(double, 10));  // 输出: 20
    println!("{}", apply_operation(triple, 10));  // 输出: 30
}
```

**解析**: 泛型参数 `F` 约束为 `Fn(i32) -> i32`,可接受任何匹配的闭包。
</details>

### 练习 4：迭代器过滤

使用闭包过滤数字集合:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// TODO: 过滤出偶数
let evens: Vec<i32> = numbers
    .iter()
    .filter(|n| /* TODO */)
    .cloned()
    .collect();

println!("{:?}", evens);  // 应输出: [2, 4, 6]
```

<details>
<summary>点击查看答案</summary>

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let evens: Vec<i32> = numbers
    .iter()
    .filter(|n| n % 2 == 0)
    .cloned()
    .collect();

println!("{:?}", evens);  // 输出: [2, 4, 6]
```

**解析**: `filter` 接受返回 `bool` 的闭包,闭包判断每个元素是否保留。
</details>

---

## 故障排查 (FAQ)

### Q: Fn, FnMut, FnOnce 如何选择?

**A**: 作为参数时根据需求选择:

```rust
// Fn: 只读,可多次调用 (最灵活)
fn process_fn<F>(f: F) where F: Fn() {
    f();
    f();  // ✅ 可多次调用
}

// FnMut: 需修改环境
fn process_mut<F>(mut f: F) where F: FnMut() {
    f();  // ✅ 可修改环境
}

// FnOnce: 消耗所有权 (最严格)
fn process_once<F>(f: F) where F: FnOnce() {
    f();  // ✅ 仅一次调用
}
```

**选择原则**:
- 只读访问 → `Fn` (推荐,最灵活)
- 需要修改环境 → `FnMut`
- 需要消耗所有权 → `FnOnce` (最严格)

### Q: 闭包和函数有什么区别?

**A**: 

```rust
// 函数: 不能捕获环境
fn add_fn(x: i32, factor: i32) -> i32 {
    x + factor
}

// 闭包: 可以捕获环境
let factor = 2;
let add_closure = |x| x + factor;  // 捕获 factor

// 函数类型: fn (函数指针)
let fn_ptr: fn(i32) -> i32 = add_fn;

// 闭包类型: 匿名,实现 Fn trait
// let closure_ptr: ??? = add_closure;  // 类型匿名
```

**区别总结**:
- 函数不能捕获环境变量
- 闭包可以捕获 (灵活)
- 函数有明确类型 `fn(T) -> R`
- 闭包类型匿名,通过 trait 表示

### Q: move 何时必须使用?

**A**: 当闭包需要离开定义作用域时:

```rust
// 1. 返回闭包: 必须 move
fn create_closure() -> impl Fn() -> String {
    let text = String::from("Hello");
    move || text.clone()  // move 捕获 text
}

// 2. 线程: 必须 move
use std::thread;
thread::spawn(move || {
    // 独立线程需要所有权
});

// 3. 长时间存储: 建议 move
let closure = move || {
    // 避免生命周期问题
};
```

### Q: 如何调试闭包类型?

**A**: 

```rust
// 闭包类型是匿名的
let closure = |x| x + 1;
// 类型: impl Fn(i32) -> i32

// 需要存储时用 Box<dyn Fn>
let boxed: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);

// 需要具体类型时用函数指针
let fn_ptr: fn(i32) -> i32 = |x| x + 1;  // 不捕获环境的闭包
```

---

## 延伸阅读

学习完本章，你可能还想了解：

- [`Fn trait`](https://doc.rust-lang.org/std/ops/trait.Fn.html) - 标准库 Fn trait
- [`Box<dyn Fn>`](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) - 特征对象
- [`rayon`](https://github.com/rayon-rs/rayon) - 并行迭代器库

**选择建议**：
- 学习标准 API → Fn trait
- 需要动态类型 → Box<dyn Fn>  
- 并行处理 → rayon

---

## 知识检查

**问题 1** 🟢 (基础概念)

以下代码的输出是什么?

```rust
let add_one = |x: i32| x + 1;
let result = add_one(5);
println!("Result: {}", result);
```

A) `5`  
B) `6`  
C) `编译错误`  
D) `运行时错误`

<details>
<summary>答案与解析</summary>

**答案**: B) `6`

**解析**: 闭包 `add_one` 将输入加 1,调用 `add_one(5)` 返回 `5 + 1 = 6`。
</details>

**问题 2** 🟡 (捕获环境)

这段代码会输出什么?

```rust
let captured_value = 10;
let add_captured = |x: i32| x + captured_value;
let result = add_captured(5);
println!("Captured Result: {}", result);
```

A) `5`  
B) `10`  
C) `15`  
D) `编译错误`

<details>
<summary>答案与解析</summary>

**答案**: C) `15`

**解析**: 闭包捕获 `captured_value = 10`,调用 `add_captured(5)` 返回 `5 + 10 = 15`。
</details>

**问题 3** 🟡 (Fn trait)

以下哪个闭包实现 `FnOnce`?

```rust
// A
let x = 5;
let a = || println!("{}", x);

// B
let mut y = 0;
let mut b = || y += 1;

// C
let z = String::from("hi");
let c = move || z;  // 返回 z
```

<details>
<summary>答案与解析</summary>

**答案**: C

**解析**:
- A: 只读捕获 → 实现 `Fn`
- B: 可变捕获 → 实现 `FnMut`
- C: move 并返回所有权 → 实现 `FnOnce` (消耗 z)
</details>

**问题 4** 🔴 (高级场景)

如何修复这段代码使其可编译?

```rust
let mut counter = 0;
let mut increment = |x: i32| {
    counter += x;
    counter
};

fn apply_mut<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(value)
}

apply_mut(&mut increment, 10);
println!("After apply_mut: {}", counter);  // ❌ 错误
```

<details>
<summary>答案与解析</summary>

**答案**: 在调用 `apply_mut` 后,`increment` 的借用已释放

```rust
let mut counter = 0;
let mut increment = |x: i32| {
    counter += x;
    counter
};

apply_mut(&mut increment, 10);
// 借用已释放,可以访问 counter
println!("Counter: {}", counter);  // ✅ 输出: 10
```

**解析**: `apply_mut` 接受 `&mut increment`,调用后借用释放,`counter` 可访问。
</details>

---

## 小结

**核心要点**:

1. **闭包是匿名函数** - 简洁语法,类型推断,可存储在变量中
2. **捕获环境变量** - 自动选择借用或移动,实现"记忆"功能
3. **三种 trait 层次** - `Fn` (只读) < `FnMut` (修改) < `FnOnce` (消耗)
4. **作为参数传递** - 泛型 + trait 约束,传递"行为"
5. **move 关键字** - 强制所有权转移,用于线程、返回等场景
6. **类型推断机制** - 第一次调用时固定类型参数

**源码示例对照**:

| 源码位置 | 概念 | 示例 |
|---------|------|------|
| `closure_sample.rs:4` | 基本闭包 | `let add_one = |x: i32| x + 1;` |
| `closure_sample.rs:24` | 捕获环境 | `let add_captured = |x| x + captured_value;` |
| `closure_sample.rs:58` | FnMut | `let mut increment = |x| { mutable_value += x; }` |
| `closure_sample.rs:80` | FnOnce | `let consume_string = move || owned_value;` |
| `closure_sample.rs:11` | 函数参数 | `fn apply<F>(f: F, value: i32) where F: Fn(i32) -> i32` |

**术语**:

- **Closure (闭包)**: 可捕获环境的匿名函数
- **Capture (捕获)**: 闭包记住环境变量的方式 (by ref, by mut ref, by value)
- **Environment (环境)**: 闭包定义时的作用域
- **Trait Bound (特征约束)**: 限制闭包能力的 trait (Fn/FnMut/FnOnce)
- **Move Keyword (move 关键字)**: 强制所有权转移

**下一步**:

- 继续: 迭代器 - 闭包的最佳应用场景
- 相关: [线程](threads.md) - `move` 闭包的典型用法
- 相关: [所有权](ownership.md) - 理解捕获机制的基础

---

## 术语表

| English | 中文 | 说明 |
| ------- | ---- | ---- |
| Closure | 闭包 | 可捕获环境的匿名函数 |
| Capture Environment | 捕获环境 | 闭包访问外部变量的机制 |
| Fn Trait | Fn 特征 | 只读捕获,可多次调用 |
| FnMut Trait | FnMut 特征 | 可变捕获,可修改环境 |
| FnOnce Trait | FnOnce 特征 | 消耗所有权,仅一次调用 |
| Move Keyword | move 关键字 | 强制所有权转移 |
| Anonymous Function | 匿名函数 | 无名称的函数定义 |
| Type Inference | 类型推断 | 编译器自动确定类型 |
| Environment | 环境 | 闭包定义时的作用域 |

---

## 项目实例

完整示例位于: `src/basic/closure_sample.rs`

**代码示例覆盖**:

1. 基本闭包定义 (第 4-7 行)
2. 闭包作为函数参数 (第 11-20 行)
3. 捕获环境变量 (第 23-37 行)
4. 返回不同类型 (第 40-51 行)
5. FnMut 使用 (第 54-76 行)
6. FnOnce 和 move (第 79-90 行)

**运行示例**:

```bash
# 在项目根目录执行
cargo run

# 输出包含所有闭包示例结果
```

---

> 💡 **提示**: 闭包是 Rust 函数式编程的核心 - 把"行为"当作"数据"传递,让代码更具表达力!

---

## 继续学习

**前一章**: [泛型](generic.md)  
**下一章**: [线程与并发](threads.md)

**相关章节**:
- [特征](trait.md) - Fn trait
- [泛型](generic.md) - 泛型函数

**返回**: [基础入门](basic-overview.md)
