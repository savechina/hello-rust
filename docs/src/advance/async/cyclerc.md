# CycleRC - 引用计数循环检测

## 开篇故事

想象你和朋友互相保管对方的钥匙：你把家门钥匙给他，他把家门钥匙给你。现在你们都被锁在外面了——因为要拿钥匙需要对方开门，但对方也需要你的钥匙才能开门。这就是循环引用的本质：**两个对象互相持有对方的引用，永远无法释放**。

在 Rust 中，`Rc` 和 `Arc` 是引用计数的智能指针，但单纯的引用计数无法检测循环引用。CycleRC 就是为了解决这个问题而设计的。

---

## 本章适合谁

如果你需要使用 `Rc` 或 `Arc` 构建复杂的数据结构（如图、树），担心循环引用导致内存泄漏，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解循环引用的成因和危害
2. 使用 `Weak` 打破循环引用
3. 识别何时会发生循环引用
4. 设计避免循环的数据结构

---

## 前置要求

学习本章前，你需要理解：

- [所有权](../../basic/ownership.md) - 理解所有权和借用
- 智能指针 - 理解 `Rc` 和 `Arc`
- 引用计数 - 理解引用计数原理

---

## 第一个例子

让我们看一个循环引用的例子：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 使用 Weak 打破循环
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

**发生了什么？**

- `children` 使用 `Rc<Node>`（强引用）
- `parent` 使用 `Weak<Node>`（弱引用）
- 弱引用不增加引用计数，**打破循环**

---

## 原理解析

### 1. 循环引用的成因

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node { value: 1, next: RefCell::new(None) });
    let b = Rc::new(Node { value: 2, next: RefCell::new(None) });

    // ❌ 创建循环引用
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.next.borrow_mut() = Some(Rc::clone(&a));

    // 即使 a 和 b 离开作用域，内存也不会释放
    // 因为引用计数永远 > 0
}
```

**问题**：
- `a` 持有 `b` 的强引用（计数 +1）
- `b` 持有 `a` 的强引用（计数 +1）
- 即使外部引用消失，内部引用仍在

### 2. 使用 Weak 打破循环

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
    prev: RefCell<Option<Weak<Node>>>,  // 弱引用
}

fn main() {
    let a = Rc::new(Node { 
        value: 1, 
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });
    
    let b = Rc::new(Node { 
        value: 2, 
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });

    // ✅ 正确：一个强引用，一个弱引用
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.prev.borrow_mut() = Some(Rc::downgrade(&a));

    // 当 a 和 b 离开作用域，内存会被释放
}
```

**解决方案**：
- 单向关系使用 `Rc`（强引用）
- 反向关系使用 `Weak`（弱引用）

### 3. Weak 的使用模式

```rust
use std::rc::{Rc, Weak};

fn main() {
    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);

    // 检查弱引用是否有效
    if let Some(value) = weak.upgrade() {
        println!("值：{}", value);
    } else {
        println!("弱引用已失效");
    }

    // 获取引用计数
    println!("强引用计数：{}", Rc::strong_count(&strong));
    println!("弱引用计数：{}", Rc::weak_count(&strong));

    drop(strong);

    // 强引用释放后，弱引用失效
    assert!(weak.upgrade().is_none());
}
```

**关键点**：
- `Weak::upgrade()` 返回 `Option<Rc<T>>`
- 如果强引用还在，返回 `Some(rc)`
- 如果强引用已释放，返回 `None`

### 4. 典型应用场景

**场景 1: 树结构**
```rust
struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,  // 子节点：强引用
    parent: RefCell<Weak<TreeNode>>,  // 父节点：弱引用
}
```

**场景 2: 图结构**
```rust
struct GraphNode {
    value: i32,
    neighbors: Vec<Rc<GraphNode>>,  // 出边：强引用
    incoming: RefCell<Vec<Weak<GraphNode>>>,  // 入边：弱引用
}
```

**场景 3: 观察者模式**
```rust
struct Subject {
    observers: RefCell<Vec<Weak<Observer>>>,  // 观察者：弱引用
}

impl Subject {
    fn notify(&self) {
        // 清理失效的弱引用
        self.observers.borrow_mut().retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.update();
                true
            } else {
                false  // 自动清理
            }
        });
    }
}
```

---

## 常见错误

### 错误 1: 双向都使用强引用

```rust
// ❌ 错误：内存泄漏
struct Node {
    next: Option<Rc<Node>>,
    prev: Option<Rc<Node>>,  // 应该是 Weak<Node>
}
```

**修复**：反向引用使用 `Weak`。

### 错误 2: 忘记检查 Weak 有效性

```rust
// ❌ 错误：直接 unwrap
let parent = self.parent.borrow().upgrade().unwrap();

// ✅ 正确：检查有效性
if let Some(parent) = self.parent.borrow().upgrade() {
    // 使用 parent
} else {
    // 处理父节点不存在的情况
}
```

### 错误 3: 滥用 Weak 导致频繁失败

```rust
// ❌ 错误：过度使用 Weak
struct Cache {
    data: Weak<Data>,  // 可能总是失效
}

// ✅ 正确：缓存应该持有强引用
struct Cache {
    data: Rc<Data>,
}
```

---

## 动手练习

### 练习 1: 创建双向链表

使用 `Rc` 和 `Weak` 创建双向链表：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
    prev: RefCell<Option<Weak<Node>>>,
}

// TODO: 实现创建和遍历函数
```

<details>
<summary>点击查看答案</summary>

```rust
impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        })
    }

    fn append(this: &Rc<Node>, next: Rc<Node>) {
        *next.prev.borrow_mut() = Some(Rc::downgrade(this));
        *this.next.borrow_mut() = Some(next);
    }
}
```

**解析**: `next` 使用强引用，`prev` 使用弱引用，避免循环。
</details>

---

## 故障排查

### Q: 什么时候使用 Weak？

**A**: 当你需要"观察"或"回溯"但不拥有对象时：
- 父节点引用（子节点不拥有父节点）
- 观察者模式（观察者不拥有被观察者）
- 缓存反向索引

### Q: Weak 的性能开销？

**A**: 
- `upgrade()` 是原子操作（轻微开销）
- 弱引用计数需要维护
- 但对于打破循环的价值，开销可接受

### Q: 如何调试循环引用？

**A**: 
```rust
println!("强引用计数：{}", Rc::strong_count(&rc));
println!("弱引用计数：{}", Rc::weak_count(&rc));
```

如果预期应该释放但计数 > 0，可能存在循环。

---

## 小结

**核心要点**：

1. **循环引用**: 两个对象互相持有强引用，无法释放
2. **Weak 智能指针**: 弱引用不增加计数，打破循环
3. **upgrade()**: 安全地从弱引用获取强引用
4. **设计模式**: 单向强引用，反向弱引用

**关键术语**：

- **Strong Reference**: 强引用，增加引用计数
- **Weak Reference**: 弱引用，不增加计数
- **Cycle Detection**: 循环检测
- **Memory Leak**: 内存泄漏

**下一步**：

- 学习 智能指针
- 理解 Rc 和 Arc
- 探索 观察者模式

---

## 术语表

| English          | 中文       |
| ---------------- | ---------- |
| Reference Count  | 引用计数   |
| Cycle            | 循环       |
| Weak Reference   | 弱引用     |
| Strong Reference | 强引用     |
| Memory Leak      | 内存泄漏   |
| Upgrade          | 升级       |

---

完整示例：`src/advance/async/cyclerc_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. 为什么 `Rc` 会导致循环引用问题？

2. `Weak::upgrade()` 返回什么类型？

3. 如何检测循环引用？

<details>
<summary>点击查看答案与解析</summary>

1. `Rc` 是强引用，互相持有导致引用计数永远 > 0
2. `Option<Rc<T>>` - 如果强引用还在返回 `Some`，否则 `None`
3. 使用 `Weak` 弱引用打破循环，或使用调试工具检查引用计数

**关键理解**: 循环引用是 Rust 中少数会导致内存泄漏的情况。
</details>

## 延伸阅读

学习完循环引用检测后，你可能还想了解：

- [Rc 和 Arc 的区别](https://doc.rust-lang.org/book/ch15-04-rc.html) - 何时使用哪个
- [Weak 引用深入](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) - 弱引用原理
- [内存泄漏检测工具](https://github.com/brson/rust-memory-leak-checker) - 自动化检测

**选择建议**:
- 想学习异步 → 继续学习 [异步编程](async.md)
- 想学习数据库 → 跳到 [数据库操作](../database/database.md)

## 继续学习

- 下一步：智能指针
- 进阶：无锁编程
- 回顾：[所有权](../../basic/ownership.md)

> 💡 **记住**：循环引用是 Rust 中少数会导致内存泄漏的情况。使用 Weak 打破循环，确保内存安全！
