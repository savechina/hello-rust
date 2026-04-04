# 指针与不安全代码

## 🔴 高危警告

本章涉及 Rust 的 `unsafe` 特性。这些内容仅用于理解 Rust 的底层机制。**除非绝对必要且有充分理由，否则不要在生产线代码中使用 unsafe。**

---

## 开篇故事

想象你在驾驶一辆汽车。安全模式就像有安全气囊、ABS 刹车辅助、车道偏离警告——系统会保护你不犯错。不安全代码就像关闭所有安全系统，直接操控引擎——你能获得极致性能，但一次失误就可能车毁人亡。

Rust 的 `unsafe` 就是那个"关闭安全系统"的开关。它不是"邪恶"的，而是**强大但危险**的工具。本章教你理解它、尊重它、必要时安全地使用它。

---

## 本章适合谁

如果你想理解 Rust 内存安全的底层机制，或者需要与 C 代码交互、实现高性能数据结构，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解原始指针语法和创建方式
2. 掌握 unsafe 块的 5 种操作
3. 识别何时必须使用 unsafe
4. 使用安全抽象封装 unsafe 代码
5. 理解未定义行为（UB）的危害

---

## 前置要求

- [所有权](ownership.md) - 内存安全基础
- [借用规则](lifetimes.md) - 引用安全性

---

## 第一个例子

```rust,ignore
fn main() {
    let mut num = 5;
    
    // ✅ 安全引用
    let r1 = &num;
    let r2 = &num;
    
    // ⚠️ 原始指针（unsafe）
    let r3 = &num as *const i32;
    let r4 = &mut num as *mut i32;
    
    // ❌ 解引用原始指针需要 unsafe
    unsafe {
        println!("r3 是：{}", *r3);
        *r4 = 10;  // ⚠️ 危险！
    }
    
    println!("num 现在是：{}", num);
}
```

**发生了什么？**

- `*const T` - 不可变原始指针
- `*mut T` - 可变原始指针
- 创建指针是安全的，但**解引用**需要 `unsafe`

---

## 原理解析

### 1. 原始指针 vs 引用

```rust,ignore
let x = 10;
let ref_x = &x;          // 引用：安全，遵循借用规则
let ptr_x = &x as *const i32;  // 原始指针：不安全，无借用检查

// 引用保证：
// ✅ 永远不为空
// ✅ 指向有效数据
// ✅ 遵循借用规则（可变/不可变互斥）

// 原始指针不保证：
// ❌ 可能为空
// ❌ 可能指向已释放内存
// ❌ 可以同时有多个可变指针
```

### 2. 内存布局可视化

```
栈内存                    堆内存
+---------------+         
| x: 10         |         
| ref_x: ───────+────────→ (借用检查保证安全)
| ptr_x: ───────+────────→ (无保证，可能悬垂)
+---------------+

安全引用:
ref_x ───→ [有效数据]
           ↑
       借用检查器保证

原始指针:
ptr_x ───→ [???]
           ↑
       可能是悬垂指针！
```

### 3. unsafe 的 5 种操作

只有以下 5 种操作需要 `unsafe`：

```rust,ignore
unsafe {
    // 1. 解引用原始指针
    let x = *ptr;
    
    // 2. 调用 unsafe 函数
    unsafe_function();
    
    // 3. 访问或修改可变静态变量
    STATIC_VAR = 10;
    
    // 4. 实现 unsafe trait
    impl UnsafeTrait for MyType {}
    
    // 5. 访问 union 的字段
    let field = my_union.variant1;
}
```

### 4. 何时必须使用 unsafe

**合法场景**：

1. **FFI（外部函数接口）**：
```rust,ignore
// 调用 C 库
extern "C" {
    fn printf(format: *const i8, ...) -> i32;
}

unsafe {
    printf(b"Hello from C!\0".as_ptr() as *const i8);
}
```

2. **高性能数据结构**：
```rust,ignore
// 实现 Vec 的底层
pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn push(&mut self, value: T) {
        unsafe {
            // 直接写入内存，跳过边界检查
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
    }
}
```

3. **硬件操作**：
```rust,ignore
// 内存映射 I/O
const GPIO_BASE: *mut u32 = 0x40020000 as *mut u32;

unsafe {
    GPIO_BASE.write(0x01);  // 直接写硬件寄存器
}
```

### 5. 安全抽象封装

**关键原则**：unsafe 代码应该被安全的公共接口封装。

```rust,ignore
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        let ptr = unsafe {
            // unsafe 内部：分配内存
            alloc::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap())
        };
        SafeBuffer { ptr, len: size }
    }
    
    // ✅ 安全公共接口
    pub fn read(&self, offset: usize) -> Option<u8> {
        if offset < self.len {
            Some(unsafe { *self.ptr.add(offset) })
        } else {
            None  // 安全：越界返回 None
        }
    }
    
    // ✅ 安全公共接口
    pub fn write(&mut self, offset: usize, value: u8) -> bool {
        if offset < self.len {
            unsafe { *self.ptr.add(offset) = value };
            true
        } else {
            false  // 安全：越界返回 false
        }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            alloc::alloc::dealloc(self.ptr, std::alloc::Layout::from_size_align(self.len, 1).unwrap());
        }
    }
}
```

### 6. MaybeUninit：未初始化内存

当你需要创建未初始化的内存时（如 C FFI 或性能优化），使用 `MaybeUninit`：

```rust,ignore
use std::mem::MaybeUninit;

// ❌ 错误：未初始化的 Vec
let mut data: [u8; 1024] = [0; 1024];  // 初始化为 0

// ✅ 正确：使用 MaybeUninit
let mut data: [MaybeUninit<u8>; 1024] = MaybeUninit::uninit_array();

// 填充数据
for i in 0..1024 {
    data[i].write(i as u8);
}

// 安全地转换为初始化数组
let data: [u8; 1024] = unsafe {
    MaybeUninit::array_assume_init(data)
};
```

### 7. ManuallyDrop：阻止自动 Drop

当你想手动控制资源释放时：

```rust,ignore
use std::mem::ManuallyDrop;

let mut x = ManuallyDrop::new(Box::new(42));
println!("{}", *x);

// 手动释放
let boxed: Box<i32> = unsafe { ManuallyDrop::take(&mut x) };
// 现在 boxed 会正常 drop
```

### 8. 实现 Send 和 Sync

当你需要让自定义类型跨线程时：

```rust,ignore
use std::sync::Arc;

struct MyWrapper(*mut i32);

// ❌ 默认不是 Send/Sync
// 手动实现（需要确保线程安全）
unsafe impl Send for MyWrapper {}
unsafe impl Sync for MyWrapper {}

// ✅ 更安全的方式：使用 Arc
struct SafeWrapper(Arc<i32>);
// Arc 自动实现 Send 和 Sync
```

---

## 常见错误

### 错误 1: 不安全的 UTF-8 转换

```rust,ignore
let bytes = vec![0, 159, 146, 150];

// ❌ 假设字节是有效的 UTF-8
let s = unsafe {
    std::str::from_utf8_unchecked(&bytes)  // ⚠️ 如果无效就是未定义行为
};

// ✅ 安全版本
let s = std::str::from_utf8(&bytes).unwrap();  // 会检查
```

**什么时候可以用** `from_utf8_unchecked`**？**

仅在以下情况：
- 你已经手动验证了字节是有效的 UTF-8
- 性能关键路径且有基准测试证明瓶颈
- 你有测试确保不会传入无效数据

### 错误 2: 悬垂指针

```rust,ignore
// ❌ 错误：返回悬垂指针
fn dangling_pointer() -> *const i32 {
    let x = 5;
    &x as *const i32  // x 在函数结束时被丢弃！
}

// ✅ 正确：返回拥有的值
fn safe_value() -> i32 {
    let x = 5;
    x  // 返回值，不是指针
}
```

### 错误 3: 数据竞争

```rust,ignore
use std::thread;

// ❌ 错误：多线程同时修改同一数据
let mut data = vec![1, 2, 3];
let ptr = data.as_mut_ptr();

let handles: Vec<_> = (0..3).map(|i| {
    thread::spawn(move || {
        unsafe {
            *ptr.add(i) += 1;  // 数据竞争！
        }
    })
}).collect();

// ✅ 正确：使用 Arc<Mutex<T>>
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(vec![1, 2, 3]));
```

---

## 动手练习

### 练习 1: 安全的指针包装器

创建一个安全的指针包装器，防止空指针解引用：

```rust,ignore
// TODO: 实现 NonNullPtr<T> 结构体
// - 内部使用 *mut T
// - 提供安全的 new() 方法（拒绝空指针）
// - 提供安全的 get() 方法返回 &T
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use std::ptr::NonNull;

pub struct NonNullPtr<T> {
    ptr: NonNull<T>,
}

impl<T> NonNullPtr<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        NonNullPtr {
            ptr: NonNull::new(ptr).unwrap(),  // 保证非空
        }
    }
    
    pub fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Drop for NonNullPtr<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}
```
</details>

---

## 故障排查

### Q: unsafe 真的不安全吗？

**A**: 不是。`unsafe` 意味着**你**负责保证安全，而不是编译器。如果你正确使用了 unsafe，代码是安全的。

### Q: 如何调试 unsafe 代码？

**A**: 
1. 使用 Miri 工具检测未定义行为：`cargo +nightly miri run`
2. 启用 AddressSanitizer：`RUSTFLAGS="-Z sanitizer=address" cargo run`
3. 编写充分的单元测试
4. 使用 `#[deny(unsafe_op_in_unsafe_fn)]` 强制显式 unsafe

### Q: 标准库中有多少 unsafe 代码？

**A**: 约 10-15%。像 `Vec`、`String`、`HashMap` 这样的核心数据结构底层都使用 unsafe，但它们提供了安全的公共接口。

### Q: 如何安全地实现自定义集合？

**A**: 遵循以下模式：
1. 使用 `MaybeUninit` 管理未初始化内存
2. 在 `Drop` 中正确释放资源
3. 提供安全的公共接口
4. 编写充分的测试（包括边界情况）
5. 使用 Miri 验证未定义行为

---

## 知识扩展（选学）

### unsafe 在标准库中的应用

**Vec 的 push 实现**（简化版）：
```rust,ignore
impl<T> Vec<T> {
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();  // 重新分配
        }
        
        unsafe {
            // 直接写入，跳过边界检查
            std::ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
}
```

---

## 小结

**核心原则**：

1. **unsafe 不是"随便用"**: 只在必要且可控时使用
2. **封装 unsafe**: 提供安全的接口
3. **记录安全契约**: 为什么 unsafe 是安全的
4. **优先安全抽象**: Rust 标准库已经提供了大部分需要的工具

**关键术语**：

- **Raw Pointer**: 原始指针
- **Unsafe Block**: unsafe 块
- **Undefined Behavior (UB)**: 未定义行为
- **FFI**: 外部函数接口
- **Safety Invariant**: 安全不变量

---

## 术语表

| English          | 中文       |
| ---------------- | ---------- |
| Raw pointer      | 原始指针   |
| Unsafe block     | unsafe 块  |
| Undefined behavior | 未定义行为 |
| Safety contract  | 安全契约   |
| FFI              | 外部函数接口 |
| Memory safety    | 内存安全   |

---

完整示例：`src/basic/pointer_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. 原始指针和引用有什么区别？

2. 什么时候需要使用 `unsafe`？

3. `*const T` 和 `*mut T` 的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 原始指针不遵循借用规则，可以为空或悬垂
2. 解引用原始指针、调用 unsafe 函数、访问可变静态
3. `*const T` 不可变，`*mut T` 可变

**关键理解**: unsafe 是强大但危险的工具，应谨慎使用并封装在安全接口中。
</details>

## 延伸阅读

学习完指针与不安全代码后，你可能还想了解：

- [Rustonomicon](https://doc.rust-lang.org/nomicon/) - 不安全 Rust 指南
- [FFI 指南](https://doc.rust-lang.org/nomicon/ffi.html) - 与 C 代码交互
- [智能指针深入](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) - Box, Rc, Arc

**选择建议**:
- 想学习日志 → 继续学习 [日志记录](logger.md)
- 想学习追踪 → 跳到 [追踪 (Tracing)](tracing.md)

## 继续学习

- 下一步：[日志记录](logger.md)
- 进阶：[Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- 回顾：[所有权](ownership.md)

> 🔴 **记住**：unsafe 让你对编译器说"我知道我在做什么，相信我"。确保你真的知道！
