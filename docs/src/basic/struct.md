# 结构体

## 开篇故事

想象你正在设计一个游戏角色。每个角色有名字、生命值、等级、装备等属性。你不会为每个属性创建单独的变量，而是把它们组合在一起形成一个"角色"。Rust 的**结构体**就是这样的工具箱 - 它把相关的数据打包在一起，让它们作为一个整体被管理。

---

## 本章适合谁

如果你已经理解了变量和所有权，现在想学习如何组织复杂的数据，本章适合你。结构体是 Rust 中最常用的数据组织方式，所有 Rust 程序员每天都在使用。

---

## 你会学到什么

完成本章后，你可以：

1. 定义结构体并创建实例
2. 使用字段初始化简写语法
3. 实现结构体方法（关联函数）
4. 理解所有权在结构体中的工作方式
5. 使用结构体更新语法和元组结构体

---

## 前置要求

学习本章前，你需要理解：

- [变量表达式](expression.md) - 变量绑定基础
- [所有权](ownership.md) - 移动和借用概念

---

## 第一个例子

让我们定义一个简单的矩形结构体：

```rust,ignore
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形面积是：{} 平方像素", rect1.width * rect1.height);
}
```

**发生了什么？**

- `struct Rectangle` - 定义了一个名为 `Rectangle` 的结构体
- `width: 30` - 创建实例时给字段赋值
- `rect1.width` - 访问字段，使用点号

---

## 原理解析

### 1. 结构体是什么？

结构体是**自定义数据类型**，允许你将多个值组合成一个有意义的整体。

**类比**：
> 结构体就像数据库中的一行记录。比如"用户"表中的一行：用户名、邮箱、年龄、激活状态 - 这些信息共同描述一个用户。

### 2. 定义结构体

```rust,ignore
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

**结构体字段规则**：

- 每个字段有**名称**和**类型**
- 字段之间用逗号分隔
- 最后一个字段也**可以**有逗号（推荐，方便添加字段）
- 字段可以是任何类型（包括其他结构体）

### 3. 创建实例

创建结构体使用**字段初始化语法**：

```rust,ignore
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};
```

**字段顺序重要吗？**：

```rust,ignore
// 这些是等价的！
let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("bob"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    username: String::from("bob"),
    active: true,
    email: String::from("test@example.com"),
    sign_in_count: 1,
};
```

✅ 顺序不重要！Rust 通过字段名匹配，不是位置。

### 4. 访问字段

使用**点号**访问：

```rust,ignore
let user = User {
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

println!("用户名：{}", user.username);  // alice
println!("邮箱：{}", user.email);       // test@example.com

user.active = false;  // ✅ 可以修改（如果变量是可变的）
```

**所有权规则**：

```rust,ignore
let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

let email = user1.email;  // email 获得 String 的所有权
println!("{}", email);     // ✅ 可以
// println!("{}", user1.email); // ❌ 错误！email 已经移动了
```

### 5. 字段初始化简写

当变量名和字段名**相同时**，可以简写：

```rust,ignore
fn build_user(email: String, username: String) -> User {
    User {
        email: email,     // 重复
        username: username, // 重复
        active: true,
        sign_in_count: 1,
    }
}

// 简写版本
fn build_user(email: String, username: String) -> User {
    User {
        email,      // ✅ 简写！
        username,   // ✅ 简写！
        active: true,
        sign_in_count: 1,
    }
}
```

**为什么这样设计？**
- 减少重复代码
- 参数名和字段名通常相同
- 代码更清晰

### 6. 结构体更新语法

使用已有实例创建新实例：

```rust,ignore
let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 其他字段从 user1 复制
};
```

**发生了什么？**

- `email` 使用了新值
- `username`, `active`, `sign_in_count` 从 `user1` **移动**到 `user2`

**注意**：

```rust,ignore
// println!("{}", user1.username); // ❌ 错误！已经移动给 user2
println!("{}", user1.email); // ✅ 可以，email 是新创建的
```

### 7. 元组结构体

当结构体只有**一个字段**，或者你不想给字段命名时：

```rust,ignore
struct Color(i32, i32, i32);  // RGB
struct Point(i32, i32, i32);  // 3D 坐标

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

println!("黑色：({},{},{})", black.0, black.1, black.2);
println!("原点：({},{},{})", origin.0, origin.1, origin.2);
```

**使用场景**：

- 当结构体就是一个包装类型
- 字段有明显顺序（如坐标 x, y, z）
- 不需要字段名

### 8. 单元结构体

没有任何字段的结构体：

```rust,ignore
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

**有什么用？**：

- 实现 trait 但不存储数据
- 标记类型（marker type）
- 泛型编程中的占位符

### 9. 组合 vs 继承

**为什么 Rust 没有继承？**

如果你来自 Java、C++ 或 Python，可能会疑惑：为什么 Rust 没有 `class` 和 `extends`？答案是 **Rust 选择了组合而非继承**。

**继承的问题**：

想象一家餐厅。老板规定"所有员工都必须会做饭"。这听起来合理，但如果你雇佣了一个收银员呢？收银员继承"员工"的行为，但不需要做饭。这就是**脆弱基类问题**——父类的改变会破坏子类。

```java
// Java 继承的困境
class Employee {
    void cook() { /* 做饭 */ }
}

class Cashier extends Employee {
    // 收银员被迫"会做饭"？但实际不需要！
    // 父类改变会影响所有子类
}
```

**组合的解决方案**：

Rust 用 trait + 组合解决这个问题。每个员工有不同的能力组合：

```rust,ignore
// Rust 的组合模式
trait Cook {
    fn cook(&self);
}

trait HandleCash {
    fn handle_cash(&self);
}

struct Chef;
struct Cashier;

impl Cook for Chef {
    fn cook(&self) {
        println!("制作美食");
    }
}

impl HandleCash for Cashier {
    fn handle_cash(&self) {
        println!("处理收银");
    }
}
```

**三个关键差异**：

| 维度 | 继承 (Java/C++) | 组合 (Rust/Go) |
|------|----------------|----------------|
| 耦合度 | 紧耦合，父类改动影响子类 | 松耦合，trait 独立变化 |
| 灵活性 | 单继承限制，难以混合行为 | 自由组合多个 trait |
| 可测试性 | 需要模拟整个父类 | 只需模拟依赖的 trait |
| 代码复用 | 通过继承链 | 通过 trait + 组合 |
| 运行时行为 | 编译时固定 | 动态分发可选 |

**实战对比：游戏角色**

```java
// Java: 继承链越来越深
class Character {
    void move() {}
}
class FlyingCharacter extends Character {
    void fly() {}
}
class SwimmingFlyingCharacter extends FlyingCharacter {
    void swim() {}  // 继承链爆炸！
}
```

```rust,ignore
// Rust: 灵活组合
trait Move { fn move(&self); }
trait Fly { fn fly(&self); }
trait Swim { fn swim(&self); }

struct Dragon;

impl Move for Dragon { fn move(&self) {} }
impl Fly for Dragon { fn fly(&self) {} }
impl Swim for Dragon { fn swim(&self) {} }
// 自由组合，无需继承链
```

**最佳实践**：

- ✅ 用 trait 定义行为接口
- ✅ 用组合组装复杂对象
- ✅ 用 `impl Trait for Type` 实现多态
- ❌ 避免深层继承链
- ❌ 避免为复用代码而继承

---

## 常见错误

### 错误 1: 忘记字段类型

```rust,ignore
struct User {
    username,  // ❌ 编译错误!
    email,
}
```

**编译器输出**:
```
error: expected `:`, found `,`
 --> src/main.rs:2:14
  |
2 |     username,  // ❌ 编译错误!
  |              ^ expected `:`
```

**修复方法**：

添加类型注解：
```rust,ignore
struct User {
    username: String,  // ✅
    email: String,
}
```

---

### 错误 2: 移动后使用字段

```rust,ignore
let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // user1 的字段移动给 user2
};

println!("{}", user1.username); // ❌ 编译错误!
```

**编译器输出**:
```
error[E0382]: borrow of partially moved value: `user1`
  --> src/main.rs:14:20
   |
9  |       email: String::from("another@example.com"),
   |                            --------------------- value partially moved here
...
14 |     println!("{}", user1.username);
   |                    ^^^^^^^^^^^^^^ value borrowed here after partial move
```

**修复方法**：

1. **使用引用而不是移动**：
   ```rust,ignore
   let user2 = &user1; // 借用，不移动
   println!("{}", user1.username); // ✅ 可以
   ```

2. **不要使用更新语法**，手动复制所有字段：
   ```rust,ignore
   let user2 = User {
       email: String::from("another@example.com"),
       username: user1.username.clone(), // 克隆
       active: user1.active,
       sign_in_count: user1.sign_in_count,
   };
   ```

---

### 错误 3: 试图修改不可变结构体

```rust,ignore
let user = User {
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

user.active = false; // ❌ 编译错误!
```

**编译器输出**:
```
error[E0594]: cannot assign to `user.active`, as `user` is not declared as mutable
  --> src/main.rs:10:5
   |
2  |     let user = User {
   |         ---- help: consider changing this to be mutable: `mut user`
...
10 |     user.active = false;
   |     ^^^^^^^^^^^^^^^^^^^ cannot assign
```

**修复方法**：

声明为可变：
```rust,ignore
let mut user = User {  // 添加 mut
    email: String::from("test@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

user.active = false; // ✅ 现在可以了
```

---

## 动手练习

### 练习 1: 预测所有权

预测下面代码哪些会编译通过：

```rust,ignore
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let p2 = p1;
    
    println!("{}", p1.name); // A: 编译通过还是失败？
    println!("{}", p2.name); // B: 编译通过还是失败？
    println!("{}", p2.age);  // C: 编译通过还是失败？
}
```

<details>
<summary>点击查看答案</summary>

**答案**:
- A: ❌ 失败 - `p1.name` 已经移动给 `p2`
- B: ✅ 通过 - `p2` 拥有 `name`
- C: ✅ 通过 - `age` 是 `u32`，实现了 `Copy` trait

**解析**:
`String` 会被移动，但 `u32` 会复制。所以`p1.age`仍然可用，但`p1.name`不可用。

</details>

---

### 练习 2: 使用更新语法

使用更新语法补全代码，使得 `user2` 的 `email` 不同，其他字段和 `user1` 相同：

```rust,ignore
struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("alice"),
        active: true,
    };
    
    let user2 = User {
        // TODO: 使用更新语法
    };
}
```

<details>
<summary>点击查看答案</summary>

**答案**:
```rust,ignore
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

**注意**: `user1.username` 和 `user1.active` 会移动到`user2`。

</details>

---

### 练习 3: 字段初始化简写

使用字段初始化简写重写函数：

```rust,ignore
fn create_point(x: i32, y: i32, z: i32) -> Point {
    Point {
        x: x,
        y: y,
        z: z,
    }
}
```

<details>
<summary>点击查看答案</summary>

**答案**:
```rust,ignore
fn create_point(x: i32, y: i32, z: i32) -> Point {
    Point {
        x,  // 简写！
        y,  // 简写！
        z,  // 简写！
    }
}
```

**规则**: 当变量名和字段名相同时，可以省略冒号和值。

</details>

---

## 故障排查 (FAQ)

### Q: 什么时候应该用结构体，什么时候用元组？

**A**: 遵循这个原则：

**使用结构体**：
- 字段有**明确含义**（如 `name`、`age`）
- 需要字段名提高可读性
- 字段可能变化或扩展

**使用元组结构体**：
- 字段是**同类数据**（如坐标 x, y, z）
- 只需要一个简单的包装
- 字段有固定顺序且很明显

示例：
```rust,ignore
// ✅ 结构体 - 字段有不同含义
struct Person {
    name: String,
    age: u32,
    email: String,
}

// ✅ 元组结构体 - 都是坐标
struct Point(i32, i32, i32);
```

---

### Q: 如何让结构体可以打印（Debug）？

**A**: 使用 `#[derive(Debug)]` 属性：

```rust,ignore
#[derive(Debug)]
struct User {
    username: String,
    email: String,
}

let user = User {
    username: String::from("alice"),
    email: String::from("test@example.com"),
};

println!("{:?}", user);  // ✅ 可以打印
// println!("{}", user); // ❌ 仍然不可以，需要实现 Display trait
```

输出：
```
User { username: "alice", email: "test@example.com" }
```

---

### Q: 结构体字段可以是私有的吗？

**A**: ✅ 可以！使用访问控制：

```rust,ignore
mod user_module {
    pub struct User {
        pub username: String,  // 公开
        email: String,         // 私有！
    }
    
    impl User {
        pub fn get_email(&self) -> &str {
            &self.email  // 模块内可以访问
        }
    }
}
```

**默认是私有的**：
- `pub struct` - 结构体本身公开
- `pub field` - 字段公开
- 没有 `pub` - 私有

---

## 知识扩展 (选学)

### 方法（关联函数）

结构体可以有**方法** - 这是下一章节的内容，先预览一下：

```rust,ignore
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(30, 50);  // 关联函数
println!("面积：{}", rect.area());  // 方法
```

---

### 生命周期标注

当结构体包含**引用**时，需要生命周期标注：

```rust,ignore
struct RectangleRef<'a> {
    width: &'a u32,
    height: &'a u32,
}
```

生命周期 `'a` 告诉编译器：引用的有效期至少和结构体一样长。

这是高级主题，后续章节会详细讨论。

---

## 小结

**核心要点**：

1. **结构体组合相关数据** - 像数据库记录一样组织信息
2. **字段初始化简写** - 当变量名和字段名相同时可以省略
3. **更新语法 `..instance`** - 从已有实例创建新实例
4. **所有权规则适用** - 字段可以移动、借用、复制
5. **元组结构体用于简单包装** - 当只需要组合不需要字段名时

**关键术语**：

- **Struct (结构体)**: 自定义数据类型，包含命名字段
- **Field (字段)**: 结构体的数据成员
- **Instance (实例)**: 结构体的具体值
- **Tuple struct (元组结构体)**: 没有字段名的结构体
- **Field init shorthand (字段初始化简写)**: `x` 替代 `x: x`
- **Update syntax (更新语法)**: `..instance` 复制其他字段

**下一步**：

- 继续：[结构体方法](struct-methods.md) - 为结构体添加功能
- 相关：[枚举](enums.md) - 另一种数据组合方式
- 进阶：[特征](trait.md) - 定义行为接口

---

## 术语表

| English       | 中文       |
| ------------- | ---------- |
| Struct        | 结构体     |
| Field         | 字段       |
| Instance      | 实例       |
| Tuple struct  | 元组结构体 |
| Field init shorthand | 字段初始化简写 |
| Update syntax | 更新语法   |

---

完整示例：`src/basic/rectangle.rs`

---

## 延伸阅读

学习完结构体后，你可能还想了解：

- [元组结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields) - 匿名结构体
- [单元结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields) - 标记类型
- [更新语法](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) - 结构体更新

**选择建议**:
- 想学习枚举 → 继续学习 [枚举](enums.md)
- 想学习方法 → 跳到 [结构体方法](struct-methods.md)

## 继续学习

- 下一步：[结构体方法](struct-methods.md)
- 相关：[字段访问](struct-fields.md)
- 进阶：[生命周期](lifetimes.md)

> 💡 **提示**：结构体是你每天都会在 Rust 中使用的工具。多练习创建、访问和组织数据，你会很快掌握它！

---

## 💡 小知识：结构体 vs 元组

**元组的问题**：
```rust,ignore
// 元组表示用户
let user = ("Alice", 30, "alice@example.com");

// 访问字段 - 需要记住每个位置的含义
println!("姓名：{}", user.0);  // 0 是什么？
println!("年龄：{}", user.1);  // 1 是什么？
println!("邮箱：{}", user.2);  // 2 是什么？
```

**结构体的优势**：
```rust,ignore
// 结构体表示用户
struct User {
    name: String,
    age: u32,
    email: String,
}

let user = User {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
};

// 访问字段 - 名称自说明
println!("姓名：{}", user.name);    // ✅ 一目了然
println!("年龄：{}", user.age);     // ✅
println!("邮箱：{}", user.email);   // ✅
```

**何时使用**：
- **元组**: 临时组合、返回值、模式匹配
- **结构体**: 有明确含义的数据、需要长期存储

**元组结构体** (混合方案)：
```rust,ignore
struct Color(i32, i32, i32);  // RGB
let black = Color(0, 0, 0);
println!("R: {}", black.0);  // 仍用数字索引
```

---

## 🌟 工业界应用：游戏角色系统

**场景**：RPG 游戏角色定义

```rust,ignore
struct Character {
    name: String,           // 角色名
    level: u32,             // 等级
    health: f32,            // 生命值 (0.0 - 100.0)
    experience: u64,        // 经验值
    inventory: Vec<Item>,   // 背包物品
    position: Position,     // 当前位置
}

struct Position {
    x: f32,
    y: f32,
    z: f32,  // 3D 坐标
}

struct Item {
    name: String,
    item_type: ItemType,
}

enum ItemType {
    Weapon,
    Armor,
    Potion,
}

// 使用示例
fn main() {
    let hero = Character {
        name: String::from("勇者"),
        level: 10,
        health: 85.5,
        experience: 1500,
        inventory: Vec::new(),
        position: Position { x: 0.0, y: 0.0, z: 0.0 },
    };
    
    println!("{} 等级 {}", hero.name, hero.level);
}
```

**为什么用结构体**：
1. **可读性** - 字段名称说明用途
2. **类型安全** - 编译器检查字段类型
3. **可维护性** - 添加新字段不影响现有代码
4. **文档化** - 结构本身就是文档

---

## 🧪 动手试试：设计结构体

**练习**：为图书管理系统设计结构体

```rust,ignore
// TODO: 定义 Book 结构体
// 字段：title, author, year, isbn, available

// TODO: 定义 Library 结构体
// 字段：name, books (Vec<Book>)

// 提示：
// - ISBN 是字符串
// - year 是整数
// - available 是布尔值
// - books 是 vector
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
    available: bool,
}

struct Library {
    name: String,
    books: Vec<Book>,
}

// 使用示例
fn main() {
    let book = Book {
        title: String::from("Rust 编程"),
        author: String::from("张三"),
        year: 2024,
        isbn: String::from("978-7-121-12345-6"),
        available: true,
    };
    
    let library = Library {
        name: String::from("市图书馆"),
        books: vec![book],
    };
}
```

**扩展思考**：
- 如何表示借阅记录？
- 如何处理多册同一本书？
- 如何快速查找某本书？

</details>

---

## 内存布局可视化

### 1. 结构体内存布局

```
Rectangle struct (8 bytes):
+0x00        +0x04
+------------+------------+
| width(u32) | height(u32)|
|  4 bytes   |  4 bytes   |
+------------+------------+
```

**说明**:
- u32 类型占用 4 字节
- 无填充，紧密排列
- 总计 8 字节

### 2. 字段访问模式

```
rect ──────────→ [Rectangle struct]
                   ├─ width: 10
                   └─ height: 20

rect.width  ───→ 直接字段访问 (10)
rect.height ───→ 直接字段访问 (20)
```

### 3. 方法调用流程

```
rect.area()
    │
    ↓
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
        // 10 * 20 = 200
    }
}
```

### 4. 结构体更新语法

```
let user1 = User { active: true, username: "alice" };

let user2 = User {
    username: "bob",  // 新值
    ..user1           // 其他字段从 user1 复制
};

内存布局:
user1 无效 (username 已转移)
user2 有效 (拥有新 username)
```

---

## 知识检查

**问题 1** 🟢 (字段访问)

如何修改结构体字段的值？

```rust,ignore
struct Point {
    x: i32,
    y: i32,
}

let mut p = Point { x: 5, y: 10 };
// 如何将 x 改为 15？
```

<details>
<summary>答案与解析</summary>

**答案**: `p.x = 15;`

**解析**: 需要 mut 标记变量可变，然后通过点号访问修改字段。
</details>

**问题 2** 🟡 (方法语法)

以下哪种方法是正确的？

```rust,ignore
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

A) 只有 area 正确  
B) 只有 square 正确  
C) 都正确  
D) 都不正确

<details>
<summary>答案与解析</summary>

**答案**: C) 都正确

**解析**: area 是实例方法（使用&self），square 是关联函数（构造器模式）。
</details>

**问题 3** 🔴 (结构体更新)

这段代码的输出是什么？

```rust,ignore
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
}

let user1 = User {
    active: true,
    username: String::from("alice"),
};

let user2 = User {
    username: String::from("bob"),
    ..user1
};

println!("{}", user1.active);
```

<details>
<summary>答案与解析</summary>

**答案**: 编译错误！

**解析**: String 不是 Copy 类型，`..user1` 会转移 username 的所有权，导致 user1 无效。

**修复**: 使用 `..user1.clone()` 或改用 `&str`
</details>

