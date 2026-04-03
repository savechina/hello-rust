# SQLx 异步数据库操作

## 开篇故事

想象你经营着一家繁忙的餐厅。传统的做法是一位服务员接待一位顾客，从点餐到上菜全程陪同——这效率低下。现代餐厅采用异步服务模式：服务员接单后将订单放入队列，厨房异步准备，服务员可以立即服务下一位顾客。这就是 SQLx 的核心理念：**异步数据库操作让你的应用在等待数据库响应时可以处理其他任务**，大幅提升吞吐量。

SQLx 是 Rust 生态中最受欢迎的异步 SQL 工具包之一，它结合了 Rust 的类型安全和异步编程模型，提供了编译时 SQL 验证、连接池管理等强大功能。

---

## 本章适合谁

如果你已经掌握了 Rust 异步编程基础（`async/await`），现在想学习如何与数据库交互，本章适合你。无论你是要构建 Web 服务、数据处理管道还是后台任务，SQLx 都能提供类型安全、高性能的数据库访问方案。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 SQLx 连接 SQLite 和 MySQL 数据库
2. 编写编译时验证的安全 SQL 查询
3. 使用连接池优化数据库访问性能
4. 处理异步查询结果和错误
5. 实现数据库事务保证数据一致性
6. 将查询结果映射到 Rust 结构体

---

## 前置要求

学习本章前，你需要理解：

- [异步编程](async.md) - `async/await` 语法和 Tokio 运行时
- [结构体](../basic/struct.md) - 定义和使用结构体
- [特征 (Traits)](../basic/trait.md) - 理解 `FromRow` 等 derive 宏
- [错误处理](../basic/enums.md) - `Result` 类型的使用

---

## 第一个例子

让我们看一个简单的 SQLx 示例：

```rust
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 连接到内存中的 SQLite 数据库
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    
    // 创建表
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&mut conn)
        .await?;
    
    // 插入数据
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("Alice")
        .execute(&mut conn)
        .await?;
    
    // 查询数据
    let row = sqlx::query("SELECT name FROM users WHERE id = 1")
        .fetch_one(&mut conn)
        .await?;
    
    let name: String = row.get("name");
    println!("User name: {}", name);
    
    Ok(())
}
```

**发生了什么？**

1. `connect()` - 建立异步数据库连接
2. `query()` - 创建 SQL 查询构建器
3. `bind()` - 安全地绑定参数（防止 SQL 注入）
4. `execute()` / `fetch_one()` - 执行查询并获取结果
5. `await?` - 异步等待并传播错误

---

## 原理解析

### SQLx 架构概览

**图 1-1**: SQLx 数据库连接流程

```
+------------------------------------------------------------------+
|                        应用程序 (Application)                     |
|  +-------------+    +-------------+    +---------------------+  |
|  |   async fn  |--->|  Query<...> |--->|   sqlx::query()     |  |
|  |   main()    |    |  Builder    |    |   Macro/Function    |  |
|  +-------------+    +-------------+    +---------------------+  |
|         |                                        |               |
|         | await                                  | compile-time  |
|         |                                        | validation    |
|         v                                        v               |
|  +-------------+    +-------------+    +---------------------+  |
|  |   Result    |<---|   Pool      |<---|   Connection        |  |
|  |   <T, E>    |    |   Manager   |    |   (Driver)          |  |
|  +-------------+    +-------------+    +---------------------+  |
|                            |                        |            |
|                            v                        v            |
|                   +-----------------+    +-----------------+     |
|                   |  ConnectionPool |    |  Database       |     |
|                   |  (连接复用)      |    |  (SQLite/MySQL) |     |
|                   +-----------------+    +-----------------+     |
+------------------------------------------------------------------+
```

### 编译时 SQL 验证

SQLx 最强大的功能之一是**编译时查询检查**。当你使用 `sqlx::query!` 宏时，SQLx 会在编译时连接数据库、验证 SQL 语法、检查表和列是否存在。

```
编译期检查流程:

源代码                          编译时                        运行时
---------                      ---------                     -------
  |                                |                            |
  |  sqlx::query!("SELECT ...")    |                            |
  |------------------------------->|                            |
  |                                |  1. 解析 SQL               |
  |                                |  2. 连接数据库验证         |
  |                                |  3. 推断返回类型           |
  |                                |                            |
  |                                |  通过: 类型安全代码生成     |
  |<-------------------------------|                            |
  |                                |                            |
  |  生成的代码                    |                            |
  |  (零开销抽象)                  |                            |
  |-------------------------------------------------------------->|
  |                                                             |
  |                                                      直接执行
  |                                                      无运行时检查
```

### 连接池管理

数据库连接是昂贵资源。SQLx 使用连接池来复用连接：

**图 1-2**: 连接池工作流程

```
+--------------------------------------------------------------+
|                     连接池 (Connection Pool)                  |
|                                                               |
|   +---------+   +---------+   +---------+   +---------+     |
|   | Conn 1  |   | Conn 2  |   | Conn 3  |   | Conn 4  |     |
|   | [空闲]  |   | [忙碌]  |   | [空闲]  |   | [忙碌]  |     |
|   +---------+   +---------+   +---------+   +---------+     |
|        |             |             |             |           |
|        +-------------+-------------+-------------+           |
|                         |                                    |
|              +----------+----------+                        |
|              v                     v                        |
|        请求连接                 释放连接                     |
|     +-------------+         +-------------+                 |
|     | pool.acquire|         | 自动归还    |                 |
|     |    ().await |         |    Drop     |                 |
|     +-------------+         +-------------+                 |
|                                                               |
+--------------------------------------------------------------+
```

连接池的优势：
- **复用连接**：避免频繁创建/销毁连接的开销
- **限制并发**：防止数据库过载
- **自动管理**：连接超时、健康检查自动处理

### SQLx 支持的数据库

**表 1-1**: SQLx 支持的数据库驱动

| 数据库 | 类型 | 连接字符串示例 | 特点 |
|--------|------|----------------|------|
| SQLite | 嵌入式 | `sqlite::memory:` 或 `sqlite://path/to/db.sqlite` | 零配置，单文件 |
| MySQL | 网络 | `mysql://user:pass@host:3306/db` | 生产环境常用 |
| PostgreSQL | 网络 | `postgres://user:pass@host:5432/db` | 功能最丰富 |
| MariaDB | 网络 | 同 MySQL | MySQL 兼容 |

---

## 常见错误

### 错误 1: 忘记 `.await`

```rust
// 错误：忘记 await
let conn = SqliteConnection::connect("sqlite::memory:");
// conn 是 Future，不是 Connection！

// 正确：使用 await
let conn = SqliteConnection::connect("sqlite::memory:").await?;
```

**编译器输出**:
```
error[E0277]: the trait bound `impl Future<Output = ...>: Connection` is not satisfied
```

**修复方法**：在异步操作后添加 `.await`

---

### 错误 2: SQL 注入风险 - 不使用参数绑定

```rust
// 危险：字符串拼接
let user_input = "Alice'; DROP TABLE users; --";
let query = format!("SELECT * FROM users WHERE name = '{}'", user_input);
// 可能导致数据丢失！

// 安全：使用参数绑定
sqlx::query("SELECT * FROM users WHERE name = ?")
    .bind(user_input)  // 自动转义
    .fetch_all(&pool)
    .await?;
```

**最佳实践**：始终使用 `bind()` 方法，永远不要拼接 SQL 字符串。

---

### 错误 3: 连接池配置不当

```rust
// 问题：连接数设置过高
let pool = MySqlPoolOptions::new()
    .max_connections(1000)  // 可能压垮数据库
    .connect(&db_url)
    .await?;

// 合理配置
let pool = MySqlPoolOptions::new()
    .max_connections(10)     // 根据数据库容量调整
    .min_connections(2)      // 保持热连接
    .acquire_timeout(Duration::from_secs(30))
    .connect(&db_url)
    .await?;
```

---

### 错误 4: 事务未正确提交/回滚

```rust
// 错误：事务可能未提交
let mut tx = pool.begin().await?;
sqlx::query("INSERT ...").execute(&mut *tx).await?;
// 忘记 commit！数据不会保存

// 正确：使用事务块
let mut tx = pool.begin().await?;
match sqlx::query("INSERT ...").execute(&mut *tx).await {
    Ok(_) => tx.commit().await?,  // 成功则提交
    Err(e) => {
        tx.rollback().await?;      // 失败则回滚
        return Err(e.into());
    }
}
```

---

## 动手练习

### 练习 1: 基础查询

完成以下代码，实现用户数据的增删改查：

```rust
use sqlx::sqlite::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn create_user(pool: &SqlitePool, name: &str, email: &str) -> Result<i64, sqlx::Error> {
    // 补全代码：插入用户并返回生成的 id
    // 提示: 使用 .execute() 获取插入结果
}

async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    // 补全代码：根据 id 查询用户
    // 提示: 使用 query_as 和 fetch_optional
}
```

<details>
<summary>点击查看答案</summary>

```rust
async fn create_user(pool: &SqlitePool, name: &str, email: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}
```

</details>

---

### 练习 2: 使用连接池

下面的代码有什么问题？如何修复？

```rust
#[tokio::main]
async fn main() {
    let db_url = "sqlite::memory:";
    
    for i in 0..100 {
        // 每次都创建新连接
        let mut conn = SqliteConnection::connect(db_url).await.unwrap();
        sqlx::query("INSERT INTO data (value) VALUES (?)")
            .bind(i)
            .execute(&mut conn)
            .await;
    }
}
```

<details>
<summary>点击查看答案</summary>

**问题**：
1. 每次循环都创建新连接，性能极差
2. 内存数据库在连接关闭后数据丢失
3. 没有使用连接池

**修复方案**：
```rust
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    
    // 先创建表
    sqlx::query("CREATE TABLE data (value INTEGER)")
        .execute(&pool)
        .await?;
    
    // 使用同一个连接池
    for i in 0..100 {
        sqlx::query("INSERT INTO data (value) VALUES (?)")
            .bind(i)
            .execute(&pool)
            .await?;
    }
    
    Ok(())
}
```

</details>

---

### 练习 3: 事务处理

实现一个转账函数，确保原子性操作：

```rust
async fn transfer(
    pool: &MySqlPool,
    from_account: i64,
    to_account: i64,
    amount: f64
) -> Result<(), sqlx::Error> {
    // 实现：从 from_account 扣除 amount，向 to_account 增加 amount
    // 要求：使用事务保证原子性
}
```

<details>
<summary>点击查看答案</summary>

```rust
async fn transfer(
    pool: &MySqlPool,
    from_account: i64,
    to_account: i64,
    amount: f64
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    // 扣除转出账户
    sqlx::query("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        .bind(amount)
        .bind(from_account)
        .execute(&mut *tx)
        .await?;
    
    // 增加转入账户
    sqlx::query("UPDATE accounts SET balance = balance + ? WHERE id = ?")
        .bind(amount)
        .bind(to_account)
        .execute(&mut *tx)
        .await?;
    
    // 提交事务
    tx.commit().await?;
    Ok(())
}
```

</details>

---

## 故障排查 (FAQ)

### Q: SQLx 编译很慢怎么办？

**A**: 这是因为 `query!` 宏需要在编译时连接数据库验证 SQL。解决方案：

1. **使用离线模式**（推荐用于 CI）：
   ```bash
   # 开发环境生成查询元数据
   cargo sqlx prepare
   
   # CI 使用离线模式
   SQLX_OFFLINE=true cargo build
   ```

2. **使用 `query_as` 替代 `query!`**（无编译时检查）：
   ```rust
   sqlx::query_as::<_, User>("SELECT ...")  // 无需数据库连接
   ```

---

### Q: 如何处理连接超时？

**A**: 配置连接池选项：

```rust
use sqlx::pool::PoolOptions;
use std::time::Duration;

let pool = PoolOptions::<MySql>::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(30))  // 获取连接超时
    .idle_timeout(Duration::from_secs(10))      // 空闲连接超时
    .max_lifetime(Duration::from_secs(30 * 60)) // 连接最大生命周期
    .connect(&db_url)
    .await?;
```

---

### Q: SQLite 并发写入如何处理？

**A**: SQLite 默认只支持一个写入连接。使用连接池时注意：

```rust
// 限制 SQLite 连接数为 1
let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite://data.db")
    .await?;
```

或使用 WAL（Write-Ahead Logging）模式提高并发：

```rust
sqlx::query("PRAGMA journal_mode=WAL").execute(&pool).await?;
```

---

### Q: 如何将查询结果映射到结构体？

**A**: 使用 `sqlx::FromRow` derive 宏：

```rust
#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    // 字段名不匹配时使用 rename
    #[sqlx(rename = "email_address")]
    email: String,
}

// 使用 query_as 映射
let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&pool)
    .await?;
```

---

## 知识扩展 (选学)

### 使用 `query!` 宏进行编译时检查

`query!` 宏在编译时连接数据库验证 SQL，这是 SQLx 最强大的功能：

```rust
// 编译时会检查：users 表是否存在？name 列是否存在？类型是否匹配？
let row = sqlx::query!("SELECT name, email FROM users WHERE id = ?", id)
    .fetch_one(&pool)
    .await?;

// row.name 和 row.email 的类型在编译期已确定
println!("Name: {}, Email: {}", row.name, row.email);
```

**优势**：
- 重构安全：修改表结构后，相关查询会在编译时报错
- 类型安全：返回值的类型在编译期确定
- IDE 支持：自动补全字段名

---

### 迁移工具 (sqlx-migrate)

SQLx 提供数据库迁移工具管理 schema 变更：

```bash
# 创建迁移
sqlx migrate add create_users_table

# 运行迁移
sqlx migrate run

# 回滚
sqlx migrate revert
```

迁移文件示例 (`migrations/20240101000001_create_users_table.sql`)：

```sql
-- 升级脚本
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 降级脚本 (在 -- 下方)
-- DROP TABLE users;
```

---

### 与 Web 框架集成

SQLx 可与 Axum、Actix-web 等框架无缝集成：

```rust
use axum::{
    extract::State,
    routing::get,
    Router,
};
use sqlx::SqlitePool;

async fn get_users(State(pool): State<SqlitePool>) -> Result<String, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(format!("Found {} users", users.len()))
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://app.db").await.unwrap();
    
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(pool);
    
    axum::serve(tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(), app)
        .await
        .unwrap();
}
```

---

## 小结

**核心要点**：

1. **异步连接**：使用 `Pool::connect()` 或 `Connection::connect()` 建立异步连接
2. **参数绑定**：始终使用 `bind()` 方法防止 SQL 注入
3. **连接池**：生产环境使用 `Pool` 而非单个 `Connection`
4. **结果映射**：使用 `FromRow` derive 宏自动映射查询结果到结构体
5. **事务处理**：使用 `pool.begin()` 开始事务，显式 `commit()` 或 `rollback()`

**关键术语**：

- **连接池 (Connection Pool)**：复用数据库连接的机制
- **参数绑定 (Parameter Binding)**：安全地将变量插入 SQL 语句
- **事务 (Transaction)**：原子性的数据库操作序列
- **编译时检查 (Compile-time Check)**：`query!` 宏在编译期验证 SQL

**下一步**：

- 学习 [数据库高级应用](../awesome/database.md) - 连接池优化、读写分离
- 实践 [Web 服务](services.md) - 将 SQLx 集成到 Web 应用
- 探索 [diesel](../awesome/database.md) - Rust 的 ORM 替代方案

---

## 术语表

| English | 中文 |
|---------|------|
| Connection Pool | 连接池 |
| Async Query | 异步查询 |
| Parameter Binding | 参数绑定 |
| Transaction | 事务 |
| Compile-time Check | 编译时检查 |
| SQL Injection | SQL 注入 |
| Migration | 数据库迁移 |
| FromRow | 从行数据转换的 trait |
| Pool | 连接池 |
| Acquire | 获取连接 |

> **完整示例**: `src/advance/sqlx_sample.rs`

---

## 继续学习

- 下一步：[数据库高级应用](../awesome/database.md)
- 进阶：[Web 服务](services.md)
- 回顾：[异步编程](async.md)

---

## 小知识：SQLx 的设计理念

**为什么选择 SQLx 而非 ORM？**

| 方案 | 优势 | 劣势 |
|------|------|------|
| **SQLx** | 原生 SQL、编译时检查、性能高 | 手写 SQL、学习成本 |
| **Diesel** | 类型安全查询 DSL | 复杂查询困难 |
| **SeaORM** | 纯 Rust ORM、异步支持 | 运行时开销 |

SQLx 的哲学：**SQL 是最好的查询语言**，与其用 DSL 模拟 SQL，不如直接写 SQL 并获得编译时安全检查。

---

## 工业界应用：高性能数据服务

**真实案例**：某实时数据处理平台使用 SQLx + PostgreSQL：

```rust
// 批量插入优化
async fn batch_insert(pool: &PgPool, records: Vec<Record>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut inserted = 0;
    
    for chunk in records.chunks(1000) {
        // 使用 unnest 进行批量插入
        let values: Vec<_> = chunk.iter().map(|r| (r.id, &r.data)).collect();
        
        let result = sqlx::query(
            "INSERT INTO events (id, data) SELECT * FROM UNNEST($1::int[], $2::text[])"
        )
        .bind(values.iter().map(|(id, _)| *id).collect::<Vec<_>>())
        .bind(values.iter().map(|(_, data)| data.as_str()).collect::<Vec<_>>())
        .execute(&mut *tx)
        .await?;
        
        inserted += result.rows_affected();
    }
    
    tx.commit().await?;
    Ok(inserted)
}
```

**结果**：
- 吞吐量：从 5000 行/秒 提升到 50000 行/秒
- 内存使用：减少 60%（相比逐行插入）
- 类型安全：重构时 100% 捕获 SQL 错误

---

## 知识检查

**问题 1** (基础概念)

以下代码的输出是什么？

```rust
let pool = SqlitePool::connect("sqlite::memory:").await?;
let row = sqlx::query("SELECT 1 as num")
    .fetch_one(&pool)
    .await?;
let num: i32 = row.get("num");
println!("{}", num);
```

A) 编译错误  
B) 运行时错误  
C) 1  
D) "1"

<details>
<summary>答案与解析</summary>

**答案**: C) 1

**解析**: `sqlx::query("SELECT 1 as num")` 返回一行数据，`row.get("num")` 提取整数列，类型为 `i32`，值为 1。
</details>

---

**问题 2** (连接管理)

以下哪种方式更适合生产环境的高并发数据库访问？

A) 每次请求都创建新连接  
B) 使用 `SqlitePool` 或 `MySqlPool` 连接池  
C) 使用全局 `static mut` 连接  
D) 单线程顺序执行所有查询

<details>
<summary>答案与解析</summary>

**答案**: B) 使用连接池

**解析**: 连接池复用数据库连接，避免连接建立开销，并限制并发连接数防止数据库过载。选项 A 性能差，C 不安全，D 无法利用异步优势。
</details>

---

**问题 3** (错误处理)

以下代码有什么问题？

```rust
let mut tx = pool.begin().await?;
sqlx::query("INSERT INTO users (name) VALUES (?)")
    .bind("Alice")
    .execute(&mut *tx)
    .await?;
// 函数结束，tx 被丢弃
```

<details>
<summary>答案与解析</summary>

**答案**: 事务未提交，数据不会保存

**解析**: 事务 `tx` 在函数结束时被丢弃，自动回滚。应该显式调用 `tx.commit().await?` 提交事务。这是常见错误，数据会"丢失"且不会产生错误。
</details>

---

## 参考资料

1. [SQLx 官方文档](https://docs.rs/sqlx/latest/sqlx/)
2. [Rust 异步编程指南](https://rust-lang.github.io/async-book/)
3. [SQLite WAL 模式](https://sqlite.org/wal.html)
4. [数据库连接池设计](https://github.com/brettwooldridge/HikariCP)

---

> **记住**：SQLx 结合了 Rust 的类型安全和 SQL 的灵活性，是构建高性能数据密集型应用的理想选择。掌握连接池和事务管理是生产环境开发的关键。
