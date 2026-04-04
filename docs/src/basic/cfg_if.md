# 条件编译

## 开篇故事

想象你在组装一台电脑。不同地区需要不同的电源插头：美国用 110V 两脚插头，欧洲用 220V 圆脚插头。你不会为每个地区生产不同型号的电脑，而是设计一个通用主板，根据目标地区安装不同的电源模块。条件编译就是 Rust 的"电源适配器"——**同一份代码，根据不同平台编译出不同的程序**。

---

## 本章适合谁

如果你需要编写跨平台代码（Windows/macOS/Linux），或者想实现可选功能（如日志、调试模式），本章适合你。条件编译是系统级编程的必备技能。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `#[cfg]` 属性控制编译
2. 编写平台特定代码
3. 使用 `cfg_if` 宏简化条件编译
4. 定义和使用特性标志（feature flags）
5. 理解编译时 vs 运行时的区别

---

## 前置要求

- [模块系统](module.md) - 理解代码组织
- [宏编程](../advance/testing/macros.md) - 理解宏基础

---

## 第一个例子

```rust
#[cfg(target_os = "linux")]
fn get_platform_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
fn get_platform_name() -> &'static str {
    "macOS"
}

#[cfg(target_os = "windows")]
fn get_platform_name() -> &'static str {
    "Windows"
}

fn main() {
    println!("当前平台：{}", get_platform_name());
}
```

**发生了什么？**

- `#[cfg(...)]` - 条件编译属性
- 只有匹配的平台代码会被编译
- 其他平台代码完全不存在（零开销）

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念       | Python               | Java                 | C++                    | Rust                       | 关键差异                  |
| ---------- | -------------------- | -------------------- | ---------------------- | -------------------------- | ------------------------- |
| 条件编译   | 无（动态类型）       | 无                   | `#ifdef` / `#if`       | `#[cfg(...)]`              | Rust 用属性声明           |
| 平台判断   | 运行时检查           | 运行时检查           | `#ifdef _WIN32`        | `#[cfg(target_os = "linux")]` | Rust 编译时决定           |
| 编译时判断 | 不支持               | 不支持               | 预处理器               | 编译器                     | Rust 无预处理器           |
| 特性标志   | 无                   | 无                   | 无                     | `feature = "logging"`      | Cargo.toml 定义           |
| 代码消除   | 无                   | 无                   | 部分消除               | 完全消除                   | Rust 零运行时开销         |

**核心差异**: Python/Java 无条件编译，C++ 用预处理器，Rust 用编译器属性且完全消除不匹配代码。

---

## 原理解析

### 1. cfg 属性语法

```rust
// 单个条件
#[cfg(target_os = "linux")]
fn linux_only() {}

// 多个条件（AND）
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn linux_x86_64() {}

// 多个条件（OR）
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_like() {}

// 取反
#[cfg(not(debug_assertions))]
fn release_mode() {}
```

### 2. 常用条件变量

| 变量            | 说明       | 示例值                               |
| --------------- | ---------- | ------------------------------------ |
| `target_os`     | 操作系统   | "linux", "macos", "windows", "android" |
| `target_arch`   | CPU 架构   | "x86_64", "aarch64", "arm"             |
| `target_family` | 平台家族   | "unix", "windows"                      |
| `debug_assertions` | 调试模式 | 仅在 `cargo build` 时为 true           |
| `feature`       | 特性标志   | `feature = "logging"`                  |

### 3. cfg_if 宏

```rust
// 使用 cfg_if crate
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        use std::os::linux::raw;
    } else if #[cfg(target_os = "macos")] {
        use std::os::macos::raw;
    } else if #[cfg(target_os = "windows")] {
        use windows_sys::Win32::Foundation;
    } else {
        compile_error!("不支持的平台");
    }
}
```

**优势**：
- 避免重复 `#[cfg]` 属性
- 更清晰的分支结构
- 支持 `else` 和 `compile_error!`

### 4. 特性标志（Feature Flags）

**Cargo.toml**:
```toml
[features]
default = ["logging"]
logging = []
debug_mode = []
```

**代码中使用**:
```rust
#[cfg(feature = "logging")]
fn log(message: &str) {
    println!("[LOG] {}", message);
}

#[cfg(not(feature = "logging"))]
fn log(_message: &str) {
    // 空实现，零开销
}
```

**编译时启用**:
```bash
# 启用 logging 特性
cargo build --features logging

# 禁用默认特性
cargo build --no-default-features

# 组合使用
cargo build --no-default-features --features debug_mode
```

---

## 常见错误

### 错误 1: cfg 语法错误

```rust
// ❌ 错误：缺少引号
#[cfg(target_os = linux)]
fn foo() {}

// ✅ 正确：字符串值需要引号
#[cfg(target_os = "linux")]
fn foo() {}
```

### 错误 2: 函数签名不匹配

```rust
// ❌ 错误：不同平台函数签名不同
#[cfg(target_os = "linux")]
fn get_path() -> String { "/home".to_string() }

#[cfg(target_os = "windows")]
fn get_path() -> &str { "C:\\" }  // 返回类型不同！

// ✅ 正确：保持相同签名
#[cfg(target_os = "linux")]
fn get_path() -> &'static str { "/home" }

#[cfg(target_os = "windows")]
fn get_path() -> &'static str { "C:\\" }
```

### 错误 3: 忘记处理所有情况

```rust
// ❌ 错误：只处理了 Linux
#[cfg(target_os = "linux")]
fn init() { /* ... */ }

fn main() {
    init();  // 在非 Linux 平台编译失败！
}

// ✅ 正确：提供默认实现
#[cfg(target_os = "linux")]
fn init() { /* Linux 特定初始化 */ }

#[cfg(not(target_os = "linux"))]
fn init() { /* 通用初始化 */ }
```

---

## 动手练习

### 练习 1: 平台信息打印

编写程序打印当前平台信息：

```rust
// TODO: 实现 get_platform_info() 函数
// 返回格式："OS: xxx, Arch: xxx"
```

<details>
<summary>点击查看答案</summary>

```rust
fn get_platform_info() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            let os = "Linux";
        } else if #[cfg(target_os = "macos")] {
            let os = "macOS";
        } else if #[cfg(target_os = "windows")] {
            let os = "Windows";
        } else {
            let os = "Unknown";
        }
    }
    
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            let arch = "x86_64";
        } else if #[cfg(target_arch = "aarch64")] {
            let arch = "ARM64";
        } else {
            let arch = "Unknown";
        }
    }
    
    // 简化版本
    #[cfg(target_os = "linux")]
    return "OS: Linux, Arch: x86_64";
    
    #[cfg(target_os = "macos")]
    return "OS: macOS, Arch: x86_64";
    
    #[cfg(target_os = "windows")]
    return "OS: Windows, Arch: x86_64";
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return "OS: Unknown, Arch: Unknown";
}
```
</details>

### 练习 2: 调试模式日志

实现只在调试模式下打印日志的功能：

```rust
// TODO: 实现 debug_log() 宏
// 在 debug_assertions 为 true 时打印日志，否则什么都不做
```

<details>
<summary>点击查看答案</summary>

```rust
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
        
        #[cfg(not(debug_assertions))]
        let _ = format!($($arg)*);  // 计算但不输出
    };
}

fn main() {
    debug_log!("用户登录：{}", "alice");
    debug_log!("数据库连接成功");
}
```
</details>

---

## 故障排查

### Q: cfg 和 if 有什么区别？

**A**: 
- `cfg` 是**编译时**决定：不匹配的代码根本不存在
- `if` 是**运行时**决定：所有代码都编译，运行时选择分支

```rust
// 编译时：不匹配的代码不会被编译
#[cfg(target_os = "linux")]
fn linux_only() {}

// 运行时：所有代码都编译，运行时判断
fn runtime_check(is_linux: bool) {
    if is_linux {
        // ...
    }
}
```

### Q: 如何查看 cfg 展开后的代码？

**A**: 使用 `cargo rustc -- --emit=mir` 或 `cargo expand`

### Q: 特性标志和环境变量有什么区别？

**A**:
- **特性标志**: 编译时决定，影响编译结果
- **环境变量**: 运行时读取，不影响编译

---

## 知识扩展（选学）

### 条件编译的实际应用

**Tokio 异步运行时**：
```rust
// 只在 Unix 系统上支持信号
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

// 只在 Windows 上支持控制台事件
#[cfg(windows)]
use tokio::signal::windows;
```

**跨平台路径处理**：
```rust
#[cfg(unix)]
const PATH_SEPARATOR: &str = "/";

#[cfg(windows)]
const PATH_SEPARATOR: &str = "\\";
```

---

## 小结

**核心要点**：

1. **cfg 属性**: `#[cfg(...)]` 控制编译
2. **特性标志**: 可选功能，零开销
3. **平台代码**: `target_os`, `target_arch`
4. **cfg_if 宏**: 简化条件编译

**关键术语**：

- **Conditional Compilation**: 条件编译
- **Feature Flag**: 特性标志
- **Compile-time Decision**: 编译时决定
- **Zero-cost Abstraction**: 零开销抽象

---

## 术语表

| English          | 中文       |
| ---------------- | ---------- |
| Conditional compilation | 条件编译   |
| Feature flag     | 特性标志   |
| Platform-specific | 平台特定   |
| Compile-time     | 编译时     |
| Runtime          | 运行时     |
| Attribute        | 属性       |

---

完整示例：`src/basic/cfg_if_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[cfg]` 和 `#[cfg_attr]` 有什么区别？

2. 如何在编译时定义自定义特性标志？

3. `cfg_if!` 宏相比多个 `#[cfg]` 有什么优势？

<details>
<summary>点击查看答案与解析</summary>

1. `#[cfg]` 条件编译代码，`#[cfg_attr]` 条件添加属性
2. `cargo rustc --cfg my_feature` 或在 Cargo.toml 的 `[features]` 中定义
3. `cfg_if!` 更清晰，支持 `else` 分支，避免重复 `#[cfg]`

**关键理解**: 条件编译是编译时决定，零运行时开销。
</details>

## 继续学习

- 下一步：[指针与不安全代码](pointer.md)
- 进阶：[宏编程](../advance/testing/macros.md)
- 回顾：[模块系统](module.md)

> 💡 **记住**：条件编译让你的代码跨平台运行，但要保持函数签名一致！
