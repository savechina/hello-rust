# 结构体字段

## 开篇故事

想象你正在填写一个表格。表格有"姓名"、"年龄"、"邮箱"等栏目。每个栏目只能填写特定类型的信息。Rust 结构体的**字段**就像这些表格栏目 - 它们定义了结构体可以存储什么类型的数据。

---

## 本章适合谁

如果你已经学完了结构体基础，现在想深入了解如何定义、访问和操作结构体的字段，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 定义结构体字段的类型
2. 使用公有 (pub) 和私有字段
3. 理解字段所有权的移动规则
4. 使用字段初始化简写语法

---

## 前置要求

- [结构体基础](struct.md) - 结构体定义和实例创建
- [所有权](ownership.md) - 字段移动机制

---

## 第一个例子

```rust,ignore
struct User {
    username: String,  // 用户名
    email: String,     // 邮箱
    active: bool,      // 是否激活
    sign_in_count: u64, // 登录次数
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};

println!("用户名：{}", user.username);
```

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念       | Python               | Java                   | C++                    | Rust                       | 关键差异                  |
| ---------- | -------------------- | ---------------------- | ---------------------- | -------------------------- | ------------------------- |
| 字段定义   | `class: self.x`      | `private int x;`       | `int x;`               | `x: i32`                   | Rust 必须标注类型         |
| 字段访问   | `obj.x`              | `obj.getX()` / `obj.x` | `obj.x`                | `obj.x` 或 `pub` 字段      | Rust 可控制字段可见性     |
| 字段可见性 | 公开（无控制）       | `private` 默认         | 公开                   | 私有默认，需 `pub`         | Rust 默认私有             |
| 字段所有权 | 引用                 | 引用                   | 可复制或引用           | 移动或 Copy                | Rust 有所有权语义         |
| 字段简写   | 无                   | 无                     | 无                     | `field: field` → `field`   | Rust 有简写语法           |

**核心差异**: Python 类字段动态，Java 用 getter/setter，C++ 公开字段，Rust 默认私有且必须标注类型。

---

## 原理解析

### 1. 字段类型规则

字段可以是任何 Rust 类型：

```rust,ignore
struct Example {
    text: String,
    number: i32,
    flag: bool,
    point: (i32, i32),     // 元组
}
```

**嵌套结构体**：

```rust,ignore
struct Point { x: f64, y: f64 }

struct Circle {
    center: Point,  // 字段是结构体
    radius: f64,
}
```

### 2. 字段所有权

**String 字段**会移动：

```rust,ignore
let user2 = User { username: user1.username };
// println!("{}", user1.username); // ❌ 已移动
```

**Copy 类型的字段**会复制：

```rust,ignore
struct Point { x: i32, y: i32 } // i32 实现了 Copy

let p1 = Point { x: 10, y: 20 };
let p2 = Point { x: p1.x, y: p1.y };
println!("p1.x = {}", p1.x); // ✅ 可以
```

### 3. 字段初始化简写

变量名和字段名相同时可省略：

```rust,ignore
fn build_user(username: String, email: String) -> User {
    User {
        username,  // ✅ 简写 (等同于 username: username)
        email,
        active: true,
    }
}
```

### 4. 公有和私有字段

```rust,ignore
mod my_module {
    pub struct User {
        pub username: String,  // 公有
        email: String,         // 私有!
    }
}
```

### 5. 字段必须标注类型

```rust,ignore
struct Valid {
    name: String,  // ✅
}

// struct Invalid { name, } // ❌ 缺少类型
```

---

## 常见错误

### 错误 1: 缺少类型注解

```rust,ignore
struct User {
    username,  // ❌ 编译错误!
}
```

**修复**：添加类型

```rust,ignore
struct User {
    username: String,  // ✅
}
```

### 错误 2: 移动私有字段

```rust,ignore
let user = my_module::User {
    // email: ... ❌ 私有字段不可访问
};
```

**修复**：使用 `pub` 或提供公共方法

### 错误 3: 忘记字段

```rust,ignore
let user = User {
    username: String::from("alice"),
    // ❌ 缺少 email 和 active
};
```

---

## 动手练习

### 练习 1: 预测结果

```rust,ignore
struct Point { x: i32, y: i32 }

let p1 = Point { x: 10, y: 20 };
let p2 = Point { x: p1.x, y: p1.y };

println!("p1: ({}, {})", p1.x, p1.y); // 通过吗？
```

<details>
<summary>点击查看答案</summary>

✅ **通过** - `i32` 实现 Copy，`p1` 仍可访问
</details>

### 练习 2: 使用简写

```rust,ignore
// 重写：
Person {
    name: name,
    age: age,
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
Person {
    name,  // ✅ 简写
    age,
}
```
</details>

---

## 故障排查

### Q: 字段顺序重要吗？

**A**: 不重要！按名称匹配，不是位置。

### Q: 字段可以是函数吗？

**A**: ✅ 可以！字段可以是函数指针。

### Q: 如何获取字段数量？

**A**: 无法运行时获取，字段在编译时确定。

---

## 小结

**要点**：

1. **语法**: `字段名：类型`
2. **Copy 类型复制**: 非 Copy 类型移动
3. **公有/私有**: `pub` 控制访问
4. **字段简写**: 变量名=字段名可省略

**术语**：

- **Field (字段)**: 结构体数据成员
- **Copy trait**: 决定是否复制
- **Visibility (可见性)**: 访问控制

**下一步**：

- 继续：[结构体方法](struct-methods.md)
- 相关：[结构体基础](struct.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Field | 字段 |
| Public field | 公有字段 |
| Private field | 私有字段 |

---

完整示例：`src/basic/rectangle.rs`

---

> 💡 **提示**：字段是你与结构体交互的主要方式！

---

## 知识检查

**快速测验**（答案在下方）：

1. 如何初始化结构体时省略字段？

2. 结构体更新语法是什么？

3. 元组结构体和普通结构体有什么区别？

<details>
<summary>点击查看答案与解析</summary>

1. 不能省略 - 所有字段必须初始化（除非有默认值）
2. `Struct { field1: value, ..existing_instance }`
3. 元组结构体有匿名命名字段，普通结构体有命名字段

**关键理解**: 结构体更新语法可以减少重复代码。
</details>

## 延伸阅读

学习完结构体字段后，你可能还想了解：

- [元组结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields) - 匿名结构体
- [单元结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields) - 标记类型
- [字段级属性](https://doc.rust-lang.org/reference/attributes.html) - serde 派生等

**选择建议**:
- 想学习方法 → 继续学习 [结构体方法](struct-methods.md)
- 想学习枚举 → 跳到 [枚举](enums.md)

## 继续学习

**前一章**: [结构体](struct.md)  
**下一章**: [结构体方法](struct-methods.md)

**相关章节**:
- [结构体](struct.md)
- [结构体方法](struct-methods.md)
- [枚举](enums.md)

**返回**: [基础入门](basic-overview.md)
