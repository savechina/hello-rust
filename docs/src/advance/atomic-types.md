# 原子类型 (Atomic Types)

## 开篇故事

想象你在银行柜台办理业务。如果只有一个柜台，所有人必须排队等待（Mutex 锁）。如果银行开了多个窗口，并且有一个电子显示屏显示当前排队号码，每个人都可以查看号码并决定何时去办理，而不需要死等。

Rust 的**原子类型**就像那个电子显示屏。它们允许你在多线程环境下安全地共享和修改数据，而不需要加锁。这是实现高性能并发编程的关键。

---

## 本章适合谁

如果你已经理解了基本的并发概念（如线程、Mutex），现在想进一步提升并发性能，或者对无锁编程感兴趣，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解原子操作的概念
2. 使用 `std::sync::atomic` 模块中的原子类型
3. 理解内存序 (Memory Ordering) 的基本概念
4. 对比原子类型与 Mutex 的性能差异
5. 实现简单的无锁计数器

---

## 前置要求

- [线程与并发](../basic/threads.md) - 基础并发概念
- [智能指针](smart-pointers.md) - 智能指针基础

---

## 第一个例子

使用 `AtomicUsize` 实现线程安全的计数器：

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 原子增加
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数：{}", counter.load(Ordering::Relaxed));
}
```

**发生了什么？**
- `AtomicUsize`: 一个可以安全跨线程修改的整数。
- `fetch_add`: 原子地增加数值。
- `Ordering::Relaxed`: 指定内存序，这里只关心原子性，不关心顺序。

---

## 原理解析

### 1. 什么是原子操作？

原子操作是**不可中断**的操作。在多线程环境下，要么操作完全完成，要么完全没发生，不会出现中间状态。

**对比 Mutex**:

| 特性 | 原子类型 (Atomic) | 互斥锁 (Mutex) |
|------|------------------|----------------|
| **性能** | 极高 (硬件指令) | 较低 (系统调用) |
| **粒度** | 单个值 | 任意数据结构 |
| **阻塞** | 无锁 (Non-blocking) | 会阻塞线程 |
| **复杂性** | 较高 (需理解内存序) | 较低 (API 简单) |

### 2. 常用原子类型

Rust 提供了一系列原子类型：

- **整数**: `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicIsize`
- **无符号整数**: `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicUsize`
- **布尔**: `AtomicBool`
- **指针**: `AtomicPtr<T>`

### 3. 内存序 (Memory Ordering)

内存序定义了原子操作如何影响内存的可见性。这是原子类型中最难理解的部分。

**常见内存序**:

1. **Relaxed**: 只保证原子性，不保证顺序。适用于计数器。
2. **Acquire**: 保证此操作之后的读写不会被重排到操作之前。用于获取锁。
3. **Release**: 保证此操作之前的读写不会被重排到操作之后。用于释放锁。
4. **AcqRel**: 同时包含 Acquire 和 Release。用于读-改-写操作。
5. **SeqCst**: 顺序一致性。最严格，所有线程看到的操作顺序一致。

**简单指南**:
- 计数器 → `Relaxed`
- 简单的标志位同步 → `Acquire` / `Release`
- 不确定用什么 → `SeqCst` (最安全，性能稍差)

### 4. 常见操作

- `load(Ordering)`: 读取值。
- `store(val, Ordering)`: 写入值。
- `fetch_add(val, Ordering)`: 原子地增加。
- `fetch_sub(val, Ordering)`: 原子地减少。
- `compare_exchange(current, new, success, failure)`: CAS 操作，无锁编程的核心。

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到原子类型时的困惑——你并不孤单！**

### 困惑 1: "既然原子类型这么快，为什么不用它替代 Mutex？"

**解答**: 原子类型只能保证**单个值**的原子性。如果你需要保护复杂的数据结构（如 `HashMap`），必须用 Mutex。

```rust
// ❌ 错误：无法用原子类型保护 HashMap
let map: Atomic<HashMap<String, i32>> ... // 不存在这种类型

// ✅ 正确：使用 Mutex
let map: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
```

### 困惑 2: "内存序到底是什么？"

**解答**: 现代 CPU 会重排指令以优化性能。内存序告诉 CPU 哪些重排是允许的。
- `Relaxed`: 随便排，只要操作本身是原子的。
- `SeqCst`: 别乱排，按代码顺序来。

---

## 常见错误

### 错误 1: 误用 Relaxed 导致同步失败

```rust,ignore
// 线程 1
data_ready.store(true, Ordering::Relaxed); // ❌ 其他线程可能还没看到 data 的更新

// 线程 2
if data_ready.load(Ordering::Relaxed) {
    // 可能读到旧数据！
}
```

**修复方法**: 使用 `Release` / `Acquire`。

```rust
// 线程 1
data.store(42, Ordering::Relaxed);
data_ready.store(true, Ordering::Release); // ✅ 保证 data 的更新在 flag 之前可见

// 线程 2
if data_ready.load(Ordering::Acquire) {
    // 一定能读到 42
}
```

---

## 动手练习

### 练习 1: 实现线程安全的计数器

使用 `AtomicUsize` 实现一个计数器，10 个线程各增加 1000 次。

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    // TODO: 创建 10 个线程，每个线程循环 1000 次增加 counter
    // TODO: 等待所有线程结束
    // TODO: 打印最终结果 (应该是 10000)
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut handles = vec![];
for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            c.fetch_add(1, Ordering::Relaxed);
        }
    }));
}
for h in handles { h.join().unwrap(); }
println!("Result: {}", counter.load(Ordering::Relaxed));
```
</details>

---

## 小结

**核心要点**:

1. **原子类型**: 硬件级原子操作，无锁。
2. **适用场景**: 简单状态、计数器、标志位。
3. **内存序**: `Relaxed` (计数器), `Acquire/Release` (同步), `SeqCst` (默认)。
4. **对比 Mutex**: 原子类型快但只能保护单个值；Mutex 慢但能保护复杂结构。

**关键术语**:

- **Atomic Operation (原子操作)**: 不可中断的操作。
- **Memory Ordering (内存序)**: 定义操作顺序的规则。
- **CAS (Compare-And-Swap)**: 比较并交换，无锁编程基础。
- **Lock-free (无锁)**: 不阻塞线程的并发算法。

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Atomic Type | 原子类型 |
| Memory Ordering | 内存序 |
| Compare-And-Swap (CAS) | 比较并交换 |
| Lock-free | 无锁 |
| Sequential Consistency | 顺序一致性 |

---

## 延伸阅读

- [Rust Book - Atomic Types](https://doc.rust-lang.org/std/sync/atomic/index.html)
- [C++ Memory Model](https://en.cppreference.com/w/cpp/atomic/memory_order) - Rust 内存序基于 C++ 模型

## 继续学习

- 下一步：[并发编程进阶](concurrency-advanced.md)
- 回顾：[线程与并发](../basic/threads.md)

---

## 知识检查

**问题 1** 🟢 (基础)

`AtomicUsize` 主要适用于什么场景？

A) 保护复杂的 HashMap  
B) 线程安全的计数器或标志位  
C) 替代所有的 Mutex  
D) 存储大对象

<details>
<summary>点击查看答案</summary>

**答案**: B) 线程安全的计数器或标志位
</details>

**问题 2** 🟡 (中等)

以下哪种内存序最严格？

A) Relaxed  
B) Acquire  
C) Release  
D) SeqCst

<details>
<summary>点击查看答案</summary>

**答案**: D) SeqCst (顺序一致性)
</details>
