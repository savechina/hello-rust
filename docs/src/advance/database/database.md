# 数据库操作

## 开篇故事

想象你去餐厅吃饭。传统方式是：看菜单 → 告诉服务员 → 等待上菜。数据库编程就像这个过程：应用程序发出 SQL 查询 → 数据库执行 → 返回结果。Rust 的数据库库（SQLx 和 Diesel）就像智能点餐系统——在下单前就告诉你菜品是否存在、是否符合饮食限制。

在 Rust 中，数据库操作分为两大流派：**SQLx**（异步、编译时检查）和 **Diesel**（ORM、类型安全）。本章介绍这两种方法的核心概念。

---

## 本章适合谁

如果你需要在 Rust 程序中存储和检索数据，本章适合你。无论你是构建 Web 应用、CLI 工具还是微服务，数据库都是不可或缺的部分。

---

## 你会学到什么

完成本章后，你可以：

1. 选择适合的数据库库（SQLx vs Diesel）
2. 理解异步数据库操作的优势
3. 掌握类型安全查询的原理
4. 设计数据库连接管理策略

---

## 前置要求

学习本章前，你需要理解：

- [异步编程](../async/async.md) - 理解 async/await
- [错误处理](../tools/error-handling.md) - 理解 Result 和错误传播
- [Serde 序列化](../data/json.md) - 理解结构体序列化

---

## 第一个例子

让我们看一个最简单的 SQLx 查询示例：

```rust
use sqlx::{Pool, Postgres, Row};

async fn get_users(pool: &Pool<Postgres>) -> sqlx::Result<Vec<String>> {
    let rows = sqlx::query("SELECT name FROM users")
        .fetch_all(pool)
        .await?;
    
    Ok(rows.iter().map(|row| row.get(0)).collect())
}
```

**发生了什么？**

- `Pool<Postgres>`: PostgreSQL 连接池
- `query()`: 编译时检查 SQL 语法
- `fetch_all()`: 异步执行查询
- `Row::get()`: 类型安全的列访问

---

## 原理解析

### 1. SQLx vs Diesel 对比

| 特性         | SQLx                    | Diesel                |
| ------------ | ----------------------- | --------------------- |
| **查询方式**   | 原生 SQL                | DSL（领域特定语言）     |
| **检查时机**   | 编译时                  | 编译时                |
| **异步支持**   | ✅ 原生异步              | ❌ 同步（需手动包装）     |
| **ORM 功能**   | 有限                    | 完整                  |
| **学习曲线**   | 低（会 SQL 即可）         | 中（需学 DSL）          |
| **适合场景**   | 复杂查询、存储过程       | CRUD、业务逻辑         |

### 2. SQLx 编译时检查

```rust
// 编译时会检查 SQL 语法和表结构
let user = sqlx::query_as!(
    User,
    r#"SELECT id, name, email FROM users WHERE id = $1"#,
    user_id
)
.fetch_one(&pool)
.await?;
```

**优势**：
- SQL 语法错误在编译时发现
- 表结构变更立即报错
- 参数类型自动推断

### 3. Diesel ORM 模式

```rust
// schema.rs - 数据库模式
table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

// models.rs - 数据模型
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// 查询
let users = users::table
    .filter(users::email.eq("test@example.com"))
    .load::<User>(&conn)?;
```

**优势**：
- 完全类型安全
- Rust 代码即查询
- 自动关联映射

### 4. 连接池管理

```rust
use sqlx::postgres::PgPoolOptions;

async fn create_pool() -> sqlx::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect("postgresql://localhost/mydb")
        .await
}
```

**配置要点**：
- `max_connections`: 最大连接数
- `min_connections`: 最小空闲连接
- `acquire_timeout`: 获取连接超时

---

## 常见错误

### 错误 1: 连接池过小

```rust
// ❌ 错误：连接池太小
PgPoolOptions::new().max_connections(1)

// ✅ 正确：根据负载配置
PgPoolOptions::new().max_connections(10)
```

### 错误 2: 忘记处理 NULL

```rust
// ❌ 错误：假设列非空
let name: String = row.get(0);

// ✅ 正确：处理 NULL
let name: Option<String> = row.get(0);
```

### 错误 3: 在事务中长时间持有连接

```rust
// ❌ 错误：事务未提交
let mut tx = pool.begin().await?;
// ... 长时间操作 ...
// 忘记 tx.commit().await

// ✅ 正确：及时提交或回滚
let mut tx = pool.begin().await?;
// ... 操作 ...
tx.commit().await?;
```

---

## 动手练习

### 练习 1: 创建用户表

使用 SQLx 创建用户表并插入数据：

```rust
// TODO: 实现创建表和插入用户
async fn create_user(pool: &Pool<Postgres>, name: &str, email: &str) -> sqlx::Result<i32> {
    // 实现代码
}
```

<details>
<summary>点击查看答案</summary>

```rust
async fn create_user(pool: &Pool<Postgres>, name: &str, email: &str) -> sqlx::Result<i32> {
    let row = sqlx::query_scalar!(
        r#"INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id"#,
        name,
        email
    )
    .fetch_one(pool)
    .await?;
    
    Ok(row)
}
```

**解析**: 使用 `query_scalar!` 返回单个值。
</details>

---

## 故障排查

### Q: SQLx 编译时检查失败怎么办？

**A**: 确保：
1. 数据库正在运行
2. 连接字符串正确
3. 表结构已创建

### Q: Diesel 迁移如何管理？

**A**: 使用 Diesel CLI：
```bash
diesel migration generate create_users
diesel migration run
```

### Q: 连接池耗尽如何处理？

**A**: 
- 增加 `max_connections`
- 检查是否有未释放的连接
- 使用连接池监控

---

## 小结

**核心要点**：

1. **SQLx**: 异步、编译时检查、原生 SQL
2. **Diesel**: ORM、类型安全、DSL 查询
3. **连接池**: 管理数据库连接复用
4. **错误处理**: 使用 Result 传播数据库错误

**关键术语**：

- **Connection Pool**: 连接池
- **ORM**: 对象关系映射
- **Query Builder**: 查询构建器
- **Migration**: 数据库迁移

**下一步**：

- 学习 [SQLx 详细用法](sqlx.md)
- 探索 [Diesel ORM](diesel.md)
- 理解 [事务管理](transactions.md)

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| Connection Pool   | 连接池     |
| ORM               | 对象关系映射 |
| Query             | 查询       |
| Transaction       | 事务       |
| Migration         | 迁移       |
| Prepared Statement| 预编译语句 |

---

完整示例：`src/advance/database/sqlx_sample.rs`, `src/advance/database/diesel_sample.rs`

---

## 继续学习

- 下一步：[SQLx 异步查询](sqlx.md)
- 进阶：[Diesel ORM](diesel.md)
- 回顾：[异步编程](../async/async.md)

> 💡 **记住**：选择合适的数据库库取决于你的需求。复杂查询用 SQLx，业务逻辑用 Diesel！
