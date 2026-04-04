# Rkyv 零拷贝序列化

## 开篇故事

想象你要寄一本很厚的书。传统方式是：复印整本书 → 打包 → 邮寄 → 收件人阅读。零拷贝就像是：直接把书递给收件人，不需要复印。Rkyv 就是这样的零拷贝序列化库，特别适合大数据集。

---

## 本章适合谁

如果你需要高性能序列化（处理大数据集、网络传输），本章适合你。Rkyv 是零拷贝序列化库，性能远超传统序列化方法。

---

## 你会学到什么

完成本章后，你可以：

1. 理解零拷贝序列化概念
2. 使用 rkyv 序列化和反序列化
3. 使用归档类型 (Archived types)
4. 自定义序列化配置
5. 处理复杂数据结构

---

## 前置要求

- 结构体 - 结构体定义
- [Serde 序列化](serialization.md) - 序列化基础
- 枚举 - 枚举类型

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add rkyv --features alloc
```

## 第一个例子

最简单的 Rkyv 序列化：

```rust
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
struct TestStruct {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

fn main() {
    let value = TestStruct {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    // 序列化
    let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();
    
    // 零拷贝反序列化
    let archived = rkyv::access::<ArchivedTestStruct>(&bytes[..]).unwrap();
    
    // 使用归档数据（零拷贝）
    assert_eq!(archived.int, 42);
    assert_eq!(archived.string, "hello world");
    
    // 完整反序列化
    let deserialized = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(deserialized, value);
}
```

**完整示例**: [rkyv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/rkyv_sample.rs)

---

## 原理解析

### Rkyv 特性

**Rkyv 是零拷贝序列化库**：

- ✅ 零拷贝反序列化
- ✅ 高性能
- ✅ 支持归档类型
- ✅ 无运行时开销

### 什么是零拷贝？

**传统序列化**：
```
数据 → 序列化 → 字节数组 → 反序列化 → 新数据
       (复制)              (复制)
```

**零拷贝序列化**：
```
数据 → 序列化 → 字节数组 → 直接访问 (无复制)
                      ↑
                  直接使用内存
```

### 派生宏

**使用 Archive derive**：

```rust
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug)]
#[rkyv(derive(Debug, PartialEq))]
struct Person {
    name: String,
    age: u32,
}
```

**生成的归档类型**：
- `Person` → `ArchivedPerson`
- 可以直接访问字段，无需反序列化

### 序列化

**简单序列化**：

```rust
let value = Person {
    name: "Alice".to_string(),
    age: 30,
};

// 序列化到字节数组
let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();
```

**自定义序列化**：

```rust
use rkyv::ser::allocator::Arena;

let mut arena = Arena::new();
let bytes = rkyv::to_bytes_with_alloc::<_, 256>(&value, arena.acquire()).unwrap();
```

### 反序列化

**零拷贝访问**：

```rust
let archived = rkyv::access::<ArchivedPerson>(&bytes[..]).unwrap();

// 直接访问字段（零拷贝）
println!("Name: {}", archived.name);
println!("Age: {}", archived.age);
```

**完整反序列化**：

```rust
let deserialized = archived
    .deserialize(&mut rkyv::Infallible)
    .unwrap();

assert_eq!(deserialized.name, "Alice");
```

### 复杂类型

**Vec 和 Option**：

```rust
#[derive(Archive, Deserialize, Serialize)]
struct Complex {
    numbers: Vec<i32>,
    maybe_string: Option<String>,
}

let value = Complex {
    numbers: vec![1, 2, 3],
    maybe_string: Some("hello".to_string()),
};

let bytes = rkyv::to_bytes::<_, 1024>(&value).unwrap();
let archived = rkyv::access::<ArchivedComplex>(&bytes[..]).unwrap();

// 零拷贝访问
for num in archived.numbers.iter() {
    println!("{}", num);
}

if let Some(s) = &archived.maybe_string {
    println!("{}", s);
}
```

---

## 常见错误

### 错误 1: 忘记 derive Archive

```rust
#[derive(Serialize, Deserialize)]  // ❌ 忘记 Archive
struct Person {
    name: String,
}
```

**错误信息**:
```
the trait `Archive` is not implemented for `Person`
```

**修复方法**:
```rust
#[derive(Archive, Serialize, Deserialize)]  // ✅ 添加 Archive
struct Person {
    name: String,
}
```

### 错误 2: 缓冲区太小

```rust
let bytes = rkyv::to_bytes::<_, 16>(&value).unwrap();
// ❌ 16 字节太小，无法容纳数据
```

**错误信息**:
```
Out of space
```

**修复方法**:
```rust
let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();  // ✅ 增加缓冲区
```

### 错误 3: 访问已释放内存

```rust
let bytes = serialize_data();
let archived = access::<ArchivedData>(&bytes);
drop(bytes);  // ❌ 释放内存

// 使用 archived 会导致未定义行为
println!("{}", archived.field);
```

**修复方法**:
确保字节数组的生命周期覆盖整个访问过程。

---

## 动手练习

### 练习 1: 定义简单结构体

```rust
use rkyv::{Archive, Deserialize, Serialize};

// TODO: 定义 Point 结构体
// 字段：x (i32), y (i32)
// 派生 Archive, Deserialize, Serialize
```

<details>
<summary>点击查看答案</summary>

```rust
#[derive(Archive, Deserialize, Serialize, Debug)]
#[rkyv(derive(Debug))]
struct Point {
    x: i32,
    y: i32,
}
```
</details>

### 练习 2: 序列化和反序列化

```rust
let point = Point { x: 10, y: 20 };

// TODO: 序列化为字节
// TODO: 零拷贝访问
// TODO: 完整反序列化
```

<details>
<summary>点击查看答案</summary>

```rust
let bytes = rkyv::to_bytes::<_, 64>(&point).unwrap();
let archived = rkyv::access::<ArchivedPoint>(&bytes[..]).unwrap();
assert_eq!(archived.x, 10);
assert_eq!(archived.y, 20);

let deserialized = archived.deserialize(&mut rkyv::Infallible).unwrap();
assert_eq!(deserialized.x, 10);
```
</details>

### 练习 3: 处理复杂类型

```rust
#[derive(Archive, Deserialize, Serialize)]
struct User {
    name: String,
    hobbies: Vec<String>,
}

// TODO: 创建 User 实例
// TODO: 序列化
// TODO: 零拷贝访问 hobbies
```

<details>
<summary>点击查看答案</summary>

```rust
let user = User {
    name: "Alice".to_string(),
    hobbies: vec!["Reading".to_string(), "Coding".to_string()],
};

let bytes = rkyv::to_bytes::<_, 256>(&user).unwrap();
let archived = rkyv::access::<ArchivedUser>(&bytes[..]).unwrap();

for hobby in archived.hobbies.iter() {
    println!("{}", hobby);
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Rkyv 和 Serde 有什么区别？

**A**: 
- **Serde**: 通用序列化框架，支持多种格式
- **Rkyv**: 零拷贝序列化，性能最优，仅支持 rkyv 格式
- **推荐**: Rkyv（性能关键场景），Serde（通用场景）

### Q: 如何选择合适的缓冲区大小？

**A**: 
```rust
// 小数据：64-256 字节
rkyv::to_bytes::<_, 256>(&value).unwrap()

// 大数据：使用 Vec
let mut serializer = rkyv::Serializer::new();
rkyv::serialize_into(&mut serializer, &value).unwrap();
let bytes = serializer.into_serializer().into_inner();
```

### Q: 如何处理循环引用？

**A**: Rkyv 不支持循环引用。使用 `Rc` 或 `Arc` 重构数据结构。

---

## 知识扩展

### 自定义分配器

```rust
use rkyv::ser::allocator::Arena;

let mut arena = Arena::new();
let bytes = rkyv::to_bytes_with_alloc::<_, 256>(&value, arena.acquire()).unwrap();
```

### 验证归档

```rust
use rkyv::validation::validators::DefaultValidator;

let archived = rkyv::access::<ArchivedData>(&bytes[..]).unwrap();
archived.validate(&mut DefaultValidator::default()).unwrap();
```

### 性能对比

```rust
// Serde
let bytes = bincode::serialize(&value).unwrap();
let deserialized: Data = bincode::deserialize(&bytes).unwrap();

// Rkyv (更快)
let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();
let archived = rkyv::access::<ArchivedData>(&bytes[..]).unwrap();
```

---

## 小结

**核心要点**：

1. **Rkyv**: 零拷贝序列化库
2. **Archive**: 归档类型，直接访问内存
4. **零拷贝**: 无需反序列化即可访问数据
5. **高性能**: 远超传统序列化方法
6. **缓冲区**: 需要足够大小的缓冲区

**关键术语**：

- **Zero-copy (零拷贝)**: 无需复制即可访问数据
- **Archive (归档)**: 序列化后的内存布局
- **Archived type (归档类型)**: 自动生成的归档类型
- **Serializer (序列化器)**: 序列化数据的工具

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Zero-copy | 零拷贝 |
| Archive | 归档 |
| Archived type | 归档类型 |
| Serializer | 序列化器 |
| Deserializer | 反序列化器 |
| Buffer | 缓冲区 |

---

## 知识检查

**快速测验**（答案在下方）：

1. rkyv 和 serde 的主要区别是什么？

2. 什么是"零拷贝"反序列化？

3. 什么时候应该使用 rkyv 而不是 serde？

<details>
<summary>点击查看答案与解析</summary>

1. rkyv 是零拷贝序列化，serde 需要反序列化到内存
2. 直接访问序列化后的内存，无需复制到新结构
3. 性能关键场景：游戏、数据库、网络传输大数据

**关键理解**: rkyv 牺牲兼容性换取极致性能。
</details>

## 继续学习

**前一章**: [CSV 处理](csv.md)  
**下一章**: 临时文件

**相关章节**:
- [CSV 处理](csv.md)
- [JSON 序列化](json.md)
- [Serde 序列化](serialization.md)

**返回**: 高级进阶

---

**完整示例**: [rkyv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/rkyv_sample.rs)
