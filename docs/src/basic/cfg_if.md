# 条件编译

## 你会学到什么

1. cfg 属性语法
2. 平台特定代码
3. cfg_if 宏
4. 特性标志

---

## 第一个例子

```rust
#[cfg(target_os = "linux")]
fn linux_only() {
    println!("只在 Linux 上运行");
}

#[cfg(target_os = "windows")]
fn windows_only() {
    println!("只在 Windows 上运行");
}

fn main() {
    linux_only();  // Linux 上才会编译
}
```

---

## 常见用法

### 平台特定

```rust
#[cfg(target_os = "macos")]
use std::os::macos::raw;

#[cfg(target_os = "linux")]  
use std::os::linux::raw;
```

### 特性标志

```rust
#[cfg(feature = "logging")]
fn log(message: &str) {
    println!("[LOG] {}", message);
}

#[cfg(not(feature = "logging"))]
fn log(_message: &str) {
    // 空实现
}
```

---

## 小结

**要点**：

1. **cfg 属性**: 条件编译
2. **特性标志**: 可选功能
3. **平台代码**: `target_os`, `target_arch`

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Conditional compilation | 条件编译 |
| Feature flag | 特性标志 |

---

> 💡 **提示**：条件编译让你的代码能在多个平台运行！

---

## 继续学习

**前一章**: [线程与并发](threads.md)  
**下一章**: [指针与不安全代码](pointer.md)

**相关章节**:
- [线程与并发](threads.md)
- [模块系统](module.md)

**返回**: [基础入门](basic-overview.md)
