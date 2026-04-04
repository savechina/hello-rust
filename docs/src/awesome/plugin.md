# 插件系统 (Plugin System)

## 开篇故事

想象你在开发一个图像编辑器。最初只支持 JPEG 和 PNG 格式。但随着用户需求增长，你需要支持 WebP、AVIF、HEIC 等新格式。如果每次都要修改核心代码、重新编译整个应用，开发效率会非常低。

插件系统就像给应用预留了"扩展插槽"——新功能可以作为独立模块插入，无需修改核心代码。在 Rust 中，由于编译时类型安全和无运行时反射的特性，实现插件系统需要特殊的设计模式。本章将介绍 Rust 中实现插件系统的多种方案。

---

## 本章适合谁

如果你想学习：
- Rust 中如何实现可扩展的插件架构
- 编译时插件注册 vs 运行时动态加载的区别
- 如何设计可插拔的服务架构

本章适合你。插件系统是构建可扩展应用的核心技术。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Rust 插件系统的三种主要实现方式
2. 使用 `inventory` crate 实现编译时插件注册
3. 使用 `libloading` crate 实现运行时动态加载
4. 使用 `dlopen` crate 实现 C 兼容的动态库
5. 根据场景选择合适的插件方案
6. 设计可插拔的服务架构

---

## 前置要求

- [特征](../basic/trait.md) - trait 定义和实现
- [依赖注入](dependency_injection.md) - 服务容器模式
- 理解动态库编译（`cdylib`）

---

### 依赖安装

不同插件方案需要不同的依赖：

```bash
# 方案 1: 编译时插件注册
cargo add inventory

# 方案 2: 运行时动态加载
cargo add libloading

# 方案 3: C 兼容动态库
cargo add dlopen
```

---

## 第一个例子

使用 `inventory` crate 实现编译时插件注册：

```rust,ignore
use inventory::submit;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 1. 定义插件 trait
trait InventoryOp: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32);
}

// 2. 定义插件注册结构体
#[derive(Clone, Copy)]
struct InventoryPlugin {
    name: &'static str,
    handler: &'static dyn InventoryOp,
}

// 3. 收集所有提交的插件
inventory::collect!(InventoryPlugin);

// 4. 提交插件（可以在任何文件中）
struct AddItem;
impl InventoryOp for AddItem {
    fn name(&self) -> &'static str { "add" }
    fn execute(&self, inv: &Mutex<HashMap<String, u32>>, item: &str, qty: u32) {
        let mut inv = inv.lock().unwrap();
        let current = inv.get(item).copied().unwrap_or(0);
        inv.insert(item.to_string(), current + qty);
        println!("Added {} {}", qty, item);
    }
}

inventory::submit! {
    InventoryPlugin { name: "add", handler: &AddItem }
}

// 5. 主程序：遍历并执行插件
fn main() {
    let inventory = Arc::new(Mutex::new(HashMap::new()));
    
    // 收集所有注册的插件
    let mut plugins: HashMap<&str, &dyn InventoryOp> = HashMap::new();
    for plugin in inventory::iter::<InventoryPlugin> {
        if !plugin.name.is_empty() {
            plugins.insert(plugin.name, plugin.handler);
        }
    }
    
    // 执行插件
    if let Some(op) = plugins.get("add") {
        op.execute(&inventory, "apple", 5);
    }
}
```

完整示例：[crates/awesome/src/services/inventory_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/inventory_sample.rs)

---

## 原理解析

### Rust 插件系统的三种方案

```
Rust 插件系统
├── 方案 1: 编译时插件注册 (inventory)
│   ├── 原理: 链接时收集所有 submit! 宏提交的插件
│   ├── 优点: 类型安全、零运行时开销、编译时检查
│   ├── 缺点: 需要重新编译、不支持热插拔
│   └── 适用: 编译时扩展、内置插件
│
├── 方案 2: 运行时动态加载 (libloading)
│   ├── 原理: 运行时加载 .so/.dll 文件，查找符号
│   ├── 优点: 热插拔、无需重新编译主程序
│   ├── 缺点: 不安全 (unsafe)、ABI 兼容性问题
│   └── 适用: 第三方插件、热更新
│
└── 方案 3: C 兼容动态库 (dlopen + cdylib)
    ├── 原理: 使用 C ABI 导出函数，动态加载
    ├── 优点: 跨语言兼容、标准方式
    ├── 缺点: 需要手动管理内存、unsafe
    └── 适用: 跨语言插件、C/C++ 兼容
```

### 方案 1: 编译时插件注册 (inventory)

**原理**：`inventory` crate 利用 Rust 的链接器特性，在编译时收集所有通过 `submit!` 宏提交的插件。

**核心概念**：

```
┌─────────────────────────────────────────────────────────────┐
│                    编译时插件注册流程                         │
│                                                              │
│  main.rs              plugin_add.rs          plugin_rm.rs   │
│  ┌─────────┐          ┌───────────┐          ┌───────────┐  │
│  │collect! │          │submit!    │          │submit!    │  │
│  │         │          │           │          │           │  │
│  │ 收集所有 │◄─────────┤ 注册 add  │          │ 注册 remove│  │
│  │ 插件    │          │ 插件      │          │ 插件      │  │
│  │         │          │           │          │           │  │
│  └────┬────┘          └───────────┘          └───────────┘  │
│       │                                                       │
│       ▼                                                       │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  运行时: inventory::iter::<Plugin> 遍历所有已注册插件     │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

**完整示例**（来自项目代码）：

```rust,ignore
use inventory::submit;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 定义插件 trait
trait InventoryOp: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32);
}

// 定义插件注册结构体
#[derive(Clone, Copy)]
struct InventoryPlugin {
    name: &'static str,
    handler: &'static dyn InventoryOp,
}

// 收集所有提交的插件
inventory::collect!(InventoryPlugin);

// 插件 1: 添加物品
struct AddItem;
impl InventoryOp for AddItem {
    fn name(&self) -> &'static str { "add" }
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32) {
        let mut inv = inventory.lock().unwrap();
        let current = inv.get(item).copied().unwrap_or(0);
        inv.insert(item.to_string(), current + quantity);
        println!("Added {} {} to inventory", quantity, item);
    }
}

inventory::submit! {
    InventoryPlugin { name: "add", handler: &AddItem }
}

// 插件 2: 移除物品
struct RemoveItem;
impl InventoryOp for RemoveItem {
    fn name(&self) -> &'static str { "remove" }
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32) {
        let mut inv = inventory.lock().unwrap();
        if let Some(current) = inv.get_mut(item) {
            if *current >= quantity {
                *current -= quantity;
                println!("Removed {} {} from inventory", quantity, item);
            } else {
                println!("Error: Not enough {} in inventory", item);
            }
        } else {
            println!("Error: {} not found in inventory", item);
        }
    }
}

inventory::submit! {
    InventoryPlugin { name: "remove", handler: &RemoveItem }
}

fn inventory_main() {
    let inventory = Arc::new(Mutex::new(HashMap::new()));
    inventory.lock().unwrap().insert("apple".to_string(), 10);
    
    // 收集所有插件
    let mut plugins: HashMap<&str, &dyn InventoryOp> = HashMap::new();
    for plugin in inventory::iter::<InventoryPlugin> {
        if !plugin.name.is_empty() {
            plugins.insert(plugin.name, plugin.handler);
        }
    }
    
    // 执行操作
    let ops = vec![
        ("add", "apple", 5),
        ("remove", "apple", 3),
        ("add", "sword", 2),
    ];
    
    for (op_name, item, quantity) in ops {
        if let Some(op) = plugins.get(op_name) {
            op.execute(&inventory, item, quantity);
        }
    }
}
```

### 方案 2: 运行时动态加载 (libloading)

**原理**：运行时加载 `.so`（Linux）、`.dylib`（macOS）、`.dll`（Windows）文件，通过符号名查找函数。

```rust,ignore
use libloading::{Library, Symbol};

// 定义插件函数签名
type PluginInit = unsafe extern "C" fn() -> *mut std::os::raw::c_void;
type PluginExecute = unsafe extern "C" fn(*mut std::os::raw::c_void);

fn load_plugin(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // 加载动态库
        let lib = Library::new(path)?;
        
        // 查找符号
        let init: Symbol<PluginInit> = lib.get(b"plugin_init")?;
        let execute: Symbol<PluginExecute> = lib.get(b"plugin_execute")?;
        
        // 初始化插件
        let handle = init();
        
        // 执行插件
        execute(handle);
        
        Ok(())
    }
}
```

**插件端代码**（需要编译为 `cdylib`）：

```rust,ignore
// Cargo.toml: [lib] crate-type = ["cdylib"]

#[no_mangle]
pub extern "C" fn plugin_init() -> *mut std::os::raw::c_void {
    println!("Plugin initialized");
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn plugin_execute(_handle: *mut std::os::raw::c_void) {
    println!("Plugin executed");
}
```

### 方案 3: C 兼容动态库 (dlopen)

**原理**：使用 `dlopen` 加载 C 兼容的动态库，适合跨语言插件。

```rust,ignore
use dlopen::symbor::Library;
use dlopen_derive::StructSymbols;

#[derive(StructSymbols)]
struct PluginApi {
    init: extern "C" fn(),
    execute: extern "C" fn(),
}

fn load_plugin_dlopen(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lib = Library::open(path)?;
    let api: PluginApi = unsafe { lib.symbols() }?;
    
    (api.init)();
    (api.execute)();
    
    Ok(())
}
```

### Rust 插件系统设计思想

**1. 编译时 vs 运行时**

| 特性 | 编译时 (inventory) | 运行时 (libloading) |
|------|-------------------|---------------------|
| 类型安全 | ✅ 完全安全 | ❌ 需要 unsafe |
| 热插拔 | ❌ 需要重新编译 | ✅ 支持 |
| 性能 | ✅ 零开销 | ⚠️ 动态查找 |
| 跨语言 | ❌ 仅 Rust | ✅ C ABI 兼容 |
| 调试 | ✅ 容易 | ⚠️ 困难 |

**2. 插件注册模式**

```
┌─────────────────────────────────────────────────────────────┐
│                        插件注册模式                          │
│                                                              │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐               │
│  │ Plugin A │    │ Plugin B │    │ Plugin C │               │
│  └────┬─────┘    └────┬─────┘    └────┬─────┘               │
│       │               │               │                      │
│       ▼               ▼               ▼                      │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │                    Plugin Registry                       │ │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐                 │ │
│  │  │ Plugin A│  │ Plugin B│  │ Plugin C│                 │ │
│  │  └─────────┘  └─────────┘  └─────────┘                 │ │
│  └────────────────────────┬────────────────────────────────┘ │
│                           │                                  │
│                           ▼                                  │
│                  ┌─────────────────┐                         │
│                  │   Host App      │                         │
│                  │  (主应用程序)    │                         │
│                  └─────────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

**3. 插件生命周期**

```
加载 → 初始化 → 注册 → 执行 → 卸载
  │        │        │        │        │
  ▼        ▼        ▼        ▼        ▼
load    init    register  execute  unload
```

---

## Rust 插件框架生态

| 框架 | 类型 | 特点 | 适用场景 |
|------|------|------|---------|
| **inventory** | 编译时 | 类型安全、零开销 | 内置插件、编译时扩展 |
| **extism** | 运行时 | WASM 插件、沙箱安全 | 第三方插件、安全隔离 |
| **libloading** | 运行时 | 原生动态库 | 热插拔、C 兼容 |
| **dlopen** | 运行时 | C ABI 兼容 | 跨语言插件 |
| **pluginator** | 编译时 | 宏驱动 | 简单插件系统 |
| **wasmer** | 运行时 | WASM 运行时 | 安全沙箱插件 |

### Extism (WASM 插件)

Extism 是目前 Rust 生态最流行的 WASM 插件框架：

```rust,ignore
use extism::{Manifest, Plugin, Wasm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 WASM 插件
    let manifest = Manifest::new([Wasm::file("plugin.wasm")]);
    let mut plugin = Plugin::new(&manifest, [], true)?;
    
    // 调用插件函数
    let res = plugin.call("greet", "World")?;
    println!("Plugin response: {}", res);
    
    Ok(())
}
```

**优势**：
- 沙箱安全（WASM 隔离）
- 跨语言（任何能编译 WASM 的语言）
- 热插拔
- 资源限制

---

## 常见错误

### 错误 1: inventory 插件未注册

```rust,ignore
// ❌ 错误：忘记调用 collect!
// inventory::collect!(InventoryPlugin);  // 缺少这行！

// ✅ 正确：必须先 collect
inventory::collect!(InventoryPlugin);
```

### 错误 2: libloading 符号查找失败

```rust,ignore
// ❌ 错误：符号名不匹配
let func: Symbol<FnType> = lib.get(b"wrong_name")?;

// ✅ 正确：使用 #[no_mangle] 确保符号名一致
#[no_mangle]
pub extern "C" fn plugin_init() { ... }
```

### 错误 3: 动态库 ABI 不兼容

```rust,ignore
// ❌ 错误：使用 Rust ABI（不稳定）
pub fn plugin_init() { ... }

// ✅ 正确：使用 C ABI
#[no_mangle]
pub extern "C" fn plugin_init() { ... }
```

---

## 动手练习

### 练习 1: 添加新插件

在 inventory 示例中添加一个 "clear" 操作插件，清空所有物品：

```rust,ignore
// TODO: 实现 ClearAll 插件
// 1. 定义 ClearAll 结构体
// 2. 实现 InventoryOp trait
// 3. 使用 inventory::submit! 注册
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
struct ClearAll;
impl InventoryOp for ClearAll {
    fn name(&self) -> &'static str { "clear" }
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, _item: &str, _quantity: u32) {
        let mut inv = inventory.lock().unwrap();
        inv.clear();
        println!("Cleared all items from inventory");
    }
}

inventory::submit! {
    InventoryPlugin { name: "clear", handler: &ClearAll }
}
```

</details>

### 练习 2: 设计插件接口

为一个日志系统设计插件接口，支持不同的日志输出方式（控制台、文件、网络）：

```rust,ignore
// TODO: 定义 LoggerPlugin trait
// TODO: 定义插件注册结构体
// TODO: 实现 ConsoleLogger 插件
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use inventory::submit;

trait LoggerPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn log(&self, level: &str, message: &str);
}

#[derive(Clone, Copy)]
struct LoggerRegistry {
    name: &'static str,
    handler: &'static dyn LoggerPlugin,
}

inventory::collect!(LoggerRegistry);

struct ConsoleLogger;
impl LoggerPlugin for ConsoleLogger {
    fn name(&self) -> &'static str { "console" }
    fn log(&self, level: &str, message: &str) {
        println!("[{}] {}", level, message);
    }
}

inventory::submit! {
    LoggerRegistry { name: "console", handler: &ConsoleLogger }
}
```

</details>

---

## 小结

**核心要点**：

1. **编译时插件** (inventory) - 类型安全、零开销，适合内置扩展
2. **运行时插件** (libloading) - 热插拔、灵活，需要 unsafe
3. **WASM 插件** (extism) - 沙箱安全、跨语言，推荐用于第三方插件
4. **设计原则** - 定义清晰的 trait 接口、使用注册模式管理插件

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| Plugin | 插件 | 可扩展的功能模块 |
| Registry | 注册表 | 管理所有已注册插件 |
| Hot Reload | 热重载 | 运行时加载/卸载插件 |
| WASM | WebAssembly | 安全的沙箱执行环境 |
| ABI | 应用二进制接口 | 跨语言兼容的接口规范 |
| cdylib | C 动态库 | 编译为 C 兼容的动态库 |

**下一步**：

- 学习 [依赖注入](dependency_injection.md) - 服务容器模式
- 探索 [Extism](https://extism.org/) - WASM 插件框架
- 了解 [服务框架](services.md) - 生产级服务架构

---

## 术语表

| English | 中文 |
|---------|------|
| Plugin System | 插件系统 |
| Registry | 注册表 |
| Hot Reload | 热重载 |
| Dynamic Library | 动态库 |
| Symbol | 符号 |
| ABI | 应用二进制接口 |
| WASM | WebAssembly |
| Sandbox | 沙箱 |
| cdylib | C 动态库 |
| rlib | Rust 静态库 |

完整示例：[crates/awesome/src/services/inventory_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/inventory_sample.rs)

---

## 继续学习

- 上一步：[服务依赖注入](dependency_injection.md) - 服务容器模式
- 下一步：[消息队列](mq.md) - 异步通信
- 回顾：[特征](../basic/trait.md) - trait 基础

> 💡 **记住**：Rust 的插件系统设计核心是"编译时安全 + 运行时灵活"。根据需求选择 inventory（编译时）、libloading（运行时）、或 extism（WASM 沙箱）！
