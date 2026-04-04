# SQLx 数据库操作

## 开篇故事

想象你去银行办业务。传统方式是：排队 → 到窗口 → 说明需求 → 等待办理 → 完成。SQLx 就像银行的自助终端——你提交请求，它异步处理，完成后通知你。这样你不需要一直等待，可以同时处理其他事情。

---

## 本章适合谁

如果你需要在 Rust 程序中操作数据库（SQLite、MySQL、PostgreSQL 等），本章适合你。SQLx 是类型安全的异步数据库库，是构建数据驱动应用的首选。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 SQLx 连接数据库
2. 执行 SQL 查询和插入操作
3. 使用类型安全的查询绑定
4. 将查询结果映射到结构体
5. 使用连接池管理数据库连接

---

## 前置要求

- Tokio 异步运行时 - 异步基础
- 结构体 - 结构体定义
- Serde 序列化 - 序列化基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add sqlx --features runtime-tokio,postgres
```

## 第一个例子

最简单的 SQLx SQLite 示例：

```rust,ignore
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 连接内存数据库
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    
    // 创建表
    sqlx::query(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
    )
    .execute(&mut conn)
    .await?;
    
    // 插入数据
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("Alice")
        .execute(&mut conn)
        .await?;
    
    // 查询数据
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(&mut conn)
        .await?;
    
    for row in rows {
        let name: String = row.get("name");
        println!("用户：{}", name);
    }
    
    Ok(())
}
```

**完整示例**: [sqlx_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/sqlx_sample.rs)

---

## 原理解析

### SQLx 特性

**SQLx 是一个异步数据库库**：

- ✅ 编译时 SQL 验证（可选）
- ✅ 类型安全查询
- ✅ 异步执行
- ✅ 支持多种数据库（SQLite, MySQL, PostgreSQL）

### 连接数据库

**连接字符串**：

```rust,ignore
// SQLite 内存数据库
let conn = SqliteConnection::connect("sqlite::memory:").await?;

// SQLite 文件数据库
let conn = SqliteConnection::connect("sqlite:database.db").await?;

// MySQL
let conn = MySqlConnection::connect("mysql://user:pass@localhost/dbname").await?;

// PostgreSQL
let conn = PgConnection::connect("postgres://user:pass@localhost/dbname").await?;
```

### 执行 SQL 查询

**query 方法**：

```rust,ignore
// 执行无返回值的 SQL
sqlx::query("DELETE FROM users WHERE id = ?")
    .bind(user_id)
    .execute(&mut conn)
    .await?;

// 执行并获取影响的行数
let result = sqlx::query("UPDATE users SET name = ? WHERE id = ?")
    .bind(new_name)
    .bind(user_id)
    .execute(&mut conn)
    .await?;

println!("影响了 {} 行", result.rows_affected());
```

### 查询并获取结果

**fetch_all 获取所有行**：

```rust,ignore
let rows = sqlx::query("SELECT id, name, email FROM users")
    .fetch_all(&mut conn)
    .await?;

for row in rows {
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let email: String = row.get("email");
    
    println!("ID: {}, Name: {}, Email: {}", id, name, email);
}
```

**fetch_one 获取单行**：

```rust,ignore
let row = sqlx::query("SELECT * FROM users WHERE id = ?")
    .bind(user_id)
    .fetch_one(&mut conn)
    .await?;
```

### 映射到结构体

**定义结构体**：

```rust,ignore
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}
```

**查询并映射**：

```rust,ignore
let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&mut conn)
    .await?;

for user in users {
    println!("用户：{:?}", user);
}
```

### 连接池

**创建连接池**：

```rust,ignore
use sqlx::sqlite::SqlitePool;

// 创建连接池
let pool = SqlitePool::connect("sqlite:database.db").await?;

// 使用连接池（自动管理连接）
let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

**连接池优势**：
- 自动管理连接
- 控制最大连接数
- 连接复用，提高性能

---

## 常见错误

### 错误 1: SQL 语法错误

```rust,ignore
sqlx::query("SELEC * FROM users")  // ❌ SELEC 拼写错误
    .fetch_all(&mut conn)
    .await?;
```

**错误信息**:
```
SQL logic error: near "SELEC": syntax error
```

**修复方法**:
```rust,ignore
sqlx::query("SELECT * FROM users")  // ✅ 正确拼写
```

### 错误 2: 参数绑定类型不匹配

```rust,ignore
sqlx::query("SELECT * FROM users WHERE id = ?")
    .bind("123")  // ❌ id 是 INTEGER，绑定了字符串
    .fetch_all(&mut conn)
    .await?;
```

**修复方法**:
```rust,ignore
sqlx::query("SELECT * FROM users WHERE id = ?")
    .bind(123i64)  // ✅ 绑定正确的类型
    .fetch_all(&mut conn)
    .await?;
```

### 错误 3: 忘记 await

```rust,ignore
let rows = sqlx::query("SELECT * FROM users")
    .fetch_all(&mut conn);  // ❌ 忘记 .await

// rows 是 Future，不是结果
```

**修复方法**:
```rust,ignore
let rows = sqlx::query("SELECT * FROM users")
    .fetch_all(&mut conn)
    .await?;  // ✅ 添加 .await
```

---

## 动手练习

### 练习 1: 创建用户表

```rust,ignore
use sqlx::sqlite::SqliteConnection;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    
    // TODO: 创建 users 表
    // 字段：id (INTEGER PRIMARY KEY), name (TEXT), email (TEXT)
    
    Ok(())
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
sqlx::query(
    "CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL
    )"
)
.execute(&mut conn)
.await?;
```
</details>

### 练习 2: 插入和查询用户

```rust,ignore
// TODO: 插入 3 个用户
// TODO: 查询所有用户
// TODO: 打印用户列表
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
// 插入
sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
    .bind("Alice")
    .bind("alice@example.com")
    .execute(&mut conn)
    .await?;

// 查询
let users = sqlx::query("SELECT name, email FROM users")
    .fetch_all(&mut conn)
    .await?;

for row in users {
    let name: String = row.get("name");
    let email: String = row.get("email");
    println!("{} - {}", name, email);
}
```
</details>

### 练习 3: 使用结构体映射

```rust,ignore
#[derive(Debug, sqlx::FromRow)]
struct User {
    // TODO: 定义字段
}

// TODO: 查询并映射到 User 结构体
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&mut conn)
    .await?;

for user in users {
    println!("{:?}", user);
}
```
</details>

---

## 故障排查 (FAQ)

### Q: SQLx 和 Diesel 有什么区别？

**A**: 
- **SQLx**: 异步，编译时 SQL 验证，更灵活
- **Diesel**: 同步，ORM 风格，类型安全更强
- **推荐**: SQLx（异步性能更好）

### Q: 什么时候使用连接池？

**A**: 
- Web 应用（多用户并发）
- 高并发场景
- 需要控制连接数时

```rust,ignore
// 单连接
let conn = SqliteConnection::connect(url).await?;

// 连接池
let pool = SqlitePool::connect(url).await?;
```

### Q: 如何处理事务？

**A**: 
```rust,ignore
let mut tx = conn.begin().await?;

sqlx::query("INSERT ...").execute(&mut tx).await?;
sqlx::query("UPDATE ...").execute(&mut tx).await?;

tx.commit().await?;  // 或 tx.rollback().await?;
```

---

## 知识扩展

### 查询构建器

```rust,ignore
// 动态构建查询
let mut query = sqlx::query("SELECT * FROM users WHERE 1=1");

if let Some(name) = filter_name {
    query = query.bind(name);
}

if let Some(email) = filter_email {
    query = query.bind(email);
}

let users = query.fetch_all(&pool).await?;
```

### 迁移 (Migrations)

```rust,ignore
// 运行数据库迁移
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

### 性能优化

```rust,ignore
// 使用 prepare 预编译查询
let query = sqlx::query("SELECT * FROM users WHERE id = ?");
let cached = query.persistent(true);

// 多次执行更高效
cached.bind(1).fetch_one(&pool).await?;
cached.bind(2).fetch_one(&pool).await?;
```

---

## 小结

**核心要点**：

1. **SQLx**: 异步数据库库，支持多种数据库
2. **query**: 执行 SQL 查询
3. **bind**: 参数绑定，防止 SQL 注入
4. **fetch_all/fetch_one**: 获取查询结果
5. **FromRow**: 自动映射到结构体
6. **连接池**: 管理多个连接，提高性能

**关键术语**：

- **Connection (连接)**: 数据库连接
- **Pool (连接池)**: 连接池管理
- **Query (查询)**: SQL 查询
- **Bind (绑定)**: 参数绑定
- **FromRow**: 结果映射 trait

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Connection | 连接 |
| Connection Pool | 连接池 |
| Query | 查询 |
| Bind | 绑定 |
| Transaction | 事务 |
| Migration | 迁移 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `query!` 和 `query_as!` 有什么区别？

2. SQLx 的编译时检查需要什么条件？

3. 如何处理可选参数？

<details>
<summary>点击查看答案与解析</summary>

1. `query!` 返回匿名结构体，`query_as!` 返回指定类型
2. 需要数据库连接和 `DATABASE_URL` 环境变量
3. 使用 `Option<T>` 参数，SQL 中用 `IS NULL` 处理

**关键理解**: SQLx 在编译时验证 SQL，减少运行时错误。
</details>

## 继续学习

**前一章**: Tokio 异步运行时  
**下一章**: [Diesel ORM](diesel.md)

**相关章节**:
- Tokio 异步运行时
- [数据库 ORM](diesel.md)
- 序列化

**返回**: 高级进阶

---

**完整示例**: [sqlx_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/sqlx_sample.rs)
