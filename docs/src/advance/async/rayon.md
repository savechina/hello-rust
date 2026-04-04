# Rayon 数据并行库

## 开篇故事

想象你是一家大型工厂的厂长。工厂里有一堆零件需要加工，传统做法是让一个工人从头到尾完成所有零件。这个工人干得再快，也只是一个 worker 在干活。

现在换一种思路：你把零件分成若干份，让每个工人处理一小批。工人们各司其职，互不干扰，最后汇总结果。这就是**并行计算**的威力。

但是管理多个工人也有挑战：
- 如何分配任务？有的工人干得快，有的干得慢
- 如何避免有的工人闲着，有的工人忙不过来？
- 如何确保最终结果正确汇总？

**Rayon** 就是这个"智能工厂管理系统"——它是 Rust 生态中高性能的数据并行库，让你轻松地将顺序代码转换成并行代码，自动处理任务分配和负载均衡。最妙的是，它通过**工作窃取（Work Stealing）**算法，让空闲线程自动"偷取"忙碌线程的任务，确保所有 CPU 核心都被充分利用。

---

## 本章适合谁

如果你已经掌握了 Rust 基础，现在想要：

- 利用多核 CPU 加速数据处理
- 学习如何将顺序迭代器转换为并行迭代器
- 理解工作窃取调度算法的原理
- 掌握线程池和任务并行的高级用法

本章适合你。Rayon 的学习曲线非常平缓——很多时候，你只需要把 `.iter()` 改成 `.par_iter()`，就能立即获得并行加速。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是数据并行以及 Rayon 的核心优势
2. 使用 `par_iter()` 和 `par_iter_mut()` 进行并行迭代
3. 使用 `into_par_iter()` 进行所有权转移的并行处理
4. 使用 `join()` 并行执行两个独立任务
5. 使用 `scope()` 创建嵌套并行任务
6. 理解工作窃取调度的工作原理
7. 避免常见的并行编程错误

---

## 前置要求

学习本章前，你需要理解：

- [所有权](../basic/ownership.md) - 特别是移动语义和所有权转移
- [闭包](../basic/closure.md) - 闭包作为并行操作的参数
- [迭代器](../basic/iterator.md) - 迭代器的基本用法
- Cargo.toml 中添加 rayon 依赖

**添加依赖**：

```toml
[dependencies]
rayon = "1.7"
```

---

## 第一个例子

让我们从一个最简单的并行计算开始——将数组中的每个元素乘以 2：

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 顺序处理
    println!("顺序处理:");
    for x in &data {
        print!("{} ", x * 2);
    }
    println!();

    // 并行处理 - 只需添加 par_ 前缀！
    println!("并行处理:");
    data.par_iter().for_each(|x| {
        print!("{} ", x * 2);
    });
    println!();
}
```

**发生了什么？**

1. `use rayon::prelude::*` 导入 Rayon 的 trait，使 `.par_iter()` 方法可用
2. `.par_iter()` 创建一个并行迭代器
3. `.for_each()` 并行处理每个元素
4. 执行顺序不确定，但每个元素都会被处理

---

## 原理解析

### 数据并行 vs 任务并行

```
数据并行 (Data Parallelism)          任务并行 (Task Parallelism)
═══════════════════════════════════════════════════════════════════

同一份代码处理不同数据               不同代码并发执行

┌─────────┐                         ┌─────────┐  ┌─────────┐
│ 任务 A  │                         │ 任务 A  │  │ 任务 B  │
├─────────┤                         ├─────────┤  ├─────────┤
│ 数据 1  │──→ 线程1                 │ 下载图片 │  │ 分析日志 │
│ 数据 2  │──→ 线程2                 └─────────┘  └─────────┘
│ 数据 3  │──→ 线程3                         ↓           ↓
│ 数据 4  │──→ 线程4                    ┌─────────────────────┐
└─────────┘                            │     线程池调度      │
                                       └─────────────────────┘

Rayon 主要专注于数据并行，通过并行迭代器让同一份操作在多个数据上并发执行。
```

### Rayon 的核心设计：工作窃取（Work Stealing）

```
工作窃取调度器的工作原理：

初始状态：                           工作窃取发生后：
┌─────────┐  ┌─────────┐            ┌─────────┐  ┌─────────┐
│ 线程 1  │  │ 线程 2  │            │ 线程 1  │  │ 线程 2  │
├─────────┤  ├─────────┤            ├─────────┤  ├─────────┤
│ 任务 1  │  │ 任务 3  │            │ 任务 1  │  │ 任务 3  │
│ 任务 2  │  │ 任务 4  │            │ 任务 2  │  │ 任务 4  │
│ 任务 5  │  │ 任务 6  │            │         │  │ 任务 6  │
│ 任务 7  │  │ 任务 8  │            │ 任务 8  │◄─┤ 任务 7  │  ← 线程2窃取任务7
│ 任务 9  │  │ 任务 10 │            │ 任务 9  │  │ 任务 10 │
│ 任务 11 │  │         │            │ 任务 11 │  │         │
│ 任务 12 │  │         │            │ 任务 12 │  │         │
└─────────┘  └─────────┘            └─────────┘  └─────────┘
  忙碌 ↑       空闲 ↓                  两个线程都在工作！

关键点：
1. 每个线程有自己的任务队列
2. 当线程空闲时，从其他线程"窃取"任务
3. 窃取从队列尾部开始，减少与所有者线程的冲突
4. 确保负载均衡，所有 CPU 核心都被利用
```

### 并行迭代器类型

Rayon 提供了三种主要的并行迭代器：

```
三种并行迭代方式：

┌─────────────────────────────────────────────────────────────────┐
│  原始数据                                                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]                      │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
           │
           ▼
┌────────────────────┬────────────────────┬──────────────────────┐
│   par_iter()       │ par_iter_mut()     │ into_par_iter()      │
├────────────────────┼────────────────────┼──────────────────────┤
│ 不可变借用           │ 可变借用             │ 转移所有权            │
│ &self               │ &mut self          │ self                 │
├────────────────────┼────────────────────┼──────────────────────┤
│ data.par_iter()    │ data.par_iter_mut()│ data.into_par_iter() │
├────────────────────┼────────────────────┼──────────────────────┤
│ 原数据仍可用         │ 原数据仍可用         │ 原数据不可用          │
│ 只读操作             │ 可修改元素           │ 消耗性操作            │
└────────────────────┴────────────────────┴──────────────────────┘
```

**1. par_iter() - 不可变并行迭代**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 并行求和
let sum: i32 = numbers.par_iter().sum();

// 并行过滤并收集
let evens: Vec<&i32> = numbers.par_iter().filter(|&&x| x % 2 == 0).collect();
```

**2. par_iter_mut() - 可变并行迭代**

```rust
let mut data = vec![1, 2, 3, 4, 5];

// 并行修改每个元素
data.par_iter_mut().for_each(|x| {
    *x *= 2;
});
// data 现在是 [2, 4, 6, 8, 10]
```

**3. into_par_iter() - 所有权转移并行迭代**

```rust
let words = vec!["hello".to_string(), "world".to_string()];

// 转移所有权，并行转换
let upper: Vec<String> = words
    .into_par_iter()
    .map(|s| s.to_uppercase())
    .collect();

// words 变量在这里不再可用
```

### 线程池管理

Rayon 自动管理全局线程池，默认线程数等于 CPU 核心数：

```rust
use rayon::ThreadPoolBuilder;

// 自定义线程池配置
let pool = ThreadPoolBuilder::new()
    .num_threads(4)           // 使用 4 个线程
    .thread_name(|i| format!("worker-{}", i))  // 自定义线程名
    .build()
    .unwrap();

// 在自定义线程池中执行
pool.install(|| {
    let sum: i32 = (0..100).into_par_iter().sum();
    println!("Sum: {}", sum);
});
```

---

## 常见错误

### 错误 1: 并行执行顺序不确定

```rust
// ❌ 错误：期望按顺序输出
let data = vec![1, 2, 3, 4, 5];
data.par_iter().for_each(|x| {
    print!("{} ", x);  // 输出顺序可能是 3 1 4 2 5
});

// ✅ 正确：如果需要顺序，使用 collect
let results: Vec<i32> = data.par_iter().map(|x| x * 2).collect();
// results 保持原始顺序
```

**问题**：并行迭代器不保证执行顺序，只保证每个元素被处理。

---

### 错误 2: 在并行代码中使用非线程安全类型

```rust
use std::cell::RefCell;

// ❌ 错误：RefCell 不是线程安全的
let counter = RefCell::new(0);
(0..100).into_par_iter().for_each(|_| {
    *counter.borrow_mut() += 1;  // 编译错误！
});

// ✅ 正确：使用原子类型或 Rayon 的 reduce
use std::sync::atomic::{AtomicUsize, Ordering};
let counter = AtomicUsize::new(0);
(0..100).into_par_iter().for_each(|_| {
    counter.fetch_add(1, Ordering::Relaxed);
});

// 或者更好的方式 - 使用并行归约
let sum: usize = (0..100).into_par_iter().sum();
```

---

### 错误 3: 过度并行化（小数据集）

```rust
// ❌ 不推荐：小数据集并行化开销大于收益
let small = vec![1, 2, 3];
let sum = small.par_iter().sum();  // 顺序执行可能更快

// ✅ 正确：大数据集才使用并行
let large: Vec<i32> = (0..1_000_000).collect();
let sum = large.par_iter().sum();  // 并行优势明显
```

**指导原则**：数据量小于 1000 时，顺序执行可能更快。Rayon 有内置启发式来决定是否真正并行执行。

---

## 动手练习

### 练习 1: 并行过滤与归约

补全下面的代码，实现并行计算偶数平方和：

```rust
use rayon::prelude::*;

fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    // 你的代码：
    // 1. 并行迭代
    // 2. 过滤出偶数
    // 3. 映射为平方
    // 4. 求和
    
}

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();
    let result = sum_of_even_squares(&numbers);
    println!("偶数平方和: {}", result);  // 期望输出: 171700
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .map(|x| x * x)            // 计算平方
        .sum()                     // 并行求和
}
```

**解析**：
- `.par_iter()` 开始并行迭代
- 过滤和映射操作自动并行化
- `.sum()` 使用高效的并行归约算法

</details>

---

### 练习 2: 使用 join 并行执行任务

使用 `rayon::join` 并行计算两个独立任务的结果：

```rust
use rayon::join;
use std::thread;
use std::time::Duration;

fn fetch_data_from_db() -> Vec<i32> {
    thread::sleep(Duration::from_secs(1));
    vec![1, 2, 3, 4, 5]
}

fn fetch_data_from_api() -> Vec<i32> {
    thread::sleep(Duration::from_secs(1));
    vec![6, 7, 8, 9, 10]
}

fn main() {
    // 你的代码：使用 join 并行执行两个 fetch 函数
    // 然后合并结果并计算总和
    
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn main() {
    // join 并行执行两个任务
    let (db_data, api_data) = join(
        fetch_data_from_db,
        fetch_data_from_api
    );
    
    // 合并结果
    let all_data: Vec<i32> = db_data.into_iter()
        .chain(api_data.into_iter())
        .collect();
    
    let sum: i32 = all_data.par_iter().sum();
    println!("总和: {}", sum);  // 输出: 55
    
    // 总耗时约 1 秒，而不是 2 秒！
}
```

**关键点**：
- `join` 并行执行两个闭包
- 等待两者都完成后返回结果
- 适用于两个独立任务并行化

</details>

---

### 练习 3: 理解所有权转移

预测以下代码的输出，并解释为什么：

```rust
use rayon::prelude::*;

fn main() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
    ];
    
    // A. 使用 par_iter
    let upper1: Vec<String> = words.par_iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("{:?}", words);  // ❓ 可以编译吗？
    
    // B. 使用 into_par_iter
    let upper2: Vec<String> = words.into_par_iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("{:?}", words);  // ❓ 可以编译吗？
}
```

<details>
<summary>点击查看解析</summary>

**答案**：

**A 部分**：✅ 可以编译，`words` 仍然可用
- `par_iter()` 只借用数据，不转移所有权
- `map` 中的闭包接收 `&String`，需要 `to_uppercase()` 创建新 String

**B 部分**：❌ 编译错误
- `into_par_iter()` 转移所有权
- `words` 被消耗，后续不能再使用

**正确版本**：
```rust
fn main() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
    ];
    
    // A 部分 - 借用版本
    let upper1: Vec<String> = words.par_iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("A 之后 words 仍可用: {:?}", words);
    
    // B 部分 - 所有权转移版本
    let upper2: Vec<String> = words.into_par_iter()
        .map(|s| s.to_uppercase())
        .collect();
    // println!("{:?}", words);  // ❌ 编译错误：value used here after move
    println!("B 结果: {:?}", upper2);
}
```

</details>

---

## 故障排查 (FAQ)

### Q: Rayon 和 Tokio 有什么区别？什么时候用哪个？

**A**:

| 特性 | Rayon | Tokio |
|-----|-------|-------|
| 并行类型 | 数据并行（CPU 密集型） | 异步 IO（IO 密集型） |
| 主要用途 | 大数据处理、计算 | 网络服务、文件 IO |
| 阻塞 | 会阻塞线程 | 非阻塞 |
| 典型场景 | 图像处理、科学计算 | Web 服务器、数据库连接 |

```rust
// 使用 Rayon：CPU 密集型任务
let sum = (0..1_000_000).into_par_iter().map(|x| x * x).sum();

// 使用 Tokio：IO 密集型任务
let response = reqwest::get("https://api.example.com").await?;
```

---

### Q: 如何控制 Rayon 的线程数？

**A**: 两种方式：

```rust
use rayon::ThreadPoolBuilder;

// 方式 1：设置全局线程池（在程序启动时调用）
ThreadPoolBuilder::new()
    .num_threads(4)
    .build_global()
    .unwrap();

// 方式 2：创建局部线程池
let pool = ThreadPoolBuilder::new()
    .num_threads(2)
    .build()
    .unwrap();

pool.install(|| {
    let sum: i32 = (0..100).into_par_iter().sum();
});
```

---

### Q: Rayon 会保证使用所有线程吗？

**A**: 不一定。Rayon 使用自适应调度：

```rust
// 小数据集 - Rayon 可能选择顺序执行
let small_sum = [1, 2, 3].par_iter().sum();  // 可能单线程

// 大数据集 - Rayon 会使用多线程
let large_sum: i32 = (0..1_000_000).into_par_iter().sum();  // 多线程
```

Rayon 内部有阈值判断，小任务直接顺序执行反而更快（避免线程切换开销）。

---

### Q: 如何在并行代码中处理错误？

**A**: 使用 `try_for_each` 或 `try_reduce`：

```rust
use rayon::prelude::*;

fn process_items(items: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    items.par_iter().try_for_each(|item| {
        if item.is_empty() {
            Err("Empty item found")?;
        }
        println!("Processing: {}", item);
        Ok(())
    })?;
    Ok(())
}
```

---

## 知识扩展 (选学)

### 自定义并行迭代器

为自定义类型实现并行迭代：

```rust
use rayon::prelude::*;
use rayon::iter::plumbing::*;

struct MyCollection<T>(Vec<T>);

impl<T: Send> IntoParallelIterator for MyCollection<T> {
    type Item = T;
    type Iter = rayon::vec::IntoIter<T>;
    
    fn into_par_iter(self) -> Self::Iter {
        self.0.into_par_iter()
    }
}
```

### 并行排序

Rayon 提供并行排序算法：

```rust
use rayon::prelude::*;

let mut data: Vec<i32> = vec![5, 2, 8, 1, 9, 3];
data.par_sort();  // 并行快速排序
```

### 并行递归

使用 `join` 实现分治算法：

```rust
fn parallel_sum(data: &[i32]) -> i32 {
    const THRESHOLD: usize = 1000;
    
    if data.len() <= THRESHOLD {
        data.iter().sum()
    } else {
        let mid = data.len() / 2;
        let (left, right) = data.split_at(mid);
        
        let (sum_left, sum_right) = rayon::join(
            || parallel_sum(left),
            || parallel_sum(right)
        );
        
        sum_left + sum_right
    }
}
```

---

## 小结

**核心要点**：

1. **数据并行**是 Rayon 的核心——同一份操作在多个数据上并发执行
2. **工作窃取**算法自动负载均衡，空闲线程窃取忙碌线程的任务
3. **三种迭代器**：`par_iter()`（借用）、`par_iter_mut()`（可变借用）、`into_par_iter()`（转移所有权）
4. **低开销**：只需添加 `par_` 前缀，大部分顺序代码可直接并行化
5. **自动调度**：Rayon 自动决定是否真正并行，小任务可能顺序执行

**关键术语**：

- **Data Parallelism（数据并行）**: 对大数据集并行执行相同操作
- **Work Stealing（工作窃取）**: 空闲线程从忙碌线程窃取任务的调度算法
- **Thread Pool（线程池）**: 预创建的线程集合，避免频繁创建销毁
- **Parallel Iterator（并行迭代器）**: 支持并行遍历的迭代器
- **Join**: 并行执行两个任务并等待结果
- **Scope**: 创建任务作用域，自动等待子任务完成

**下一步**：

- 学习 [Tokio](tokio.md) - 异步 IO 运行时
- 理解 [线程基础](../basic/threads.md) - 并发编程基础
- 探索 [服务框架](../awesome/services.md) - 生产级并行应用

---

## 术语表

| English | 中文 | 说明 |
|---------|------|------|
| Data Parallelism | 数据并行 | 在多个数据上并行执行相同操作 |
| Work Stealing | 工作窃取 | 动态负载均衡调度算法 |
| Thread Pool | 线程池 | 复用线程的执行环境 |
| Parallel Iterator | 并行迭代器 | 支持并行遍历的迭代器 trait |
| Join | 任务合并 | 并行执行两个任务并等待完成 |
| Scope | 作用域 | 任务的生命周期边界 |
| Map | 映射 | 对每个元素应用函数 |
| Filter | 过滤 | 按条件筛选元素 |
| Reduce | 归约 | 将多个值合并为单个值 |
| Split | 分割 | 将任务/数据分成多份 |

完整示例：[src/advance/rayon_sample.rs](../../src/advance/rayon_sample.rs)

---

## 继续学习

- 下一步：[Tokio 异步运行时](tokio.md)
- 进阶：[线程与并发基础](../basic/threads.md)
- 回顾：[闭包](../basic/closure.md)

> 💡 **记住**：Rayon 的设计哲学是"顺序代码优先，并行化简单"——先写正确的顺序代码，然后加上 `par_` 前缀获得并行加速。让 Rayon 处理复杂的线程管理和负载均衡！

---

## 知识检查点

### 检查点 1 🟢 (基础概念)

以下代码的输出是什么？

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.par_iter().sum();
    println!("{}", sum);
    println!("{:?}", data);
}
```

A) 编译错误，data 被消耗了  
B) 输出 "15" 然后编译错误  
C) 输出 "15" 然后 "[1, 2, 3, 4, 5]"  
D) 运行时错误

<details>
<summary>答案与解析</summary>

**答案**: C) 输出 "15" 然后 "[1, 2, 3, 4, 5]"

**解析**:
- `par_iter()` 只借用数据，不转移所有权
- `sum()` 计算后会返回结果
- `data` 仍然可用，可以正常打印

</details>

---

### 检查点 2 🟡 (所有权理解)

以下代码为什么不能编译？如何修复？

```rust
use rayon::prelude::*;

fn main() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
    ];
    
    let upper: Vec<String> = words.into_par_iter()
        .map(|s| s.to_uppercase())
        .collect();
    
    println!("原始: {:?}", words);  // ❌ 编译错误
    println!("大写: {:?}", upper);
}
```

A) `into_par_iter()` 转移了所有权，words 不能再使用  
B) `to_uppercase()` 返回的不是 String  
C) `collect()` 消耗了 upper  
D) 应该用 `par_iter()` 代替

<details>
<summary>答案与解析</summary>

**答案**: A 和 D 都是正确的分析

**解析**:
- `into_par_iter()` 转移 `words` 的所有权
- 迭代完成后 `words` 被消耗，不能再访问

**修复方案**（二选一）：

方案 1：使用 `par_iter()`（如果不需转移所有权）
```rust
let upper: Vec<String> = words.par_iter()
    .map(|s| s.to_uppercase())
    .collect();
println!("原始: {:?}", words);  // ✅ 可以访问
```

方案 2：调整代码顺序（如果确实需要转移所有权）
```rust
let upper: Vec<String> = words.into_par_iter()
    .map(|s| s.to_uppercase())
    .collect();
// 不再访问 words
println!("大写: {:?}", upper);
```

</details>

---

### 检查点 3 🔴 (并行 vs 顺序)

以下哪种场景最适合使用 Rayon？

A) 处理单个 HTTP 请求，需要异步等待响应  
B) 对 1000 万个浮点数进行复杂的数学计算  
C) 读取用户输入并立即响应  
D) 管理数据库连接池

<details>
<summary>答案与解析</summary>

**答案**: B) 对 1000 万个浮点数进行复杂的数学计算

**解析**:

| 选项 | 推荐方案 | 原因 |
|-----|---------|------|
| A | Tokio/async | IO 密集型，需要异步等待 |
| B | Rayon | CPU 密集型，大数据集并行计算 |
| C | 顺序执行 | 用户交互，需要及时响应 |
| D | 连接池库 | 资源管理，非计算任务 |

Rayon 最适合**数据并行**场景——大数据集上的 CPU 密集型计算。

</details>

---

## 扩展阅读

### 官方资源

- [Rayon 官方文档](https://docs.rs/rayon/)
- [Rayon GitHub](https://github.com/rayon-rs/rayon)
- [Rust 并行编程指南](https://rust-lang.github.io/async-book/)

### 相关项目

- [crossbeam](https://github.com/crossbeam-rs/crossbeam) - 并发编程工具集
- [parking_lot](https://github.com/Amanieu/parking_lot) - 高性能同步原语

### 进阶主题

- **NUMA 感知调度**：多路服务器的内存局部性优化
- **SIMD 并行**：结合 `packed_simd` 进行向量化计算
- **GPU 计算**：结合 `rust-gpu` 或 `cust` 进行异构并行
