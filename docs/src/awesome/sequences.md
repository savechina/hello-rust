# 序列 ID 生成

## 开篇故事

想象你在设计一个电商平台。每当有新订单产生，系统需要为订单分配一个**唯一的标识符**。如果两个订单获得相同的 ID，后果可能是客户 A 支付了客户 B 的订单、库存系统混乱、财务报表数据错误。

在分布式系统中，这个问题更加棘手。多个服务器同时处理请求，如何保证生成的 ID 在全局范围内唯一？这就是 **UUID (Universally Unique Identifier)** 的价值所在。

---

## 本章适合谁

如果你需要为数据库记录生成唯一主键、在分布式系统中标识资源、实现订单号或流水号等业务 ID，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `uuid` crate 生成不同版本的 UUID (v3, v4, v5, v7)
2. 根据业务场景选择合适的 UUID 版本
3. 基于业务字段生成确定性的 GUID
4. 解析和格式化 UUID 字符串
5. 使用 URN 格式表示 UUID

---

## 前置要求

- Rust 基础语法
- 理解字符串和字节切片
- 熟悉 Cargo 依赖管理

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add uuid --features v4,v5,v7,md5
cargo add md-5
cargo add chrono
```

## 第一个例子

最常用的 UUID v4 随机生成：

```rust,ignore
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    println!("生成的 UUID: {}", id);
    // 输出示例: 550e8400-e29b-41d4-a716-446655440000
}
```

完整示例：[crates/awesome/src/sequences/uuid_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/sequences/uuid_sample.rs)

---

## 原理解析

### UUID 版本对比

| 版本 | 生成方式 | 特点 | 适用场景 |
|------|---------|------|---------|
| v1 | 时间戳+MAC 地址 | 可排序，暴露硬件 | 遗留系统兼容 |
| v3 | MD5 哈希 | 确定性，已弃用 | 旧系统兼容 |
| v4 | 随机数 | 最常用，简单 | 通用唯一标识 |
| v5 | SHA-1 哈希 | 确定性，安全 | 相同输入相同输出 |
| v7 | 时间戳+随机 | 可排序，现代标准 | 数据库主键 |
| NIL | 全零 | 特殊值 | 表示"空"或"无效" |

### UUID v4 - 随机生成

```rust,ignore
// 最常用的 UUID 版本，使用加密安全的随机数生成器
let uuid_v4 = Uuid::new_v4();
println!("Version 4 UUID: {}", uuid_v4);
// 输出: 936c342f-76a0-4a8b-8e1d-3b7c8a9e0f1d
```

### UUID v3/v5 - 基于名称的确定性生成

```rust,ignore
// 使用命名空间和名称生成确定性 UUID
// 相同输入总是产生相同输出
let namespace_url = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8")?;
let name = "example.com";

// v3 使用 MD5（已弃用，仅用于兼容）
let uuid_v3 = Uuid::new_v3(&namespace_url, name.as_bytes());
println!("Version 3 UUID: {}", uuid_v3);

// v5 使用 SHA-1（推荐用于确定性场景）
let uuid_v5 = Uuid::new_v5(&namespace_url, name.as_bytes());
println!("Version 5 UUID: {}", uuid_v5);
```

### UUID v7 - 时间排序（现代标准）

```rust,ignore
// 最简单的 v7 生成方式
let uuid_v7 = Uuid::now_v7();
println!("UUID v7: {}", uuid_v7);

// 使用自定义时间戳
let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)?;

let ts = Timestamp::from_unix(NoContext, now.as_secs(), now.subsec_nanos());
let uuid_v7_custom = Uuid::new_v7(ts);
println!("UUID v7 (custom timestamp): {}", uuid_v7_custom);
```

**优势**：
- 按时间排序，适合数据库索引
- 无需暴露 MAC 地址
- 比 v4 更适合做主键（索引友好）

### NIL UUID - 特殊值

```rust,ignore
// 全零 UUID，表示"空"或"无效"
let uuid_nil = Uuid::nil();
println!("NIL UUID: {}", uuid_nil);
// 输出: 00000000-0000-0000-0000-000000000000
```

### URN 格式和解析

```rust,ignore
// URN (Uniform Resource Name) 格式
let uuid = Uuid::new_v4();
println!("URN: {}", uuid.urn());
// 输出: urn:uuid:936c342f-76a0-4a8b-8e1d-3b7c8a9e0f1d

// 从字符串解析 UUID
let uuid_str = "f8a7e0d1-c2b3-4a5b-6c7d-8e9f0a1b2c3d";
match Uuid::parse_str(uuid_str) {
    Ok(parsed) => println!("Parsed: {}", parsed),
    Err(e) => println!("Parse failed: {}", e),
}
```

### 基于业务字段生成 GUID

```rust,ignore
/// 基于多个业务字段生成确定性 GUID
pub fn generate_guid_from_fields(
    tenant_id: &str,
    area_id: u64,
    area_code: &str,
    object_code: &str,
    object_type: i32,
) -> Uuid {
    // 组合所有业务字段
    let combined = format!(
        "{}-{}-{}-{}-{}",
        tenant_id, area_id, area_code, object_code, object_type
    );

    // 使用 MD5 哈希生成 16 字节
    let mut hasher = md5::Context::new();
    hasher.write(combined.as_bytes());
    let md5_bytes = hasher.finalize().into();

    // 从 MD5 字节创建 UUID
    Uuid::from_bytes(md5_bytes)
}

// 使用示例
let guid = generate_guid_from_fields("T001", 1001, "A01", "OBJ-001", 0);
println!("Generated GUID: {}", guid);
```

---

## 常见错误

### 错误 1: 混淆 UUID 字符串格式

使用标准 36 字符格式（含连字符）：`550e8400-e29b-41d4-a716-446655440000`

### 错误 2: 假设 UUID 完全唯一

关键系统应检查唯一性（v4 碰撞概率极低但非零）

### 错误 3: 在数据库主键中使用 v4 而非 v7

v4 随机 UUID 不适合做聚簇索引（插入位置随机导致页分裂），v7 时间排序更适合

---

## 知识检查

**问题 1**: 哪个 UUID 版本使用随机数生成？

**问题 2**: 数据库主键应该选择哪个 UUID 版本？

**问题 3**: UUID v3 和 v5 的区别是什么？

<details>
<summary>点击查看答案与解析</summary>

1. UUID v4 使用加密安全的随机数生成器
2. UUID v7（时间排序，索引友好，避免页分裂）
3. v3 使用 MD5（已弃用），v5 使用 SHA-1（推荐）

**关键理解**: 选择 UUID 版本取决于你的需求：唯一性用 v4，排序用 v7，确定性用 v5。
</details>

---

## 延伸阅读

学习完序列 ID 生成后，你可能还想了解：

- [Snowflake ID](https://github.com/twitter-archive/snowflake) - Twitter 的分布式 ID 生成算法
- [ULID](https://github.com/ulid/spec) - 类似 UUID v7 的可排序 ID
- [NanoID](https://github.com/ai/nanoid) - 紧凑的 URL 安全 ID

**选择建议**:
- 通用唯一标识 → UUID v4
- 数据库主键 → UUID v7 或 Snowflake
- 确定性 ID → UUID v5 或 ULID

---

## 小结

核心要点：UUID v4 随机生成、UUID v7 时间排序、UUID v5 确定性生成、NIL 表示空值

完整示例：[crates/awesome/src/sequences/uuid_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/sequences/uuid_sample.rs)
