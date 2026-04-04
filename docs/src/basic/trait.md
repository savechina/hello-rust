# 特征 (Traits)

## 开篇故事

想象你开了一家手机店。店里有 iPhone、Android 手机、老人机。虽然它们不一样，但都能"打电话"和"发短信"。Rust 的**特征**就像这个"手机"的概念 - 它定义了"能做什么"，而不关心具体是什么手机。

---

## 本章适合谁

如果你已经学完了结构体和枚举，现在想学习如何定义通用行为，本章适合你。特征是 Rust 泛型和代码复用的核心。

---

## 你会学到什么

1. 定义和实现特征
2. 使用特征作为约束
3. 理解特征对象
4. 实现标准库特征
5. 使用默认方法实现

---

## 前置要求

- [结构体](struct.md)
- [枚举](enums.md)

---

## 第一个例子

定义一个可以"叫"的特征：

```rust
trait Speak {
    fn speak(&self) -> &str;
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) -> &str {
        "汪汪！"
    }
}

impl Speak for Cat {
    fn speak(&self) -> &str {
        "喵喵！"
    }
}

fn make_sound(animal: &impl Speak) {
    println!("{}", animal.speak());
}
```

---

## 原理解析

### 1. 特征定义

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

### 2. 特征实现

```rust
struct NewsArticle {
    headline: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}", self.headline, self.location)
    }
}
```

### 3. 默认方法

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // 默认实现
    }
}
```

### 4. 特征约束

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 或者
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

### 5. 多个约束

```rust
fn notify<T: Summary + Clone>(item: &T) {
    // T 必须同时实现 Summary 和 Clone
}
```

---

## 常见错误

### 错误 1: 忘记实现必需方法

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article;

impl Summary for Article {
    // ❌ 缺少 summarize 方法
}
```

### 错误 2: 方法签名不匹配

```rust
impl Summary for Article {
    fn summarize(&self) -> &str {  // ❌ 返回类型应该是 String
        "summary"
    }
}
```

### 错误 3: 自相矛盾的特征约束

```rust
fn process<T: Summary + !Summary>(item: &T) {
    // ❌ 逻辑矛盾
}
```

---

## 动手练习

### 练习 1: 定义特征

定义一个可飞行的特征：

```rust
// TODO: 定义 Fly 特征，包含 fly 方法
```

<details>
<summary>点击查看答案</summary>

```rust
trait Fly {
    fn fly(&self) -> &str;
}
```
</details>

---

## 小结

**要点**：

1. **特征定义行为**: 描述类型能做什么
2. **实现提供具体方法**: `impl Trait for Type`
3. **默认方法减少重复**: 可选覆盖
4. **特征约束限制泛型**: `T: Trait`
5. **特征对象实现多态**: `dyn Trait`

**下一步**：

- 相关：[泛型](generic.md)
- 进阶：[特征对象](trait-objects.md)

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Trait | 特征 |
| Implement | 实现 |
| Default method | 默认方法 |

---

> 💡 **提示**：特征是 Rust 多态的核心 - 掌握它就掌握了泛型编程的钥匙！

---

## 项目实例

### 实际代码示例

```rust
// src/basic/traits_sample.rs

/// Printable 特性示例
trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32,
}

/// 实现 Printable 特性
impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    person.print();  // 输出：Name: Alice, Age: 30
}
```

### 特性继承

```rust
// 定义 Trait A
trait A {
    fn method_a(&self);
}

// Trait B 继承 Trait A
trait B: A {
    fn method_b(&self);
}

struct MyStruct;

impl B for MyStruct {
    fn method_b(&self) {
        println!("Implemented method_b");
    }
}

// 必须也要实现 A
impl A for MyStruct {
    fn method_a(&self) {
        println!("Implemented method_a");
    }
}
```

### 对象安全警告

```rust
// ⚠️ 注意：这个方法不是对象安全的
trait UnObjectSafeTrait {
    fn create() -> Self;  // Error: 不能对象安全
}

// ❌ 以下代码会失败：
// fn create_trait_object() -> Box<dyn UnObjectSafeTrait> {
//     Box::new(UnObjectSafeTrait::create())
// }
```


---

## 延伸阅读

学习完特征后，你可能还想了解：

- [孤儿规则](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type) - 为什么不能为外部类型实现外部 trait
- [默认实现](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) - trait 方法默认行为
- [关联类型 vs 泛型](associated-types-vs-generics.md) - 何时使用哪个

**选择建议**:
- 想学习运行时多态 → 继续学习 [特征对象](trait-objects.md)
- 想学习编译时多态 → 跳到 [泛型](generic.md)

## 继续学习

**前一章**: [枚举](enums.md)  
**下一章**: [泛型](generic.md)

**相关章节**:
- [枚举](enums.md)
- [结构体](struct.md)
- [泛型](generic.md)

**返回**: [基础入门](basic-overview.md)
