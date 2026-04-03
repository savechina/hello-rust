# Diesel ORM

## 开篇故事

想象你是一个图书馆管理员。每天你要处理成千上万本书的借阅记录：哪些书借出去了、谁借的、什么时候归还。如果用纯文本文件记录，查询一本书的借阅历史需要翻阅所有记录；如果用 SQL 直接操作数据库，代码里到处是字符串拼接的 SQL 语句，很容易出错而且难以维护。

这时你发现了 **Diesel** —— Rust 的类型安全 ORM（对象关系映射）库。它让你用 Rust 代码来描述数据库操作，编译器会帮你检查 SQL 是否正确，字段类型是否匹配。就像有了一个智能助手，在你写代码时就帮你发现错误，而不是等到运行时才出问题。

> **什么是 ORM？**
> ORM（Object-Relational Mapping，对象关系映射）是一种技术，让你在编程语言中使用对象（或结构体）来操作数据库，而不需要直接编写 SQL 语句。

---

## 本章适合谁

如果你已经掌握了 Rust 基础，现在想要：

- 学习如何在 Rust 中操作数据库
- 理解类型安全的数据库查询
- 掌握现代化的数据库访问模式
- 告别手写 SQL 字符串拼接

本章适合你。Diesel 的学习曲线比直接使用 SQL 驱动稍陡，但它带来的类型安全和编译时检查会让你的数据库代码更可靠。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是 ORM 以及 Diesel 的优势
2. 使用 `diesel-cli` 管理数据库迁移
3. 定义 Schema（表结构）和 Models（数据模型）
4. 编写类型安全的 CRUD 操作
5. 使用 Diesel 的查询构建器进行复杂查询
6. 理解 Diesel 的类型安全机制

---

## 前置要求

学习本章前，你需要理解：

- [结构体](struct.md) - 自定义数据类型
- [特征 (Traits)](trait.md) - Derive 宏和 trait 系统
- [泛型](generic.md) - 类型参数化
- [生命周期](lifetimes.md) - 引用有效性（可选但有帮助）

**环境要求**：

```bash
# 安装 diesel-cli 工具（用于管理迁移）
cargo install diesel_cli --no-default-features --features sqlite

# 或者使用 PostgreSQL
cargo install diesel_cli --no-default-features --features postgres
```

---

## 第一个例子

让我们从一个简单的博客系统开始，创建 `posts` 表来存储文章：

```rust
use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use std::env;

// 建立数据库连接
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("database")
        .unwrap_or_else(|_| "test.db".into());
    
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

**发生了什么？**

这段代码建立了一个 SQLite 数据库连接。Diesel 支持多种数据库后端：SQLite、PostgreSQL 和 MySQL。

---

## 原理解析

### ORM 映射关系

Diesel 的核心是建立 Rust 类型与数据库表之间的映射：

```
Rust 代码                    Diesel ORM                  数据库
─────────                   ───────────                  ──────

struct Post {               Schema Definition            posts table
    id: i32,    ────────►   table! { posts {            +─────────────+
    title: String,              id -> Integer,           │ id          │
    body: String,               title -> Text,           │ title       │
    published: bool,            body -> Text,        ◄───│ body        │
}                              published -> Bool,         │ published   │
                            }                            +─────────────+

                            [Queryable] ──────►   SELECT 查询结果映射到 Post
                            [Insertable] ──────►  INSERT 使用 NewPost
```

**图 1-1**: Diesel ORM 映射关系

### Schema 定义

Schema 描述数据库表的结构：

```rust
mod schema {
    use diesel::table;
    table! {
        posts {
            id -> Integer,
            title -> Text,
            body -> Text,
            published -> Bool,
        }
    }
}
```

**关键点**：

- `table!` 宏生成表结构定义
- `id -> Integer` 表示数据库字段名和 Rust 类型映射
- 类型安全：编译时检查字段是否存在

### 数据模型 (Models)

定义与表对应的 Rust 结构体：

```rust
/// 查询结果结构体 - 对应完整表记录
#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

/// 插入结构体 - 不需要自增 ID
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
```

**表 1-1**: Derive 宏对比

| Derive 宏 | 用途 | 说明 |
|-----------|------|------|
| `Queryable` | 查询结果映射 | SELECT 返回的数据自动转换为结构体 |
| `Identifiable` | 标识主键 | 用于更新、删除单条记录 |
| `Selectable` | 字段选择 | 支持部分字段查询 |
| `Insertable` | 插入数据 | 定义可插入表的字段 |

### 查询构建器

Diesel 提供类型安全的查询 API：

```rust
// 插入数据
let new_post = NewPost { title, body };
let post = diesel::insert_into(posts::table)
    .values(&new_post)
    .returning(Post::as_returning())
    .get_result(conn)?;

// 条件查询
let results = posts::table
    .filter(posts::published.eq(false))
    .limit(5)
    .load::<Post>(conn)?;

// 删除数据
let num_deleted = diesel::delete(
    posts::table.filter(posts::title.like(pattern))
)
.execute(conn)?;
```

**优势**：

- 编译时检查 SQL 语法
- 类型安全的字段访问 (`posts::published.eq(false)`)
- 防止 SQL 注入

### 类型安全机制

```
编译时检查示例：

posts::title.eq(123)    ❌ 编译错误：类型不匹配，title 是 Text，不是 Integer
posts::unknow.eq(true)  ❌ 编译错误：字段不存在
posts::published.eq(1)  ❌ 编译错误：Bool 不能和 Integer 比较
```

**图 1-2**: Diesel 类型安全机制

---

## 常见错误

### 错误 1: 忘记导入 Schema

```rust
// ❌ 错误：没有导入 schema
fn query_post(conn: &mut SqliteConnection) {
    let results = table           // table 未定义
        .filter(published.eq(false))  // published 未定义
        .load::<Post>(conn);
}

// ✅ 修复：导入 schema
fn query_post(conn: &mut SqliteConnection) {
    use schema::posts;            // 导入表定义
    let results = posts::table
        .filter(posts::published.eq(false))
        .load::<Post>(conn);
}
```

### 错误 2: Queryable 和 Insertable 混淆

```rust
// ❌ 错误：插入时使用了 Queryable 的结构体
#[derive(Queryable)]  // 缺少 Insertable
struct Post { ... }

diesel::insert_into(posts::table)
    .values(&post)  // ❌ Post 没有实现 Insertable
    .execute(conn)?;

// ✅ 修复：创建专门的插入结构体
#[derive(Insertable)]
#[diesel(table_name = posts)]
struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
}
```

### 错误 3: 生命周期问题

```rust
// ❌ 错误：NewPost 持有短生命周期的引用
fn create_post(conn: &mut SqliteConnection, 
               title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };
    // new_post 在函数结束时被销毁，但插入操作可能需要它
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

// ✅ 修复：正确实现（Diesel 会正确处理）
// 实际上上面的代码是正确的，因为 execute/get_result 是立即执行的
```

---

## 动手练习

### 练习 1: 添加更新功能

补全下面的代码，实现文章的发布功能（将 `published` 设为 `true`）：

```rust
fn publish_post(conn: &mut SqliteConnection, post_id: i32) {
    use schema::posts;
    
    // 你的代码：更新 posts 表中 id = post_id 的记录，
    // 将 published 设为 true
    
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn publish_post(conn: &mut SqliteConnection, post_id: i32) {
    use schema::posts;
    
    diesel::update(posts::table.find(post_id))
        .set(posts::published.eq(true))
        .execute(conn)
        .expect("Error publishing post");
}
```

**解析**：
- `update()` 开始一个更新查询
- `find(post_id)` 定位特定记录（需要 `Identifiable` trait）
- `set()` 指定要更新的字段

</details>

---

### 练习 2: 理解模型设计

为什么需要区分 `Post` 和 `NewPost` 两个结构体？

<details>
<summary>点击查看答案</summary>

**原因**：

1. **id 字段差异**：
   - `Post` 包含 `id`，用于查询已有记录
   - `NewPost` 不包含 `id`，因为插入时数据库会自动生成

2. **所有权差异**：
   - `Post` 使用 `String` 拥有数据
   - `NewPost` 使用 `&str` 借用数据，避免不必要的克隆

3. **灵活性**：
   - 插入时可以只提供部分字段
   - 查询时总是返回完整记录

```rust
// 插入时不需要 id
let new_post = NewPost {
    title: "标题",  // &str 借用
    body: "内容",   // &str 借用
};

// 查询时总有 id
let post: Post = posts::table.first(conn)?;  // id, title, body, published 都有
```

</details>

---

### 练习 3: 复杂查询

编写一个查询，找出标题包含特定关键词的已发布文章，并按 id 倒序排列：

```rust
fn search_published_posts(conn: &mut SqliteConnection, 
                          keyword: &str) -> Vec<Post> {
    use schema::posts;
    
    // 你的代码：
    // 1. 查询 posts 表
    // 2. 过滤 published = true
    // 3. 过滤 title 包含 keyword（使用 like）
    // 4. 按 id 倒序排列
    // 5. 限制返回 10 条
    
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn search_published_posts(conn: &mut SqliteConnection, 
                          keyword: &str) -> Vec<Post> {
    use schema::posts;
    
    let pattern = format!("%{}%", keyword);
    
    posts::table
        .filter(posts::published.eq(true))
        .filter(posts::title.like(pattern))
        .order(posts::id.desc())
        .limit(10)
        .load::<Post>(conn)
        .expect("Error loading posts")
}
```

**关键 API**：
- `like()` - SQL LIKE 操作符
- `order()` / `desc()` - 排序
- `limit()` - 限制结果数量

</details>

---

## 故障排查 (FAQ)

### Q: Diesel 和 sqlx 有什么区别？

**A**: 两者都是 Rust 的数据库工具，但有不同设计理念：

| 特性 | Diesel | sqlx |
|------|--------|------|
| 类型检查 | 编译时 | 编译时 |
| Schema 定义 | 使用 `table!` 宏定义 | 从数据库实时检查 |
| SQL 写法 | DSL 风格（Rust 代码） | 原生 SQL |
| 学习曲线 | 较陡（需要学习 DSL） | 较平缓 |
| 适用场景 | 复杂项目，团队协作 | 快速开发，简单查询 |

> 💡 **提示**：如果你是 Diesel 新手，建议先用 SQLite 练习，因为它配置最简单。

---

### Q: 如何处理数据库迁移？

**A**: 使用 `diesel-cli` 管理迁移：

```bash
# 创建迁移目录
diesel setup

# 创建新迁移
diesel migration create create_posts_table

# 生成迁移文件后，编辑 up.sql 和 down.sql
# up.sql:
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    body TEXT,
    published BOOLEAN DEFAULT FALSE
);

# down.sql:
DROP TABLE posts;

# 运行迁移
diesel migration run

# 回滚迁移
diesel migration redo
```

> 📝 **注意**：示例代码 `diesel_sample.rs` 使用 `sql_query` 手动创建表，这是为了演示。生产环境建议使用迁移。

---

### Q: 如何调试 Diesel 查询？

**A**: 打印生成的 SQL：

```rust
use diesel::debug_query;
use diesel::sqlite::Sqlite;

let query = posts::table.filter(posts::published.eq(true));
println!("SQL: {}", debug_query::<Sqlite, _>(&query));
// 输出: SQL: SELECT "posts".* FROM "posts" WHERE "posts"."published" = ?
```

---

### Q: 连接池怎么配置？

**A**: 生产环境应该使用连接池：

```rust
use diesel::r2d2::{ConnectionManager, Pool};

// 创建连接池
let manager = ConnectionManager::<SqliteConnection>::new("test.db");
let pool = Pool::builder()
    .max_size(5)  // 最大连接数
    .build(manager)?;

// 使用连接池
let mut conn = pool.get()?;
let posts = posts::table.load::<Post>(&mut conn)?;
```

---

## 知识扩展 (选学)

### 关联查询

Diesel 支持表之间的关联：

```rust
// 假设有 users 表和 posts 表
// users.id -> posts.user_id

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

// 查询用户及其文章
let data: Vec<(User, Vec<Post>)> = users::table
    .left_join(posts::table)
    .load(conn)?;
```

### 自定义类型映射

将数据库类型映射到 Rust 枚举：

```rust
#[derive(SqlType)]
#[diesel(sqlite_type(name = "Text"))]
pub struct PostStatus;

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = PostStatus)]
pub enum Status {
    Draft,
    Published,
    Archived,
}
```

### 异步支持

使用 `diesel-async` 进行异步操作：

```toml
[dependencies]
diesel-async = { version = "0.4", features = ["sqlite"] }
```

```rust
use diesel_async::RunQueryDsl;
use diesel_async::sqlite::AsyncSqliteConnection;

async fn async_query(conn: &mut AsyncSqliteConnection) -> Vec<Post> {
    posts::table.load(conn).await.unwrap()
}
```

---

## 小结

**核心要点**：

1. **ORM 是桥梁**：在 Rust 结构体和数据库表之间建立类型安全的映射
2. **Schema 定义表结构**：使用 `table!` 宏描述数据库表
3. **Models 分两种**：`Queryable` 用于查询，`Insertable` 用于插入
4. **查询构建器是类型安全的**：编译时检查字段名和类型
5. **迁移管理 Schema 变更**：使用 `diesel-cli` 管理数据库版本

**关键术语**：

- **ORM (Object-Relational Mapping)**：对象关系映射
- **Schema**：数据库表结构定义
- **Queryable**：可查询 trait，用于 SELECT
- **Insertable**：可插入 trait，用于 INSERT
- **Migration**：数据库迁移，管理 Schema 变更
- **Connection Pool**：连接池，复用数据库连接

**下一步**：

- 学习 [异步编程](async.md) - 结合 tokio 使用异步 ORM
- 理解 [数据库连接池](../awesome/database.md) - 生产环境最佳实践
- 探索 [高级查询模式](../awesome/database.md) - 复杂关联和聚合

---

## 术语表

| English | 中文 | 说明 |
|---------|------|------|
| ORM | 对象关系映射 | Object-Relational Mapping |
| Schema | 模式/结构 | 数据库表结构定义 |
| Model | 模型 | 对应数据库表的 Rust 结构体 |
| Migration | 迁移 | 数据库 Schema 版本管理 |
| Queryable | 可查询的 | Derive trait，用于 SELECT |
| Insertable | 可插入的 | Derive trait，用于 INSERT |
| Connection Pool | 连接池 | 数据库连接复用机制 |
| DSL | 领域特定语言 | Domain-Specific Language |
| CRUD | 增删改查 | Create, Read, Update, Delete |

---

> **完整示例**: `src/advance/diesel_sample.rs`

---

## 继续学习

- 下一步：[异步编程](async.md)
- 进阶：[数据库高级应用](../awesome/database.md)
- 回顾：[序列化与反序列化](serialization.md)

> 💡 **记住**：Diesel 的强大之处在于编译时保证类型安全。如果代码能编译通过，大部分 SQL 错误就已经被排除了。充分利用编译器的帮助，写出更可靠的数据库代码！

---

## 知识检查

**问题 1** 🟢 (基础概念)

Diesel 中，`#[derive(Queryable)]` 和 `#[derive(Insertable)]` 的主要区别是什么？

A) 没有区别，可以互换使用  
B) `Queryable` 用于查询，`Insertable` 用于插入，且 `Insertable` 通常不包含自增主键  
C) `Queryable` 只能用于 PostgreSQL  
D) `Insertable` 会自动生成主键

<details>
<summary>答案与解析</summary>

**答案**: B) `Queryable` 用于查询，`Insertable` 用于插入，且 `Insertable` 通常不包含自增主键

**解析**：
- `Queryable`：将查询结果映射到结构体，包含所有字段（包括 id）
- `Insertable`：定义可插入数据，通常不包含自增字段
- 这种分离使得插入时无需提供 id，查询时又能获取完整数据

</details>

---

**问题 2** 🟡 (查询构建)

以下 Diesel 查询语句会被编译器拒绝吗？为什么？

```rust
posts::table
    .filter(posts::published.eq("true"))
    .load::<Post>(conn)?;
```

A) 不会，编译通过  
B) 会，`eq()` 方法不存在  
C) 会，类型不匹配：`published` 是 Bool，不能和字符串比较  
D) 会，`load` 方法需要更多参数

<details>
<summary>答案与解析</summary>

**答案**: C) 会，类型不匹配：`published` 是 Bool，不能和字符串比较

**解析**：
Diesel 的 `eq()` 方法要求类型匹配。在 Schema 中 `published -> Bool`，所以只能和 `bool` 类型比较：

```rust
// ✅ 正确
.filter(posts::published.eq(true))

// ❌ 错误
.filter(posts::published.eq("true"))  // 类型不匹配
```

这就是 Diesel 的类型安全特性！

</details>

---

**问题 3** 🔴 (综合应用)

要实现一个按标题搜索文章的功能，以下代码有什么问题？如何修复？

```rust
fn search_posts(conn: &mut SqliteConnection, keyword: &str) -> Vec<Post> {
    use schema::posts;
    
    posts::table
        .filter(posts::title.like(keyword))
        .load::<Post>(conn)
        .unwrap()
}
```

<details>
<summary>答案与解析</summary>

**答案**：SQL LIKE 模式匹配需要通配符 `%`

**修复**:
```rust
fn search_posts(conn: &mut SqliteConnection, keyword: &str) -> Vec<Post> {
    use schema::posts;
    
    // 添加通配符
    let pattern = format!("%{}%", keyword);
    
    posts::table
        .filter(posts::title.like(pattern))
        .load::<Post>(conn)
        .unwrap()
}
```

**解析**：
- SQL `LIKE` 需要 `%` 作为通配符：`%keyword%` 表示包含 keyword
- 直接传递 keyword 会进行精确匹配，不是模糊搜索
- `format!("%{}%", keyword)` 构建正确的 LIKE 模式

</details>

---

## 实践项目

### 简易博客系统

使用 Diesel 实现一个命令行博客系统：

**功能要求**：
1. 创建文章（标题、内容、默认未发布）
2. 列出所有已发布文章
3. 搜索文章（按标题模糊搜索）
4. 发布文章（将状态改为已发布）
5. 删除文章

**数据结构**：
```rust
// posts 表
// id: 自增主键
// title: 标题，非空
// body: 内容
// published: 是否发布，默认 false
// created_at: 创建时间
```

**进阶挑战**：
- 添加 `users` 表，实现作者关联
- 添加 `categories` 表，实现文章分类
- 使用连接池优化性能

参考实现：`src/advance/diesel_sample.rs`
