# 线程 (Threads)

## ⚠️ 安全警告

本章包含 `unsafe` 代码示例。这些仅用于教学目的，展示**不应该做什么**。生产代码中请始终使用安全的并发原语。

---

## 你会学到什么

1. 创建线程
2. 线程 join
3. 移动数据到线程
4. 使用消息传递
5. 理解线程安全

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

---

## ⚠️ 危险模式（仅教学）

### 不安全的 static mut

```rust
// ⚠️ 不要这样做！仅用于演示危险
static mut COUNTER: i32 = 0;

fn unsafe_example() {
    // ❌ 没有同步，数据竞争
    unsafe {
        COUNTER += 1;
    }
}

// ✅ 安全替代：使用 Mutex
use std::sync::Mutex;

static COUNTER_SAFE: Mutex<i32> = Mutex::new(0);

fn safe_example() {
    let mut num = COUNTER_SAFE.lock().unwrap();
    *num += 1;
}
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
```

### 错误 2: 数据竞争

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

// ❌ 多个线程同时修改
thread::spawn(|| {
    data.push(4);  // ❌ 需要 Mutex
});
```

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

| English | 中文 |
| ------- | ---- |
| Thread | 线程 |
| Data race | 数据竞争 |
| Mutex | 互斥锁 |

---

> 🔴 **警告**：并发编程容易出错。始终使用高级抽象（Arc、Mutex、通道），避免原始线程操作！

---

## 继续学习

**前一章**: [闭包](closure.md)  
**下一章**: [条件编译](cfg_if.md)

**相关章节**:
- [所有权](ownership.md) - 线程安全
- [指针与不安全代码](pointer.md) - unsafe 代码

**返回**: [基础入门](basic-overview.md)
