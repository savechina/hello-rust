# CSV 文件操作

安装`csv` 文件cartes包

```bash
cargo add csv
```

或者修改Cargo.toml配置：
```toml
csv = "1.3.1"
```

样例代码：

```rust
use std::{error::Error, io, process};

use anyhow::Result;
use csv::Reader;
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

/// 定义 CSV 结构体 并使用 serde 进行序列化和反序列化
/// 使用 serde 的属性来指定字段名和映射
#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Age")]
    age: u8,
    #[serde(rename = "Department")]
    department: String,
    #[serde(rename = "Salary")]
    salary: f64,
}

/// 使用 serde 的属性来指定字段名和映射
#[derive(Debug, serde::Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

/// 处理 CSV 文件
fn process_employees(input_path: &str, output_path: &str) -> Result<()> {
    // 读取 CSV 文件 (自动推断分隔符)
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)?;

    // 反序列化为 Employee 结构体
    let employees: Vec<Employee> = reader.deserialize().collect::<Result<_, _>>()?; // 收集时处理错误

    // 过滤数据: 保留薪资 > 5000 且年龄 < 50 的员工
    let filtered: Vec<_> = employees
        .into_iter()
        .filter(|e| e.salary > 5000.0 && e.age < 50)
        .collect();

    // 写入新 CSV
    let mut writer = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(output_path)?;

    // 写入表头
    writer.write_record(&["ID", "Name", "Age", "Department", "Salary"])?;

    // 序列化并写入数据
    for employee in filtered {
        //序列化并写入数据
        writer.serialize(employee)?;
    }

    writer.flush()?; // 确保数据写入磁盘
    Ok(())
}

/// 处理大型 CSV 文件
fn process_large_csv() -> Result<()> {
    // 读取 CSV 文件
    let mut reader = Reader::from_path("data/large_data.csv")?;

    // 逐行读取并处理
    // 使用迭代器逐行读取
    for result in reader.deserialize::<Employee>() {
        let employee: Employee = result?;
        // 逐条处理避免内存溢出
        println!("{}", employee.name)
    }

    Ok(())
}

/// 读取 CSV 文件并处理数据
fn csv_sample() -> Result<()> {
    // 输入输出文件路径
    let input = "data/employees.csv";
    let output = "data/filtered_employees.csv";

    // 处理数据
    process_employees(input, output)?;

    println!("CSV 处理完成！结果已保存至 {}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_sample() {
        assert!(csv_sample().is_ok());
    }

    #[test]
    fn test_process_large_csv() {
        assert!(process_large_csv().is_ok());
    }
}

```