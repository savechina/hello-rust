# 模块 (Modules)

## 开篇故事

想象你有一个大型工具箱。如果所有工具（螺丝刀、锤子、扳手）都堆在一起，很难找到需要的工具。你会把它们分类放在不同的隔间里。Rust 的**模块**就是这样 - 它帮你组织代码，让大型项目保持清晰。

---

## 本章适合谁

如果你已经能写 Rust 代码，现在想学习如何组织大型项目，本章适合你。模块是 Rust 项目结构的基础。

---

## 你会学到什么

1. 创建和使用模块
2. 理解 pub 可见性
3. 使用 use 导入
4. 组织多文件项目
5. 理解模块路径

---

## 前置要求

- [结构体](struct.md)
- [特征](trait.md)

---

## 第一个例子

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
    }
}
```

---

## 原理解析

### 1. 可见性

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,  // 公有字段
        seasonal_fruit: String,  // 私有字段
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

### 2. use 导入

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

### 3. 路径

```rust
// 绝对路径
let x = std::collections::HashMap::new();

// 相对路径
use crate::front_of_house::hosting;

// 使用 super
use super::front_of_house;
```

### 4. 重命名

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // 避免冲突
```

---

## 常见错误

### 错误 1: 访问私有模块

```rust
mod restaurant {
    mod kitchen {  // 私有
        pub fn cook() {}
    }
}

fn main() {
    // ❌ 无法访问私有模块
    restaurant::kitchen::cook();
}
```

### 错误 2: 忘记导入

```rust
fn main() {
    HashMap::new();  // ❌ 未导入
}
```

### 错误 3: 循环依赖

```rust
mod a {
    use crate::b;  // ❌ 循环
}

mod b {
    use crate::a;
}
```

---

## 动手练习

### 练习 1: 创建模块

```rust
// TODO: 创建 garden 模块，包含 Tree 和 Vegetable
```

<details>
<summary>点击查看答案</summary>

```rust
mod garden {
    pub struct Tree;
    pub struct Vegetable;
}
```
</details>

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

| English | 中文 |
| ------- | ---- |
| Module | 模块 |
| Visibility | 可见性 |
| Path | 路径 |

---

> 💡 **提示**：好的模块结构让代码像好文章一样易读！
