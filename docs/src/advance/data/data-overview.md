# 数据处理

Rust 提供多种数据处理方案，从常见的 JSON/CSV 到高性能的零拷贝序列化，覆盖各种应用场景。

## 为什么关注数据处理？

- **性能关键**：序列化/反序列化往往是 Web 服务的性能瓶颈
- **类型安全**：Rust 的类型系统确保数据结构在编译时验证
- **零拷贝优化**：rkyv 等库提供极致的反序列化性能

## 本章节内容

| 主题 | 说明 |
|------|------|
| [序列化基础](./serialization.md) | Serde 框架核心概念，derive 宏使用 |
| [JSON 序列化](./json.md) | JSON 数据的读写、格式化、流式处理 |
| [CSV 处理](./csv.md) | CSV 文件的读取、写入、类型转换 |
| [零拷贝序列化](./rkyv.md) | Rkyv 零拷贝优化，适合高性能场景 |

## Serde 快速示例

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: "张三".to_string(),
        age: 28,
        email: "zhangsan@example.com".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
}
```

## 下一步

- 从 [序列化基础](./serialization.md) 开始学习 Serde
- 掌握 [JSON](./json.md) 和 [CSV](./csv.md) 的实际应用
- 了解 [Rkyv](./rkyv.md) 零拷贝优化
