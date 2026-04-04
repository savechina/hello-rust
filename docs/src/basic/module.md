# 模块系统

## 开篇故事

想象你在经营一家大型超市。如果所有商品（食品、日用品、电器）都堆在一个大房间里，顾客会疯掉，员工也找不到东西。你需要把商品分类放在不同的区域：食品区、日用品区、电器区。每个区域有自己的入口，有些区域对所有顾客开放，有些区域（如仓库）只允许员工进入。

Rust 的**模块系统**就是超市的分区系统——它帮你组织代码，控制访问权限，让大型项目保持清晰和可维护。

---

## 本章适合谁

如果你正在编写超过 500 行的 Rust 项目，或者想学习如何组织多文件项目，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `mod` 创建模块层次结构
2. 使用 `pub` 控制可见性
3. 使用 `use` 简化路径
4. 组织多文件项目结构
5. 理解 `pub(crate)` 和 `pub(super)`
6. 避免循环依赖

---

## 前置要求

- [结构体](struct.md) - 数据类型
- [特征](trait.md) - 接口定义

---

## 第一个例子

```rust,ignore
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
    }
}

fn main() {
    // 调用模块内函数
    front_of_house::hosting::add_to_waitlist();
}
```

**发生了什么？**

- `mod` - 定义模块
- `front_of_house::hosting` - 模块路径
- 默认情况下，模块是**私有**的

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念         | Python               | Java                 | C++                    | Rust                       | 关键差异                  |
| ------------ | -------------------- | -------------------- | ---------------------- | -------------------------- | ------------------------- |
| 模块定义     | 文件即模块           | `package` + 文件夹   | `namespace`            | `mod` 关键字               | Rust 需显式声明 `mod`     |
| 可见性       | 无强制控制           | `public/private`     | 无强制控制             | `pub` + 默认私有           | Rust 默认私有             |
| 导入语法     | `import module`      | `import pkg.Class`   | `using namespace`      | `use crate::module`        | Rust 用 `use` + 路径      |
| 嵌套模块     | `import pkg.sub`     | `package pkg.sub`    | `namespace ns::sub`    | `mod outer { mod inner }`  | Rust 用树形结构           |
| 文件映射     | 自动                 | 文件夹映射包         | 无映射                 | `mod.rs` 或同名文件        | Rust 有两种风格           |

**核心差异**: Python 最简单（文件即模块），Java 用 package + class，Rust 用 `mod` + 文件映射且默认私有。

---

## 原理解析

### 1. 模块树形结构

```
crate (根)
├── front_of_house
│   ├── hosting
│   │   ├── add_to_waitlist()
│   │   └── seat_at_table()
│   └── serving
│       └── take_order()
└── back_of_house
    ├── Breakfast (pub struct)
    └── Appetizer (pub enum)
```

### 2. 可见性规则

```rust,ignore
mod restaurant {
    // 私有模块（默认）
    mod kitchen {
        pub fn cook() {}  // 即使函数是 pub，模块私有也无法从外部访问
    }
    
    // 公有模块
    pub mod dining_area {
        pub fn seat_customer() {}  // 可以从外部访问
    }
}

fn main() {
    // ❌ 错误：kitchen 是私有模块
    // restaurant::kitchen::cook();
    
    // ✅ 正确：dining_area 是公有模块
    restaurant::dining_area::seat_customers();
}
```

### 3. pub 修饰符详解

```rust,ignore
pub mod public_module {        // 公开模块
    pub fn public_fn() {}      // 公开函数
    fn private_fn() {}         // 私有函数
    
    pub struct PublicStruct {  // 公开结构体
        pub field1: i32,       // 公开字段
        field2: String,        // 私有字段
    }
    
    pub enum PublicEnum {      // 公开枚举
        Variant1,              // 枚举变体总是公开的
        Variant2,
    }
}

// 受限可见性
pub(crate) mod crate_public {  // 只在 crate 内可见
    pub fn internal_api() {}
}

pub(super) mod parent_public {  // 只在父模块可见
    pub fn helper() {}
}

pub(in crate::my_module) mod restricted {  // 在特定路径可见
    pub fn limited_api() {}
}
```

### 4. use 导入

```rust,ignore
// 绝对路径
use std::collections::HashMap;

// 相对路径
use crate::front_of_house::hosting;

// 使用 super 访问父模块
mod parent {
    mod child {
        use super::sibling_function;
    }
}

// 重命名避免冲突
use std::fmt::Result;
use std::io::Result as IoResult;

// 导入多个
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

// 重新导出（pub use）
pub use crate::internal_module::PublicApi;
```

### 5. 多文件项目组织

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs           // 入口点
│   ├── lib.rs            // 库根（如果有）
│   ├── models/           // 数据模型模块
│   │   ├── mod.rs        // 模块声明
│   │   ├── user.rs       // 用户模型
│   │   └── post.rs       // 帖子模型
│   ├── services/         // 业务逻辑模块
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   └── post_service.rs
│   └── utils/            // 工具函数模块
│       ├── mod.rs
│       └── helpers.rs
```

**src/models/mod.rs**:
```rust,ignore
pub mod user;
pub mod post;
```

**src/models/user.rs**:
```rust,ignore
pub struct User {
    pub id: i32,
    pub name: String,
}
```

### 6. 模块最佳实践

```rust,ignore
// ✅ 好的模块设计
pub mod api {
    pub mod v1 {
        pub mod users {
            pub fn list() -> Vec<User> { vec![] }
        }
    }
}

// ✅ 使用 pub use 简化 API
pub mod prelude {
    pub use super::api::v1::users::list;
    pub use super::models::User;
}

// 用户现在可以这样使用：
// use my_crate::prelude::*;
```

---

## 常见错误

### 错误 1: 访问私有模块

```rust,ignore
mod restaurant {
    mod kitchen {  // 私有
        pub fn cook() {}
    }
}

fn main() {
    // ❌ 无法访问私有模块
    // restaurant::kitchen::cook();
    
    // ✅ 正确：将模块设为 pub
    // pub mod kitchen { ... }
}
```

### 错误 2: 忘记导入

```rust,ignore
fn main() {
    // ❌ 错误：未导入
    // let map = HashMap::new();
    
    // ✅ 正确：先导入
    use std::collections::HashMap;
    let map = HashMap::new();
}
```

### 错误 3: 循环依赖

```rust,ignore
// ❌ 错误：模块 A 依赖 B，B 依赖 A
mod a {
    use crate::b::function_b;  // 循环！
}

mod b {
    use crate::a::function_a;  // 循环！
}

// ✅ 正确：重构为单向依赖
mod common {
    pub fn shared_logic() {}
}

mod a {
    use crate::common::shared_logic;
}

mod b {
    use crate::common::shared_logic;
}
```

---

## 动手练习

### 练习 1: 创建花园模块

```rust,ignore
// TODO: 创建 garden 模块
// - 包含 Tree 和 Vegetable 结构体
// - Tree 是公开的，Vegetable 是私有的
// - 提供 public_api() 函数返回 Tree
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
pub mod garden {
    pub struct Tree {
        pub name: String,
        height: f32,
    }
    
    struct Vegetable {
        name: String,
    }
    
    pub fn public_api() -> Tree {
        Tree {
            name: String::from("Oak"),
            height: 10.0,
        }
    }
}
```
</details>

### 练习 2: 组织多文件项目

```rust,ignore
// TODO: 设计以下项目结构
// library/
// ├── src/
// │   ├── lib.rs
// │   ├── models/
// │   │   ├── mod.rs
// │   │   ├── book.rs
// │   │   └── member.rs
// │   └── services/
// │       ├── mod.rs
// │       └── catalog.rs
```

---

## 故障排查

### Q: mod.rs 和同名文件有什么区别？

**A**: 
- `mod.rs` - 旧风格（Rust 2015），模块内容在 `mod.rs` 中
- `module_name.rs` - 新风格（Rust 2018+），模块内容在 `module_name.rs` 中
- 推荐使用新风格

### Q: 如何在测试中访问私有模块？

**A**: 使用 `#[cfg(test)]` 模块：
```rust,ignore
mod private_module {
    fn secret_function() -> i32 { 42 }
}

#[cfg(test)]
mod tests {
    use super::private_module::secret_function;
    
    #[test]
    fn test_secret() {
        assert_eq!(secret_function(), 42);
    }
}
```

### Q: pub(crate) 和 pub 有什么区别？

**A**: 
- `pub` - 对所有 crate 可见
- `pub(crate)` - 只对当前 crate 可见（内部 API）

---

## 知识扩展（选学）

### 模块与文件系统映射

Rust 2018+ 的模块解析规则：

```rust,ignore
// src/lib.rs
mod models;  // 查找 src/models.rs 或 src/models/mod.rs

// src/models.rs
pub mod user;  // 查找 src/models/user.rs
```

---

## 小结

**要点**：

1. **mod 定义模块**: 组织代码结构
2. **pub 控制可见**: 默认私有
3. **use 导入名称**: 简化路径
4. **路径分层**: `crate::module::item`
5. **文件分离**: 大模块放独立文件

---

## 术语表

| English       | 中文       |
| ------------- | ---------- |
| Module        | 模块       |
| Visibility    | 可见性     |
| Path          | 路径       |
| Crate         | 包         |
| Re-export     | 重新导出   |
| Prelude       | 预导入模块 |

---

完整示例：`src/basic/module_sample.rs`

---

## 延伸阅读

学习完模块系统后，你可能还想了解：

- [use 关键字深入](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html) - 重导出和私有性
- [crate 组织](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) - 包与 crate
- [工作空间](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) - 多 crate 项目

**选择建议**:
- 想学习并发 → 继续学习 [线程与并发](threads.md)
- 想学习条件编译 → 跳到 [条件编译](cfg_if.md)

## 知识检查

**快速测验**（答案在下方）：

1. `mod` 和 `use` 的区别是什么？

2. 如何使模块公开？

3. `pub(crate)` 的作用是什么？

<details>
<summary>点击查看答案与解析</summary>

1. `mod` 定义模块，`use` 引入名称到当前作用域
2. 在模块前加 `pub`：`pub mod my_module`
3. 只在当前 crate 内可见，对外部 crate 私有

**关键理解**: 模块系统帮助组织大型项目和控制可见性。
</details>

## 继续学习

- 下一步：[线程与并发](threads.md)
- 进阶：[可见性控制](visiable.md)
- 回顾：[特征](trait.md)

> 💡 **提示**：好的模块结构让代码像好文章一样易读！
