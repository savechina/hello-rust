# 类型别名

## 开篇故事

想象你经常写很长的类型名：`Arc<RefCell<HashMap<String, Vec<User>>>>`。每次都写很繁琐。类型别名就像是：给长类型起个短名字——`type UserData = Arc<...>`。这样代码更清晰易读。

---

## 本章适合谁

如果你想简化复杂类型、提高代码可读性，本章适合你。类型别名是 Rust 代码组织的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 理解类型别名概念
2. 创建类型别名
3. 简化复杂类型
4. 使用类型别名组织代码
5. 实现双向链表

---

## 前置要求

- [智能指针](../basic/pointer.md) - Arc, Weak 基础
- [泛型](../basic/generic.md) - 泛型基础
- [结构体](../basic/struct.md) - 结构体基础

---

## 第一个例子

最简单的类型别名：

```rust
// 类型别名
type NodeCell = RefCell<TreeNode>;
type NodeArcPtr = Arc<NodeCell>;
type NodeWeakPtr = Weak<NodeCell>;

// 使用别名
fn create_node() -> NodeArcPtr {
    Arc::new(RefCell::new(TreeNode {
        value: 42,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    }))
}
```

**完整示例**: [typealias_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/typealias_sample.rs)

---

## 原理解析

### type 关键字

**使用 type 创建别名**：

```rust
type Result<T> = std::result::Result<T, MyError>;
type Callback = Box<dyn Fn(i32) -> String>;
```

### 简化复杂类型

**简化嵌套类型**：

```rust
// 不使用别名
fn process(data: Arc<RefCell<HashMap<String, Vec<User>>>>) {
    // ...
}

// 使用别名
type UserData = Arc<RefCell<HashMap<String, Vec<User>>>>;

fn process(data: UserData) {
    // 更清晰
}
```

### 双向链表应用

**使用类型别名实现双向链表**：

```rust
type NodeCell = RefCell<TreeNode>;
type NodeArcPtr = Arc<NodeCell>;
type NodeWeakPtr = Weak<NodeCell>;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    next: RefCell<Option<NodeArcPtr>>,
    prev: RefCell<Option<NodeWeakPtr>>,
}

impl TreeNode {
    fn new(value: i32) -> NodeArcPtr {
        Arc::new(RefCell::new(TreeNode {
            value,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        }))
    }
    
    fn set_next(&self, next_node: Option<NodeArcPtr>) {
        self.next.borrow_mut().replace(next_node.unwrap());
    }
    
    fn get_next(&self) -> Option<NodeArcPtr> {
        self.next.borrow().clone()
    }
    
    fn set_prev(&self, prev_node: Option<&NodeArcPtr>) {
        let weak_ptr = prev_node.map(|arc| Arc::downgrade(arc));
        self.prev.borrow_mut().replace(weak_ptr.unwrap());
    }
    
    fn get_prev(&self) -> Option<NodeArcPtr> {
        self.prev
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
    }
}
```

### 类型别名优势

**提高可读性**：

```rust
// 之前
fn connect(
    Arc<RefCell<dyn ConnectionHandler + Send + Sync>>
) -> Result<(), Box<dyn Error>>;

// 之后
type ConnectionPtr = Arc<RefCell<dyn ConnectionHandler + Send + Sync>>;

fn connect(conn: ConnectionPtr) -> Result<(), Box<dyn Error>>;
```

---

## 常见错误

### 错误 1: 循环依赖

```rust
type A = B;
type B = A;  // ❌ 循环定义
```

**错误信息**:
```
cycle detected when computing type of `B`
```

**修复方法**:
```rust
type A = i32;  // ✅ 打破循环
type B = A;
```

### 错误 2: 类型不匹配

```rust
type MyType = String;

let s: MyType = "hello";  // ❌ &str != String
```

**修复方法**:
```rust
let s: MyType = "hello".to_string();  // ✅ 转换为 String
```

### 错误 3: 生命周期错误

```rust
type NodePtr = Rc<RefCell<Node>>;

fn create_node() -> NodePtr {
    let node = Node::new();
    Rc::new(RefCell::new(node))  // ✅ 正确
}
```

---

## 动手练习

### 练习 1: 创建简单别名

```rust
// TODO: 创建 UserId 类型别名 (u64)
// TODO: 创建 UserName 类型别名 (String)

fn create_user(id: UserId, name: UserName) {
    println!("User {}: {}", id, name);
}
```

<details>
<summary>点击查看答案</summary>

```rust
type UserId = u64;
type UserName = String;
```
</details>

### 练习 2: 简化复杂类型

```rust
use std::sync::{Arc, Mutex};

// TODO: 创建 DataPtr 类型别名
// Arc<Mutex<Vec<String>>>

fn process_data(data: DataPtr) {
    let locked = data.lock().unwrap();
    println!("Data: {:?}", *locked);
}
```

<details>
<summary>点击查看答案</summary>

```rust
type DataPtr = Arc<Mutex<Vec<String>>>;
```
</details>

### 练习 3: 实现双向链表节点

```rust
use std::cell::RefCell;
use std::sync::{Arc, Weak};

// TODO: 定义类型别名
// NodeCell = RefCell<Node>
// NodePtr = Arc<NodeCell>
// NodeWeak = Weak<NodeCell>

struct Node {
    value: i32,
    next: RefCell<Option<NodePtr>>,
    prev: RefCell<Option<NodeWeak>>,
}
```

<details>
<summary>点击查看答案</summary>

```rust
type NodeCell = RefCell<Node>;
type NodePtr = Arc<NodeCell>;
type NodeWeak = Weak<NodeCell>;
```
</details>

---

## 故障排查 (FAQ)

### Q: 类型别名和新类型有什么区别？

**A**: 
- **类型别名**: `type A = B`，只是别名
- **新类型**: `struct A(B)`，新类型，类型安全

### Q: 什么时候使用类型别名？

**A**: 
- 简化复杂类型
- 提高可读性
- 统一类型使用

### Q: 类型别名会影响性能吗？

**A**: 
- 不会，编译后完全一样
- 零成本抽象

---

## 知识扩展

### 泛型别名

```rust
type Result<T> = std::result::Result<T, MyError>;
type Callback<T> = Box<dyn Fn(T) -> String>;
```

### 条件类型别名

```rust
#[cfg(target_os = "linux")]
type PlatformPath = std::path::UnixPath;

#[cfg(target_os = "windows")]
type PlatformPath = std::path::WindowsPath;
```

### 关联类型别名

```rust
trait Container {
    type Item;
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;
}
```

---

## 小结

**核心要点**：

1. **type**: 创建类型别名
2. **简化**: 简化复杂类型
3. **可读性**: 提高代码可读性
4. **零成本**: 无性能开销
5. **组织**: 组织代码结构

**关键术语**：

- **Type Alias**: 类型别名
- **Arc**: 原子引用计数
- **Weak**: 弱引用
- **RefCell**: 运行时借用检查

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Type Alias | 类型别名 |
| Arc | 原子引用计数 |
| Weak | 弱引用 |
| RefCell | 引用单元 |
| Bidirectional List | 双向链表 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `type` 和 `newtype` 有什么区别？

2. 类型别名会增加运行时开销吗？

3. 什么时候应该使用类型别名？

<details>
<summary>点击查看答案与解析</summary>

1. `type` 只是别名（编译时），`newtype` 是新类型（包装结构体）
2. 不会 - 类型别名在编译时消除
3. 简化复杂类型、提高可读性、减少重复

**关键理解**: 类型别名是零成本抽象。
</details>

## 继续学习

**前一章**: [宏编程](macros.md)  
**下一章**: [对象存储](objectstore.md)

**相关章节**:
- [宏编程](macros.md)
- [智能指针](../basic/pointer.md)
- [泛型](../basic/generic.md)

**返回**: [高级进阶](advance-overview.md)

---

**完整示例**: [typealias_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/typealias_sample.rs)
