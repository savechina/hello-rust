use sqlx::sqlite::SqlitePool;
use sqlx::Connection;
use sqlx::Row;
use sqlx::SqliteConnection;
use std::borrow::Borrow;
use std::env;
use std::fmt::Pointer;
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main(flavor = "current_thread")]
async fn sqlx_sqlite_example() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    // 创建一个示例表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(&mut conn)
    .await?;

    // 插入数据
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("Alice")
        .bind("alice@example.com")
        .execute(&mut conn)
        .await?;

    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("Bob")
        .bind("bob@example.com")
        .execute(&mut conn)
        .await?;

    // 查询数据
    let rows = sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&mut conn)
        .await?;

    for (idx, row) in rows.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    // 查询数据,并反射结果对象
    let rows = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&mut conn)
        .await?;

    // 处理查询结果
    for row in rows {
        let id: i64 = row.id;
        let name: String = row.name;
        let email: String = row.email;

        println!("id: {}, name: {}, email: {}", id, name, email);
    }

    Ok(())
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {

    use tokio::time::sleep;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_sqlx_sqlite_example() {
        // tokio_client_sample();
        sqlx_sqlite_example();
    }
}