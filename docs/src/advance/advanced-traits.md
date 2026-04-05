# 高级特征 (Advanced Traits)

## 开篇故事

想象你在设计一个通用的遥控器。基础遥控器只能开关电视。高级遥控器不仅能开关，还能根据电视型号自动调整频道，甚至能学习你的习惯。Rust 的特征系统也是如此——除了基础的方法定义，它还支持关联类型、默认泛型参数等高级功能，让你设计出更灵活、更强大的接口。

---

## 本章适合谁

如果你已经掌握了特征的基础用法，现在想深入理解标准库中的复杂特征（如 `Iterator`、`Add`），或者想解决同名方法冲突，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用关联类型 (Associated Types) 定义特征
2. 使用默认泛型类型参数
3. 使用完全限定语法 (Fully Qualified Syntax) 解决冲突
4. 理解 Supertraits (特征继承)
5. 使用 Newtype 模式实现外部类型的特征

---

## 前置要求

- [特征](../basic/trait.md) - 特征基础
- [泛型](../basic/generic.md) - 泛型基础

---

## 第一个例子

使用关联类型定义特征：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

**发生了什么？**
- `type Item`: 这是一个**关联类型**占位符。
- 实现者必须指定 `Item` 的具体类型。
- `Self::Item`: 使用关联类型。

---

## 原理解析

### 1. 关联类型 (Associated Types)

关联类型允许我们在特征定义中声明一个类型占位符，而不需要在使用特征时指定它。

**对比泛型**:

```rust
// 使用泛型
trait Container<T> {
    fn get(&self) -> &T;
}

// 使用关联类型
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

**为什么使用关联类型**:
- **更简洁**: 实现时不需要写 `impl Container<i32> for MyType`，只需 `impl Container for MyType`。
- **唯一性**: 一个类型只能实现一次带有特定关联类型的特征，而泛型可以实现多次（不同参数）。

**标准库示例**: `std::iter::Iterator`

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // 指定关联类型

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 2. 默认泛型类型参数 (Default Generic Type Parameters)

我们可以为泛型参数指定默认值，通常用于运算符重载。

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add trait 定义: trait Add<RHS = Self>
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    assert_eq!(p1 + p2, Point { x: 4, y: 6 });
}
```

### 3. 完全限定语法 (Fully Qualified Syntax)

当多个特征或类型有同名方法时，编译器可能无法推断使用哪个。

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) { println!("Pilot flying"); }
}

impl Wizard for Human {
    fn fly(&self) { println!("Wizard flying"); }
}

impl Human {
    fn fly(&self) { println!("Human flying"); }
}

fn main() {
    let person = Human;
    person.fly(); // 调用 Human 的 fly (方法优先级高于 trait)
    
    // 调用特定 trait 的方法
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}
```

### 4. Supertraits (特征继承)

Rust 没有传统的类继承，但可以通过 Supertraits 实现类似功能：一个特征要求实现者必须也实现另一个特征。

```rust
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}
```

### 5. Newtype 模式 (Newtype Pattern)

为了在外部类型上实现外部特征（孤儿规则限制），我们可以使用 Newtype 模式：创建一个包含该类型的元组结构体。

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w); // w = [hello, world]
}
```

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到高级特征时的困惑——你并不孤单！**

### 困惑 1: "关联类型和泛型有什么区别？"

**解答**: 
- **泛型**: 允许实现者为**同一个类型**实现特征**多次**（使用不同的类型参数）。
- **关联类型**: 一个类型只能实现特征**一次**，关联类型是固定的。

```rust
// 泛型：可以实现多次
impl Container<i32> for MyType { ... }
impl Container<String> for MyType { ... }

// 关联类型：只能实现一次
impl Container for MyType { type Item = i32; ... }
```

### 困惑 2: "为什么要用完全限定语法？"

**解答**: 当方法名冲突时。比如 `Human` 有自己的 `fly` 方法，同时也实现了 `Pilot` 和 `Wizard` 的 `fly` 方法。编译器默认调用类型自身的方法。要调用特征的，必须用 `<Type as Trait>::method()`。

---

## 常见错误

### 错误 1: 孤儿规则 (Orphan Rule) 违规

```rust,ignore
// ❌ 错误：Vec 和 Display 都来自外部 crate
impl std::fmt::Display for Vec<String> { ... }
```

**修复方法**: 使用 Newtype 模式。

```rust
struct MyVec(Vec<String>);
impl std::fmt::Display for MyVec { ... }
```

---

## 动手练习

### 练习 1: 定义带关联类型的特征

定义一个 `Mapper` 特征，包含关联类型 `Output` 和方法 `map`。

```rust
// TODO: 定义 Mapper trait

struct MyList;

// TODO: 为 MyList 实现 Mapper，Output 为 i32

fn main() {
    let list = MyList;
    // 使用 list.map(...)
}
```

<details>
<summary>点击查看答案</summary>

```rust
trait Mapper {
    type Output;
    fn map(&self, input: i32) -> Self::Output;
}

struct MyList;

impl Mapper for MyList {
    type Output = Vec<i32>;
    fn map(&self, input: i32) -> Self::Output {
        vec![input * 2]
    }
}
```
</details>

---

## 小结

**核心要点**:

1. **关联类型**: 简化特征实现，确保唯一性。
2. **默认泛型参数**: 用于运算符重载等场景。
3. **完全限定语法**: 解决同名方法冲突。
4. **Supertraits**: 特征之间的依赖关系。
5. **Newtype 模式**: 绕过孤儿规则。

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Associated Type | 关联类型 |
| Default Generic Parameter | 默认泛型参数 |
| Fully Qualified Syntax | 完全限定语法 |
| Supertrait | 超特征 |
| Orphan Rule | 孤儿规则 |
| Newtype Pattern | 新类型模式 |

---

## 延伸阅读

- [Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [Operator Overloading](https://doc.rust-lang.org/std/ops/trait.Add.html)

## 继续学习

- 下一步：[原子类型](atomic-types.md)
- 回顾：[特征](../basic/trait.md)

---

## 知识检查

**问题 1** 🟢 (基础)

关联类型的主要优势是什么？

A) 允许一个类型实现特征多次  
B) 简化实现语法，确保唯一性  
C) 提高运行时性能  
D) 允许运行时类型检查

<details>
<summary>点击查看答案</summary>

**答案**: B) 简化实现语法，确保唯一性
</details>
