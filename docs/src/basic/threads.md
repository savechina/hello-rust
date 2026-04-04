# 线程与并发

## 开篇故事

想象你在经营一家餐厅。如果只有一个厨师（单线程），他必须按顺序完成每道菜：切菜 → 炒菜 → 装盘 → 下一道。这很慢，但不会出错。

如果你雇了多个厨师（多线程），他们可以同时做菜——但问题来了：如果两个厨师都想用同一把刀怎么办？如果一个厨师还没切完菜，另一个就拿走了怎么办？这就是并发编程的核心挑战：**协调共享资源的访问**。

Rust 的线程系统就像一位经验丰富的餐厅经理——它在编译时就确保不会出现"抢刀"的情况。

---

## 本章适合谁

如果你想编写多线程程序提高性能，或者理解 Rust 如何防止数据竞争，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `thread::spawn` 创建线程
2. 使用 `join()` 等待线程完成
3. 使用 `move` 闭包转移所有权到线程
4. 使用通道（channel）在线程间传递消息
5. 使用 `Arc<Mutex<T>>` 安全共享可变状态
6. 理解 Rust 的线程安全保证

---

## 前置要求

- [所有权](ownership.md) - 线程安全基础
- [闭包](closure.md) - move 闭包语法
- [特征](trait.md) - Send 和 Sync trait

---

## 第一个例子

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程：数字 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("主线程：数字 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();  // 等待子线程完成
}
```

**发生了什么？**

- `thread::spawn` - 创建新线程
- 闭包在**新线程**中执行
- `join()` - 阻塞等待线程完成
- 主线程和子线程**并发**执行

---

## 原理解析

### 1. 线程生命周期

```
创建                执行              完成
  │                  │                │
  ├─→ spawn() ───→ 运行中 ───→ join() ──→ 已结束
  │                  │                │
  │                  ↓                │
  │              panic!() ────────────→ 异常结束
  │
主线程继续...
```

### 2. 线程生命周期详解

```
创建阶段:
  thread::spawn(closure)
      │
      ├── 分配线程栈 (默认 2MB)
      ├── 调度器注册新线程
      └── 返回 JoinHandle

执行阶段:
  运行中
      │
      ├── 正常执行 → 返回结果
      ├── panic!() → 线程终止
      └── 被 join() → 阻塞调用者

完成阶段:
  join() 返回
      │
      ├── Ok(value) → 线程正常完成
      └── Err(panic) → 线程 panic
```

### 3. Move 闭包

```rust
let data = vec![1, 2, 3];

// ❌ 错误：data 是引用，可能在线程结束前被释放
let handle = thread::spawn(|| {
    println!("{:?}", data);
});

// ✅ 正确：使用 move 转移所有权
let handle = thread::spawn(move || {
    println!("{:?}", data);  // data 现在属于这个线程
});

// data 不能再在主线程中使用了！
// println!("{:?}", data);  // ❌ 编译错误
```

### 4. 消息传递（通道）

```rust
use std::sync::mpsc;  // multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn(move || {
        let msg = String::from("来自子线程的消息");
        tx.send(msg).unwrap();  // 发送消息
        // msg 所有权已转移，不能再使用
    });
    
    let received = rx.recv().unwrap();  // 接收消息（阻塞）
    println!("收到：{}", received);
    
    handle.join().unwrap();
}
```

**通道特点**：
- `tx.send()` - 发送消息（非阻塞）
- `rx.recv()` - 接收消息（阻塞等待）
- `rx.try_recv()` - 尝试接收（非阻塞）
- 消息所有权**转移**到接收方

### 5. 共享状态（Arc + Mutex）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: 原子引用计数（多线程安全的 Rc）
    // Mutex: 互斥锁（保证同一时间只有一个线程访问）
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果：{}", *counter.lock().unwrap());
}
```

### 6. Send 和 Sync Trait

```rust
// Send: 类型可以安全转移所有权到其他线程
// Sync: 类型可以安全通过引用共享给其他线程

// 大多数类型自动实现 Send 和 Sync
// 以下类型 NOT Send:
// - Rc<T> (引用计数不是原子的)
// - 原始指针 (*const T, *mut T)
// - Cell<T>, RefCell<T> (不是线程安全的)
```

---

## 常见错误

### 错误 1: 忽略 join handle

```rust
let handle = thread::spawn(|| {
    // 一些工作
});

// ❌ 忘记 join，线程可能未完成程序就退出
// handle.join().unwrap();

// ✅ 正确：总是 join
handle.join().unwrap();
```

### 错误 2: 数据竞争

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

// ❌ 错误：多个线程同时修改
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    data_clone.push(4);  // ❌ 需要 Mutex
});
```

### 错误 3: 死锁

```rust
use std::sync::Mutex;

let mutex1 = Mutex::new(1);
let mutex2 = Mutex::new(2);

// ❌ 错误：两个线程以不同顺序获取锁
// 线程 1: mutex1.lock() → mutex2.lock()
// 线程 2: mutex2.lock() → mutex1.lock()
// 结果：死锁！

// ✅ 正确：总是以相同顺序获取锁
```

---

## 动手练习

### 练习 1: 并行计算

使用多线程计算向量中所有数字的和：

```rust
// TODO: 实现 parallel_sum 函数
// 将数据分成 4 块，每块用一个线程计算
// 最后汇总结果
```

<details>
<summary>点击查看答案</summary>

```rust
use std::thread;

fn parallel_sum(data: Vec<i32>) -> i32 {
    let chunk_size = data.len() / 4;
    let mut handles = vec![];
    
    for i in 0..4 {
        let chunk = data[i * chunk_size..(i + 1) * chunk_size].to_vec();
        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>()
        });
        handles.push(handle);
    }
    
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
```
</details>

---

## 故障排查

### Q: Rust 线程和 Go goroutine 有什么区别？

**A**: 
- Rust 线程 = OS 线程（重量级，但性能好）
- Go goroutine = 绿色线程（轻量级，由运行时调度）
- Rust 需要 `async/await` 实现类似 goroutine 的轻量并发

### Q: 如何限制线程数量？

**A**: 使用线程池（如 `rayon` crate）：
```rust
use rayon::prelude::*;

let sum: i32 = (1..1000).par_iter().sum();
```

### Q: Mutex 和 RwLock 有什么区别？

**A**: 
- `Mutex`: 同一时间只允许一个线程访问
- `RwLock`: 允许多个读线程或一个写线程

---

## 小结

**要点**：

1. **thread::spawn**: 创建新线程
2. **join()**: 等待线程完成
3. **消息传递**: 线程间安全通信
4. **避免共享可变状态**: 使用 Mutex 或通道

**安全规则**：

- ❌ 不要用 `static mut`
- ❌ 不要在线程间共享可变引用
- ✅ 使用 `Arc<Mutex<T>>` 安全共享
- ✅ 使用通道 (channel) 传递消息

---

## 术语表

| English     | 中文       |
| ----------- | ---------- |
| Thread      | 线程       |
| Data race   | 数据竞争   |
| Mutex       | 互斥锁     |
| Channel     | 通道       |
| Deadlock    | 死锁       |
| Send/Sync   | 线程安全 trait |

---

完整示例：`src/basic/threads_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `Rc<T>` 可以在多线程中使用吗？

2. 这段代码有什么问题？
```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(|| {
    println!("{:?}", data);
});
```

3. 通道 (channel) 和 Mutex 有什么区别？

<details>
<summary>点击查看答案与解析</summary>

1. ❌ 不能 - `Rc` 不是线程安全的，应该使用 `Arc`
2. `data` 的所有权没有转移到闭包，需要使用 `move`
3. 通道 = 消息传递（所有权转移），Mutex = 共享状态（借用）

**关键理解**: Rust 在编译时防止数据竞争。
</details>

## 继续学习

- 下一步：[条件编译](cfg_if.md)
- 进阶：[异步编程](../advance/async/async.md)
- 回顾：[所有权](ownership.md)

> 🔴 **警告**：并发编程容易出错。始终使用高级抽象（Arc、Mutex、通道），避免原始线程操作！
