use polars::prelude::*;
use polars::sql::SQLContext;

fn polars_frame_sample() -> PolarsResult<()> {
    // 1. 创建 DataFrame
    let df = df!(
        "name" => &["Alice", "Bob", "Charlie", "David"],
        "age" => &[25, 30, 35, 28],
        "city" => &["Beijing", "Shanghai", "London", "New York"]
    )?;

    println!("{:?}", df);

    // 2. 基础过滤：筛选年龄 > 28 的人
    let mask = df.column("age")?.i32()?.gt(28);
    let filtered_df = df.filter(&mask)?;

    println!("Filtered:\n{:?}", filtered_df);

    let lazy_df = df
        .lazy()
        .filter(col("age").gt(lit(30))) // 谓词下推（提前过滤以减少计算）
        .select([
            col("name"),
            col("age") * lit(10), // 列计算
        ])
        .collect()?; // 真正开始计算

    println!("Filtered:\n{:?}", lazy_df);

    Ok(())
}

/// polars dataframe handle by sql
fn polars_sql_sample() -> PolarsResult<()> {
    // 1. Prepare your data
    let df = df!(
        "name" => &["Alice", "Bob", "Charlie", "David"],
        "age" => &[25, 30, 35, 28],
        "salary" => &[5000, 8000, 12000, 7000]
    )?;

    // 2. Create a SQL Context and register the DataFrame as a table
    let mut ctx = SQLContext::new();
    ctx.register("users", df.lazy());

    // 3. Execute a SQL query
    let query = "
        SELECT name, age, salary * 1.1 AS increased_salary
        FROM users
        WHERE age > 25
        ORDER BY salary DESC
    ";

    let result = ctx.execute(query)?.collect()?;

    println!("{:?}", result);

    let lf = ctx
        .execute(
            "WITH high_earners AS (
            SELECT * FROM users WHERE salary > 10000
            )
        SELECT name FROM high_earners",
        )?
        .collect()?;

    println!("{:?}", lf);

    let lf = ctx.execute("SELECT * FROM users")?;
    let final_df = lf.with_column(col("age")).collect()?;
    println!("{:?}", final_df);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polars_frame_sample() {
        polars_frame_sample().unwrap();
    }

    #[test]
    fn test_polars_sql_sample() {
        polars_sql_sample().unwrap();
    }
}
