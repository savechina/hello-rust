# 派生宏

## 开篇故事

想象你要为结构体实现 getter 和 setter 方法。传统方式是：手动写每个方法 → 容易出错 → 代码重复。派生宏就像是：告诉编译器"帮我生成这些方法"，它自动完成。getset crate 就是这样的工具。

---

## 本章适合谁

如果你想减少样板代码（getter、setter、builder），本章适合你。派生宏是 Rust 元编程的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 理解派生宏概念
2. 使用 getset crate
3. 自动生成 getter/setter
4. 使用 derive_more
5. 创建 Builder 模式

---

## 前置要求

- 结构体 - 结构体基础
- [宏编程](macros.md) - 宏基础
- 可变性 - mutability 基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add getset
```

## 第一个例子

最简单的 getset 使用：

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Category {
    #[getset(get = "pub", set = "pub")]
    id: u64,
    
    #[getset(get = "pub", set = "pub")]
    name: String,
}

fn main() {
    let mut cat = Category {
        id: 1,
        name: "Electronics".to_string(),
    };
    
    // 使用生成的 getter
    println!("ID: {}", cat.id());
    
    // 使用生成的 setter
    cat.set_name("Books".to_string());
}
```

**完整示例**: [getset_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/getset_sample.rs)

---

## 原理解析

### getset 特性

**getset 是代码生成库**：

- ✅ 自动生成 getter
- ✅ 自动生成 setter
- ✅ 减少样板代码
- ✅ 类型安全

### 使用 Getters 派生

**使用 #[derive(Getters)]**：

```rust
use getset::Getters;

#[derive(Getters)]
pub struct User {
    #[get = "pub"]
    id: u32,
    
    #[get = "pub"]
    name: String,
}

// 生成：
// impl User {
//     pub fn id(&self) -> &u32 { &self.id }
//     pub fn name(&self) -> &String { &self.name }
// }
```

### 使用 Setters 派生

**使用 #[derive(Setters)]**：

```rust
use getset::Setters;

#[derive(Setters)]
pub struct User {
    #[set = "pub"]
    name: String,
}

// 生成：
// impl User {
//     pub fn set_name(&mut self, val: String) { self.name = val; }
// }
```

### 组合使用

**使用 Getters + Setters**：

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub struct Product {
    #[getset(get = "pub", set = "pub")]
    id: u64,
    
    #[getset(get = "pub", set = "pub")]
    name: String,
}
```

### derive_more

**使用 derive_more**：

```rust
use derive_more::Display;

#[derive(Display)]
#[display("User{{id:{}, name:{}}}", id, name)]
pub struct User {
    id: u32,
    name: String,
}

// 自动生成 Display 实现
let user = User { id: 1, name: "Alice".to_string() };
println!("{}", user);  // User{id:1, name:Alice}
```

### Builder 模式

**手动实现 Builder**：

```rust
pub struct Category {
    id: u64,
    name: String,
}

impl Category {
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder::new()
    }
}

pub struct CategoryBuilder {
    inner: Category,
}

impl CategoryBuilder {
    pub fn new() -> Self {
        Self {
            inner: Category {
                id: 0,
                name: String::new(),
            },
        }
    }
    
    pub fn with_id(mut self, id: u64) -> Self {
        self.inner.id = id;
        self
    }
    
    pub fn with_name(mut self, name: String) -> Self {
        self.inner.name = name;
        self
    }
    
    pub fn build(self) -> Category {
        self.inner
    }
}

// 使用
let cat = Category::builder()
    .with_id(1)
    .with_name("Electronics".to_string())
    .build();
```

---

## 常见错误

### 错误 1: 忘记属性

```rust
use getset::Getters;

#[derive(Getters)]
pub struct User {
    id: u32,  // ❌ 忘记 #[get] 属性
}
```

**错误信息**:
```
no method named `id` found
```

**修复方法**:
```rust
#[derive(Getters)]
pub struct User {
    #[get = "pub"]
    id: u32,  // ✅ 添加属性
}
```

### 错误 2: 可见性错误

```rust
#[derive(Getters)]
pub struct User {
    #[get = "pub"]
    id: u32,
}

let user = User { id: 1 };
user.id();  // ✅ 可以访问

// 但在其他模块:
mod other {
    user.id();  // ❌ 如果 User 不是 pub 会失败
}
```

**修复方法**:
```rust
pub struct User {  // ✅ 结构体也必须是 pub
    #[get = "pub"]
    id: u32,
}
```

### 错误 3: Builder 模式错误

```rust
pub struct Builder {
    inner: User,
}

impl Builder {
    pub fn with_name(self, name: String) -> Self {
        self.inner.name = name;  // ❌ self 不是 mut
        self
    }
}
```

**修复方法**:
```rust
pub fn with_name(mut self, name: String) -> Self {
    // ✅ 添加 mut
    self.inner.name = name;
    self
}
```

---

## 动手练习

### 练习 1: 创建 Getters

```rust
use getset::Getters;

#[derive(Getters)]
pub struct Product {
    // TODO: 添加 id 字段 (u64)
    // TODO: 添加 name 字段 (String)
    // TODO: 添加 pub getter
}
```

<details>
<summary>点击查看答案</summary>

```rust
#[derive(Getters)]
pub struct Product {
    #[get = "pub"]
    id: u64,
    #[get = "pub"]
    name: String,
}
```
</details>

### 练习 2: 创建 Setters

```rust
use getset::Setters;

#[derive(Setters)]
pub struct User {
    // TODO: 添加 name 字段
    // TODO: 添加 pub setter
}
```

<details>
<summary>点击查看答案</summary>

```rust
#[derive(Setters)]
pub struct User {
    #[set = "pub"]
    name: String,
}
```
</details>

### 练习 3: 实现 Builder

```rust
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    // TODO: 实现 builder() 方法
}

pub struct ConfigBuilder {
    inner: Config,
}

impl ConfigBuilder {
    // TODO: 实现 new() 方法
    // TODO: 实现 with_host() 方法
    // TODO: 实现 with_port() 方法
    // TODO: 实现 build() 方法
}
```

<details>
<summary>点击查看答案</summary>

```rust
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            inner: Config {
                host: String::new(),
                port: 0,
            },
        }
    }
    
    pub fn with_host(mut self, host: String) -> Self {
        self.inner.host = host;
        self
    }
    
    pub fn with_port(mut self, port: u16) -> Self {
        self.inner.port = port;
        self
    }
    
    pub fn build(self) -> Config {
        self.inner
    }
}
```
</details>

---

## 故障排查 (FAQ)

### Q: getset 和手动实现有什么区别？

**A**: 
- **getset**: 自动生成，减少样板
- **手动**: 完全控制，更灵活
- **推荐**: getset 用于简单 getter/setter

### Q: 什么时候使用 Builder 模式？

**A**: 
- 多个可选字段
- 需要链式调用
- 构建复杂对象

### Q: derive_more 和 getset 有什么区别？

**A**: 
- **derive_more**: 派生各种 trait (Display, From, Into 等)
- **getset**: 专门生成 getter/setter
- **可以一起使用**

---

## 知识扩展

### 可见性选项

```rust
#[derive(Getters)]
pub struct User {
    #[get = "pub"]      // 公开
    #[get = "pub(crate)"]  // crate 内公开
    #[get = "pub(super)"]  // 父模块公开
    #[get]              // 私有（默认）
    id: u32,
}
```

### 组合派生

```rust
use getset::{Getters, Setters, MutGetters};
use derive_more::Display;

#[derive(Getters, Setters, MutGetters, Display)]
#[display("User{{id:{}, name:{}}}", id, name)]
pub struct User {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    id: u32,
    
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    name: String,
}
```

### 条件编译

```rust
#[cfg_attr(test, derive(Getters))]
pub struct Config {
    #[cfg_attr(test, get = "pub")]
    value: String,
}
```

---

## 小结

**核心要点**：

1. **getset**: 自动生成 getter/setter
2. **derive_more**: 派生各种 trait
3. **Builder**: 链式构建对象
4. **可见性**: 控制访问级别
5. **减少样板**: 提高开发效率

**关键术语**：

- **Getter**: 获取字段值
- **Setter**: 设置字段值
- **Builder**: 构建器模式
- **Derive**: 派生宏

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Getter | 获取方法 |
| Setter | 设置方法 |
| Builder | 构建器 |
| Derive | 派生 |
| Macro | 宏 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[derive(Getters)]` 生成什么代码？

2. `#[get = "pub"]` 和 `#[get]` 有什么区别？

3. 什么时候应该使用派生宏而不是手动实现？

<details>
<summary>点击查看答案与解析</summary>

1. 为每个字段生成 `fn field_name(&self) -> &Type` 方法
2. `#[get = "pub"]` 生成公共方法，`#[get]` 生成私有方法
3. 字段多、样板代码多时使用派生宏，减少重复

**关键理解**: 派生宏是减少样板代码的有效工具。
</details>

## 延伸阅读

学习完派生宏后，你可能还想了解：

- [derive_more crate](https://docs.rs/derive_more) - 更多派生宏
- [smart-default crate](https://docs.rs/smart-default) - 默认值派生
- [typed-builder crate](https://docs.rs/typed-builder) - Builder 模式派生

**选择建议**:
- 想学习宏编程 → 继续学习 [宏编程](macros.md)
- 想学习类型别名 → 跳到 [类型别名](typealias.md)

## 继续学习

**前一章**: [RSpec 测试框架](rspec.md)  
**下一章**: [宏编程](macros.md)

**相关章节**:
- [RSpec 测试框架](rspec.md)
- [宏编程](macros.md)
- 结构体

**返回**: 高级进阶

---

**完整示例**: [getset_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/getset_sample.rs)
