# 指针与不安全代码

## 🔴 高危警告

本章涉及 Rust 的 `unsafe` 特性。这些内容仅用于理解 Rust 的底层机制。**除非绝对必要且有充分理由，否则不要在生产线代码中使用 unsafe。**

---

## 你会学到什么

1. 原始指针语法
2. unsafe 块的作用
3. 何时使用 unsafe
4. 安全抽象封装

---

## 第一个例子

```rust
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

---

## 常见错误

### 错误 1: 不安全的 UTF-8 转换

```rust
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

---

## 小结

**核心原则**：

1. **unsafe 不是"随便用"**: 只在必要且可控时使用
2. **封装 unsafe**: 提供安全的接口
3. **记录安全契约**: 为什么 unsafe 是安全的
4. **优先安全抽象**: Rust 标准库已经提供了大部分需要的工具

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Raw pointer | 原始指针 |
| Unsafe block | unsafe 块 |
| Undefined behavior | 未定义行为 |

---

> 🔴 **记住**：unsafe 让你对编译器说"我知道我在做什么，相信我"。确保你真的知道！
