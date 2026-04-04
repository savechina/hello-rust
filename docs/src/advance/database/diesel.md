# Diesel ORM

## 开篇故事

想象你要写 SQL 查询。传统方式是：手写 SQL → 执行 → 手动映射结果 → 处理类型错误。Diesel 就像智能助手——你定义数据结构，它生成类型安全的 SQL，编译时就告诉你有没有错误。

---

## 本章适合谁

如果你需要在 Rust 程序中使用 ORM（对象关系映射）操作数据库，本章适合你。Diesel 是类型安全的 ORM，在编译时捕获 SQL 错误。

---

## 你会学到什么

完成本章后，你可以：

1. 定义 Diesel Schema
2. 创建数据模型结构体
3. 执行 CRUD 操作
4. 使用类型安全的查询构建器
5. 处理数据库连接和事务

---

## 前置要求

- 结构体 - 结构体定义
- Serde 序列化 - 序列化基础
- [SQLx 数据库](sqlx.md) - 数据库基础（可选）

---

## 第一个例子

最简单的 Diesel SQLite 示例：

```rust
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// 定义 Schema
diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

// 定义模型
#[derive(Queryable)]
struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

fn main() {
    // 连接数据库
    let mut conn = SqliteConnection::establish("test.db")
        .expect("Error connecting to database");
    
    // 查询数据
    let posts = posts::table
        .load::<Post>(&mut conn)
        .expect("Error loading posts");
    
    for post in posts {
        println!("{}: {}", post.title, post.body);
    }
}
```

**完整示例**: [diesel_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/diesel_sample.rs)

---

## 原理解析

### Diesel 特性

**Diesel 是一个 ORM**：

- ✅ 编译时 SQL 验证
- ✅ 类型安全查询
- ✅ 自动映射结果
- ✅ 支持多种数据库（SQLite, MySQL, PostgreSQL）

### Schema 定义

**使用 diesel::table! 宏**：

```rust
diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        age -> Integer,
    }
}
```

**字段类型映射**：
- `Integer` → i32
- `Text` → String
- `Bool` → bool
- `Nullable<Text>` → Option<String>

### 模型定义

**使用 Queryable derive**：

```rust
use diesel::Queryable;

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: i32,
}
```

**使用 Insertable derive**：

```rust
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
    age: i32,
}
```

### 连接数据库

**建立连接**：

```rust
use diesel::sqlite::SqliteConnection;
use diesel::Connection;

let mut conn = SqliteConnection::establish("database.db")
    .expect("Error connecting to database");
```

**使用环境变量**：

```rust
use dotenvy::dotenv;
use std::env;

dotenv().ok();
let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "test.db".into());

let mut conn = SqliteConnection::establish(&database_url)
    .expect("Error connecting to database");
```

### CRUD 操作

**Create (插入)**：

```rust
use diesel::RunQueryDsl;

let new_post = NewPost {
    title: "My Post",
    body: "Post body",
    published: true,
};

diesel::insert_into(posts::table)
    .values(&new_post)
    .execute(&mut conn)
    .expect("Error saving new post");
```

**Read (查询)**：

```rust
use diesel::RunQueryDsl;

// 查询所有
let all_posts = posts::table
    .load::<Post>(&mut conn)
    .expect("Error loading posts");

// 查询单个
let post = posts::table
    .find(post_id)
    .first::<Post>(&mut conn)
    .expect("Error loading post");

// 条件查询
let published_posts = posts::table
    .filter(posts::published.eq(true))
    .load::<Post>(&mut conn)
    .expect("Error loading posts");
```

**Update (更新)**：

```rust
use diesel::RunQueryDsl;

diesel::update(posts::table.find(post_id))
    .set(posts::published.eq(true))
    .execute(&mut conn)
    .expect("Error updating post");
```

**Delete (删除)**：

```rust
diesel::delete(posts::table.find(post_id))
    .execute(&mut conn)
    .expect("Error deleting post");
```

### 查询构建器

**过滤**：

```rust
posts::table
    .filter(posts::published.eq(true))
    .filter(posts::title.like("%Rust%"))
    .load::<Post>(&mut conn)?;
```

**排序**：

```rust
posts::table
    .order(posts::created_at.desc())
    .load::<Post>(&mut conn)?;
```

**限制数量**：

```rust
posts::table
    .limit(10)
    .offset(20)
    .load::<Post>(&mut conn)?;
```

**连接表**：

```rust
posts::table
    .inner_join(users::table)
    .select((posts::all_columns, users::name))
    .load::<(Post, String)>(&mut conn)?;
```

---

## 常见错误

### 错误 1: Schema 不匹配

```rust
// Schema 定义
diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
    }
}

// 模型
#[derive(Queryable)]
struct Post {
    id: i32,
    title: String,
    body: String,  // ❌ 数据库中不存在 body 字段
}
```

**错误信息**:
```
column `body` does not exist
```

**修复方法**:
```rust
#[derive(Queryable)]
struct Post {
    id: i32,
    title: String,  // ✅ 只包含存在的字段
}
```

### 错误 2: 忘记 derive

```rust
// ❌ 忘记 #[derive(Queryable)]
struct Post {
    id: i32,
    title: String,
}
```

**错误信息**:
```
the trait `Queryable<_, __>` is not implemented for `Post`
```

**修复方法**:
```rust
#[derive(Queryable)]  // ✅ 添加 derive
struct Post {
    id: i32,
    title: String,
}
```

### 错误 3: 连接未使用 mut

```rust
let conn = SqliteConnection::establish("test.db")?;
posts::table.load::<Post>(&conn)?;  // ❌ 需要 &mut conn
```

**修复方法**:
```rust
let mut conn = SqliteConnection::establish("test.db")?;
posts::table.load::<Post>(&mut conn)?;  // ✅ 使用 &mut
```

---

## 动手练习

### 练习 1: 定义用户 Schema

```rust
// TODO: 定义 users 表 Schema
// 字段：id (Integer), name (Text), email (Text)

// TODO: 定义 User 结构体
// 实现 Queryable trait
```

<details>
<summary>点击查看答案</summary>

```rust
diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
}
```
</details>

### 练习 2: 插入和查询用户

```rust
// TODO: 创建数据库连接
// TODO: 插入 3 个用户
// TODO: 查询所有用户
// TODO: 打印用户列表
```

<details>
<summary>点击查看答案</summary>

```rust
let mut conn = SqliteConnection::establish("test.db")?;

// 插入
let new_user = NewUser {
    name: "Alice",
    email: "alice@example.com",
};

diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&mut conn)?;

// 查询
let users = users::table.load::<User>(&mut conn)?;

for user in users {
    println!("{} - {}", user.name, user.email);
}
```
</details>

### 练习 3: 条件查询

```rust
// TODO: 查询已发布的博客
// TODO: 按创建时间排序
// TODO: 限制返回 10 条
```

<details>
<summary>点击查看答案</summary>

```rust
let posts = posts::table
    .filter(posts::published.eq(true))
    .order(posts::created_at.desc())
    .limit(10)
    .load::<Post>(&mut conn)?;
```
</details>

---

## 故障排查 (FAQ)

### Q: Diesel 和 SQLx 有什么区别？

**A**: 
- **Diesel**: ORM，编译时 SQL 验证，类型安全更强
- **SQLx**: 异步，运行时 SQL 验证，更灵活
- **推荐**: Diesel（类型安全更好）

### Q: 如何处理关联关系？

**A**: 
```rust
#[derive(Associations, Queryable)]
#[diesel(belongs_to(User))]
struct Post {
    id: i32,
    user_id: i32,
    title: String,
}
```

### Q: 如何运行迁移？

**A**: 
```bash
# 安装 diesel CLI
cargo install diesel_cli

# 创建迁移
diesel migration generate create_posts_table

# 运行迁移
diesel migration run
```

---

## 知识扩展

### 事务处理

```rust
use diesel::Connection;

conn.transaction::<_, diesel::result::Error, _>(|conn| {
    // 所有操作在一个事务中
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)?;
    
    diesel::insert_into(comments::table)
        .values(&new_comment)
        .execute(conn)?;
    
    Ok(())
})?;
```

### 关联查询

```rust
// 查询用户及其所有博客
let results = users::table
    .inner_join(posts::table)
    .select((User::as_select(), Post::as_select()))
    .load::<(User, Post)>(&mut conn)?;
```

### 动态查询

```rust
let mut query = posts::table.into_boxed();

if let Some(title) = filter_title {
    query = query.filter(posts::title.like(format!("%{}%", title)));
}

if let Some(user) = filter_user {
    query = query.filter(posts::user_id.eq(user.id));
}

let posts = query.load::<Post>(&mut conn)?;
```

---

## 小结

**核心要点**：

1. **Diesel**: 类型安全的 ORM
2. **Schema**: 使用 table! 宏定义
3. **Queryable**: 自动映射查询结果
4. **Insertable**: 类型安全的插入
5. **查询构建器**: filter, order, limit 等
6. **事务**: 使用 transaction 方法

**关键术语**：

- **ORM (对象关系映射)**: 数据库表映射到结构体
- **Schema**: 数据库结构定义
- **Queryable**: 查询结果映射 trait
- **Insertable**: 插入数据 trait
- **Transaction**: 数据库事务

---

## 术语表

| English | 中文 |
| ------- | ---- |
| ORM | 对象关系映射 |
| Schema | 模式/架构 |
| Queryable | 可查询 |
| Insertable | 可插入 |
| Transaction | 事务 |
| Migration | 迁移 |

---

## 知识检查

**快速测验**（答案在下方）：

1. Diesel 的 schema 是什么？

2. `Queryable` 和 `Insertable` 有什么区别？

3. 如何处理数据库迁移？

<details>
<summary>点击查看答案与解析</summary>

1. 数据库表结构的 Rust 表示（table! 宏）
2. `Queryable` = 查询结果映射，`Insertable` = 插入数据映射
3. 使用 Diesel CLI：`diesel migration generate` 和 `diesel migration run`

**关键理解**: Diesel 是类型安全的 ORM，编译时检查查询。
</details>

## 继续学习

**前一章**: Futures 异步编程  
**下一章**: 宏编程

**相关章节**:
- [SQLx 数据库](sqlx.md)
- Futures 异步编程
- 序列化

**返回**: 高级进阶

---

**完整示例**: [diesel_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/diesel_sample.rs)
