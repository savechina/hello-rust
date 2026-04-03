# 可见性 (Visibility)

## 开篇故事

想象你在一个公司工作。有些信息是公开的（公司公告板），有些是部门内部的（部门会议），有些是私密的（HR 档案）。Rust 的**可见性**控制就像公司的信息分级 - 它决定哪些代码和数据可以被谁访问。

---

## 本章适合谁

如果你已经学完了结构体和模块，现在想学习如何控制代码的访问权限，本章适合你。可见性是封装和信息隐藏的基础。

---

## 你会学到什么

1. pub 关键字的使用
2. 结构体字段可见性
3. 模块间可见性
4. 私有构造器模式
5. 封装最佳实践

---

## 前置要求

- [模块](module.md) - 模块组织
- [结构体](struct.md) - 结构体定义

---

## 第一个例子

```rust
// src/basic/visiable_sample.rs

// 私有的结构体，带有一个公有的字段
mod visiable_sample {
    // 公有的结构体，带有一个公有的字段
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 公有的结构体，带有一个私有的字段
    pub struct ClosedBox<T> {
        contents: T,  // 私有！
    }

    impl<T> ClosedBox<T> {
        // 公有的构造器
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }

        // 私有的 get 方法
        #[allow(dead_code)]
        fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}

fn main() {
    // 公有字段可以自由访问
    let open_box = visiable_sample::OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    // 私有字段必须通过公有方法访问
    let closed_box = visiable_sample::ClosedBox::new("secret data");
    // ❌ println!("Closed: {}", closed_box.contents);  // 编译错误！
}
```

**输出**：
```
The open box contains: public information
```

---

## 原理解析

### 1. 可见性级别

Rust 有 3 种主要可见性：

```rust
// 私有（默认）
struct PrivateStruct;  // 只在当前模块内可用

// 公有
pub struct PublicStruct;  // 任何地方都可用

// 限定公有
pub(crate) struct CratePublic;  // 只在当前 crate 内公有
pub(super) struct ParentPublic;  // 只在父模块内公有
```

### 2. 结构体字段可见性

```rust
pub struct User {
    pub name: String,      // 公有字段
    pub(crate) email: String,  // crate 内公有
    age: u32,              // 私有字段
}
```

**规则**：
- 如果结构体是公有的，字段也必须标注可见性
- 私有字段只能在定义它的模块内访问

### 3. 私有构造器模式

```rust
pub struct Database {
    connection: String,  // 私有
}

impl Database {
    // 通过公有方法控制创建
    pub fn connect(url: &str) -> Result<Database, Error> {
        // 可以在这里验证 URL、处理错误
        Ok(Database {
            connection: url.to_string()
        })
    }
}

// 使用：
// let db = Database { connection: "..." };  // ❌ 字段私有
let db = Database::connect("postgres://localhost");  // ✅
```

---

## 常见错误

### 错误 1: 结构体公有但字段私有

```rust
pub struct Point {
    x: i32,  // ❌ 私有字段
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };  // ❌ 无法访问私有字段
}
```

**修复**：

```rust
pub struct Point {
    pub x: i32,  // ✅
    pub y: i32,
}
```

### 错误 2: 忘记 pub

```rust
mod my_module {
    fn public_function() {}  // ❌ 实际是私有的
}
```

**修复**：

```rust
mod my_module {
    pub fn public_function() {}  // ✅
}
```

### 错误 3: 过度使用 pub

```rust
// ❌ 所有字段都公有，破坏了封装
pub struct User {
    pub password: String,  // 不应该公开！
    pub internal_id: u64,  // 内部实现细节
}
```

**修复**：

```rust
pub struct User {
    username: String,    // ✅ 私有
    password: String,    // ✅ 私有
}

impl User {
    pub fn get_username(&self) -> &str {  // ✅ 公有访问器
        &self.username
    }
}
```

---

## 动手练习

### 练习 1: 设计可见性

为这个银行账户结构设计可见性：

```rust
// TODO: 添加适当的 pub 修饰符
struct BankAccount {
    account_number: String,  // 应该公有吗？
    balance: f64,           // 应该私有吗？
    
    fn new(number: String) -> Self {
        BankAccount {
            account_number: number,
            balance: 0.0,
        }
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
pub struct BankAccount {
    pub account_number: String,  // 账户号可以公开
    balance: f64,               // 余额私有
}

impl BankAccount {
    pub fn new(number: String) -> Self {
        BankAccount {
            account_number: number,
            balance: 0.0,
        }
    }
    
    pub fn get_balance(&self) -> f64 {  // 提供公有访问器
        self.balance
    }
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 什么时候应该用私有字段？

**A**: 
- ✅ **私有**：内部实现细节、敏感数据
- ✅ **公有**：用户需要直接访问的非敏感数据
- **一般规则**：默认私有，需要时再公开

### Q: 私有方法有什么用处？

**A**: 

```rust
pub struct Parser {
    data: String,
}

impl Parser {
    pub fn parse(&self) -> Result {
        self.validate()?;  // 内部校验
        self.transform()   // 内部转换
    }
    
    fn validate(&self) -> Result<()> {  // 私有
        // 实现细节，用户不需要知道
    }
    
    fn transform(&self) -> Result {  // 私有
        // 实现细节
    }
}
```

### Q: pub(crate) 和 pub 有什么区别？

**A**: 
- **`pub`**: 整个程序（包括其他 crate）都可以访问
- **`pub(crate)`**: 只在当前 crate 内公有，外部不可见
- **使用场景**：库的内部 API 应该用 `pub(crate)`

---

## 小结

**核心要点**：

1. **默认私有**: 模块、函数、字段默认都是私有的
2. **pub 控制**: 使用 `pub` 关键字公开
3. **封装优先**: 默认私有，需要时再公开
4. **私有构造器**: 通过方法控制对象创建
5. **信息隐藏**: 隐藏实现细节，暴露稳定接口

**术语**：

- **Visibility (可见性)**: 代码的访问权限
- **Encapsulation (封装)**: 隐藏实现细节
- **Public interface (公有接口)**: 对外暴露的方法
- **Private implementation (私有实现)**: 内部实现细节

**下一步**：

- 相关：[模块](module.md)
- 进阶：[封装模式](encapsulation-patterns.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Visibility | 可见性 |
| Encapsulation | 封装 |
| Public | 公有 |
| Private | 私有 |

---

**完整源码**：[`src/basic/visiable_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/visiable_sample.rs)

---

> 💡 **提示**：好的可见性设计让你的代码像精密的钟表 - 用户只需要看表盘，不需要知道齿轮怎么转！
