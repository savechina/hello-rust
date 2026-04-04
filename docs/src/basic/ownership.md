# 了解所有权

## 开篇故事

想象你有一本珍贵的编程书。你可以把它借给朋友阅读，但同一时间只能有一个人拿着这本书。如果你的朋友正在读，你就不能同时读它。这就是 Rust 所有权的核心思想：**每个值在任一时刻只能有一个所有者**。

---

## 本章适合谁

如果你已经学完了变量和数据类型，现在想理解 Rust 最独特的内存管理机制，本章适合你。所有权是 Rust 与其他语言最大的不同之处，需要多花些时间理解——这完全正常。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是所有权以及为什么 Rust 需要它
2. 理解值何时被移动 (move) 以及移动的后果
3. 识别所有权转移的代码模式
4. 避免"移动后使用"的常见错误
5. 理解如何正确返回函数内部创建的数据

---

## 前置要求

学习本章前，你需要理解：

- [变量表达式](expression.md) - 变量绑定基础
- [数据类型](datatype.md) - String 类型和堆/栈概念

---

## 第一个例子

让我们看一个最简单的所有权示例：

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s2); // ✅ 可以工作
// println!("{}", s1); // ❌ 编译错误！s1 已经移动给 s2 了
```

**发生了什么？**

第 2 行 `let s2 = s1;` 不是复制字符串，而是**转移所有权**。`s1` 的所有权移动给了 `s2`，`s1` 不再有效。

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念     | Python               | Java                 | C++                    | Rust                       | 关键差异                  |
| -------- | -------------------- | -------------------- | ---------------------- | -------------------------- | ------------------------- |
| 变量赋值 | `s2 = s1` (引用)       | `s2 = s1` (引用)       | `s2 = s1` (浅拷贝)       | `let s2 = s1;` (移动)        | Rust 转移所有权，其他语言共享 |
| 内存管理 | 垃圾回收 (GC)        | 垃圾回收 (GC)        | 手动 `delete` / 智能指针 | 所有权系统 (编译时检查)      | Rust 无运行时 GC            |
| 字符串复制 | `s2 = s1[:]` (显式)  | `s2 = s1.clone()`    | `s2 = s1` (浅) / 深拷贝  | `let s2 = s1.clone();`       | Rust 显式克隆，默认移动     |
| 函数参数 | 传递引用 (默认)      | 传递引用 (对象)      | 值传递 / 引用传递        | 移动 (默认) / 借用 (`&`)     | Rust 默认转移所有权         |
| 悬垂指针 | 不可能 (GC 保护)     | 不可能 (GC 保护)     | 可能 (运行时错误)        | **编译时阻止**               | Rust 在编译时防止           |

---

## 原理解析

### 所有权内存模型

```text
栈 (Stack)                    堆 (Heap)
+---------------+          +----------------+
| s1 (pointer)  |--------->| "hello"        |
| length: 5     |          |                |
| capacity: 5   |          +----------------+
+---------------+

let s2 = s1;  // 移动后:

栈 (Stack)                    堆 (Heap)
+---------------+          +----------------+
| s1 (无效)     |          | "hello"        |
| s2 (pointer)  |--------->|                |
| length: 5     |          |                |
| capacity: 5   |          +----------------+
+---------------+
```

**关键点**：
- 移动后 `s1` 变为无效，编译器防止使用
- 堆内存不会被释放两次（防止 double free）
- 只有 `s2` 可以访问和释放堆内存

### 借用 vs 移动

```text
移动 (Move):
s1 ──→ [堆内存]     s2 = s1     s1 ❌  s2 ──→ [堆内存]

借用 (Borrow):
s1 ──→ [堆内存]     r = &s1     s1 ──→ [堆内存] ←── r ✅
```

### 所有权三规则

Rust 的所有权系统遵循三条简单规则：

1. **每个值都有一个所有者**
   - 变量是值的所有者
   - 所有者负责清理值

2. **任一时刻只能有一个所有者**
   - 不像其他语言可以有多个引用指向同一数据
   - Rust 确保内存安全

3. **所有者离开作用域，值被丢弃**
   - 自动清理，无需手动释放
   - 防止内存泄漏

### 移动语义 (Move Semantics)

当你在 Rust 中赋值或传递值时：

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权移动给 s2
```

**对于栈上的数据** (如 `i32`, `bool`):
- 直接复制（非常快）
- 原变量仍然可用

**对于堆上的数据** (如 `String`, `Vec`):
- 只复制指针、长度、容量
- 原变量不再有效
- 保证只有一个变量负责清理堆内存

---

## 常见错误

### 错误 1: 移动后使用 (Use After Move)

**❌ 错误代码**：

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // ❌ 编译错误!
```

**🤔 为什么这行不编译？**

编译器会告诉你：
```text
error[E0382]: borrow of moved value: `s1`
  |
2 |     let s2 = s1;
  |              -- value moved here
3 |     println!("{}", s1); // ❌ 编译错误!
  |                    ^^ value borrowed here after move
```

**解释**：`s1` 在第 2 行已经移动给 `s2` 了。Rust 不允许使用已移动的变量，这是为了防止"悬垂指针"——如果 `s2` 清理了堆内存，`s1` 就会指向无效数据！

**✅ 修复方法 1**：如果只需读取，使用引用（借用）：

```rust
let s1 = String::from("hello");
let s2 = &s1;  // 借用，不移动
println!("{}", s1); // ✅ s1 仍然可用
println!("{}", s2); // ✅ s2 是引用
```

**✅ 修复方法 2**：如果需要两个独立的 String，使用克隆：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深度复制
println!("{}", s1); // ✅ 两者都可用
println!("{}", s2); // ✅
```

---

### 错误 2: 函数参数移动

```rust,ignore
fn takes_ownership(s: String) {
    println!("Got: {}", s);
} // s 离开作用域，被丢弃

let my_string = String::from("hello");
takes_ownership(my_string);
println!("{}", my_string); // ❌ 编译错误！my_string 已经移动给函数了
```

**修复方法**：

1. **使用引用传递**（推荐）:
   ```rust
   fn borrows_string(s: &String) {
       println!("Borrowed: {}", s);
   } // 不获取所有权，只是借用
   
   let my_string = String::from("hello");
   borrows_string(&my_string);
   println!("{}", my_string); // ✅ my_string 仍然可用
   ```

2. **返回所有权**：
   ```rust
   fn takes_and_returns(s: String) -> String {
       println!("Got: {}", s);
       s // 返回所有权
   }
   
   let my_string = String::from("hello");
   my_string = takes_and_returns(my_string);
   println!("{}", my_string); // ✅ 所有权回来了
   ```

---

### 错误 3: 试图返回局部变量的引用

```rust,ignore
fn returns_local_ref() -> &String {
    let s = String::from("hello");
    &s // ❌ 编译错误！s 会在函数结束时被清理
}
```

**编译器输出**:
```text
error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:4:5
  |
4 |     &s // ❌ 编译错误！s 会在函数结束时被清理
  |     ^^ returns a reference to data owned by the current function
```

**为什么错误**：
`s` 在函数栈上创建，函数结束时会被清理。返回指向它的引用会导致**悬垂指针**。

**修复方法**：

1. **直接返回值**（转移所有权）:
   ```rust
   fn returns_owned_string() -> String {
       let s = String::from("hello");
       s // ✅ 移动所有权给调用者
   }
   ```

2. **使用静态字符串**:
   ```rust
   fn returns_static() -> &'static str {
       "hello" // ✅ 字符串字面量有 'static 生命周期
   }
   ```

3. **使用生命周期标注**（高级主题，后续章节详述）:
   ```rust
   fn get_ref<'a>(data: &'a str) -> &'a str {
       data // ✅ 返回外部的引用
   }
   ```

---

## 动手练习

### 🟢 入门练习：识别所有权转移

下面的代码会编译通过吗？如果不会，如何修复？

> 💡 **编译器是你的老师**：尝试运行这段代码，仔细阅读编译器错误信息。它会告诉你移动发生在哪里！

```rust,ignore
fn main() {
    let x = String::from("hello");
    let y = x;
    let z = x; // 这里会发生什么？
    println!("{}", x);
}
```

<details>
<summary>点击查看答案与解析</summary>

**答案**：❌ 不会编译通过

**解析**：
第 3 行 `let z = x;` 试图使用已经移动给 `y` 的 `x`。第 4 行再次使用 `x`，编译错误。

**修复方案**：
```rust
let x = String::from("hello");
let y = x.clone(); // 克隆，不移动
let z = x.clone(); // 再次克隆
println!("{}", x); // ✅ 三者都可用
```

</details>

---

### 🟡 中级练习：修复函数参数

补全下面的函数，使得调用后 `original` 仍然可用：

> 💡 **提示**：想想"借用"和"所有权"的区别。如果你只需要读取，不需要拥有，应该用什么？

```rust,ignore
fn print_and_??? (s: ???) {
    println!("Length: {}", s.len());
}

fn main() {
    let original = String::from("hello");
    print_and_???(&???);
    println!("Still have: {}", original); // 应该可以工作
}
```

<details>
<summary>点击查看答案</summary>

**答案**：
```rust
fn print_and_keep(s: &String) {  // 或者 &str
    println!("Length: {}", s.len());
}

fn main() {
    let original = String::from("hello");
    print_and_keep(&original);  // 传递引用
    println!("Still have: {}", original); ✅
}
```

</details>

---

### 🔴 挑战练习：理解移动与复制

预测下面代码哪些会编译通过：

> 💡 **挑战**：先不看答案，自己推理每个案例。思考"这个类型实现了 Copy trait 吗？"

```rust,ignore
// A
let a = 5;
let b = a;
println!("{}", a);

// B
let s1 = String::from("test");
let s2 = s1;
println!("{}", s1);

// C
let s1 = 42;
let s2 = s1;
let s3 = s1;
println!("{}", s1);
```

<details>
<summary>点击查看解析</summary>

**A**: ✅ 通过 - `i32` 类型在栈上，直接复制
**B**: ❌ 失败 - `String` 转移所有权，`s1` 不再可用
**C**: ✅ 通过 - `i32` 类型，可以多次复制使用

**关键点**：`Copy trait` 类型（如所有整数）会自动复制，其他类型（如 `String`）会转移所有权。

</details>

---

## 故障排查 (FAQ)

### Q: 为什么 Rust 要设计这么复杂的所有权系统？

**A**: 为了**内存安全**和**零成本抽象**。

- **内存安全**：不使用垃圾回收，也能防止悬垂指针、双重释放等问题
- **零成本**：编译时检查，运行时无额外开销
- **并发安全**：所有权规则天然防止数据竞争

虽然学习曲线陡峭，但掌握后你会写出更可靠的代码。

---

### Q: 每次都要 `clone()` 会不会很慢？

**A**: 确实有性能开销，但：

1. **优先使用引用** - 大部分情况不需要克隆
2. **只在必要时克隆** - 当确实需要两份独立数据时
3. **使用 `Rc`/`Arc`** - 需要共享所有权时的智能指针

性能敏感的代码可以进行基准测试，但先保证正确性。

---

### Q: 如何调试"值已移动"的错误？

**A**: 遵循这个流程：

1. **编译器会告诉你移动发生在哪里**：
   ```
   value moved here
   ```

2. **问自己**：
   - 我真的需要所有权吗？还是只需要读取？→ 使用引用
   - 我需要两份独立的数据吗？→ 使用 `clone()`
   - 可以多线程共享吗？→ 使用 `Arc`

3. **画出所有权流程图**：
   ```
   s1 --移动--> s2 --移动--> s3
   ```

---

### Q: 所有权和 borrow 有什么区别？

**A**: 

| 所有权 (Ownership) | 借用 (Borrowing) |
| ----------------- | --------------- |
| 独占访问 | 可以共享访问 |
| 负责清理 | 不负责清理 |
| `s1 = s2` (移动) | `&s1` (引用) |
| 只能有一个所有者 | 可以有多个借用 |
| 可以修改和读取 | 取决于可变/不可变借用 |

---

## 知识扩展 (选学)

### Copy trait

有些类型实现了 `Copy trait`，赋值时自动复制而不是移动：

```rust
let x = 5;  // i32 实现了 Copy
let y = x;  // 复制值，x 仍然可用
println!("{}", x); // ✅ 可以
```

**哪些类型有 Copy trait**：

- 所有整数类型 (`i32`, `u64`, etc.)
- 布尔值 (`bool`)
- 浮点数 (`f64`, `f32`)
- 字符 (`char`)
- 元组（如果所有成员都有 Copy）
- 指针 (`&T`)

**哪些类型没有 Copy**：

- `String`
- `Vec<T>`
- 任何包含上述类型的结构体

如果你想让自己的类型有 Copy 行为，实现 `Copy` trait (所有成员必须是 Copy 类型)。

---

### Drop trait

当值离开作用域时，Rust 会自动调用 `drop` 方法清理资源：

```rust
struct MyFile {
    path: String,
}

impl Drop for MyFile {
    fn drop(&mut self) {
        println!("Cleaning up file: {}", self.path);
    }
}

fn main() {
    let f = MyFile { path: String::from("/tmp/test.txt") };
    println!("File created");
} // f 离开作用域，调用 drop 方法，打印 "Cleaning up file..."
```

你不需要手动调用 `drop`（实际上也不鼓励），Rust 会自动处理。

---

## 小结

**核心要点**：

1. **所有权**是 Rust 管理内存的方式，每个值有且只有一个所有者
2. **赋值 = 移动**（对于非 Copy 类型），原变量不再可用
3. **函数参数**默认转移所有权，使用引用避免移动
4. **不能返回局部变量的引用**，会创建悬垂指针
5. **使用 `clone()`** 在需要独立副本时，使用 **引用** 在只需读取时

**关键术语**：

- **所有权 (Ownership)**: 对值的独占访问和清理责任
- **移动 (Move)**: 所有权的转移
- **Copy trait**: 自动复制的类型
- **借用 (Borrow)**: 临时访问，不转移所有权

**🧠 学习提示**：
> 所有权是 Rust 最独特的特性，也是很多初学者的第一道坎。如果你感到困惑，这完全正常！
> 
> **Microsoft Rust 培训建议**："Struggling with the borrow checker is part of learning. If stuck >15 minutes → check solution, study, close, try again from scratch."
>
> **推荐学习流程**：
> 1. 先自己写代码，让编译器报错
> 2. 仔细阅读错误信息（Rust 的编译器是最好的老师）
> 3. 如果卡住超过 15 分钟，查看答案
> 4. 关掉答案，从头自己写一遍

**下一步**：

- 学习 借用和引用 - 如何在不转移所有权的情况下使用值
- 理解 生命周期 - 确保引用不会超出有效范围

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Ownership | 所有权 |
| Move | 移动 |
| Borrow | 借用 |
| Copy trait | Copy trait |
| Dangling pointer | 悬垂指针 |
| Drop | 丢弃/释放 |


完整示例：`src/basic/ownership_sample.rs`

---

## 延伸阅读

学习完所有权后，你可能还想了解：

- [Rust Book 所有权章节](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) - 官方深入讲解
- [借用检查器原理](https://blog.rust-lang.org/2015/08/14/First-borrow-checker-improvements.html) - 编译器如何实现
- Cell 和 RefCell - 内部可变性

**选择建议**:
- 想深入理解 → 继续学习 [生命周期](lifetimes.md)
- 想实践 → 跳到 [结构体](struct.md)

## 继续学习

- 下一步：借用和引用
- 进阶：[生命周期基础](lifetimes.md)
- 回顾：[数据类型回顾](datatype.md)

> 💡 **记住**：所有权是 Rust 最独特的特性，也是很多初学者的第一道坎。如果你感到困惑，这完全正常。多写代码，多看编译器错误，你会逐渐掌握它！

解决方法：

要将函数内部创建的数据“返回”出来，你必须转移该数据的所有权。Rust 的移动语义（Move Semantics）使得这变得简单且安全：

* 直接返回数据 (按值返回): 函数返回类型是 T，你直接返回函数内部创建的变量。数据的所有权从函数内部转移到调用者。

```rust
fn create_value_and_return_owned() -> i32 {
    let value = 42; // value 在函数栈上
    value // value 的所有权被移出函数
} // value 在这里不会被 drop，因为它已经被移出

fn create_string_and_return_owned() -> String {
    let text = String::from("hello"); // text 在函数栈上，但其数据在堆上
    text // text 的所有权被移出函数。堆上的数据不会被清理。
} // text 在这里不会被 drop

fn create_box_and_return_owned() -> Box<i32> {
    let boxed_value = Box::new(100); // boxed_value 在函数栈上，它指向堆上的数据
    boxed_value // boxed_value 的所有权被移出函数。堆上的数据不会被清理。
} // boxed_value 在这里不会被 drop

```

* 返回智能指针: 如果你需要共享数据，可以将内部创建的数据包装在 Rc 或 Arc 等智能指针中，并返回智能指针的副本。数据的实际所有权由智能指针管理，而你返回的是智能指针的共享引用或智能指针本身（所有权转移）。

```rust
use std::sync::Arc;
use std::cell::RefCell;

fn create_shared_data() -> Arc<RefCell<i32>> {
    let data = RefCell::new(0); // data 在函数栈上，它包装了堆上的数据
    Arc::new(data) // Arc::new 会将 RefCell 移动到堆上，并返回 Arc 的所有权
} // data 在这里不会被 drop，因为它内部的 RefCell 已经被移到堆上并被 Arc 拥有

fn ownership_shared_sample() {
    let shared = create_shared_data(); // shared 现在拥有 Arc 的所有权
    println!("{}", shared.borrow());
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_ownership_shared_sample() {
        ownership_shared_sample();
        println!("print test in mdbook")
    }
}

```

---

## 💡 小知识：所有权的历史渊源

**问题来源**：
在 C++ 中，内存管理是程序员的责任：
```cpp
// C++ 示例
void process() {
    std::string* s = new std::string("hello");
    // ... 使用 s ...
    delete s;  // 😰 忘记了就会内存泄漏
}

// 或者更糟：
std::string* get_string() {
    std::string s = "hello";
    return &s;  // ❌ 返回局部变量的指针！
}
```

**Rust 的创新**：
> "让编译器在编译时检查内存安全，而不是在运行时"

所有权系统的核心思想：
1. **每个值有一个所有者** - 明确责任
2. **所有者离开作用域，值被清理** - 自动内存管理
3. **借用检查** - 防止悬垂指针

**对比其他语言**：

| 语言   | 内存管理方式     | 优点             | 缺点                  |
|--------|-----------------|-----------------|----------------------|
| C/C++  | 手动管理         | 完全控制         | 容易泄漏、悬垂指针    |
| Java   | 垃圾回收 (GC)    | 简单             | 运行时开销、停顿      |
| Rust   | 所有权系统       | 零开销、编译时检查 | 学习曲线陡峭          |

---

## 🌟 工业界应用：防止内存泄漏

**真实案例**：
某金融公司用 C++ 写交易系统，遇到内存泄漏：
```cpp
// 模拟场景
void process_trade() {
    Trade* trade = new Trade();
    if (validate(trade)) {
        execute(trade);
        // 😱 忘记 delete，每次交易泄漏内存
    }
    // 一天后，系统内存耗尽崩溃
}
```

**Rust 方案**：
```rust
fn process_trade() {
    let trade = Trade::new();
    if validate(&trade) {
        execute(&trade);
    }
    // ✅ trade 自动清理，无需手动 delete
}
```

**结果**：
- 内存泄漏：**归零**
- 性能：**提升 40%** (无 GC 开销)
- 开发效率：**提高 2x** (不用调试内存问题)

---

## 🧪 动手试试：所有权规则

**练习**：预测每段代码的输出

```rust
// A. 移动语义
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s2);  // 输出：?
// println!("{}", s1);  // 会发生什么？

// B. 克隆
let s3 = String::from("world");
let s4 = s3.clone();
println!("{}, {}", s3, s4);  // 输出：?

// C. 借用
let s5 = String::from("rust");
let s6 = &s5;
println!("{}, {}", s5, s6);  // 输出：?
```

<details>
<summary>点击查看答案与解析</summary>

**答案**：

A. 
```
输出：hello
s1 那行会编译错误 - s1 已经移动给 s2
```

B.
```
输出：world, world
clone() 创建独立副本，两者都可用
```

C. 
```
输出：rust, rust
借用不转移所有权，两者都可用
```

**关键区别**：
- **移动** (`let s2 = s1`) - s1 不能再使用
- **克隆** (`s3.clone()`) - 创建独立副本
- **借用** (`&s5`) - 临时访问，不影响所有者

</details>

---

## 内存布局可视化

### 1. 栈 vs 堆内存

```
栈内存 (Stack)          堆内存 (Heap)
+---------------+       
| pointer       |------+-----> +-------------------+
| length: 5     |       |      | "hello"           |
| capacity: 5   |       |      |                   |
+---------------+       |      +-------------------+
                        |
s1 变量 (所有者) --------+
```

**说明**:
- 栈上存储：指针、长度、容量 (24 字节)
- 堆上存储：实际字符串数据 ("hello")
- s1 是指针，指向堆上数据的所有者

### 2. 所有权转移

```
转移前:
s1 ──────────→ [堆内存："hello"]
  (所有者)

执行 s2 = s1 后:
s1             [堆内存："hello"] ←────────── s2
  ❌ 无效        ↑                        ✅ 新所有者
                所有权转移
```

**关键点**:
- 移动后原变量失效
- 所有权只有一个
- 转移是浅拷贝（只复制指针，不复制数据）

### 3. 借用规则

```
不可变借用 (多个允许):
&s1 ──→ [data] ←── &s2
&s3 ──→              ↑
                     所有者

可变借用 (独占访问):
&mut s1 ──→ [data]   ←── 所有者
              ↑
         (其他引用不允许)
```

**规则**:
- 多个不可变借用 ✓
- 一个可变借用 ✓
- 同时有可变和不可变借用 ✗

---

## 知识检查

**问题 1** 🟢 (基础概念)

以下代码的输出是什么？

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s2);
```

A) 编译错误  
B) "hello"  
C) 运行时错误  
D) 空字符串

<details>
<summary>答案与解析</summary>

**答案**: B) "hello"

**解析**: s2 获得所有权后，可以正常使用。s1 不能再使用，但 s2 可以。
</details>

**问题 2** 🟡 (借用规则)

这段代码违反了什么规则？

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &mut s;  // ❌
```

<details>
<summary>答案与解析</summary>

**答案**: 违反借用规则 - 不可变借用存在时不能有可变借用

**修复**: 确保可变借用前，所有不可变借用已停止使用
</details>

**问题 3** 🔴 (生命周期)

这个函数的签名应该如何修正？

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

<details>
<summary>答案与解析</summary>

**答案**: 需要显式生命周期标注

**修复**:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
</details>

