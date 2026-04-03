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
let mut x = 5;

let closure1 = || x += 1;
let closure2 = || x += 1;  // ❌ 多次可变借用

closure1();
closure2();
```

**修复**：
```rust
let mut x = 5;
{
    let mut closure1 = || x += 1;
    closure1();  // 第一个闭包用完
}
{
    let mut closure2 = || x += 1;  // 现在可以用第二个
    closure2();
}
```

### 错误 2：move 后使用

```rust
let x = String::from("hello");

let closure = move || println!("{}", x);
println!("{}", x);  // ❌ x 已转移
```

**修复**：
```rust
let x = String::from("hello");

let closure = || println!("{}", &x);  // 借用，不转移
println!("{}", x);  // ✅ x 仍可用
```

---

## 动手练习

### 练习 1：实现迭代器过滤

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// TODO: 过滤出偶数
let evens: Vec<i32> = numbers
    .iter()
    .filter(|n| /* TODO */)
    .cloned()
    .collect();
```

<details>
<summary>点击查看答案</summary>

```rust
let evens: Vec<i32> = numbers
    .iter()
    .filter(|n| n % 2 == 0)
    .cloned()
    .collect();
```

**解析**：`filter` 接受返回 bool 的闭包。
</details>

### 练习 2：捕获方式判断

以下闭包各用什么方式捕获？

```rust
let x = 5;
let a = || println!("{}", x);

let mut y = 5;
let mut b = || y += 1;

let z = String::from("hi");
let c = move || println!("{}", z);
```

<details>
<summary>答案与解析</summary>

**答案**:
- a: 不可变借用 (&x)
- b: 可变借用 (&mut y)
- c: 转移所有权 (move)

**解析**: 
- a 只用 x → 不可变借用
- b 修改 y → 可变借用
- c 有 move → 转移所有权
</details>

---

## 故障排查 (FAQ)

### Q: 闭包和函数选哪个？

**A**:
```rust
// 需要泛型时 → 用闭包
vec.iter().filter(|x| x > &5)

// 需要具体类型时 → 用函数
fn is_positive(x: &i32) -> bool { *x > 0 }
vec.iter().filter(is_positive)
```

### Q: 如何调试闭包类型？

**A**:
```rust
// 闭包类型是匿名的
let closure = |x| x + 1;
// type: impl Fn(i32) -> i32

// 需要存储时用 Box
let boxed: Box<dyn Fn(i32) -> i32> = Box::new(closure);
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

以下代码的输出是什么？

```rust
let x = 5;
let closure = |y| x + y;
println!("{}", closure(3));
```

<details>
<summary>答案与解析</summary>

**答案**: `8`

**解析**: 闭包捕获 x 为不可变引用，使用时 x + y = 5 + 3 = 8。
</details>

**问题 2** 🟡 (Fn trait)

这段代码违反了什么规则？

```rust
let mut data = vec![1];
let mut closure = || data.push(2);

closure();
closure();  // ❌
```

<details>
<summary>答案与解析</summary>

**答案**: 违反借用规则

**解析**: 第二次调用时，第一次的可变借用还在作用域内。需要确保闭包用完前一个实例。
</details>

**问题 3** 🔴 (高级)

以下哪个闭包实现 `Fn`？

```rust
// A
let a = move |x| x + 1;

// B 
let mut count = 0;
let b = || count += 1;

// C
let data = vec![1];
let c = || println!("{}", data.len());
```

<details>
<summary>答案与解析</summary>

**答案**: C

**解析**:
- A: move 转移所有权 → FnOnce
- B: 修改 count → FnMut
- C: 只读捕获 → Fn
</details>

---

## 小结

**核心要点**：

1. **闭包是可捕获环境的匿名函数** - 记住周围变量
2. **三种捕获方式** - 不可变借用、可变借用、转移所有权
3. **Fn trait 层次** - Fn < FnMut < FnOnce
4. **类型推断** - 第一个调用确定类型参数
5. **move 转移所有权** - 用于线程、存储等场景

**术语**：

- **Closure (闭包)**: 可捕获环境的匿名函数
- **Capture (捕获)**: 闭包记住环境变量的方式
- **Move Keyword (move 关键字)**: 强制转移所有权

**下一步**：

- 继续：[迭代器](iterators.md) - 闭包的最佳应用场景
- 相关：[线程](threads.md) - move 闭包的典型用法

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Closure | 闭包 |
| Capture Environment | 捕获环境 |
| Move Keyword | move 关键字 |
| Trait (Fn/FnMut/FnOnce) | Fn 特征 |

---

完整示例：`src/basic/closure_sample.rs`

---

> 💡 **提示**：闭包是 Rust 函数式编程的心脏——多练习，你会爱上它的优雅！
