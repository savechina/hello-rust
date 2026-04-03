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

```rust
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

## 原理解析

### 1. 字段类型规则

字段可以是任何 Rust 类型：

```rust
struct Example {
    text: String,
    number: i32,
    flag: bool,
    point: (i32, i32),     // 元组
}
```

**嵌套结构体**：

```rust
struct Point { x: f64, y: f64 }

struct Circle {
    center: Point,  // 字段是结构体
    radius: f64,
}
```

### 2. 字段所有权

**String 字段**会移动：

```rust
let user2 = User { username: user1.username };
// println!("{}", user1.username); // ❌ 已移动
```

**Copy 类型的字段**会复制：

```rust
struct Point { x: i32, y: i32 } // i32 实现了 Copy

let p1 = Point { x: 10, y: 20 };
let p2 = Point { x: p1.x, y: p1.y };
println!("p1.x = {}", p1.x); // ✅ 可以
```

### 3. 字段初始化简写

变量名和字段名相同时可省略：

```rust
fn build_user(username: String, email: String) -> User {
    User {
        username,  // ✅ 简写 (等同于 username: username)
        email,
        active: true,
    }
}
```

### 4. 公有和私有字段

```rust
mod my_module {
    pub struct User {
        pub username: String,  // 公有
        email: String,         // 私有!
    }
}
```

### 5. 字段必须标注类型

```rust
struct Valid {
    name: String,  // ✅
}

// struct Invalid { name, } // ❌ 缺少类型
```

---

## 常见错误

### 错误 1: 缺少类型注解

```rust
struct User {
    username,  // ❌ 编译错误!
}
```

**修复**：添加类型

```rust
struct User {
    username: String,  // ✅
}
```

### 错误 2: 移动私有字段

```rust
let user = my_module::User {
    // email: ... ❌ 私有字段不可访问
};
```

**修复**：使用 `pub` 或提供公共方法

### 错误 3: 忘记字段

```rust
let user = User {
    username: String::from("alice"),
    // ❌ 缺少 email 和 active
};
```

---

## 动手练习

### 练习 1: 预测结果

```rust
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

```rust
// 重写：
Person {
    name: name,
    age: age,
}
```

<details>
<summary>点击查看答案</summary>

```rust
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
