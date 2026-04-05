# 智能指针 (Smart Pointers)

## 开篇故事

想象你去住酒店。普通钥匙只能打开一扇门，而且你必须随身携带。智能卡则不同——它不仅能开门，还能记录你进入的次数，甚至当最后一个人离开时自动断电。

Rust 的**智能指针**就像这种智能卡。它们不仅指向内存中的数据，还携带额外的元数据（如引用计数）或能力（如内部可变性）。它们是 Rust 高级编程的基石。

---

## 本章适合谁

如果你已经理解了所有权的概念，现在想处理更复杂的场景（如多所有权、图结构、并发共享），本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解智能指针与普通引用的区别
2. 使用 `Box<T>` 进行堆分配和定义递归类型
3. 使用 `Rc<T>` 实现多所有权
4. 使用 `RefCell<T>` 实现内部可变性
5. 使用 `Arc<T>` 在线程间安全共享数据
6. 识别并解决引用循环导致的内存泄漏

---

## 前置要求

- [所有权](../basic/ownership.md) - 所有权基础
- [泛型](../basic/generic.md) - 泛型语法

---

## 第一个例子

最简单的智能指针 `Box<T>`：

```rust
fn main() {
    // 将 i32 分配到堆上
    let b = Box::new(5);
    println!("b = {}", b);
    // b 离开作用域时，堆内存被释放
}
```

**发生了什么？**
- `Box::new(5)` 在堆上分配内存存储 `5`。
- `b` 拥有这块堆内存的所有权。
- 当 `b` 离开作用域，`Box` 的 `drop` 方法被调用，释放堆内存。

---

## 原理解析

### 1. Box<T> - 堆分配

`Box<T>` 是最简单的智能指针，它允许你将数据存储在**堆**上而不是**栈**上。

**使用场景**:
- 当数据太大，不想在栈上复制时。
- 当你拥有一个在编译时大小未知的类型，但又需要在需要固定大小的上下文中使用它时。
- **定义递归类型**时（这是 `Box` 最重要的用途）。

**递归类型示例**:

```rust
// ❌ 错误：编译器不知道 List 有多大
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// ✅ 正确：使用 Box 指向下一个节点
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
```

### 2. Rc<T> - 引用计数

`Rc<T>` (Reference Counting) 允许**多个所有者**指向同一份数据。每当有一个新的所有者，计数加 1；当一个所有者离开作用域，计数减 1。当计数为 0 时，数据被清理。

**内存布局**:

```
Rc<String> 结构:
┌──────────────────────────────┐
│ 指针 (指向控制块)             │
└──────────┬───────────────────┘
           │
           ▼
┌──────────────────────────────┐
│ 强引用计数: 3                │
│ 弱引用计数: 0                │
│ 数据: "Hello"                │
└──────────────────────────────┘
```

**使用场景**: 图形结构（如 DOM 树）、事件监听器列表。

```rust
use std::rc::Rc;

fn main() {
    let s = Rc::new(String::from("Hello"));
    
    // 克隆 Rc 增加引用计数
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);
    
    println!("引用计数：{}", Rc::strong_count(&s)); // 3
    
    // s2 离开作用域，计数减 1
}
```

### 3. RefCell<T> - 内部可变性

Rust 的借用规则通常要求在编译时确定可变性。但 `RefCell<T>` 允许你在**运行时**检查借用规则。这被称为**内部可变性**模式。

**对比**:
- `RefCell<T>`: 运行时检查，单线程。
- `Mutex<T>`: 运行时检查，多线程。

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // 可变借用
    {
        let mut d = data.borrow_mut();
        *d += 1;
    } // 借用在这里结束
    
    // 不可变借用
    let d = data.borrow();
    println!("data: {}", *d); // 6
}
```

**注意**: 如果违反借用规则（如同时有两个可变借用），程序会 **panic**。

### 4. Arc<T> - 线程安全的 Rc

`Arc<T>` (Atomic Reference Counting) 是 `Rc<T>` 的原子版本，可以安全地在线程间共享。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到智能指针时的困惑——你并不孤单！**

### 困惑 1: "为什么有了引用还需要智能指针？"

**解答**: 引用 (`&T`) 只是借用，不拥有数据。智能指针**拥有**数据，并提供额外功能。

```rust
// 引用：不拥有数据
let s = String::from("hello");
let r = &s; // r 只是借用

// Box: 拥有数据
let b = Box::new(String::from("hello")); // b 拥有堆上的 String
```

### 困惑 2: "Rc 和 Arc 有什么区别？"

**解答**: 
- **Rc**: 非原子操作，**不能**跨线程发送 (`!Send`)，性能稍高。
- **Arc**: 原子操作，**可以**跨线程发送 (`Send + Sync`)，性能稍低。

**选择指南**: 
- 单线程多所有权 → `Rc`
- 多线程多所有权 → `Arc`

### 困惑 3: "RefCell 和 Mutex 有什么区别？"

**解答**:
- **RefCell**: 运行时检查借用规则，**不**锁定，只能单线程使用。
- **Mutex**: 运行时检查借用规则，**锁定**，可以跨线程使用。

---

## 常见错误

### 错误 1: 引用循环导致内存泄漏

```rust,ignore
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    // parent: Rc<Node>, // ❌ 如果加上这一行，会形成循环引用！
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // 如果 leaf 也有对 branch 的 Rc 引用，两者都不会被释放
}
```

**修复方法**: 使用 `Weak<T>` 打破循环。

```rust
use std::rc::Weak;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // ✅ 使用 Weak
    children: RefCell<Vec<Rc<Node>>>,
}
```

### 错误 2: 运行时借用冲突

```rust,ignore
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(42);
    
    let a = x.borrow_mut();
    let b = x.borrow_mut(); // ❌ Panic! 已经有一个可变借用了
    
    println!("{}, {}", a, b);
}
```

**修复方法**: 确保借用作用域不重叠。

```rust
let mut a = x.borrow_mut();
// a 在这里被使用
drop(a); // 显式释放借用
let b = x.borrow_mut(); // ✅ 现在可以了
```

---

## 动手练习

### 练习 1: 使用 Rc 共享数据

创建一个程序，让三个列表共享同一组数据：

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    
    // TODO: 创建 list1, list2, list3 都引用 data
    // TODO: 打印每个列表的引用计数
}
```

<details>
<summary>点击查看答案</summary>

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    
    let list1 = Rc::clone(&data);
    let list2 = Rc::clone(&data);
    let list3 = Rc::clone(&data);
    
    println!("引用计数：{}", Rc::strong_count(&data)); // 4
}
```
</details>

### 练习 2: 使用 RefCell 修改不可变数据

```rust
use std::cell::RefCell;

struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }
    
    // TODO: 实现 increment 方法，即使 self 是不可变引用也能修改 count
    fn increment(&self) {
        // *self.count.borrow_mut() += 1;
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Count: {}", *counter.count.borrow());
}
```

<details>
<summary>点击查看答案</summary>

```rust
impl Counter {
    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 什么时候使用 `Box`，什么时候直接存值？

**A**: 
- 数据很大（如大结构体）→ `Box`
- 需要多态（Trait Object）→ `Box<dyn Trait>`
- 递归类型 → `Box`
- 否则 → 直接存值（栈上更快）

### Q: `Rc<RefCell<T>>` 是什么组合？

**A**: 这是 Rust 中非常常见的模式：
- `Rc`: 提供多所有权。
- `RefCell`: 提供内部可变性。
- 组合起来：多个所有者都可以修改数据。

### Q: 为什么 Rust 不自动处理循环引用？

**A**: Rust 没有垃圾回收器 (GC)。它依赖确定性的析构（Drop）。循环引用会导致引用计数永远不为 0，从而内存泄漏。这是无 GC 语言的权衡。

---

## 知识扩展

### Weak<T> 详解

`Weak<T>` 是 `Rc<T>` 的非拥有版本。它不会增加强引用计数，因此不会阻止数据被清理。

```rust
let strong = Rc::new(42);
let weak = Rc::downgrade(&strong);

// 尝试升级 Weak 为 Rc
if let Some(strong_ref) = weak.upgrade() {
    println!("数据还在：{}", strong_ref);
} else {
    println!("数据已被清理");
}
```

---

## 小结

**核心要点**:

1. **智能指针**: 拥有数据并提供额外元数据或能力。
2. **Box<T>**: 堆分配，递归类型。
3. **Rc<T>**: 单线程多所有权。
4. **RefCell<T>**: 运行时借用检查（内部可变性）。
5. **Arc<T>**: 多线程多所有权。
6. **Weak<T>**: 打破循环引用。

**关键术语**:

- **Smart Pointer (智能指针)**: 拥有数据的指针。
- **Deref Trait**: 允许智能指针像引用一样使用。
- **Interior Mutability (内部可变性)**: 即使数据是不可变的也能修改它。
- **Reference Cycle (引用循环)**: 导致内存泄漏的循环引用。

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Smart Pointer | 智能指针 |
| Reference Counting | 引用计数 |
| Interior Mutability | 内部可变性 |
| Reference Cycle | 引用循环 |
| Weak Reference | 弱引用 |
| Strong Reference | 强引用 |

---

## 延伸阅读

- [Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rc 文档](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [RefCell 文档](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

## 继续学习

- 下一步：[高级特征](advanced-traits.md)
- 回顾：[所有权](../basic/ownership.md)

---

## 知识检查

**问题 1** 🟢 (基础)

`Box<T>` 的主要用途是什么？

A) 多线程共享数据  
B) 将数据分配到堆上  
C) 运行时借用检查  
D) 增加引用计数

<details>
<summary>点击查看答案</summary>

**答案**: B) 将数据分配到堆上

**解析**: `Box` 允许你将数据存储在堆上，并拥有一个指向该数据的指针。
</details>

**问题 2** 🟡 (中等)

以下代码会 Panic 吗？

```rust,ignore
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let a = x.borrow();
    let b = x.borrow();
    println!("{}, {}", a, b);
}
```

<details>
<summary>点击查看答案</summary>

**答案**: 不会 Panic。

**解析**: `RefCell` 允许多个不可变借用同时存在。只有当存在可变借用时才会冲突。
</details>

**问题 3** 🔴 (困难)

为什么 `Rc<T>` 不能跨线程使用？

<details>
<summary>点击查看答案</summary>

**答案**: 因为 `Rc` 的引用计数操作不是原子的。

**解析**: 在多线程环境下，两个线程可能同时修改引用计数，导致数据竞争。应使用 `Arc<T>`。
</details>
