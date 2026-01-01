use polars::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polars_frame_sample() {
        polars_frame_sample().unwrap();
    }
}
