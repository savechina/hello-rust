# 数据库高级应用

## 开篇故事

想象你正在开发一个智能推荐系统，需要存储数百万用户的行为数据，并快速找到相似用户。传统关系型数据库擅长结构化查询，但在处理向量相似度搜索时力不从心。现代应用需要**多模态数据库**——既能处理传统表格数据，又能进行高效的向量检索。本章介绍两种强大的 Rust 数据库方案：**SurrealDB**（多模型云原生数据库）和 **SQLite + sqlite-vec**（轻量级向量扩展），让你的应用具备 AI 时代的核心竞争力。

---

## 本章适合谁

如果你已经掌握了 SQLx 或 Diesel 等传统 ORM，现在想探索更先进的数据库技术，本章适合你。无论你是要构建 AI 应用、处理复杂数据关系，还是需要轻量级嵌入式向量搜索，这里都有适合你的方案。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 SurrealDB 进行文档存储和关系查询
2. 理解 SurrealDB 的 RecordID 和命名空间概念
3. 集成 sqlite-vec 扩展进行向量相似度搜索
4. 将向量数据库用于推荐系统和语义搜索
5. 在多模型数据库中管理复杂数据关系
6. 选择合适的嵌入式数据库方案

---

## 前置要求

学习本章前，你需要理解：

- [异步编程](../advance/async.md) - `async/await` 语法和 Tokio 运行时
- [结构体](../basic/struct.md) - 定义和使用结构体
- [Serde 序列化](../advance/serialization.md) - JSON 序列化和反序列化
- [SQLx 基础](../advance/sqlx.md) - 数据库连接和查询基础

---

## 第一个例子

### SurrealDB 内存数据库示例

```rust
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;

#[derive(Debug, Serialize)]
struct Person {
    title: &'static str,
    name: Name,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Name {
    first: &'static str,
    last: &'static str,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // 创建内存数据库连接
    let db = Surreal::new::<Mem>(()).await?;
    
    // 选择命名空间和数据库
    db.use_ns("test").use_db("test").await?;
    
    // 创建记录
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    
    println!("Created: {:?}", created);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}
```

**发生了什么？**

1. `Surreal::new::<Mem>(())` - 创建内存数据库（无需安装服务器）
2. `use_ns("test").use_db("test")` - 选择命名空间和数据库
3. `create("person")` - 在 person 表中创建记录
4. `.content()` - 使用 Serde 序列化 Rust 结构体
5. `RecordId` - SurrealDB 自动生成的唯一标识符

---

## 原理解析

### SurrealDB 架构概览

SurrealDB 是一个多模型数据库，支持文档、图、键值和关系模型：

```
+---------------------+     +---------------------+
|   Rust Application  |     |   SurrealDB Server  |
|                     |     |                     |
|  +---------------+  |     |  +---------------+  |
|  |  Surreal<...> |  |     |  |  Namespace    |  |
|  |  Client       |<-+---->+  |  (test)       |  |
|  +---------------+  |     |  +---------------+  |
|         |           |     |         |           |
|         | await     |     |         |           |
|         v           |     |         v           |
|  +---------------+  |     |  +---------------+  |
|  |  RecordId     |  |     |  |  Database     |  |
|  |  Typed Data   |  |     |  |  (test)       |  |
|  +---------------+  |     |  +---------------+  |
|                     |     |         |           |
+---------------------+     |         v           |
                            |  +---------------+  |
                            |  |  Tables       |  |
                            |  |  - person     |  |
                            |  |  - product    |  |
                            |  +---------------+  |
                            +---------------------+
```

### RecordID 与记录管理

SurrealDB 使用 `RecordId` 唯一标识每条记录：

```
RecordID 格式: table_name:record_id

示例:
- person:jaime          (指定 ID)
- person:ulid()         (自动生成 ULID)
- person:uuid()         (自动生成 UUID)
- person:rand()         (随机 ID)
```

**表 1-1**: RecordID 生成方式对比

| 方式 | 语法 | 特点 | 适用场景 |
|------|------|------|----------|
| 指定 ID | `person:jaime` | 可预测、可读 | 用户 ID、固定配置 |
| ULID | `person:ulid()` | 时间排序、唯一 | 日志、事件 |
| UUID | `person:uuid()` | 全局唯一 | 分布式系统 |
| 随机 | `person:rand()` | 不可预测 | 临时数据 |

### sqlite-vec 向量搜索原理

sqlite-vec 为 SQLite 添加向量索引能力：

```
+------------------+     +------------------+     +------------------+
|   查询向量       |     |   向量索引       |     |   相似度计算     |
|   [0.3, 0.3,     |     |   (vec0)         |     |   distance       |
|    0.3, 0.3]     |---->|                  |---->|                  |
+------------------+     +------------------+     +------------------+
                               |                           |
                               v                           v
                    +------------------+     +------------------+
                    |   向量表          |     |   返回 Top-K     |
                    |   vec_items       |     |   最相似记录     |
                    +------------------+     +------------------+
```

**向量相似度算法**：
- 欧几里得距离 (L2): `distance = sqrt(sum((a[i] - b[i])^2))`
- 余弦相似度: 适合语义搜索
- 内积: 适合推荐系统

---

## 常见错误

### 错误 1: 忘记切换命名空间

```rust
// 错误：未选择命名空间和数据库
let db = Surreal::new::<Mem>(()).await?;
// db.create("person")...  // 会失败！

// 正确：先选择 ns 和 db
db.use_ns("test").use_db("test").await?;
```

**修复方法**：创建连接后立即调用 `use_ns()` 和 `use_db()`

---

### 错误 2: sqlite-vec 扩展未加载

```rust
// 错误：未注册扩展，vec0 表无法创建
let db = Connection::open_in_memory()?;
db.execute("CREATE VIRTUAL TABLE vec_items USING vec0(...)", []);  // 失败！

// 正确：先注册扩展
unsafe {
    sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
}
```

**修复方法**：使用 `sqlite3_auto_extension` 在连接时加载扩展

---

### 错误 3: 向量维度不匹配

```rust
// 错误：插入 3 维向量到 4 维表
let db = Connection::open_in_memory()?;
db.execute("CREATE VIRTUAL TABLE vec_items USING vec0(embedding float[4])", [])?;

let v: Vec<f32> = vec![0.1, 0.2, 0.3];  // 只有 3 维！
// 插入时会报错或产生意外结果

// 正确：确保维度一致
let v: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];  // 4 维匹配
```

**修复方法**：创建表时指定的维度必须与插入数据一致

---

### 错误 4: 错误处理 Result 类型

```rust
// 问题：SurrealDB 和 SQLite 返回不同的 Result 类型
use surrealdb::Result;  // 这是 surrealdb::Result
use rusqlite::Result;   // 这是 rusqlite::Result

// 混淆会导致编译错误
```

**修复方法**：显式指定或使用完全限定名：

```rust
let result: surrealdb::Result<_> = db.create("person").content(data).await;
```

---

## 动手练习

### 练习 1: SurrealDB CRUD 操作

补全以下代码，实现完整的 CRUD 操作：

```rust
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
}

async fn create_product(db: &Surreal<Mem>, name: &str, price: f64) -> surrealdb::Result<()> {
    // 补全：创建产品记录
}

async fn get_product(db: &Surreal<Mem>, id: &str) -> surrealdb::Result<Option<Product>> {
    // 补全：根据 ID 查询产品
}

async fn update_price(db: &Surreal<Mem>, id: &str, new_price: f64) -> surrealdb::Result<()> {
    // 补全：更新产品价格
}
```

<details>
<summary>点击查看答案</summary>

```rust
async fn create_product(db: &Surreal<Mem>, name: &str, price: f64) -> surrealdb::Result<()> {
    db.create("product")
        .content(Product {
            name: name.to_string(),
            price,
        })
        .await?;
    Ok(())
}

async fn get_product(db: &Surreal<Mem>, id: &str) -> surrealdb::Result<Option<Product>> {
    db.select(("product", id)).await
}

async fn update_price(db: &Surreal<Mem>, id: &str, new_price: f64) -> surrealdb::Result<()> {
    db.update(("product", id))
        .merge(serde_json::json!({"price": new_price}))
        .await?;
    Ok(())
}
```

</details>

---

### 练习 2: 向量相似度搜索

完成以下代码，实现产品推荐功能：

```rust
use rusqlite::{Connection, Result};

fn setup_vector_db(db: &Connection) -> Result<()> {
    // 创建向量表存储产品特征
    // 每个产品有 4 维特征向量
    db.execute(
        "CREATE VIRTUAL TABLE products USING vec0(features float[4])",
        [],
    )?;
    
    // 插入产品数据
    let products = vec![
        (1, vec![0.9, 0.1, 0.0, 0.0]),  // 电子产品
        (2, vec![0.1, 0.9, 0.0, 0.0]),  // 服装
        (3, vec![0.8, 0.2, 0.0, 0.0]),  // 电子产品（类似产品1）
    ];
    
    // 补全：插入产品数据
}

fn find_similar_products(db: &Connection, query_vec: Vec<f32>, limit: usize) -> Result<Vec<(i64, f64)>> {
    // 补全：查询最相似的产品
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn setup_vector_db(db: &Connection) -> Result<()> {
    db.execute(
        "CREATE VIRTUAL TABLE products USING vec0(features float[4])",
        [],
    )?;
    
    let products = vec![
        (1, vec![0.9, 0.1, 0.0, 0.0]),
        (2, vec![0.1, 0.9, 0.0, 0.0]),
        (3, vec![0.8, 0.2, 0.0, 0.0]),
    ];
    
    let mut stmt = db.prepare("INSERT INTO products(rowid, features) VALUES (?, ?)")?;
    for (id, features) in products {
        stmt.execute(rusqlite::params![id, features.as_bytes()])?;
    }
    Ok(())
}

fn find_similar_products(db: &Connection, query_vec: Vec<f32>, limit: usize) -> Result<Vec<(i64, f64)>> {
    db.prepare(
        "SELECT rowid, distance FROM products WHERE features MATCH ?1 ORDER BY distance LIMIT ?2"
    )?
    .query_map(rusqlite::params![query_vec.as_bytes(), limit], |r| {
        Ok((r.get(0)?, r.get(1)?))
    })?
    .collect()
}
```

</details>

---

### 练习 3: 预测输出

以下代码的输出是什么？

```rust
use rusqlite::{Connection, Result};
use sqlite_vec::sqlite3_vec_init;
use rusqlite::ffi::sqlite3_auto_extension;

fn main() -> Result<()> {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
    
    let db = Connection::open_in_memory()?;
    
    let (sqlite_ver, vec_ver): (String, String) = db.query_row(
        "SELECT sqlite_version(), vec_version()",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    
    println!("SQLite: {}, vec: {}", sqlite_ver, vec_ver);
    Ok(())
}
```

<details>
<summary>点击查看解析</summary>

**输出示例**：
```
SQLite: 3.44.0, vec: v0.1.1
```

**解析**：
1. `sqlite_version()` 返回 SQLite 版本号
2. `vec_version()` 返回 sqlite-vec 扩展版本
3. 两个函数都在 SQL 层可用，说明扩展正确加载

</details>

---

## 故障排查 (FAQ)

### Q: SurrealDB 内存模式和持久化模式如何选择？

**A**: 根据使用场景选择：

```rust
// 内存模式 - 测试、临时数据
let db = Surreal::new::<Mem>(()).await?;

// RocksDB 持久化 - 生产环境
// let db = Surreal::new::<RocksDb>("/path/to/db").await?;

// 远程服务器模式
// let db = Surreal::new::<Ws>("ws://localhost:8000").await?;
```

**选择建议**：
- **Mem**：单元测试、CI/CD、演示
- **RocksDb**：单机应用、边缘计算
- **Ws/Wss**：分布式系统、多客户端

---

### Q: sqlite-vec 适合多大的数据集？

**A**: sqlite-vec 适合小到中等规模：

| 数据规模 | 建议方案 | 延迟 |
|----------|----------|------|
| < 10k 向量 | sqlite-vec | < 10ms |
| 10k - 100k | sqlite-vec + 索引优化 | 10-100ms |
| > 100k | 专用向量数据库 (pgvector, Milvus) | - |

---

### Q: SurrealDB 如何与 Rust 类型系统配合？

**A**: 使用 Serde 实现类型安全：

```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Thing,  // SurrealDB 的 Thing 类型对应 RecordId
    name: String,
    tags: Vec<String>,  // 支持复杂类型
}

// 查询时自动反序列化
let users: Vec<User> = db.select("user").await?;
```

---

### Q: 如何优化向量搜索性能？

**A**: 几种优化策略：

1. **维度降维**：使用 PCA 将高维向量降至 128-512 维
2. **量化**：使用 int8 替代 float32，减少 75% 存储
3. **分区**：按类别分区，缩小搜索空间
4. **缓存**：缓存热门查询结果

---

## 知识扩展 (选学)

### SurrealDB 高级查询

```rust
// 使用 SurrealQL 进行复杂查询
let results = db
    .query("SELECT * FROM person WHERE marketing = true ORDER BY name.first")
    .await?;

// 带参数查询（防止注入）
let results = db
    .query("SELECT * FROM person WHERE name.first = \$name")
    .bind(("name", "Tobie"))
    .await?;

// 使用变量
db.set("min_price", 100.0).await?;
let products = db
    .query("SELECT * FROM product WHERE price > \$min_price")
    .await?;
```

---

### 与 SQLx 集成

SurrealDB 和 sqlite-vec 可以与 SQLx 互补使用：

```rust
// SQLx 处理关系数据
let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;

// SurrealDB 处理文档/图数据
let related = db
    .query("SELECT ->purchased->product.* FROM user:alice")
    .await?;

// sqlite-vec 处理向量搜索
let similar = find_similar_products(&sqlite_conn, query_vector, 10)?;
```

---

## 小结

**核心要点**：

1. **SurrealDB** 是多模型数据库，支持文档、图、关系模型
2. **RecordId** 是 SurrealDB 的核心概念，支持多种 ID 生成策略
3. **sqlite-vec** 为 SQLite 添加轻量级向量搜索能力
4. **向量搜索**使用距离算法（欧几里得、余弦）找到相似项
5. **选择数据库**需考虑数据规模、查询模式和部署环境

**关键术语**：

- **RecordId**: SurrealDB 记录唯一标识符
- **Namespace/Database**: SurrealDB 的命名空间层级
- **向量数据库 (Vector DB)**: 支持相似度搜索的数据库
- **Embedding**: 文本/图像的向量表示
- **Similarity Search**: 基于向量距离的内容检索

**下一步**：

- 探索 [SurrealDB 官方文档](https://surrealdb.com/docs)
- 学习 [sqlite-vec 项目](https://github.com/asg017/sqlite-vec)
- 实践 [向量搜索应用](https://github.com/tantivy-search/tantivy)

---

## 术语表

| English | 中文 |
|---------|------|
| RecordId | 记录标识符 |
| Namespace | 命名空间 |
| Vector Search | 向量搜索 |
| Embedding | 嵌入向量 |
| Similarity | 相似度 |
| Dimension | 维度 |
| Distance Metric | 距离度量 |
| Approximate Nearest Neighbor | 近似最近邻 |

完整示例：
- [SurrealDB 示例](../../crates/awesome/src/database/surrealdb_sample.rs)
- [SQLite 向量扩展](../../crates/awesome/src/database/sqlite_vec_sample.rs)

---

## 继续学习

- 下一步：[消息队列](mq.md)
- 进阶：[Web 服务](services.md)
- 回顾：[SQLx 基础](../advance/sqlx.md)

---

## 💡 小知识：向量数据库的兴起

**为什么需要向量数据库？**

传统数据库擅长精确匹配（`WHERE id = 123`），但 AI 应用需要语义搜索：

```
用户查询: "便宜好用的耳机"

传统搜索: 匹配包含这些词的产品
向量搜索: 找到语义相似的产品
         "性价比高的蓝牙耳机"
         "平价入耳式耳机"
         "学生党耳机推荐"
```

**应用场景**：
- 语义搜索引擎
- 商品推荐系统
- 图像相似搜索
- 聊天机器人检索

---

## 🌟 工业界应用：智能推荐系统

**真实案例**：某电商平台使用 SQLite + sqlite-vec 构建推荐系统：

```rust
// 用户行为向量
fn user_embedding(purchase_history: &[Product]) -> Vec<f32> {
    // 基于购买历史生成用户画像向量
    let mut embedding = vec![0.0; 128];
    for product in purchase_history {
        for (i, &val) in product.category_vec.iter().enumerate() {
            embedding[i] += val;
        }
    }
    normalize(&embedding)
}

// 推荐相似用户喜欢的商品
async fn recommend(
    db: &Connection,
    user_vec: Vec<f32>,
) -> Result<Vec<Product>> {
    // 找到相似用户
    let similar_users: Vec<(i64, f64)> = db
        .prepare(
            "SELECT user_id, distance FROM user_vectors WHERE embedding MATCH ? ORDER BY distance LIMIT 10"
        )?
        .query_map([user_vec.as_bytes()], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>>>()?;
    
    // 返回这些用户购买的商品...
}
```

**结果**：
- 推荐准确率提升 35%
- 系统响应时间 < 50ms
- 单机可处理百万级用户

---

## 🧪 知识检查

**问题 1** 🟢 (基础概念)

SurrealDB 的 RecordId 格式是什么？

```rust
let id: RecordId = ("person", "jaime").into();
```

A) `jaime:person`
B) `person:jaime`
C) `person/jaime`
D) `jaime.person`

<details>
<summary>答案与解析</summary>

**答案**: B) `person:jaime`

**解析**: SurrealDB 的 RecordId 格式是 `table_name:record_id`，用于唯一标识表中的记录。
</details>

---

**问题 2** 🟡 (向量搜索)

sqlite-vec 的 `MATCH` 操作返回的结果按什么排序？

```rust
SELECT rowid, distance FROM vec_items WHERE embedding MATCH ? ORDER BY distance
```

A) 向量 ID
B) 相似度分数（越大越相似）
C) 距离（越小越相似）
D) 插入时间

<details>
<summary>答案与解析</summary>

**答案**: C) 距离（越小越相似）

**解析**: `distance` 表示查询向量与存储向量的距离（通常使用欧几里得距离），值越小表示越相似。通常需要 `ORDER BY distance` 获取最相似的结果。
</details>

---

**问题 3** 🔴 (多模型数据库)

以下哪个场景最适合使用 SurrealDB？

A) 只需要简单的键值存储
B) 需要文档存储 + 图关系 + 实时查询的社交应用
C) 只需要执行标准 SQL 查询的传统报表系统
D) 只需要存储二进制大对象的对象存储

<details>
<summary>答案与解析</summary>

**答案**: B) 需要文档存储 + 图关系 + 实时查询的社交应用

**解析**: SurrealDB 是多模型数据库，特别适合需要同时处理文档数据、复杂关系（图）和实时查询的应用。A 适合 Redis，C 适合传统 SQL 数据库，D 适合对象存储服务。
</details>

---

## 参考资料

1. [SurrealDB 官方文档](https://surrealdb.com/docs)
2. [sqlite-vec GitHub](https://github.com/asg017/sqlite-vec)
3. [向量数据库对比](https://github.com/milvus-io/milvus)
4. [Rust Embedded Databases](https://lib.rs/search?q=embedded+database)

---

> **记住**：现代应用往往需要多种数据模型。SurrealDB 提供一站式解决方案，而 sqlite-vec 为轻量级应用带来向量能力。选择适合你场景的工具，而不是试图用一个方案解决所有问题。
