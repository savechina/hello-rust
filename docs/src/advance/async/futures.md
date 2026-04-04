# Futures 异步编程

## 开篇故事

想象你在餐厅点餐。传统方式是：点餐 → 等待 → 取餐 → 吃。异步方式是：点餐 → 拿到号牌 → 继续做其他事 → 号牌响了去取餐。Futures 就像这个号牌——它代表一个将来会完成的任务。

---

## 本章适合谁

如果你已经了解了 async/await 基础，现在想深入理解 Future trait 和异步组合子，本章适合你。Futures 是 Rust 异步编程的核心抽象。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Future trait 的工作原理
2. 使用 block_on 执行 Future
3. 链式组合多个 Future
4. 并发执行多个任务
5. 使用 join! 宏并发等待

---

## 前置要求

- [Tokio 异步运行时](tokio.md) - async/await 基础
- 闭包 - 闭包语法
- 特征 - trait 基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add futures
```

## 第一个例子

最简单的 Future 使用：

```rust,ignore
use futures::{executor::block_on, Future};

// 定义异步函数
async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    // 创建 Future
    let future = hello_world();
    
    // 执行 Future
    block_on(future);  // 输出：hello, world!
}
```

**完整示例**: [futures_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/futures_sample.rs)

---

## 原理解析

### Future Trait

**Future 是一个 trait**：

```rust,ignore
trait Future {
    type Output;
    
    // 轮询 Future 是否完成
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

**Poll 枚举**：
```rust,ignore
enum Poll<T> {
    Ready(T),      // 完成，返回结果
    Pending,       // 未完成，等待
}
```

### block_on 执行器

**最简单的执行器**：

```rust,ignore
use futures::executor::block_on;

async fn task1() -> i32 {
    42
}

async fn task2() -> String {
    "Hello".to_string()
}

fn main() {
    let result1 = block_on(task1());
    let result2 = block_on(task2());
    
    println!("{} {}", result1, result2);
}
```

### Future 链式组合

**使用 then 链式调用**：

```rust,ignore
use futures::{future, FutureExt};

let future = future::ready(42)
    .then(|x| async move { x * 2 })
    .then(|x| async move { x + 1 });

let result = block_on(future);
println!("{}", result);  // 85
```

### 并发执行

**使用 join 并发等待**：

```rust,ignore
use futures::future::join;

async fn task1() -> i32 {
    42
}

async fn task2() -> String {
    "Hello".to_string()
}

fn main() {
    let (result1, result2) = block_on(join(task1(), task2()));
    println!("{} {}", result1, result2);
}
```

### join! 宏

**并发执行多个异步任务**：

```rust,ignore
async fn learn_song() -> Song {
    Song
}

async fn sing_song(song: Song) {
    println!("Singing: {:?}", song);
}

async fn dance() {
    println!("Dancing!");
}

async fn async_main() {
    // 使用 join! 并发执行
    futures::join!(
        learn_and_sing(),
        dance()
    );
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}
```

---

## 常见错误

### 错误 1: Future 未执行

```rust,ignore
async fn task() {
    println!("Hello");
}

fn main() {
    task();  // ❌ 什么都不发生
    // Future 需要被 await 或 block_on
}
```

**修复方法**:
```rust,ignore
fn main() {
    block_on(task());  // ✅ 执行 Future
}
```

### 错误 2: 在同步上下文中 await

```rust,ignore
fn sync_function() {
    async { 42 }.await;  // ❌ await 只能在 async 函数中使用
}
```

**修复方法**:
```rust,ignore
async fn async_function() {
    async { 42 }.await;  // ✅
}
```

### 错误 3: 阻塞异步运行时

```rust,ignore
#[tokio::main]
async fn main() {
    // ❌ 这会阻塞整个运行时
    std::thread::sleep(Duration::from_secs(5));
}
```

**修复方法**:
```rust,ignore
#[tokio::main]
async fn main() {
    // ✅ 使用异步 sleep
    tokio::time::sleep(Duration::from_secs(5)).await;
}
```

---

## 动手练习

### 练习 1: 创建简单 Future

```rust,ignore
use futures::executor::block_on;

// TODO: 定义异步函数
// - 返回 i32
// - 返回 42

fn main() {
    // TODO: 执行 Future
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use futures::executor::block_on;

async fn get_answer() -> i32 {
    42
}

fn main() {
    let answer = block_on(get_answer());
    println!("答案是：{}", answer);
}
```
</details>

### 练习 2: 链式 Future

```rust,ignore
use futures::{future, FutureExt};

fn main() {
    // TODO: 创建 Future 链
    // 1. 从 ready(5) 开始
    // 2. 乘以 2
    // 3. 加 10
    // 4. 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use futures::{future, FutureExt};

fn main() {
    let result = block_on(
        future::ready(5)
            .then(|x| async move { x * 2 })
            .then(|x| async move { x + 10 })
    );
    println!("{}", result);  // 20
}
```
</details>

### 练习 3: 并发执行

```rust,ignore
use futures::future::join;

async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }

fn main() {
    // TODO: 并发执行 task1 和 task2
    // TODO: 打印两个结果的和
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use futures::future::join;

fn main() {
    let (r1, r2) = block_on(join(task1(), task2()));
    println!("和：{}", r1 + r2);  // 30
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Future 和 async/await 有什么区别？

**A**: 
- **Future**: trait，表示异步计算
- **async/await**: 语法糖，让 Future 更易使用

```rust,ignore
// 使用 async/await
async fn task() -> i32 { 42 }

// 底层是 Future
fn task() -> impl Future<Output = i32> {
    future::ready(42)
}
```

### Q: 什么时候使用 block_on？

**A**: 
- **block_on**: 在同步上下文中执行 Future（如 main 函数）
- **.await**: 在异步上下文中等待 Future

```rust,ignore
fn main() {
    block_on(async_main());  // 入口
}

async fn async_main() {
    task().await;  // 异步等待
}
```

### Q: join 和 select 有什么区别？

**A**: 
- **join**: 等待所有 Future 完成
- **select**: 等待第一个完成的 Future

```rust,ignore
// join - 都完成
let (r1, r2) = join(task1(), task2()).await;

// select - 第一个完成
let result = select(task1(), task2()).await;
```

---

## 知识扩展

### select 使用

```rust,ignore
use futures::future::{select, Either};

async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }

fn main() {
    let result = block_on(async {
        match select(task1(), task2()).await {
            Either::Left((v, _)) => v,
            Either::Right((v, _)) => v,
        }
    });
    println!("{}", result);  // 10 或 20
}
```

### Future 超时

```rust,ignore
use futures::{FutureExt, TryFutureExt};
use std::time::Duration;

async fn slow_task() -> i32 {
    tokio::time::sleep(Duration::from_secs(5)).await;
    42
}

fn main() {
    let result = block_on(
        slow_task()
            .timeout(Duration::from_secs(2))
            .unwrap_or(0)
    );
    println!("{}", result);  // 0 (超时)
}
```

### 自定义 Future

```rust,ignore
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Future;

struct MyFuture {
    value: i32,
}

impl Future for MyFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}
```

---

## 小结

**核心要点**：

1. **Future trait**: 表示异步计算的 trait
2. **block_on**: 在同步上下文执行 Future
3. **链式组合**: then, map, then 等方法
4. **并发执行**: join, select 组合多个 Future
5. **错误处理**: 使用 Result 和错误转换

**关键术语**：

- **Future**: 未来值/异步任务
- **Poll**: 轮询
- **Executor**: 执行器
- **Combinator**: 组合子
- **block_on**: 阻塞执行
- **await**: 异步等待

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Future | 未来值/异步任务 |
| Poll | 轮询 |
| Executor | 执行器 |
| Combinator | 组合子 |
| block_on | 阻塞执行 |
| await | 异步等待 |
| join | 并发等待 |
| select | 选择第一个完成 |

---

## 知识检查

**快速测验**（答案在下方）：

1. Future 是惰性的还是立即执行的？

2. `poll()` 返回 `Pending` 后，谁会再次调用 `poll()`？

3. `async/await` 和 Future 是什么关系？

<details>
<summary>点击查看答案与解析</summary>

1. 惰性的 - 需要被 poll 才会执行
2. Waker（由执行器提供）
3. `async/await` 是 Future 的语法糖，编译器转换为状态机

**关键理解**: Future 本身不执行，需要执行器 (Executor) 驱动。
</details>

## 延伸阅读

学习完 Futures 后，你可能还想了解：

- [Future 组合子](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html) - map, then, and_then
- [Stream trait](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html) - 异步迭代器
- [futures-util crate](https://docs.rs/futures-util) - 实用工具集

**选择建议**:
- 想学习并行 → 继续学习 [并行计算](rayon.md)
- 想学习底层 I/O → 跳到 [MIO](mio.md)

## 继续学习

**前一章**: [Tokio 异步运行时](tokio.md)  
**下一章**: 宏编程

**相关章节**:
- [Tokio 异步运行时](tokio.md)
- [并发异步编程](async.md)
- 闭包

**返回**: 高级进阶

---

**完整示例**: [futures_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/futures_sample.rs)
