# 迭代器 (Iterators)

## 开篇故事

想象你在流水线上检查产品。传统方式是先拿到所有产品放在桌子上，然后一个一个检查。迭代器则不同——它像是一个传送带，产品在生产出来的同时就被检查，不需要一次性占用所有空间。

Rust 的**迭代器**不仅高效，而且提供了丰富的函数式编程接口（如 `map`, `filter`, `fold`），让你写出更简洁、更安全的代码。

---

## 本章适合谁

如果你已经掌握了基础的循环（`for` 循环），现在想学习更高级的数据处理方式，本章适合你。迭代器是 Rust 标准库中最强大的工具之一。

---

## 你会学到什么

完成本章后，你可以：

1. 理解迭代器的惰性求值 (Lazy Evaluation) 特性
2. 使用 `Iterator` trait 处理集合
3. 使用适配器 (Adapters)：`map`, `filter`, `enumerate`
4. 使用消费者 (Consumers)：`collect`, `fold`, `sum`
5. 创建自定义迭代器

---

## 前置要求

- [闭包](../basic/closure.md) - 闭包语法
- [泛型](../basic/generic.md) - 泛型基础

---

## 第一个例子

使用迭代器处理向量：

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用迭代器处理数据
    let sum: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)  // 只保留偶数
        .map(|&x| x * 2)           // 每个数乘以 2
        .sum();                    // 求和
        
    println!("结果：{}", sum); // 2*2 + 4*2 = 12
}
```

**发生了什么？**
- `iter()`: 获取迭代器。
- `filter`: 适配器，过滤元素。
- `map`: 适配器，转换元素。
- `sum`: 消费者，计算结果。

---

## 原理解析

### 1. 惰性求值 (Lazy Evaluation)

迭代器是**惰性**的。在你调用消费者方法之前，什么都不会发生。

```rust
let numbers = vec![1, 2, 3];

// 只是创建了迭代器，没有执行任何操作
let iter = numbers.iter().map(|x| {
    println!("Processing {}", x);
    x * 2
});

// 调用 collect 时才会执行
let result: Vec<i32> = iter.collect();
```

**输出**:
```
Processing 1
Processing 2
Processing 3
```

### 2. 常用适配器 (Adapters)

适配器将迭代器转换为另一种迭代器。

**map**: 转换每个元素。
```rust
let nums = vec![1, 2, 3];
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

**filter**: 过滤元素。
```rust
let nums = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
```

**enumerate**: 添加索引。
```rust
let names = vec!["Alice", "Bob"];
for (i, name) in names.iter().enumerate() {
    println!("{}: {}", i, name);
}
```

### 3. 常用消费者 (Consumers)

消费者消耗迭代器并产生结果。

**collect**: 收集到集合。
```rust
let nums = vec![1, 2, 3];
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

**fold**: 累积计算。
```rust
let nums = vec![1, 2, 3, 4];
let sum = nums.iter().fold(0, |acc, x| acc + x);
println!("Sum: {}", sum); // 10
```

**find**: 查找第一个匹配项。
```rust
let nums = vec![1, 2, 3, 4];
let first_even = nums.iter().find(|&&x| x % 2 == 0);
println!("{:?}", first_even); // Some(2)
```

### 4. 创建自定义迭代器

实现 `Iterator` trait 即可创建自定义迭代器。

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);
    let sum: u32 = counter.sum();
    println!("Sum: {}", sum); // 1+2+3+4+5 = 15
}
```

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到迭代器时的困惑——你并不孤单！**

### 困惑 1: "迭代器和循环有什么区别？"

**解答**: 
- **循环 (`for`)**: 命令式，关注"怎么做"。
- **迭代器**: 声明式，关注"做什么"。
- **性能**: 迭代器通常和循环一样快（甚至更快），因为编译器可以进行内联优化。

### 困惑 2: "为什么 `iter()` 返回的是引用？"

**解答**: `iter()` 返回元素的引用，避免移动数据。如果你想获取所有权，使用 `into_iter()`。

```rust
let strings = vec![String::from("hi"), String::from("there")];

// iter(): 借用
for s in strings.iter() {
    println!("{}", s);
}
// strings 仍然可用

// into_iter(): 移动
for s in strings.into_iter() {
    println!("{}", s);
}
// strings 不再可用
```

---

## 常见错误

### 错误 1: 忘记调用消费者方法

```rust,ignore
let nums = vec![1, 2, 3];
nums.iter().map(|x| x * 2); // ❌ 警告：未使用的迭代器

// ✅ 正确：调用 collect
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

### 错误 2: 类型推断失败

```rust,ignore
let nums = vec![1, 2, 3];
let result = nums.iter().sum(); // ❌ 错误：不知道 sum 的类型

// ✅ 正确：指定类型
let result: i32 = nums.iter().sum();
```

---

## 动手练习

### 练习 1: 过滤并转换

给定一个字符串列表，过滤出长度大于 3 的字符串，并将其转换为大写。

```rust
fn main() {
    let words = vec!["hi", "hello", "hey", "yo"];
    
    // TODO: 使用 filter 和 map 处理 words
    // 结果应该是 ["HELLO", "HEY"]
}
```

<details>
<summary>点击查看答案</summary>

```rust
let result: Vec<String> = words.iter()
    .filter(|w| w.len() > 3)
    .map(|w| w.to_uppercase())
    .collect();
```
</details>

### 练习 2: 使用 fold 计算阶乘

```rust
fn factorial(n: u32) -> u32 {
    // TODO: 使用 (1..=n).fold(...) 计算阶乘
}

fn main() {
    println!("5! = {}", factorial(5)); // 120
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn factorial(n: u32) -> u32 {
    (1..=n).fold(1, |acc, x| acc * x)
}
```
</details>

---

## 小结

**核心要点**:

1. **惰性求值**: 迭代器在调用消费者前不执行。
2. **适配器**: `map`, `filter`, `enumerate` 等转换迭代器。
3. **消费者**: `collect`, `fold`, `sum` 等消耗迭代器产生结果。
4. **自定义迭代器**: 实现 `Iterator` trait。

**关键术语**:

- **Iterator (迭代器)**: 产生一系列值的对象。
- **Adapter (适配器)**: 转换迭代器的方法。
- **Consumer (消费者)**: 消耗迭代器的方法。
- **Lazy Evaluation (惰性求值)**: 延迟计算直到需要结果。

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Iterator | 迭代器 |
| Adapter | 适配器 |
| Consumer | 消费者 |
| Lazy Evaluation | 惰性求值 |
| Closure | 闭包 |

---

## 延伸阅读

- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## 继续学习

- 下一步：[异步编程](async/async.md)
- 回顾：[闭包](../basic/closure.md)

---

## 知识检查

**问题 1** 🟢 (基础)

迭代器的主要优势是什么？

A) 代码更复杂  
B) 惰性求值和丰富的函数式接口  
C) 占用更多内存  
D) 只能用于 Vec

<details>
<summary>点击查看答案</summary>

**答案**: B) 惰性求值和丰富的函数式接口
</details>
