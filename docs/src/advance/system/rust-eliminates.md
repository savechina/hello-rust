# Rust 消除的问题列表

## 开篇故事

想象你从 C++ 迁移到 Rust。在 C++ 中，你需要时刻担心：内存泄漏、悬垂指针、数据竞争、缓冲区溢出……这些 bug 可能导致崩溃、安全漏洞，甚至被黑客利用。

Rust 的创新在于：**它在编译时就防止了这些问题**。你不需要在运行时担心，因为编译器已经帮你检查过了。

本章列出了 Rust 在编译时消除的所有常见问题，让你了解 Rust 相比其他语言的安全优势。

---

## Rust 消除的问题完整列表

### 内存安全问题

| 问题 | C 有吗？ | C++ 有吗？ | Rust 如何防止 | 编译时机制 |
|------|---------|-----------|--------------|-----------|
| **缓冲区溢出** | ✅ 是 | ✅ 是 | 边界检查 | `Vec` 自动检查索引 |
| **悬垂指针** | ✅ 是 | ✅ 是 | 生命周期检查 | 借用检查器 |
| **Use-After-Free** | ✅ 是 | ✅ 是 | 所有权系统 | 移动语义 |
| **双重释放** | ✅ 是 | ✅ 是 | 所有权系统 | 每个值只有一个所有者 |
| **内存泄漏** | ✅ 是 | ⚠️ 部分 | `Drop` trait | 自动清理 |
| **未初始化内存** | ✅ 是 | ⚠️ 部分 | 必须初始化 | 编译器强制 |
| **空指针解引用** | ✅ 是 | ✅ 是 | `Option<T>` | 无 `null` 关键字 |
| **数据竞争** | ✅ 是 | ✅ 是 | 所有权 + `Send`/`Sync` | 编译时检查 |
| **Iterator 失效** | ✅ 是 | ✅ 是 | 借用规则 | 编译时阻止 |
| **释放后使用** | ✅ 是 | ✅ 是 | 生命周期系统 | 编译时检查 |

### 并发安全问题

| 问题 | C 有吗？ | C++ 有吗？ | Rust 如何防止 | 编译时机制 |
|------|---------|-----------|--------------|-----------|
| **数据竞争** | ✅ 是 | ✅ 是 | 可变借用独占 | 编译时检查 |
| **死锁** | ✅ 是 | ✅ 是 | 不直接防止 | 需要设计模式 |
| **竞态条件** | ✅ 是 | ✅ 是 | 不可变性默认 | `mut` 显式声明 |
| **线程不安全共享** | ✅ 是 | ✅ 是 | `Send`/`Sync` trait | 编译时检查 |

### 类型安全问题

| 问题 | C 有吗？ | C++ 有吗？ | Rust 如何防止 | 编译时机制 |
|------|---------|-----------|--------------|-----------|
| **类型转换错误** | ✅ 是 | ⚠️ 部分 | 显式转换 | `as` 关键字 |
| **整数溢出** | ✅ 是 | ⚠️ 部分 | Debug 模式 panic | 编译时/运行时检查 |
| **符号溢出** | ✅ 是 | ✅ 是 | 显式类型 | 类型系统 |
| **未定义行为** | ✅ 是 | ✅ 是 | `unsafe` 标记 | 显式声明 |

---

## 具体案例分析

### 案例 1: 缓冲区溢出

**C 代码（有漏洞）**：
```c
void process_input(char* input) {
    char buffer[64];
    strcpy(buffer, input);  // 😱 如果 input > 64 字节，溢出！
}
```

**Rust 代码（安全）**：
```rust
fn process_input(input: &str) {
    let mut buffer = [0u8; 64];
    // Rust 会检查长度，超出会 panic 而不是溢出
    if input.len() <= 64 {
        buffer[..input.len()].copy_from_slice(input.as_bytes());
    }
}
```

---

### 案例 2: 悬垂指针

**C++ 代码（有漏洞）**：
```cpp
int* get_pointer() {
    int x = 42;
    return &x;  // ❌ 返回局部变量的指针！
}  // x 在这里被销毁，指针悬垂
```

**Rust 代码（编译时阻止）**：
```rust
fn get_pointer() -> &i32 {
    let x = 42;
    &x  // ❌ 编译错误！
}  // 编译器：cannot return reference to local variable `x`
```

---

### 案例 3: Use-After-Free

**C++ 代码（有漏洞）**：
```cpp
std::unique_ptr<int> ptr = std::make_unique<int>(42);
std::unique_ptr<int> ptr2 = std::move(ptr);
std::cout << *ptr;  // ❌ 编译通过！但运行时 UB
```

**Rust 代码（编译时阻止）**：
```rust
let vec = vec![1, 2, 3];
let vec2 = vec;           // 移动 — vec 被消耗
// vec.len();             // ❌ 编译错误：value used after move
```

---

### 案例 4: 数据竞争

**C++ 代码（有漏洞）**：
```cpp
std::vector<int> data = {1, 2, 3};

// 线程 1
std::thread t1([&]() {
    for (int& x : data) x *= 2;  // 写入
});

// 线程 2
std::thread t2([&]() {
    for (int x : data) std::cout << x;  // 读取
});
// 😱 数据竞争！结果不确定
```

**Rust 代码（编译时阻止）**：
```rust
let mut data = vec![1, 2, 3];

// 线程 1：可变借用
let t1 = std::thread::spawn(|| {
    for x in &mut data { *x *= 2; }
});

// 线程 2：不可变借用
let t2 = std::thread::spawn(|| {
    for x in &data { println!("{}", x); }
});
// ❌ 编译错误：cannot borrow `data` as immutable because it is also borrowed as mutable
```

---

### 案例 5: Iterator 失效

**C++ 代码（有漏洞）**：
```cpp
std::vector<int> vec = {1, 2, 3, 4, 5};
for (auto it = vec.begin(); it != vec.end(); ++it) {
    if (*it % 2 == 0) {
        vec.erase(it);  // 😱 iterator 失效！
    }
}
```

**Rust 代码（编译时阻止或安全 API）**：
```rust
let mut vec = vec![1, 2, 3, 4, 5];

// ❌ 编译错误：cannot borrow `vec` as mutable because it is also borrowed as immutable
// for x in &vec {
//     if x % 2 == 0 {
//         vec.retain(|&y| y != *x);
//     }
// }

// ✅ 正确：使用 retain
vec.retain(|&x| x % 2 != 0);
```

---

## 工业界影响

### 真实案例：Microsoft 的 CVE 统计

Microsoft 发现 **70% 的 CVE（安全漏洞）是内存安全问题**。如果这些代码用 Rust 编写，这些漏洞在编译时就会被阻止。

### 真实案例：Google 的 Android

Google 发现 Android 中 **~70% 的安全漏洞是内存安全问题**。他们正在用 Rust 重写关键组件。

### 真实案例：AWS Firecracker

AWS 用 Rust 构建 Firecracker（微虚拟机），**零内存安全漏洞**。

---

## 总结

Rust 通过编译时检查消除了以下类别的问题：

| 类别 | 问题数量 | Rust 消除率 |
|------|---------|------------|
| **内存安全** | 10+ | ~95% |
| **并发安全** | 4+ | ~90% |
| **类型安全** | 4+ | ~95% |
| **总计** | **18+** | **~93%** |

> 💡 **记住**：Rust 不是完美的——它不能防止所有 bug（如逻辑错误、死锁）。但它消除了最常见的、最危险的安全漏洞类别。

---

## 术语表

| English | 中文 |
|---------|------|
| Buffer Overflow | 缓冲区溢出 |
| Dangling Pointer | 悬垂指针 |
| Use-After-Free | 释放后使用 |
| Double Free | 双重释放 |
| Data Race | 数据竞争 |
| Null Pointer Dereference | 空指针解引用 |
| Iterator Invalidation | Iterator 失效 |
| Undefined Behavior (UB) | 未定义行为 |

完整示例：[crates/awesome/src/](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/)

---

## 继续学习

- 上一步：[所有权](../../basic/ownership.md) - Rust 内存安全的核心
- 下一步：[错误处理](../tools/error-handling.md) - Rust 错误处理最佳实践
- 相关：[指针与 unsafe](../../basic/pointer.md) - 何时需要 unsafe

> 💡 **记住**：Rust 的编译时检查不是限制你，而是保护你。每次编译器报错，它都在帮你防止一个潜在的运行时 bug！
