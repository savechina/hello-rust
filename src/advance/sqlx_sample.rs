use dotenvy;
use sqlx::sqlite::SqlitePool;
use sqlx::Connection;
use sqlx::MySql;
use sqlx::MySqlConnection;
use sqlx::MySqlPool;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqliteConnection;
use std::borrow::Borrow;
use std::env;
use std::fmt::Pointer;
/// User 结构体
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}
/// sqlx_sqlite_example
#[tokio::main(flavor = "current_thread")]
pub(crate) async fn sqlx_sqlite_example() -> Result<(), sqlx::Error> {
    // 从环境变量中获取数据库 URL
    let db_url = "sqlite::memory:";

    // 连接数据库
    let mut conn = SqliteConnection::connect(db_url).await?;

    // Sqlite::create_database(db_url);

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

    // 处理查询结果
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

/// sqlx_mysql_example
#[tokio::main(flavor = "current_thread")]
pub(crate) async fn sqlx_mysql_example() -> Result<(), sqlx::Error> {
    dotenvy::dotenv();

    // 从环境变量中获取数据库 URL
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost:3306/test".to_string());
    // 创建一个数据库连接
    let pool = MySqlPool::connect(&db_url).await?;
    // 创建一个示例表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;
    // 插入一些示例数据
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("Alice")
        .bind("alice@example.com")
        .execute(&pool)
        .await?;
    // 查询数据
    let rows = sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        println!("id: {}, name: {}, email: {}", id, name, email);
    }
    // 查询数据,并反射结果对象
    let rows = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        let id: i64 = row.id;
        let name: String = row.name;
        let email: String = row.email;
        println!("id: {}, name: {}, email: {}", id, name, email);
    }
    // 关闭连接池
    pool.close().await;

    Ok(())
}

/// sqlx_mysql_example
#[tokio::main(flavor = "current_thread")]
pub(crate) async fn sqlx_mysql_transaction_example() -> Result<(), sqlx::Error> {
    dotenvy::dotenv();

    // 从环境变量中获取数据库 URL
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost:3306/test".to_string());
    // 创建一个数据库连接
    let pool = MySqlPool::connect(&db_url).await?;

    let mut tx = pool.begin().await?;

    let mut stmt = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("John Doe23")
        .bind("john.doe.23@example.com")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    // tx has been moved, cannot be used again. it used in context of the closure.
    // let mut stmt = sqlx::query("SELECT name,email) FROM users where name = ?")
    //     .bind("John Doe23")
    //     .execute(&mut *tx)
    //     .await?;
    // 关闭连接池
    pool.close().await;
    Ok(())
}

/// sqlx_mysql_example
#[tokio::main(flavor = "current_thread")]
pub(crate) async fn sqlx_mysql_transaction_template_example() -> Result<(), sqlx::Error> {
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
        sqlx_sqlite_example();
    }

    #[ignore = "reason: mysql connection is not available in CI"]
    #[test]
    fn test_sqlx_mysql_example() {
        sqlx_mysql_example();
    }

    #[ignore = "reason: mysql connection is not available in CI"]
    #[test]
    fn test_sqlx_mysql_transaction_example() {
        sqlx_mysql_transaction_example();
    }

    #[ignore = "reason: mysql connection is not available in CI"]
    #[test]
    fn test_sqlx_mysql_transaction_template_example() {
        sqlx_mysql_transaction_template_example();
    }
}
