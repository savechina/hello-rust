# 序列化与反序列化

## 开篇故事

想象你在寄快递。你需要把物品打包（序列化），通过快递运输（网络传输/存储），然后收件人拆包（反序列化）取出物品。如果打包方式不对，物品可能在运输中损坏，或者收件人无法拆开。

在编程中，序列化就是把内存中的数据结构转换成可以存储或传输的格式（如 JSON、CSV、二进制）。Rust 的 Serde 框架就像一位专业的打包专家——它确保你的数据在任何地方都能安全"拆包"。

---

## 本章适合谁

如果你需要在 Rust 程序中处理 JSON、CSV 或其他数据格式，本章适合你。序列化是 Web API、配置文件、数据存储的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 Serde 框架序列化和反序列化数据
2. 处理类型化 (Typed) 和无类型 (Untyped) JSON
3. 自定义序列化行为
4. 处理序列化错误
5. 理解 Serde 的工作原理

---

## 前置要求

- [结构体](../../basic/struct.md) - 数据类型定义
- [错误处理](../tools/error-handling.md) - Result 处理

---

## 第一个例子

最简单的 JSON 序列化：

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 序列化：结构体 → JSON 字符串
    let json = serde_json::to_string(&person)?;
    println!("序列化：{}", json);

    // 反序列化：JSON 字符串 → 结构体
    let parsed: Person = serde_json::from_str(&json)?;
    println!("反序列化：{:?}", parsed);

    Ok(())
}
```

**发生了什么？**

- `#[derive(Serialize, Deserialize)]` - 自动生成序列化代码
- `to_string()` - 序列化
- `from_str()` - 反序列化

---

## 原理解析

### 1. Serde 架构

```
Serde 框架
├── serde (核心)
│   ├── Serialize trait
│   └── Deserialize trait
├── serde_derive (宏)
│   ├── #[derive(Serialize)]
│   └── #[derive(Deserialize)]
└── 格式库
    ├── serde_json (JSON)
    ├── serde_yaml (YAML)
    ├── bincode (二进制)
    └── csv (CSV)
```

### 2. 无类型 JSON 处理

```rust
use serde_json::Value;

fn untyped_sample() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    // 解析为 Value (类似 HashMap)
    let v: Value = serde_json::from_str(data)?;

    // 通过索引访问
    println!("姓名：{}", v["name"]);
    println!("电话：{}", v["phones"][0]);

    Ok(())
}
```

### 3. 类型化 JSON 处理

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_sample() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": ["+44 1234567", "+44 2345678"]
    }"#;

    // 解析为具体类型
    let person: Person = serde_json::from_str(data)?;
    println!("{:?}", person);

    Ok(())
}
```

### 4. 自定义序列化

```rust
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 自定义序列化
impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 序列化为 [x, y] 数组
        serializer.serialize_seq(Some(2))?
            .serialize_element(&self.x)?
            .serialize_element(&self.y)?
            .end()
    }
}

// 自定义反序列化
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<i32>::deserialize(deserializer)?;
        Ok(Point { x: vec[0], y: vec[1] })
    }
}
```

### 5. 字段属性

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    // 序列化时使用不同名称
    #[serde(rename = "user_name")]
    name: String,

    // 如果字段为 None 则跳过
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    // 反序列化时提供默认值
    #[serde(default = "default_age")]
    age: u8,

    // 反序列化时如果字段缺失使用默认值
    #[serde(default)]
    active: bool,
}

fn default_age() -> u8 {
    18
}
```

---

## 常见错误

### 错误 1: 忘记 derive

```rust
// ❌ 错误：没有 derive
struct Person {
    name: String,
}
serde_json::to_string(&person)?;  // 编译错误！

// ✅ 正确：添加 derive
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}
```

### 错误 2: 不匹配的类型

```rust
// ❌ 错误：JSON 中的 age 是字符串，但结构体定义是数字
let json = r#"{"name": "Alice", "age": "30"}"#;
let person: Person = serde_json::from_str(json)?;  // 反序列化失败！

// ✅ 正确：类型匹配
let json = r#"{"name": "Alice", "age": 30}"#;
```

---

## 动手练习

### 练习 1: 配置结构体

创建一个配置结构体，支持从 JSON 文件加载：

```rust
// TODO: 定义 Config 结构体
// 字段：host (String), port (u16), debug (bool)
// 实现从 JSON 文件加载配置
```

<details>
<summary>点击查看答案</summary>

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default)]
    debug: bool,
}

fn default_port() -> u16 {
    8080
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```
</details>

---

## 故障排查

### Q: 如何处理未知字段？

**A**: 使用 `#[serde(deny_unknown_fields)]` 拒绝未知字段，或默认忽略。

### Q: 如何序列化枚举？

**A**: 使用 `#[serde(tag = "type")]` 控制枚举格式。

### Q: 如何提高序列化性能？

**A**: 使用 `serde_json::to_vec` 而不是 `to_string`，避免 UTF-8 转换。

---

## 小结

**核心要点**：

1. **#[derive(Serialize, Deserialize)]**: 自动生成代码
2. **serde_json**: JSON 格式处理
3. **自定义序列化**: 实现 trait
4. **字段属性**: 控制序列化行为

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| Serialization     | 序列化     |
| Deserialization   | 反序列化   |
| Derive Macro      | 派生宏     |
| Attribute         | 属性       |
| Transcode         | 转码       |

---

完整示例：`src/advance/json_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[derive(Serialize, Deserialize)]` 做了什么？

2. 如何处理序列化错误？

3. `serde_json::to_string` 和 `to_string_pretty` 的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 自动生成序列化和反序列化代码
2. 使用 `Result<T, serde_json::Error>` 处理
3. `to_string` 紧凑格式，`to_string_pretty` 格式化输出

**关键理解**: Serde 是 Rust 序列化的事实标准框架。
</details>

## 继续学习

- 下一步：[CSV 处理](csv.md)
- 进阶：[零拷贝序列化](rkyv.md)
- 回顾：[结构体](../../basic/struct.md)

> 💡 **记住**：Serde 让数据在任何地方都能安全"拆包"！
