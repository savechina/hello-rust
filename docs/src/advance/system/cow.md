# Cow 类型

## 开篇故事

想象你有一本书要修改。传统方式是：复印整本书 → 修改复印件 → 使用。Cow 就像是：如果需要修改才复印，如果不需要修改直接看原书。Cow 类型就是这样的智能类型——按需克隆。

---

## 本章适合谁

如果你需要优化内存使用（只读时借用，修改时克隆），本章适合你。Cow 是 Rust 特有的零成本抽象。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Cow 类型概念
2. 使用 Cow::Borrowed 和 Cow::Owned
3. 使用 to_mut() 按需克隆
4. 优化字符串处理
5. 实现零拷贝优化

---

## 前置要求

- 所有权 - 所有权基础
- 借用 - 借用基础
- 枚举 - 枚举类型

---

## 第一个例子

最简单的 Cow 使用：

```rust
use std::borrow::Cow;

fn filter_profanity(input: &str) -> Cow<str> {
    if input.contains("badword") {
        // 需要修改：克隆并返回 Owned
        let filtered = input.replace("badword", "****");
        Cow::Owned(filtered)
    } else {
        // 不需要修改：直接返回 Borrowed（零分配）
        Cow::Borrowed(input)
    }
}

fn main() {
    let s1 = "Hello, world!";
    let res1 = filter_profanity(s1);
    println!("结果 1: {} (owned: {})", res1, matches!(res1, Cow::Owned(_)));
    
    let s2 = "This is a badword!";
    let res2 = filter_profanity(s2);
    println!("结果 2: {} (owned: {})", res2, matches!(res2, Cow::Owned(_)));
}
```

**完整示例**: [cow_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/cow_sample.rs)

---

## 原理解析

### Cow 特性

**Cow (Clone-on-Write) 是写时克隆类型**：

- ✅ 只读时零拷贝
- ✅ 修改时自动克隆
- ✅ 智能优化
- ✅ 零成本抽象

### Cow 枚举

**Cow 是一个枚举**：

```rust
enum Cow<'a, B: ToOwned> {
    Borrowed(&'a B),      // 借用
    Owned(<B as ToOwned>::Owned),  // 拥有
}
```

**对于 Cow<str>**：
- `Cow::Borrowed(&str)`: 借用字符串切片
- `Cow::Owned(String)`: 拥有 String

### 创建 Cow

**使用 Borrowed**：

```rust
use std::borrow::Cow;

let borrowed: Cow<str> = Cow::Borrowed("Hello");
// 零分配，直接借用
```

**使用 Owned**：

```rust
let owned: Cow<str> = Cow::Owned(String::from("Hello"));
// 分配内存，拥有数据
```

### to_mut() 方法

**按需克隆**：

```rust
use std::borrow::Cow;

let mut cow: Cow<str> = Cow::Borrowed("original");

// 第一次调用 to_mut()：如果是 Borrowed，会克隆
cow.to_mut().make_ascii_uppercase();

// 第二次调用：已经是 Owned，直接返回引用
cow.to_mut().push_str("!!!");

println!("{}", cow);  // 输出：ORIGINAL!!!
```

### 性能优势

**只读场景**：

```rust
fn process_data(data: &[u8]) -> Cow<[u8]> {
    // 只读场景：直接返回借用
    Cow::Borrowed(data)
}

// 零分配
let data = vec![1, 2, 3, 4];
let result = process_data(&data);
```

**修改场景**：

```rust
fn process_data_mut(data: &[u8], modify: bool) -> Cow<[u8]> {
    let mut cow = Cow::Borrowed(data);
    
    if modify {
        // 修改场景：克隆并修改
        cow.to_mut().push(0xFF);
    }
    
    cow
}
```

### 实际应用

**页面数据处理**：

```rust
use std::borrow::Cow;

struct Page {
    id: u64,
    data: Vec<u8>,
}

fn process_page_data<'a>(
    page_data: &'a [u8],
    is_writable: bool
) -> Cow<'a, [u8]> {
    let mut cow = Cow::Borrowed(page_data);
    
    if is_writable {
        // to_mut() 会检查：如果是 Borrowed，则克隆
        // 如果已经是 Owned，直接返回引用
        let mutable_data = cow.to_mut();
        mutable_data[0] = 0xFF;  // 修改标记位
    }
    
    cow
}

fn main() {
    let disk_data = vec![0u8; 4096];  // 模拟磁盘数据
    
    // 只读场景：完全不分配内存
    let read_only = process_page_data(&disk_data, false);
    println!("只读：{:?}", read_only[0]);
    
    // 写入场景：在 to_mut() 被调用时发生一次 4KB 拷贝
    let writable = process_page_data(&disk_data, true);
    println!("可写：{:?}", writable[0]);
}
```

---

## 常见错误

### 错误 1: 忘记使用 to_mut()

```rust
use std::borrow::Cow;

let mut cow: Cow<str> = Cow::Borrowed("hello");
cow.push_str(" world");  // ❌ 编译错误
```

**错误信息**:
```
no method named `push_str` found for enum `Cow`
```

**修复方法**:
```rust
cow.to_mut().push_str(" world");  // ✅ 使用 to_mut()
```

### 错误 2: 生命周期错误

```rust
fn create_cow() -> Cow<str> {
    let s = String::from("hello");
    Cow::Borrowed(&s)  // ❌ s 会被释放
}
```

**错误信息**:
```
borrowed value does not live long enough
```

**修复方法**:
```rust
fn create_cow() -> Cow<'static, str> {
    Cow::Borrowed("hello")  // ✅ 字符串字面量有 'static 生命周期
}
```

### 错误 3: 不必要的克隆

```rust
fn process(data: &str) -> Cow<str> {
    let mut cow = Cow::Borrowed(data);
    cow.to_mut();  // ❌ 不必要的克隆
    cow
}
```

**修复方法**:
```rust
fn process(data: &str) -> Cow<str> {
    Cow::Borrowed(data)  // ✅ 只在需要时克隆
}
```

---

## 动手练习

### 练习 1: 创建 Cow

```rust
use std::borrow::Cow;

fn main() {
    // TODO: 创建 Borrowed Cow
    // TODO: 创建 Owned Cow
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
let borrowed: Cow<str> = Cow::Borrowed("Hello");
let owned: Cow<str> = Cow::Owned(String::from("World"));

println!("Borrowed: {}", borrowed);
println!("Owned: {}", owned);
```
</details>

### 练习 2: 使用 to_mut()

```rust
use std::borrow::Cow;

fn main() {
    let mut cow: Cow<str> = Cow::Borrowed("original");
    
    // TODO: 使用 to_mut() 转换为大写
    // TODO: 添加感叹号
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
cow.to_mut().make_ascii_uppercase();
cow.to_mut().push_str("!!!");

println!("{}", cow);  // 输出：ORIGINAL!!!
```
</details>

### 练习 3: 优化字符串处理

```rust
use std::borrow::Cow;

fn trim_and_process(input: &str) -> Cow<str> {
    // TODO: 如果字符串有前后空格，克隆并修剪
    // TODO: 如果没有空格，直接返回借用
    // TODO: 返回 Cow<str>
}
```

<details>
<summary>点击查看答案</summary>

```rust
if input.trim() == input {
    Cow::Borrowed(input)  // 无需修改，零分配
} else {
    Cow::Owned(input.trim().to_string())  // 需要修改，克隆
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Cow 和 Option 有什么区别？

**A**: 
- **Cow**: 优化克隆（借用 vs 拥有）
- **Option**: 表示可能有值或无值
- **用途不同**

### Q: Cow 只能用于 String 吗？

**A**: 
- 不，Cow 可以用于任何实现 `ToOwned` trait 的类型
- 常见：`Cow<str>`, `Cow<[T]>`, `Cow<Path>`

### Q: 什么时候使用 Cow？

**A**: 
- 函数可能返回借用或拥有数据
- 优化只读场景的性能
- API 设计：灵活返回类型

---

## 知识扩展

### Cow<[T]> 用于切片

```rust
use std::borrow::Cow;

fn process_slice(data: &[i32]) -> Cow<[i32]> {
    if data.iter().all(|&x| x > 0) {
        // 都是正数，直接返回借用
        Cow::Borrowed(data)
    } else {
        // 有负数，过滤并返回拥有
        let filtered: Vec<i32> = data.iter().copied().filter(|&x| x > 0).collect();
        Cow::Owned(filtered)
    }
}
```

### Cow<Path> 用于路径

```rust
use std::borrow::Cow;
use std::path::{Path, PathBuf};

fn resolve_path(input: &str) -> Cow<Path> {
    if input.starts_with('/') {
        // 绝对路径，直接借用
        Cow::Borrowed(Path::new(input))
    } else {
        // 相对路径，需要解析
        let full_path = std::env::current_dir().unwrap().join(input);
        Cow::Owned(full_path)
    }
}
```

### 性能对比

```rust
// 传统方式：总是克隆
fn process_always_clone(s: &str) -> String {
    s.to_uppercase()  // 总是分配
}

// Cow 方式：按需克隆
fn process_cow(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_ascii_uppercase()) {
        Cow::Borrowed(s)  // 已经是大写，零分配
    } else {
        Cow::Owned(s.to_uppercase())  // 需要转换，分配
    }
}
```

---

## 小结

**核心要点**：

1. **Cow**: 写时克隆类型
2. **Borrowed**: 借用，零分配
3. **Owned**: 拥有，分配内存
4. **to_mut()**: 按需克隆
5. **零成本**: 只读场景无开销

**关键术语**：

- **Cow (Clone-on-Write)**: 写时克隆
- **Borrowed**: 借用变体
- **Owned**: 拥有变体
- **Zero-copy**: 零拷贝

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Cow (Clone-on-Write) | 写时克隆 |
| Borrowed | 借用 |
| Owned | 拥有 |
| ToOwned trait | ToOwned trait |
| Zero-copy | 零拷贝 |

---

## 延伸阅读

学习完 Cow 类型后，你可能还想了解：

- [Cow 源码分析](https://doc.rust-lang.org/src/alloc/borrow.rs.html) - 标准库实现
- [零拷贝优化](https://docs.rs/bytes) - Bytes crate 的类似模式
- [写时复制文件系统](https://en.wikipedia.org/wiki/Copy-on-write) - 操作系统中的 COW

**选择建议**:
- 想学习进程管理 → 继续学习 [进程管理](process.md)
- 想学习字节处理 → 跳到 [字节处理](bytes.md)

## 知识检查

**快速测验**（答案在下方）：

1. `Cow<'a, str>` 的三个变体是什么？

2. 什么时候会发生克隆？

3. `Cow` 的使用场景？

<details>
<summary>点击查看答案与解析</summary>

1. `Borrowed(&'a str)` 和 `Owned(String)`
2. 当需要修改借用数据时
3. 读多写少场景，如配置处理、字符串处理

**关键理解**: Cow 是写时复制的零成本抽象。
</details>

## 继续学习

**前一章**: [字节处理](bytes.md)  
**下一章**: 派生宏

**相关章节**:
- [字节处理](bytes.md)
- 所有权
- 借用

**返回**: 高级进阶

---

**完整示例**: [cow_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/cow_sample.rs)
