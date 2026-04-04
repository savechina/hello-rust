# 异步编程 (Async Programming)

## 开篇故事

想象你在餐厅点餐。同步编程就像你站在柜台前等待厨师做完每一道菜才点下一道——你干等着，什么也做不了。而异步编程就像你点完餐后拿到一个"取餐呼叫器"(Future)，你可以去座位看书、玩手机，当菜好了呼叫器会通知你。这就是异步的核心思想：**发起操作后不必等待完成，可以继续做其他事情**。

在 Rust 中，异步编程通过 `Future` trait 和 `async/await` 语法实现，让你能够编写高效的并发程序，同时保持代码的可读性。

---

## 本章适合谁

如果你已经理解 Rust 的基础所有权和生命周期，现在想学习如何编写高效的异步程序——比如同时处理多个网络请求、读写文件而不阻塞线程，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是 `Future` 以及它如何工作
2. 使用 `async/await` 语法编写异步函数
3. 理解 `poll` 机制和执行器 (Executor) 的角色
4. 使用组合器 (Combinators) 链接异步操作
5. 区分 `async` 块和 `async move` 块的捕获行为

---

## 前置要求

学习本章前，你需要理解：

- 所有权 - 理解所有权转移和借用
- 生命周期 - 理解引用的有效范围
- Trait - 理解 trait 和实现

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add futures
cargo add tracing
```

## 第一个例子

让我们看一个最简单的异步示例：

```rust,ignore
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 还没有打印！
    block_on(future); // 执行 Future，打印 "hello, world!"
}
```

**发生了什么？**

第 8 行 `hello_world()` 返回一个 `Future`，但**此时并没有执行**。异步函数返回的是一个"待执行的任务"，就像餐厅里的取餐呼叫器——你拿到了它，但菜还没好。

第 9 行 `block_on(future)` 阻塞当前线程，直到 Future 完成。这就像你一直盯着呼叫器，直到它响。

---

## 原理解析

### Future Trait 的核心

```rust,ignore
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),      // Future 已完成，返回结果
    Pending,       // Future 还未完成，需要再次 poll
}
```

**Future 的执行流程：**

```
+-------------------------------------------------------------+
|                    Future 生命周期                           |
+-------------------------------------------------------------+
|                                                              |
|   创建 Future --→ 首次 poll --→ Pending? --→ 等待事件        |
|        |              |            |                         |
|        |              v            v                         |
|        |           Ready? ←------ 事件就绪                   |
|        |              |                                      |
|        |              v                                      |
|        └--------→ 返回结果                                    |
|                                                              |
+-------------------------------------------------------------+
```

**关键点：**
- Future 是**惰性**的——创建时不会执行，需要被 poll 才会推进
- `poll` 方法可能返回 `Pending`（未完成）或 `Ready`（已完成）
- 执行器 (Executor) 负责反复 poll 直到完成

### async/await 语法

`async fn` 是创建 Future 的便捷方式：

```rust,ignore
// 这两种写法等价：

// async fn 语法糖
async fn foo() -> i32 {
    42
}

// 手动实现 Future（编译器展开后类似这样）
fn foo() -> impl Future<Output = i32> {
    async {
        42
    }
}
```

`.await` 是异步等待的关键：

```rust,ignore
async fn learn_and_sing() {
    // .await 会暂停当前 Future，让出线程执行其他任务
    let song = learn_song().await;  // 学歌时可以做其他事
    sing_song(song).await;          // 唱完再继续
}
```

**执行流程可视化：**

```
同步执行 (阻塞):                    异步执行 (非阻塞):
+---------+                       +---------+
| 学歌()   | ←-- 线程被占用          | 学歌()   | ←-- 开始学歌
| (等待)   |    什么也做不了         | .await  | ←-- 挂起，做其他事
|         |                       | 跳舞()   | ←-- 同时跳舞
| 唱歌()   | ←-- 学完后才能唱        | (等待)   |
+---------+                       |         |    学歌完成，继续唱
                                  | 唱歌()   |
                                  +---------+
```

---

## 常见错误

### 错误 1: 忘记 await

```rust,ignore
async fn fetch_data() -> String {
    "data".to_string()
}

async fn main() {
    let data = fetch_data();  // ❌ 没有 .await!
    println!("{}", data);     // 打印的是 Future，不是 String!
}
```

**编译器输出**:
```
error[E0277]: `impl Future<Output = String>` doesn't implement `Display`
  --> src/main.rs:7:20
   |
7  |     println!("{}", data);
   |                    ^^^^ `impl Future<Output = String>` cannot be formatted
```

**修复方法**：
```rust,ignore
let data = fetch_data().await;  // ✅ 等待 Future 完成
```

---

### 错误 2: async 块捕获变量

```rust,ignore
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        println!("{}", my_string);  // ✅ 借用 my_string
    };

    let future_two = async {
        println!("{}", my_string);  // ✅ 也可以借用
    };

    futures::join!(future_one, future_two);  // ✅ 两者都能执行
}
```

**但 async move 不同：**

```rust,ignore
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // my_string 被 move 进 Future
        println!("{}", my_string);
    }
    // ❌ my_string 不能再在这里使用！
}
```

**区别**：
- `async {}` - 按引用捕获变量，多个 async 块可访问同一变量
- `async move {}` - 按值(move)捕获变量，只有一个能访问，但 Future 可以超出原作用域

---

### 错误 3: 在 async 中使用阻塞操作

```rust,ignore
async fn bad_example() {
    // ❌ 不要这样做！会阻塞整个线程
    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn good_example() {
    // ✅ 使用异步版本的 sleep
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

**为什么错误**：
- `thread::sleep` 会让整个线程休眠，其他 Future 无法执行
- 异步 sleep 只让当前 Future 挂起，线程可以去执行其他任务

---

## 动手练习

### 练习 1: 理解 Future 的惰性

下面的代码会打印什么？顺序是怎样的？

```rust,ignore
async fn learn_song() {
    println!("Learning song...");
}

async fn sing_song() {
    println!("Singing song...");
}

fn main() {
    let future1 = learn_song();  // 第1行
    let future2 = sing_song();   // 第2行
    println!("Created futures"); // 第3行
    
    block_on(future1);           // 第4行
    block_on(future2);           // 第5行
}
```

<details>
<summary>点击查看答案</summary>

**输出顺序**：
```
Created futures
Learning song...
Singing song...
```

**解析**：
- 第1-2行只是**创建** Future，没有执行，所以不会打印
- 第3行立即执行，打印 "Created futures"
- 第4行执行 `future1`，打印 "Learning song..."
- 第5行执行 `future2`，打印 "Singing song..."

**关键点**：Future 是惰性的，必须被 poll 才会执行！

</details>

---

### 练习 2: 并发执行

如何让 `learn_song` 和 `dance` **同时**执行？

```rust,ignore
async fn learn_song() -> Song {
    /* 学歌需要时间 */
    Song
}

async fn dance() {
    /* 跳舞需要时间 */
}

async fn main() {
    // 当前是顺序执行，如何改成并发？
    let song = learn_song().await;
    dance().await;
}
```

<details>
<summary>点击查看答案</summary>

**答案**：使用 `join!` 宏

```rust,ignore
async fn main() {
    // 同时开始两个 Future，等待两者都完成
    futures::join!(learn_song(), dance());
}
```

**或者使用 async 块：**

```rust,ignore
async fn main() {
    let f1 = learn_song();  // 创建 Future（未执行）
    let f2 = dance();       // 创建 Future（未执行）
    
    // join! 会并发执行两者
    futures::join!(f1, f2);
}
```

**执行流程：**
```
顺序执行:                    并发执行 (join!):
时间 →                      时间 →
|   学歌                    |   学歌 + 跳舞 同时开始
|   (等待)                  |   (等待)
|   跳舞                    |   两者都完成
+--------                   +--------
```

</details>

---

### 练习 3: async 块 vs async move 块

下面代码能编译通过吗？为什么？

```rust,ignore
fn example() {
    let data = vec![1, 2, 3];
    
    let f1 = async {
        println!("{:?}", data);
    };
    
    let f2 = async {
        println!("{:?}", data);
    };
    
    // 尝试执行两个 Future
    block_on(async {
        futures::join!(f1, f2);
    });
}
```

如果改成 `async move` 会怎样？

<details>
<summary>点击查看答案</summary>

**当前代码**：✅ 可以编译通过

- `async {}` 按引用捕获 `data`
- 两个 Future 都可以借用 `data`
- `data` 的生命周期足够长（在 `example` 函数结束前不会 drop）

**改成 async move**：

```rust,ignore
let f1 = async move {
    println!("{:?}", data);  // data 被 move 进 f1
};

let f2 = async move {
    println!("{:?}", data);  // ❌ 编译错误！data 已经被 move 到 f1 了
};
```

**错误信息**：
```
error[E0382]: use of moved value: `data`
```

**使用场景对比**：
- `async {}` - 当你需要在多个 async 块中访问同一数据
- `async move {}` - 当 Future 需要超出当前作用域，或你确定只有一个 Future 需要该数据

</details>

---

## 初学者常见困惑

> 💡 **这是很多学习者第一次遇到异步编程时的困惑——你并不孤单！**

### 困惑 1: "async/await 看起来像同步代码，为什么说是异步？"

**解答**: async/await 是**语法糖**，让异步代码看起来像同步代码。实际上：

```
你写的代码:
async fn fetch_data() {
    let data = fetch().await;  // 看起来像等待
    println!("{}", data);
}

实际发生的:
1. 调用 fetch() → 返回 Future
2. .await → 注册回调，释放线程
3. 当数据就绪时，回调被触发，继续执行
4. 线程在等待期间可以处理其他任务
```

**关键区别**:
- **同步等待**: 线程阻塞，什么都不做
- **异步等待**: 线程释放，处理其他任务

### 困惑 2: "Future 到底是什么？"

**解答**: Future 是一个**状态机**，表示"将来可能完成的计算"：

```
Future 状态转换:

Pending (等待中) ──────→ Ready (已完成)
     │                       │
     │ 数据未就绪             │ 数据就绪
     │ 释放线程               │ 返回结果
     ▼                       ▼
 处理其他任务              继续执行
```

### 困惑 3: "为什么需要 Tokio？不能用标准库吗？"

**解答**: Rust 标准库只提供 Future trait，但**不提供执行器**：

| 组件         | 标准库    | Tokio   |
| ------------ | --------- | ------- |
| Future trait | ✅ 提供   | ✅ 使用 |
| Executor     | ❌ 不提供 | ✅ 提供 |
| Reactor      | ❌ 不提供 | ✅ 提供 |
| 定时器       | ❌ 不提供 | ✅ 提供 |
| 网络 I/O     | ❌ 不提供 | ✅ 提供 |

**类比**: 
- Future = 菜谱（告诉你要做什么）
- Executor = 厨师（实际执行）
- Reactor = 厨房设备（I/O 事件通知）

### 困惑 4: ".await 到底做了什么？"

**解答**: `.await` 做三件事：

```
let result = future.await;

相当于:
1. 检查 future 是否就绪
2. 如果未就绪 → 保存状态，释放线程
3. 当就绪时 → 恢复状态，继续执行
```

### 困惑 5: "async fn 和普通 fn 有什么区别？"

**解答**:

```rust
// 普通函数：立即执行
fn normal_fn() -> i32 {
    42  // 立即返回
}

// 异步函数：返回 Future，需要 .await 执行
async fn async_fn() -> i32 {
    42  // 返回 Future<Output = i32>
}

// 调用:
let x = normal_fn();        // x = 42
let future = async_fn();    // future = Future (未执行)
let x = async_fn().await;   // x = 42 (执行后)
```

---

## 故障排查 (FAQ)

### Q: 为什么我的 async 函数返回的不是实际值？

**A**: async 函数返回的是 `Future`，需要用 `.await` 获取结果：

```rust,ignore
async fn get_number() -> i32 {
    42
}

// ❌ 错误
let n: i32 = get_number();  // 实际上类型是 impl Future<Output = i32>

// ✅ 正确
let n: i32 = get_number().await;  // await 后得到 i32
```

---

### Q: 如何选择使用 async fn 还是 async 块？

**A**: 

| 场景 | 推荐方式 | 原因 |
|------|----------|------|
| 可复用的异步逻辑 | `async fn` | 清晰、可复用 |
| 临时的异步代码 | `async {}` 块 | 内联、简洁 |
| 需要捕获外部变量 | `async {}` 或 `async move {}` | 灵活控制捕获方式 |
| 需要返回 Future 类型 | `async fn` 或返回 `impl Future` | 类型签名清晰 |

---

### Q: await 和 block_on 有什么区别？

**A**: 

| 特性 | `.await` | `block_on` |
|------|----------|-----------|
| 是否阻塞线程 | 否，只阻塞当前任务 | 是，阻塞整个线程 |
| 使用场景 | async 函数内部 | 同步代码中启动异步 |
| 能否并发 | 能，让出线程给其他任务 | 不能，独占线程 |
| 示例 | `let x = foo().await;` | `block_on(foo())` |

**最佳实践**：
- 在 async 函数内部总是用 `.await`
- 在 `main` 函数或测试中用 `block_on` 进入异步世界

---

### Q: 如何调试异步代码？

**A**: 

1. **添加日志追踪**：
   ```rust,ignore
   async fn my_function() {
       println!("Starting my_function");
       let result = some_async_op().await;
       println!("Got result: {:?}", result);
   }
   ```

2. **使用 tracing 库**（生产环境推荐）：
   ```rust,ignore
   use tracing::{info, instrument};

   #[instrument]
   async fn my_function() {
       info!("Starting");
       let result = some_async_op().await;
       info!(result = ?result, "Completed");
   }
   ```

---

## 知识扩展 (选学)

### Future 组合器

`futures` crate 提供了丰富的组合器：

```rust,ignore
use futures::future::{FutureExt, TryFutureExt};

// map - 转换结果
let future = fetch_data().map(|data| data.len());

// then - 链式调用
let future = fetch_user().then(|user| fetch_orders(user.id));

// join - 等待多个 Future
let (user, orders) = futures::join!(fetch_user(), fetch_orders());

// select - 等待任一 Future
futures::select! {
    user = fetch_user().fuse() => println!("Got user"),
    timeout = sleep(Duration::from_secs(5)).fuse() => println!("Timeout!"),
}
```

### Pin 和 Unpin

当 Future 自引用时需要 `Pin`：

```rust,ignore
use std::pin::Pin;

// async 块可能包含自引用，所以需要 Pin
fn poll_future(fut: Pin<&mut dyn Future<Output = ()>>) {
    // ...
}
```

大部分情况下你不需要关心 Pin，但理解它有助于调试复杂异步代码。

---

## 小结

**核心要点**：

1. **Future 是惰性的** - 创建时不会执行，需要被 poll
2. **async fn 返回 Future** - 需要 `.await` 或 `block_on` 来执行
3. **await 不阻塞线程** - 只阻塞当前任务，让出线程执行其他任务
4. **async vs async move** - 前者按引用捕获，后者按值 move 捕获
5. **join! 实现并发** - 同时执行多个 Future，等待全部完成

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| Future | 未来值 | 代表异步计算的结果 |
| async/await | 异步/等待 | Rust 的异步语法 |
| Poll | 轮询 | 推进 Future 执行的方法 |
| Executor | 执行器 | 管理和执行 Future 的运行时 |
| Combinator | 组合器 | 组合和转换 Future 的工具 |
| Join | 合并 | 并发执行多个 Future |
| Pending | 等待中 | Future 还未完成的状态 |
| Ready | 就绪 | Future 已完成的状态 |

**下一步**：

- 学习 [Tokio 运行时](tokio.md) - 最流行的 Rust 异步运行时
- 理解 并发模式 - async 与多线程的结合
- 探索 Stream - 异步版本的 Iterator

---

## 术语表

| English | 中文 |
|---------|------|
| Future | 未来值 |
| Async | 异步 |
| Await | 等待 |
| Poll | 轮询 |
| Pending | 等待中 |
| Ready | 就绪 |
| Executor | 执行器 |
| Task | 任务 |
| Blocking | 阻塞 |
| Non-blocking | 非阻塞 |
| Combinator | 组合器 |
| Concurrent | 并发 |
| Parallel | 并行 |

---

完整示例：src/advance/futures_sample.rs

---

## 继续学习

- 下一步：[Tokio 异步运行时](tokio.md)
- 进阶：并发与并行
- 相关：线程与并发

> 💡 **记住**：异步编程的核心是"等待时不浪费资源"。当你需要等待 I/O（网络、文件、数据库）时，async 能让你的程序更高效地利用资源！

---

## 异步执行流程可视化

### 1. Future 状态机

```
                    +-------------+
         +---------→|  Not Started |←--------+
         |          |   (未开始)    |         |
         |          +------+------+         |
         |                 | Poll            |
         |                 v                |
         |          +-------------+         |
         |    +-----|   Pending   |-----+   |
         |    |     |  (等待中)    |     |   |
         |    |     +------+------+     |   |
         |    |            |            |   |
    Waker|    |  Poll      |      Poll  |   |Executor
   通知   |    |  未完成    |      未完成  |   |调度
         |    +------------+------------+   |
         |                   |              |
         |                   | Poll         |
         |                   | 完成          |
         |                   v              |
         |            +-------------+       |
         +------------|    Ready    |-------+
                      |   (已完成)   |
                      +-------------+
                            |
                            v
                      返回最终结果
```

### 2. 并发 vs 并行

```
单线程并发 (Async):          多线程并行:
+-----------------+          +-----------------+
| 线程 1          |          | 线程 1    线程 2 |
| +---+---+---+  |          | +---+    +---+  |
| | A | B | A |  |          | | A |    | B |  |
| +---+---+---+  |          | +---+    +---+  |
| (任务切换)      |          | (同时执行)      |
+-----------------+          +-----------------+

A: Task A 执行    A: Task A 执行
B: Task B 执行    B: Task B 执行
```

**区别**：
- **并发** (Concurrent) - 多个任务交替执行，提高资源利用率
- **并行** (Parallel) - 多个任务同时执行，需要多核 CPU

Rust async 主要解决**并发**问题，让单线程能高效处理多个 I/O 任务。

### 3. 异步函数调用链

```
main()
  |
  +-→ block_on(async_main())
        |
        +-→ learn_and_sing().await
        |     |
        |     +-→ learn_song().await
        |     |       |
        |     |       +-→ 执行学歌...
        |     |       +-→ 返回 Song
        |     |
        |     +-→ sing_song(song).await
        |             |
        |             +-→ 执行唱歌...
        |
        +-→ dance().await
              |
              +-→ 执行跳舞...


实际并发执行 (使用 join!):

main()
  |
  +-→ block_on(async_main())
        |
        +-→ join!(learn_and_sing(), dance())
              |                    |
              +-→ 学歌 ------------+
              |    .await          +-→ 跳舞
              +-→ 唱歌              |   .await
                   .await           |
                                    |
              ←-- 两者都完成 -------+---+
```

---

## 知识检查

**问题 1** 🟢 (基础概念)

下面代码的输出顺序是什么？

```rust,ignore
async fn task1() { println!("1"); }
async fn task2() { println!("2"); }

fn main() {
    let f1 = task1();
    let f2 = task2();
    println!("start");
    block_on(f1);
    println!("middle");
    block_on(f2);
    println!("end");
}
```

A) start, 1, 2, middle, end  
B) start, middle, end  
C) start, 1, middle, 2, end  
D) 1, 2, start, middle, end

<details>
<summary>答案与解析</summary>

**答案**: C) start, 1, middle, 2, end

**解析**：
1. `let f1 = task1()` 只是创建 Future，**不会**打印 "1"
2. `let f2 = task2()` 只是创建 Future，**不会**打印 "2"
3. 立即打印 "start"
4. `block_on(f1)` 执行 Future，打印 "1"
5. 打印 "middle"
6. `block_on(f2)` 执行 Future，打印 "2"
7. 打印 "end"

**关键点**：Future 是惰性的，创建时不执行！

</details>

**问题 2** 🟡 (并发执行)

如何让 `task1` 和 `task2` **并发**执行，并等待两者都完成？

```rust,ignore
async fn task1() { 
    println!("Task 1 start");
    // 模拟耗时操作
    println!("Task 1 done"); 
}

async fn task2() { 
    println!("Task 2 start");
    // 模拟耗时操作  
    println!("Task 2 done");
}

async fn main() {
    // 当前是顺序执行，如何改成并发？
    task1().await;
    task2().await;
}
```

<details>
<summary>答案与解析</summary>

**答案**：使用 `futures::join!`

```rust,ignore
async fn main() {
    futures::join!(task1(), task2());
}
```

**可能的输出顺序**：
```
Task 1 start
Task 2 start
Task 1 done
Task 2 done
```

或：
```
Task 2 start
Task 1 start
Task 2 done
Task 1 done
```

**解析**：
- `join!` 会同时开始两个 Future
- 执行顺序取决于调度器，但两者会并发执行
- 只有当两者都完成后，`main` 才会继续

**对比**：
```
顺序执行 (.await):           并发执行 (join!):
task1() --→ task2()         task1()
   |                           |
   v                        task2()
task1完成                      |
   |                           v
   v                        两者完成
task2开始
   |
   v
task2完成
```

</details>

**问题 3** 🔴 (所有权与 async)

下面代码能编译通过吗？如果不能，如何修复？

```rust,ignore
fn create_futures() -> (impl Future<Output = ()>, impl Future<Output = ()>) {
    let data = String::from("shared");
    
    let f1 = async move {
        println!("{}", data);
    };
    
    let f2 = async move {
        println!("{}", data);  // 能访问 data 吗？
    };
    
    (f1, f2)
}
```

<details>
<summary>答案与解析</summary>

**答案**：❌ 不能编译通过

**错误信息**：
```
error[E0382]: use of moved value: `data`
```

**原因**：
- `async move` 会 move 捕获变量
- `data` 在第一个 `async move` 块中已经被 move
- 第二个 `async move` 块无法再使用 `data`

**修复方法 1 - 使用 Arc 共享所有权**：
```rust,ignore
use std::sync::Arc;

fn create_futures() -> (impl Future<Output = ()>, impl Future<Output = ()>) {
    let data = Arc::new(String::from("shared"));
    
    let f1 = async move {
        println!("{}", data);  // Arc clone
    };
    
    let data2 = Arc::clone(&data);
    let f2 = async move {
        println!("{}", data2);  // 使用 clone
    };
    
    (f1, f2)
}
```

**修复方法 2 - 使用 async (非 move)**：
```rust,ignore
fn create_futures() -> impl Future<Output = ()> {
    let data = String::from("shared");
    
    async {
        // 多个 async 块可以共享引用
        let f1 = async {
            println!("{}", data);
        };
        
        let f2 = async {
            println!("{}", data);
        };
        
        futures::join!(f1, f2);
    }
}
```

**关键点**：
- `async move` - 所有权转移，只能有一个 Future 拥有数据
- `async` - 借用捕获，多个 Future 可以共享引用（但生命周期受限）

</details>

---

> 💡 **记住**：异步编程让你用顺序的代码风格编写高效的并发程序。理解 Future 的惰性本质和 await 的挂起机制是掌握 Rust async 的关键！
