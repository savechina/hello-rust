# 序列化基础

## 开篇故事

想象你要寄快递到不同国家。每个国家有不同的包装要求：有的用纸箱，有的用木箱，有的用塑料袋。但无论你用什么包装，里面的物品都是一样的。

在 Rust 中，**Serde** 框架就是你的"通用包装系统"——它定义了一套标准的序列化接口，然后针对不同格式（JSON、YAML、TOML、CSV、二进制）提供具体的"包装"实现。你只需定义一次数据结构，就能序列化成任何格式。

---

## 本章适合谁

如果你需要在 Rust 程序中序列化/反序列化数据，本章适合你。序列化是 Web API、配置文件、数据存储的基础。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Serde 框架的工作原理
2. 使用 `#[derive(Serialize, Deserialize)]` 自动生成序列化代码
3. 自定义序列化行为
4. 使用 Serde 属性控制序列化行为
5. 处理多种数据格式（JSON、YAML、TOML、CSV）

---

## 前置要求

- [结构体](../../basic/struct.md) - 数据类型定义
- [错误处理](../error-handling/error-handling.md) - Result 处理

---

### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde --features derive
cargo add serde_json  # JSON 格式
# cargo add serde_yaml  # YAML 格式
# cargo add toml        # TOML 格式
# cargo add csv         # CSV 格式
```

---

## 第一个例子

最简单的序列化示例：

```rust,ignore
use serde::{Serialize, Deserialize};

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

    // 序列化：结构体 → 字节/字符串
    // 反序列化：字节/字符串 → 结构体
    
    println!("原始数据：{:?}", person);

    Ok(())
}
```

**发生了什么？**

- `#[derive(Serialize, Deserialize)]` - 自动生成序列化代码
- Serde 框架不关心具体格式，只负责转换数据

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

**关键点**：
- **serde** 定义接口（`Serialize` / `Deserialize` trait）
- **serde_derive** 自动生成实现（派生宏）
- **格式库** 提供具体格式的序列化/反序列化

### 2. 序列化流程

```
Rust 结构体
    ↓
Serialize trait
    ↓
Serializer (JSON/YAML/TOML/CSV)
    ↓
字符串/字节
```

**反序列化流程相反**：

```
字符串/字节
    ↓
Deserializer (JSON/YAML/TOML/CSV)
    ↓
Deserialize trait
    ↓
Rust 结构体
```

### 3. 自定义序列化

当默认行为不满足需求时，可以自定义：

```rust,ignore
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 自定义序列化：将 Point 序列化为 [x, y] 数组
impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

// 自定义反序列化：从 [x, y] 数组反序列化为 Point
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<i32>::deserialize(deserializer)?;
        if vec.len() != 2 {
            return Err(serde::de::Error::custom("Expected 2 elements"));
        }
        Ok(Point { x: vec[0], y: vec[1] })
    }
}
```

### 4. Serde 属性

Serde 提供丰富的属性控制序列化行为：

```rust,ignore
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

    // 将嵌套对象展平到当前层级
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

fn default_age() -> u8 {
    18
}
```

### 5. 多格式支持

使用 Serde，同一结构体可以序列化成多种格式：

```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
    };

    // JSON
    let json = serde_json::to_string(&config).unwrap();
    println!("JSON: {}", json);

    // YAML (需要 serde_yaml crate)
    // let yaml = serde_yaml::to_string(&config).unwrap();
    // println!("YAML: {}", yaml);

    // TOML (需要 toml crate)
    // let toml = toml::to_string(&config).unwrap();
    // println!("TOML: {}", toml);
}
```

---

## 常见错误

### 错误 1: 忘记 derive

```rust,ignore
// ❌ 错误：没有 derive
struct Person {
    name: String,
}
// serde_json::to_string(&person)?;  // 编译错误！

// ✅ 正确：添加 derive
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}
```

### 错误 2: 不匹配的类型

```rust,ignore
// ❌ 错误：JSON 中的 age 是字符串，但结构体定义是数字
let json = r#"{"name": "Alice", "age": "30"}"#;
// let person: Person = serde_json::from_str(json)?;  // 反序列化失败！

// ✅ 正确：类型匹配
let json = r#"{"name": "Alice", "age": 30}"#;
```

### 错误 3: 生命周期问题

```rust,ignore
// ❌ 错误：反序列化引用需要生命周期标注
#[derive(Deserialize)]
struct User {
    name: &str,  // 需要生命周期
}

// ✅ 正确：使用拥有的类型
#[derive(Deserialize)]
struct User {
    name: String,  // 拥有的数据
}
```

---

## 动手练习

### 练习 1: 配置结构体

创建一个配置结构体，支持从多种格式加载：

```rust,ignore
// TODO: 定义 Config 结构体
// 字段：host (String), port (u16), debug (bool)
// 使用 Serde 属性提供默认值
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use serde::{Deserialize, Serialize};

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
```

</details>

### 练习 2: 自定义序列化

为 `Duration` 类型实现自定义序列化：

```rust,ignore
// TODO: 实现 Duration 的自定义序列化
// 序列化为 "1h 30m 45s" 格式
// 反序列化从相同格式解析
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}h {}m {}s", self.hours, self.minutes, self.seconds);
        serializer.serialize_str(&s)
    }
}
```

</details>

---

## 故障排查

### Q: serde_json 和 serde 有什么区别？

**A**: 
- **serde**: 序列化/反序列化框架 (trait 定义)
- **serde_json**: JSON 格式的具体实现

### Q: 如何处理未知字段？

**A**: 使用 `#[serde(deny_unknown_fields)]` 拒绝未知字段，或默认忽略。

### Q: 如何序列化枚举？

**A**: 使用 `#[serde(tag = "type")]` 控制枚举格式。

### Q: 如何提高序列化性能？

**A**: 使用 `serde_json::to_vec` 而不是 `to_string`，避免 UTF-8 转换。

---

## 知识扩展

### 性能优化

```rust,ignore
// 使用 to_vec 而不是 to_string (避免 UTF-8 转换)
let bytes = serde_json::to_vec(&data)?;

// 使用 serde_path_to_error 获取详细错误位置
use serde_path_to_error;

let result: Result<T, _> = serde_path_to_error::deserialize(deserializer);
```

### 零拷贝反序列化

```rust,ignore
use serde::Deserialize;

// 借用字符串，避免复制
#[derive(Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u8,
}
```

---

## 小结

**核心要点**：

1. **Serde 是框架**：定义接口，格式库提供实现
2. **#[derive(Serialize, Deserialize)]**: 自动生成代码
3. **自定义序列化**: 实现 trait
4. **Serde 属性**: 控制序列化行为
5. **多格式支持**: 同一结构体可序列化成多种格式

**关键术语**：

| English           | 中文       |
| ----------------- | ---------- |
| Serialization     | 序列化     |
| Deserialization   | 反序列化   |
| Derive Macro      | 派生宏     |
| Attribute         | 属性       |
| Serializer        | 序列化器   |
| Deserializer      | 反序列化器 |

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| Serialization     | 序列化     |
| Deserialization   | 反序列化   |
| Derive Macro      | 派生宏     |
| Attribute         | 属性       |
| Transcode         | 转码       |
| Zero-copy         | 零拷贝     |

---

完整示例：`src/advance/json_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[derive(Serialize, Deserialize)]` 做了什么？

2. 如何处理序列化错误？

3. `serde` 和 `serde_json` 的区别是什么？

<details>
<summary>点击查看答案与解析</summary>

1. 自动生成序列化和反序列化代码
2. 使用 `Result<T, serde_json::Error>` 处理
3. `serde` 是框架，`serde_json` 是 JSON 实现

**关键理解**: Serde 是 Rust 序列化的事实标准框架，支持多种格式。
</details>

## 继续学习

- 下一步：[JSON 序列化](json.md) - JSON 格式详细使用
- 进阶：[零拷贝序列化](rkyv.md) - Rkyv 零拷贝优化
- 回顾：[结构体](../../basic/struct.md)

> 💡 **记住**：Serde 让数据在任何地方都能安全"拆包"！
