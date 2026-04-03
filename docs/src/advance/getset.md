# Getset 派生宏

## 开篇故事

想象你正在开发一个电商系统，需要定义 `Product`、`Category`、`Order` 等数十个数据结构。每个结构体都有十几个字段，而你不得不为每个字段手动编写 getter 和 setter 方法——这会产生数百行重复的样板代码。更糟的是，当你修改字段时，还要同步更新这些方法。

这就是 **getset** 派生宏的价值所在：**一行代码，自动生成所有访问器方法**。

---

## 本章适合谁

如果你厌倦了为结构体字段编写重复的 getter/setter，想了解 Rust 派生宏如何减少样板代码，本章适合你。这是 Rust 元编程在实际业务开发中最常见的应用之一。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `Getters`、`Setters`、`MutGetters`、`CopyGetters` 派生宏
2. 配置字段级别的访问控制（`get = "pub"` 等）
3. 区分不可变引用、可变引用和值拷贝三种 getter 类型
4. 理解派生宏的代码生成机制
5. 结合 Builder 模式构建复杂对象

---

## 前置要求

学习本章前，你需要理解：

- [结构体](../basic/struct.md) - 结构体定义和字段访问
- [宏](macros.md) - 派生宏的基本概念
- [可见性](../basic/visiable.md) - `pub`、`pub(crate)` 等访问控制

---

## 第一个例子

让我们看一个最简单的 getset 示例：

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Product {
    #[getset(get = "pub", set = "pub")]
    id: u64,
    #[getset(get = "pub", set = "pub")]
    name: String,
    #[getset(get = "pub", set = "pub")]
    price: u32,
}
```

**发生了什么？**

编译器会自动为 `Product` 结构体生成以下代码：

```rust
impl Product {
    // Getters
    pub fn id(&self) -> &u64 { &self.id }
    pub fn name(&self) -> &String { &self.name }
    pub fn price(&self) -> &u32 { &self.price }
    
    // Setters
    pub fn set_id(&mut self, val: u64) -> &mut Self { 
        self.id = val; 
        self 
    }
    pub fn set_name(&mut self, val: String) -> &mut Self { 
        self.name = val; 
        self 
    }
    pub fn set_price(&mut self, val: u32) -> &mut Self { 
        self.price = val; 
        self 
    }
}
```

原本需要 30+ 行的样板代码，现在只需几行属性标记。

---

## 原理解析

### 代码生成过程

```
┌─────────────────────────────────────────┐
│  源代码 (带 getset 属性)                 │
│                                         │
│  #[derive(Getters, Setters)]            │
│  struct Product {                       │
│      #[getset(get="pub", set="pub")]    │
│      id: u64,                           │
│  }                                      │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│  编译时 - 派生宏展开                      │
│                                         │
│  1. syn 解析结构体 AST                   │
│  2. 提取字段名和类型                      │
│  3. 根据属性生成 impl 代码               │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│  编译器看到的最终代码                    │
│                                         │
│  struct Product { id: u64 }             │
│                                         │
│  impl Product {                         │
│      pub fn id(&self) -> &u64 {         │
│          &self.id                       │
│      }                                  │
│      pub fn set_id(&mut self, ...) {...}│
│  }                                      │
└─────────────────────────────────────────┘
```

### 四种派生宏类型

| 宏名称 | 生成的方法 | 返回类型 | 适用场景 |
|--------|-----------|---------|---------|
| `Getters` | `fn field(&self)` | `&T` (不可变引用) | 大多数情况 |
| `Setters` | `fn set_field(&mut self, val: T)` | `&mut Self` | 链式调用 |
| `MutGetters` | `fn field_mut(&mut self)` | `&mut T` (可变引用) | 需要修改字段内部 |
| `CopyGetters` | `fn field(&self)` | `T` (值拷贝) | 实现了 `Copy` trait 的类型 |

**关键区别**：

```rust
#[derive(Getters, MutGetters, CopyGetters)]
struct Example {
    #[getset(get = "pub")]
    name: String,      // getter 返回 &String
    
    #[getset(get_mut = "pub")]
    category: Category, // getter 返回 &mut Category
    
    #[getset(get_copy = "pub")]
    in_stock: bool,    // getter 返回 bool (拷贝值)
}
```

---

## 常见用法

### 基础用法：商品结构体

来自 `src/advance/getset_sample.rs` 的 `Product` 结构体：

```rust
#[derive(Getters, Setters, MutGetters, CopyGetters, Default, Display)]
pub struct Product {
    /// The unique identifier for the product
    #[getset(get = "pub", set = "pub")]
    id: u64,

    /// The name of the product
    #[getset(get = "pub", set = "pub")]
    name: String,

    /// The price of the product in cents
    #[getset(get = "pub", set = "pub")]
    price: u32,

    /// Whether the product is in stock
    #[getset(get_copy = "pub", set = "pub")] // get_copy for primitive types
    in_stock: bool,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    category: Category,
}
```

**使用示例**：

```rust
fn getset_sample() {
    // Create a new product instance
    let mut product = Product {
        id: 1,
        name: "Rust Programming Book".to_string(),
        price: 3999,
        in_stock: true,
        category: Category { /* ... */ },
    };

    // 使用生成的 getter 方法
    println!("Product ID: {}", product.id());
    println!("Product Name: {}", product.name());
    println!("Price: ${}.{}", product.price() / 100, product.price() % 100);
    println!("In Stock: {}", product.in_stock());  // get_copy 返回 bool 值

    // 使用生成的 setter 方法（支持链式调用）
    product
        .set_name("Advanced Rust Programming".to_string())
        .set_price(4999)
        .set_in_stock(false);

    // 使用可变 getter 修改嵌套结构
    let category = product.category_mut();
    category.set_first_category_id(2);
}
```

### 嵌套结构：分类结构体

```rust
#[derive(Getters, Setters, Default, Display)]
#[display("Category{{first_category_id:{}}}...")]
pub struct Category {
    #[getset(get = "pub", set = "pub")]
    first_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    first_category_name: String,
    #[getset(get = "pub", set = "pub")]
    second_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    second_category_name: String,
    #[getset(get = "pub", set = "pub")]
    three_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    three_categroy_name: String,
}
```

### 结合 Builder 模式

`getset` 生成的 setter 支持**链式调用**（返回 `&mut Self`），天然适合 Builder 模式：

```rust
impl Category {
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder::new()
    }
    
    /// setting first category
    pub fn with_first_category(mut self, id: u64, name: String) -> Self {
        self.first_category_id = id;
        self.first_category_name = name;
        self
    }
}

pub struct CategoryBuilder {
    inner: Category,
}

impl CategoryBuilder {
    pub fn new() -> Self {
        Self { inner: Category::default() }
    }

    pub fn with_first_category(mut self, id: u64, name: String) -> Self {
        self.inner.set_first_category_id(id);
        self.inner.set_first_category_name(name);
        self
    }

    pub fn build(self) -> Category {
        self.inner
    }
}
```

**使用 Builder**：

```rust
// 使用 Category 自身的方法链
let category = Category::default()
    .with_first_category(1, "水果".to_string())
    .with_second_category(10, "苹果梨".to_string())
    .with_three_category(100, "苹果".to_string());

// 使用独立 Builder
let category = Category::builder()
    .with_first_category(2, "水果".to_string())
    .with_second_category(21, "苹果梨".to_string())
    .with_three_category(201, "苹果".to_string())
    .build();
```

---

## 常见错误

### 错误 1: 在 Copy 类型上使用普通 getter

```rust
#[derive(Getters)]
struct BadExample {
    #[getset(get = "pub")]  // ❌ 返回 &bool，通常不需要
    flag: bool,
}

#[derive(Getters, CopyGetters)]
struct GoodExample {
    #[getset(get_copy = "pub")]  // ✅ 返回 bool
    flag: bool,
}
```

**为什么**：基本类型（bool、i32、u64 等）实现了 `Copy` trait，直接返回值更高效。

### 错误 2: 忘记 mut 关键字调用 setter

```rust
let product = Product::default();
product.set_name("New Name".to_string());  // ❌ 编译错误！product 不是 mut

let mut product = Product::default();
product.set_name("New Name".to_string());  // ✅ 正确
```

### 错误 3: 试图通过 getter 修改字段

```rust
let mut product = Product::default();
product.name().push_str(" suffix");  // ❌ 编译错误！name() 返回 &String，不是 &mut String

product.name_mut().push_str(" suffix");  // ✅ 需要 MutGetters 和 get_mut
```

---

## 故障排查 (FAQ)

### Q: getset 和直接写方法有什么区别？

**A**: 功能完全相同，只是代码生成方式不同。

| 方式 | 代码量 | 维护成本 | 灵活性 |
|------|-------|---------|-------|
| 手写方法 | 多 | 高 | 完全自定义 |
| getset 宏 | 少 | 低 | 按模板生成 |

当你需要标准 getter/setter 时，getset 是最佳选择。需要特殊逻辑时，手动实现。

### Q: 可以混合使用吗？

**A**: 可以。getset 生成的方法只是普通的 impl 块内容，你可以在同一结构体上添加自定义方法：

```rust
#[derive(Getters, Setters)]
struct Product {
    #[getset(get = "pub", set = "pub")]
    price: u32,
}

impl Product {
    // getset 生成的 getter
    // pub fn price(&self) -> &u32
    
    // 自定义方法
    pub fn price_in_dollars(&self) -> f64 {
        self.price as f64 / 100.0
    }
}
```

### Q: 如何控制生成的可见性？

**A**: 使用 `"pub"`、`"pub(crate)"`、`"pub(super)"` 或省略（默认 private）：

```rust
#[getset(get = "pub", set = "pub(crate)")]
field: String,  // getter pub, setter pub(crate)
```

---

## 知识扩展 (选学)

### 自定义 getter 前缀

默认生成 `field()`，可以自定义：

```rust
#[getset(get = "pub", get_prefix = "get_")]
name: String,  // 生成 get_name() 而不是 name()
```

### Skip 跳过字段

某些字段不需要 getter/setter：

```rust
#[derive(Getters)]
struct User {
    #[getset(get = "pub")]
    name: String,
    
    internal_id: u64,  // 无属性，不生成方法
}
```

### 与 derive_more 结合使用

`getset_sample.rs` 中同时使用了 `Display` derive：

```rust
use derive_more::Display;
use getset::{Getters, Setters};

#[derive(Getters, Setters, Display)]
#[display("Product(id={}, name={})", id, name)]
pub struct Product {
    #[getset(get = "pub", set = "pub")]
    id: u64,
    #[getset(get = "pub", set = "pub")]
    name: String,
}
```

---

## 小结

**核心要点**：

1. **Getters 派生宏**为每个字段生成不可变引用 getter（`&T`）
2. **Setters 派生宏**生成链式 setter 方法（返回 `&mut Self`）
3. **MutGetters** 生成可变引用 getter（`&mut T`），用于修改嵌套结构
4. **CopyGetters** 用于实现了 `Copy` trait 的类型，返回值的拷贝而非引用
5. 使用 `#[getset(get = "pub", set = "pub")]` 语法配置字段级访问控制

**关键术语**：

- **派生宏 (Derive Macro)** - 通过 `#[derive(...)]` 自动实现 trait
- **Getter** - 获取字段值的访问器方法
- **Setter** - 设置字段值的访问器方法
- **Builder 模式** - 使用链式方法构建对象的构造模式
- **样板代码 (Boilerplate)** - 重复、机械性的代码

**下一步**：

- 学习 [宏](macros.md) - 理解派生宏的工作原理
- 了解 [依赖注入](../awesome/dependency_injection.md) - 在大型项目中组织数据结构

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Derive Macro | 派生宏 |
| Getter | 获取器 |
| Setter | 设置器 |
| Boilerplate | 样板代码 |
| Visibility | 可见性 |
| Copy trait | Copy trait |
| Builder Pattern | Builder 模式 |

---

完整示例：`src/advance/getset_sample.rs`

---

## 继续学习

- 上一步：[宏](macros.md)
- 下一步：[数据库操作](database.md)
- 进阶：[依赖注入模式](../awesome/dependency_injection.md)

---

## 知识检查

**问题 1** (基础概念)

以下代码使用了哪些派生宏？

```rust
#[derive(Getters, Setters, CopyGetters)]
struct Item {
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub", set = "pub")]
    quantity: u32,
}
```

A) 只有 `Getters`
B) `Getters` 和 `Setters`
C) `Getters`、`Setters` 和 `CopyGetters`
D) 以上都不是

<details>
<summary>答案与解析</summary>

**答案**: C) `Getters`、`Setters` 和 `CopyGetters`

**解析**:
- `Getters` 为 `name` 生成 `name(&self) -> &String`
- `Setters` 为 `quantity` 生成 `set_quantity(&mut self, val: u32)`
- `CopyGetters` 为 `quantity` 生成 `quantity(&self) -> u32`（返回值拷贝）

注意一个字段可以同时使用多种 getter 类型。
</details>

---

**问题 2** (代码分析)

下面的代码会编译通过吗？为什么？

```rust
let item = Item::default();
item.set_quantity(10);
```

<details>
<summary>答案与解析</summary>

**答案**: ❌ 编译错误

**原因**: `set_quantity` 需要 `&mut self`，但 `item` 不是可变绑定。

**修复**:
```rust
let mut item = Item::default();  // 添加 mut
item.set_quantity(10);
```
</details>

---

**问题 3** (实际应用)

为什么下面的结构体对 `price` 使用 `get_copy` 而不是 `get`？

```rust
#[derive(Getters, CopyGetters)]
struct Product {
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    price: u64,
}
```

<details>
<summary>答案与解析</summary>

**答案**: 

1. **`name` 是 String** - 没有实现 `Copy`，使用 `get` 返回 `&String`（引用）
2. **`price` 是 u64** - 实现了 `Copy` trait，使用 `get_copy` 直接返回值更高效
3. **性能考虑** - 基本类型（u64、f64、bool 等）拷贝开销极小，直接返回值更符合 Rust 惯例

如果写成 `#[getset(get = "pub")] price: u64`，会生成 `fn price(&self) -> &u64`，使用时需要解引用，不够便利。
</details>

---

## 代码生成可视化

### Getter 类型对比

```
结构体定义:
┌─────────────────────────────────────┐
│ struct Product {                    │
│     name: String,                   │
│     price: u64,                     │
│     category: Category,             │
│ }                                   │
└─────────────────────────────────────┘

生成的代码:
┌─────────────────────────────────────────────────────────┐
│ // #[getset(get = "pub")]                              │
│ impl Product {                                          │
│     // String 类型 - 返回引用                           │
│     pub fn name(&self) -> &String {                     │
│         &self.name                                      │
│     }                                                   │
│ }                                                       │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ // #[getset(get_copy = "pub")]  (u64 实现 Copy)        │
│ impl Product {                                          │
│     // u64 类型 - 返回值拷贝                            │
│     pub fn price(&self) -> u64 {                        │
│         self.price                                      │
│     }                                                   │
│ }                                                       │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ // #[getset(get_mut = "pub")]                          │
│ impl Product {                                          │
│     // 可变引用 - 允许修改嵌套结构                      │
│     pub fn category_mut(&mut self) -> &mut Category {   │
│         &mut self.category                              │
│     }                                                   │
│ }                                                       │
└─────────────────────────────────────────────────────────┘
```

### Setter 的链式调用

```
调用代码:
product
    .set_name("Book".to_string())
    .set_price(2999)
    .set_in_stock(true);

展开过程:
┌─────────────────────────────────────────────────────────┐
│ product.set_name("Book".to_string())                    │
│   └── 返回 &mut product                                 │
│       .set_price(2999)                                  │
│         └── 返回 &mut product                           │
│             .set_in_stock(true)                         │
│               └── 返回 &mut product                     │
└─────────────────────────────────────────────────────────┘
```

---

## 工业界应用：减少 API 开发样板代码

**真实场景**：

一个微服务有 20 个 API 请求/响应结构体，每个平均 8 个字段：

```rust
// 不使用 getset：每个结构体 ~120 行
pub struct CreateOrderRequest {
    user_id: u64,
    items: Vec<OrderItem>,
    total_amount: u64,
    // ... 还有 5 个字段
}

impl CreateOrderRequest {
    pub fn user_id(&self) -> &u64 { &self.user_id }
    pub fn set_user_id(&mut self, val: u64) -> &mut Self { self.user_id = val; self }
    // 每个字段 2 个方法 × 8 字段 = 16 个方法
}
```

**使用 getset**：

```rust
#[derive(Getters, Setters, Default)]
pub struct CreateOrderRequest {
    #[getset(get = "pub", set = "pub")]
    user_id: u64,
    #[getset(get = "pub", set = "pub")]
    items: Vec<OrderItem>,
    #[getset(get = "pub", set = "pub")]
    total_amount: u64,
    // ...
}
```

**效果**：
- 代码行数：**减少 85%**
- 维护成本：添加新字段只需一行
- Bug 减少：不会忘记同步 getter/setter
