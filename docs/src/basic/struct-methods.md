# 结构体方法

## 开篇故事

想象你有了一辆自行车。自行车不仅是一堆零件的组合（车架、轮子），它还能做动作：可以蹬踏板加速、可以刹车减速。Rust 的**方法**就像是结构体的"动作" - 它让结构体不仅能存储数据，还能对外界做出反应。

---

## 本章适合谁

如果你已经学完了结构体和字段，现在想让结构体"动起来"（不仅仅是数据容器），本章适合你。这是迈向面向对象编程的关键一步。

---

## 你会学到什么

完成本章后，你可以：

1. 为结构体实现方法
2. 理解 `self`、`&self`、`&mut self` 的区别
3. 创建关联函数（类似静态方法）
4. 使用方法修改结构体字段

---

## 前置要求

学习本章前，你需要理解：

- [结构体基础](struct.md) - 结构体定义
- [结构体字段](struct-fields.md) - 字段访问
- [所有权](ownership.md) - 借用和移动

---

## 第一个例子

定义一个矩形结构体并添加计算面积的方法：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("面积是：{} 平方像素", rect.area());
}
```

**输出**：
```
面积是：1500 平方像素
```

**关键概念**：

- `impl` - 实现块
- `&self` - 方法的第一个参数（调用者）
- 点号调用：`rect.area()`

---

### Python/Java/C++ vs Rust 对比

如果你有其他语言经验，这个对比会帮助你快速理解：

| 概念         | Python               | Java                   | C++                    | Rust                       | 关键差异                  |
| ------------ | -------------------- | ---------------------- | ---------------------- | -------------------------- | ------------------------- |
| 方法定义     | `def method(self):`  | `void method()`        | `void method()`        | `fn method(&self)`         | Rust 用 `impl` 块         |
| self 参数    | `self` 显式          | `this` 隐式            | `this` 隐式            | `&self` 显式               | Rust 显式且可选引用       |
| 可变方法     | 无需声明             | 无需声明               | 无需声明               | 需要 `&mut self`           | Rust 需显式可变借用       |
| 关联函数     | `@staticmethod`      | `static method()`      | `static method()`      | `fn new()` (无 self)       | Rust 无 `static` 关键字   |
| 方法链       | `return self`        | `return this`          | `return *this`         | `&mut Self`                | Rust 返回引用             |

**核心差异**: Python/Java/C++ 的 `this/self` 隐式传递，Rust 的 `self` 显式且需选择借用方式。

---

## 原理解析

### 1. 实现块 (impl)

使用 `impl` 关键字为结构体添加方法：

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

### 2. 方法接收者

方法的第一参数决定调用方式：

**`&self` - 借用（不修改）**：

```rust
impl Rectangle {
    fn area(&self) -> u32 {  // 只读，不修改
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect.area()); // rect 仍然可用
```

**`&mut self` - 可变借用（修改）**：

```rust
impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2;  // 修改字段
    }
}

let mut rect = Rectangle { width: 30, height: 50 };
rect.double_width();  // rect 现在 width=60
```

**`self` - 获取所有权（消耗）**：

```rust
impl Rectangle {
    fn into_components(self) -> (u32, u32) {
        (self.width, self.height)  // 消耗 rect
    }
}

let rect = Rectangle { width: 30, height: 50 };
let (w, h) = rect.into_components(); // rect 不能再使用
```

### 3. 关联函数

关联函数（类似其他语言的静态方法）：

```rust
impl Point {
    // 关联函数 (没有 self 参数)
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    // 方法 (有 self 参数)
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0);  // 关联函数
    let p2 = Point::new(3.0, 4.0);
    println!("距离：{}", p1.distance(&p2));  // 方法
}
```

### 4. 多个实现块

可以使用多个 `impl` 块：

```rust
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        // 实现
    }
}
```

**注意**：通常合并到一个 `impl` 块，除非使用 trait。

### 5. 方法链

通过返回 `&mut Self` 实现方法链：

```rust
impl Rectangle {
    fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self  // 返回自身引用
    }
    
    fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
}

let mut rect = Rectangle { width: 30, height: 50 };
rect.set_width(100).set_height(200);  // 链式调用
```

---

## 常见错误

### 错误 1: 忘记 `&`

```rust
impl Rectangle {
    fn area(self) -> u32 {  // ❌ 会消耗 rect
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect.area());
// println!("{}", rect.width); // ❌ 错误！rect 被消耗了
```

**修复**：使用 `&self`

```rust
fn area(&self) -> u32 {  // ✅ 借用，不消耗
```

### 错误 2: 忘记 `mut`

```rust
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    rect.double_width();  // ❌ 错误！rect 不是可变的
}
```

**修复**：声明为 `mut`

```rust
let mut rect = Rectangle { width: 30, height: 50 };
rect.double_width();  // ✅
```

### 错误 3: 错误使用关联函数

```rust
let p1 = Point { x: 0.0, y: 0.0 };
p1.new(1.0, 1.0);  // ❌ 错误！new 是关联函数
```

**修复**：使用结构体名调用

```rust
let p1 = Point::new(0.0, 0.0);  // ✅
```

---

## 动手练习

### 练习 1: 实现方法

为 `Person` 结构体实现 `greet` 方法：

```rust
struct Person {
    name: String,
}

impl Person {
    // TODO: 实现 greet 方法，打印"Hello, my name is {name}"
}

fn main() {
    let p = Person { name: String::from("Alice") };
    p.greet();
}
```

<details>
<summary>点击查看实现</summary>

```rust
fn greet(&self) {
    println!("Hello, my name is {}", self.name);
}
```
</details>

### 练习 2: 使用 &mut self

为 `Counter` 结构体实现 `increment` 方法：

```rust
struct Counter {
    count: u32,
}

impl Counter {
    // TODO: 实现 increment，使 count 加 1
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn increment(&mut self) {
    self.count += 1;
}
```
</details>

---

## 故障排查 (FAQ)

### Q: 什么时候用 `&self`，什么时候用 `self`？

**A**:

- **`&self`** (99% 情况): 只读操作，不需要修改
- **`&mut self`**: 需要修改结构体
- **`self`**: 需要消耗结构体，返回内部数据

### Q: `impl` 块可以嵌套吗？

**A**: ❌ 不可以。但可以写在其他 `impl` 块内。

### Q: 方法可以是私有的吗？

**A**: ✅ 可以！默认私有：

```rust
impl Rectangle {
    fn internal_helper(&self) { /* 私有方法 */ }
    pub fn public_method(&self) { /* 公有方法 */ }
}
```

---

## 小结

**核心要点**：

1. **`impl` 块**: 为结构体添加方法
2. **`&self`**: 借用，最常用
3. **`&mut self`**: 可变借用，修改字段
4. **`self`**: 获取所有权，消耗实例
5. **关联函数**: 类似静态方法，没有 `self`

**术语**：

- **Method (方法)**: 与结构体关联的函数
- **Receiver (接收者)**: 方法的第一个参数（`self`）
- **Associated function (关联函数)**: 没有 `self` 的方法

**下一步**：

- 相关：[枚举](enums.md)
- 进阶：[特征](trait.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Method | 方法 |
| impl | 实现 |
| Receiver | 接收者 |
| Associated function | 关联函数 |

---

完整示例：`src/basic/rectangle.rs`

---

> 💡 **提示**：方法让结构体"活"起来 - 它们不仅能存储数据，还能处理数据！

---

## 知识检查

**快速测验**（答案在下方）：

1. 方法和函数有什么区别？

2. `&self`、`&mut self`、`self` 的区别？

3. 关联函数和实例方法的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 方法在 `impl` 块中定义，第一个参数是 `self`
2. `&self` 借用，`&mut self` 可变借用，`self` 获取所有权
3. 关联函数没有 `self` 参数（如 `new()`），实例方法有 `self`

**关键理解**: 方法是附加到结构体上的函数，`self` 决定访问模式。
</details>

## 延伸阅读

学习完结构体方法后，你可能还想了解：

- [关联函数](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions) - new() 等工厂方法
- [方法链](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#multiple-impl-blocks) - Builder 模式
- [Deref trait](https://doc.rust-lang.org/book/ch15-02-deref.html) - 智能指针方法调用

**选择建议**:
- 想学习枚举 → 继续学习 [枚举](enums.md)
- 想学习特征 → 跳到 [特征](trait.md)

## 继续学习

**前一章**: [结构体字段](struct-fields.md)  
**下一章**: [枚举](enums.md)

**相关章节**:
- [结构体字段](struct-fields.md)
- [结构体](struct.md)
- [枚举](enums.md)

**返回**: [基础入门](basic-overview.md)
