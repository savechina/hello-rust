# JSON 序列化

## 开篇故事

想象你要寄快递。你需要把物品打包成标准格式，贴上标签，才能通过快递系统运输。JSON 序列化就像这个打包过程——把你的 Rust 数据结构转换成标准的 JSON 格式，以便在不同系统间传输。

---

## 本章适合谁

如果你需要在 Rust 程序中处理 JSON 数据（读取配置文件、调用 API、存储数据），本章适合你。JSON 是现代编程中最常用的数据交换格式。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 serde_json 解析 JSON 数据
2. 将 Rust 结构体序列化为 JSON
3. 处理无类型和有类型的 JSON 数据
4. 处理 JSON 错误和验证
5. 自定义序列化和反序列化行为

---

## 前置要求

- 结构体 - 结构体定义
- [Serde 序列化](serialization.md) - 序列化基础
- 枚举 - 枚举类型

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde --features derive
cargo add serde_json
```

## 第一个例子

最简单的 JSON 解析：

```rust,ignore
use serde_json::{Result, Value};

fn main() -> Result<()> {
    // JSON 字符串
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // 解析为 Value
    let v: Value = serde_json::from_str(data)?;

    // 访问字段
    println!("姓名：{}", v["name"]);
    println!("电话：{}", v["phones"][0]);

    Ok(())
}
```

**完整示例**: [json_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/json_sample.rs)

---

## 原理解析

### serde 生态系统

**serde 是序列化框架**：

- ✅ 支持多种格式（JSON, YAML, TOML 等）
- ✅ 类型安全
- ✅ 零开销序列化
- ✅ 派生宏简化使用

### 无类型 JSON (Value)

**适用场景**: 未知结构的 JSON 数据

```rust,ignore
use serde_json::Value;

let json_str = r#"{"name": "Alice", "age": 30}"#;
let v: Value = serde_json::from_str(json_str).unwrap();

// 访问字段
println!("{}", v["name"]);  // "Alice"
println!("{}", v["age"]);   // 30

// 检查类型
assert!(v["name"].is_string());
assert!(v["age"].is_number());
```

### 有类型 JSON (Struct)

**适用场景**: 已知结构的 JSON 数据

```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// JSON → Struct (反序列化)
let json_str = r#"{"name":"John","age":43,"phones":["123","456"]}"#;
let person: Person = serde_json::from_str(json_str).unwrap();

// Struct → JSON (序列化)
let json = serde_json::to_string(&person).unwrap();
```

### 错误处理

```rust,ignore
use serde_json::{Result, Error};

fn parse_json(json_str: &str) -> Result<()> {
    match serde_json::from_str::<Value>(json_str) {
        Ok(value) => {
            println!("解析成功：{}", value);
            Ok(())
        }
        Err(e) => {
            eprintln!("解析失败：{}", e);
            Err(e)
        }
    }
}
```

### 美化输出

```rust,ignore
let person = Person {
    name: "Alice".to_string(),
    age: 30,
    phones: vec!["123".to_string()],
};

// 紧凑格式
let compact = serde_json::to_string(&person).unwrap();
// {"name":"Alice","age":30,"phones":["123"]}

// 美化格式
let pretty = serde_json::to_string_pretty(&person).unwrap();
/*
{
  "name": "Alice",
  "age": 30,
  "phones": [
    "123"
  ]
}
*/
```

### 自定义序列化

```rust,ignore
use serde::{Serialize, Serializer};

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
        serializer.serialize_str(&format!("({}, {})", self.x, self.y))
    }
}
```

---

## 常见错误

### 错误 1: 字段不匹配

```rust,ignore
#[derive(Deserialize)]
struct Person {
    name: String,
    age: u8,
}

let json = r#"{"name": "Alice"}"#;  // ❌ 缺少 age 字段
let person: Person = serde_json::from_str(json).unwrap();
```

**错误信息**:
```
missing field `age`
```

**修复方法**:
```rust,ignore
#[derive(Deserialize)]
struct Person {
    name: String,
    #[serde(default)]  // ✅ 提供默认值
    age: u8,
}
```

### 错误 2: 类型不匹配

```rust,ignore
#[derive(Deserialize)]
struct Person {
    age: u8,
}

let json = r#"{"age": "twenty"}"#;  // ❌ 期望数字，得到字符串
let person: Person = serde_json::from_str(json).unwrap();
```

**错误信息**:
```
invalid type: string "twenty", expected u8
```

**修复方法**:
```rust,ignore
let json = r#"{"age": 20}"#;  // ✅ 正确的类型
```

### 错误 3: JSON 格式错误

```rust,ignore
let json = r#"{"name": "Alice",}"#;  // ❌ 多余的逗号
let v: Value = serde_json::from_str(json).unwrap();
```

**错误信息**:
```
trailing comma at line 1 column 20
```

**修复方法**:
```rust,ignore
let json = r#"{"name": "Alice"}"#;  // ✅ 正确的 JSON
```

---

## 动手练习

### 练习 1: 解析用户数据

```rust,ignore
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    // TODO: 定义字段
    // - id (u32)
    // - username (String)
    // - email (String)
}

fn main() {
    let json = r#"{
        "id": 1,
        "username": "rustacean",
        "email": "rust@example.com"
    }"#;

    // TODO: 解析 JSON
    // TODO: 打印用户信息
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
}

fn main() {
    let user: User = serde_json::from_str(json).unwrap();
    println!("用户：{:?}", user);
}
```
</details>

### 练习 2: 序列化配置

```rust,ignore
#[derive(Serialize)]
struct Config {
    // TODO: 定义配置字段
    // - host (String)
    // - port (u16)
    // - debug (bool)
}

fn main() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
    };

    // TODO: 序列化为 JSON
    // TODO: 打印 JSON 字符串
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Serialize)]
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

    let json = serde_json::to_string(&config).unwrap();
    println!("{}", json);
}
```
</details>

### 练习 3: 处理嵌套结构

```rust,ignore
#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    author: Author,
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

fn main() {
    let json = r#"{
        "id": 1,
        "title": "Rust JSON",
        "author": {
            "name": "张三",
            "email": "zhang@example.com"
        }
    }"#;

    // TODO: 解析嵌套 JSON
    // TODO: 打印作者邮箱
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let post: Post = serde_json::from_str(json).unwrap();
println!("作者邮箱：{}", post.author.email);
```
</details>

---

## 故障排查 (FAQ)

### Q: serde_json 和 serde 有什么区别？

**A**: 
- **serde**: 序列化/反序列化框架 (trait 定义)
- **serde_json**: JSON 格式的具体实现

```rust,ignore
use serde::{Serialize, Deserialize};  // 框架
use serde_json;  // JSON 实现
```

### Q: 如何处理可选字段？

**A**: 使用 `Option<T>`:

```rust,ignore
#[derive(Deserialize)]
struct User {
    name: String,      // 必需
    age: Option<u8>,   // 可选
}
```

### Q: 如何自定义字段名？

**A**: 使用 `#[serde(rename)]`:

```rust,ignore
#[derive(Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    name: String,
}
```

---

## 知识扩展

### 从文件读取

```rust,ignore
use std::fs::File;
use std::io::BufReader;

let file = File::open("config.json")?;
let reader = BufReader::new(file);
let config: Config = serde_json::from_reader(reader)?;
```

### 性能优化

```rust,ignore
// 使用 serde_json::Value 进行快速解析
let value: Value = serde_json::from_str(json)?;

// 使用 serde_path_to_error 获取详细错误位置
use serde_path_to_error;

let result: Result<T, _> = serde_path_to_error::deserialize(deserializer);
```

### 动态 JSON 处理

```rust,ignore
// 处理未知结构的 JSON
fn process_value(value: &Value) {
    match value {
        Value::Null => println!("null"),
        Value::Bool(b) => println!("bool: {}", b),
        Value::Number(n) => println!("number: {}", n),
        Value::String(s) => println!("string: {}", s),
        Value::Array(arr) => {
            for item in arr {
                process_value(item);
            }
        }
        Value::Object(obj) => {
            for (key, value) in obj {
                println!("{}: ", key);
                process_value(value);
            }
        }
    }
}
```

---

## 小结

**核心要点**：

1. **serde + serde_json**: Rust 的 JSON 处理标准
2. **Value vs Struct**: 无类型 vs 有类型解析
3. **derive 宏**: 自动实现 Serialize/Deserialize
4. **错误处理**: 使用 Result 处理解析错误
5. **自定义行为**: 使用 serde 属性自定义序列化

**关键术语**：

- **Serialization (序列化)**: Rust 结构体 → JSON
- **Deserialization (反序列化)**: JSON → Rust 结构体
- **Value**: 无类型的 JSON 值
- **derive macro**: 自动派生 trait 实现

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Serialization | 序列化 |
| Deserialization | 反序列化 |
| Serialize | 序列化 trait |
| Deserialize | 反序列化 trait |
| derive macro | 派生宏 |
| Value | 值类型 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `serde_json::Value` 和类型化反序列化有什么区别？

2. 如何处理 JSON 中可能缺失的字段？

3. `#[serde(flatten)]` 的作用是什么？

<details>
<summary>点击查看答案与解析</summary>

1. `Value` 是无类型的动态 JSON 对象，类型化反序列化直接得到结构体
2. 使用 `Option<T>` 字段，或 `#[serde(default)]`
3. 将嵌套对象"展平"到当前结构体中

**关键理解**: 优先使用类型化反序列化，`Value` 仅在动态场景使用。
</details>

## 继续学习

**前一章**: Hyper HTTP 库  
**下一章**: [CSV 处理](csv.md)

**相关章节**:
- Hyper HTTP 库
- [CSV 处理](csv.md)
- [零拷贝序列化](rkyv.md)

**返回**: 高级进阶

---

**完整示例**: [json_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/json_sample.rs)
