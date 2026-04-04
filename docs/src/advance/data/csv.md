# CSV 文件处理

## 开篇故事

想象你收到一份 Excel 表格，里面是员工信息：ID、姓名、年龄、部门、薪资。你需要筛选出薪资大于 5000 且年龄小于 50 的员工，然后保存到新文件。手动操作很繁琐，而 Rust 的 CSV 库可以自动完成这个任务。

---

## 本章适合谁

如果你需要在 Rust 程序中处理 CSV 文件（读取数据、分析数据、导出数据），本章适合你。CSV 是最常用的数据交换格式之一。

---

## 你会学到什么

完成本章后，你可以：

1. 读取和解析 CSV 文件
2. 将 CSV 数据映射到结构体
3. 写入 CSV 数据
4. 处理 CSV 错误和验证
5. 处理大型 CSV 文件

---

## 前置要求

- 结构体 - 结构体定义
- [Serde 序列化](serialization.md) - 序列化基础
- 错误处理 - 错误处理基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add serde --features derive
cargo add csv
```

## 第一个例子

最简单的 CSV 读取：

```rust
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("cities.csv")?;
    
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    
    Ok(())
}
```

**完整示例**: [csv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/csv_sample.rs)

---

## 原理解析

### CSV 特性

**csv crate 是一个 CSV 处理库**：

- ✅ 自动解析 CSV
- ✅ Serde 集成
- ✅ 处理各种 CSV 格式
- ✅ 高性能

### 定义结构体

**使用 serde 属性**：

```rust
use serde::{Deserialize, Serialize};

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
```

**serde 属性**：
- `rename`: 指定 CSV 列名映射
- `default`: 提供默认值
- `skip`: 跳过某列

### 读取 CSV

**从文件读取**：

```rust
use csv::Reader;

let mut reader = Reader::from_path("data.csv")?;

for result in reader.deserialize() {
    let record: Record = result?;
    println!("{:?}", record);
}
```

**从字符串读取**：

```rust
use csv::ReaderBuilder;

let data = "name,age\nAlice,30\nBob,25";
let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_reader(data.as_bytes());

for result in reader.deserialize() {
    let record: Record = result?;
    println!("{:?}", record);
}
```

### 自定义解析

**指定分隔符**：

```rust
let mut reader = ReaderBuilder::new()
    .delimiter(b';')  // 使用分号分隔
    .has_headers(true)
    .from_path("data.csv")?;
```

**无表头**：

```rust
let mut reader = ReaderBuilder::new()
    .has_headers(false)
    .from_path("data.csv")?;
```

### 写入 CSV

**写入文件**：

```rust
use csv::WriterBuilder;

let mut writer = WriterBuilder::new()
    .delimiter(b',')
    .has_headers(false)
    .from_path("output.csv")?;

// 写入表头
writer.write_record(&["ID", "Name", "Age", "Department", "Salary"])?;

// 写入数据
for employee in employees {
    writer.serialize(employee)?;
}

writer.flush()?;  // 确保数据写入磁盘
```

### 过滤和转换

**过滤数据**：

```rust
// 过滤：薪资 > 5000 且年龄 < 50
let filtered: Vec<_> = employees
    .into_iter()
    .filter(|e| e.salary > 5000.0 && e.age < 50)
    .collect();
```

**转换数据**：

```rust
// 转换：添加新字段
let transformed: Vec<_> = employees
    .into_iter()
    .map(|e| {
        (e.name, e.salary * 1.1)  // 加薪 10%
    })
    .collect();
```

### 处理大型文件

**逐行处理**：

```rust
use csv::Reader;

let mut reader = Reader::from_path("large_data.csv")?;

// 逐行读取并处理
for result in reader.deserialize::<Employee>() {
    let employee: Employee = result?;
    // 逐条处理避免内存溢出
    println!("{}", employee.name);
}
```

---

## 常见错误

### 错误 1: 列名不匹配

```rust
#[derive(Deserialize)]
struct Employee {
    id: u32,
    name: String,
    age: u8,
}

// CSV 文件列名：ID,Name,Age
// ❌ 小写不匹配
```

**错误信息**:
```
CSV deserialize error: field 'id' not found
```

**修复方法**:
```rust
#[derive(Deserialize)]
struct Employee {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Age")]
    age: u8,
}
```

### 错误 2: 类型不匹配

```rust
#[derive(Deserialize)]
struct Employee {
    age: u8,
}

// CSV 数据：age
//           twenty
// ❌ 期望数字，得到字符串
```

**错误信息**:
```
CSV deserialize error: field 'age': invalid type: string "twenty", expected u8
```

**修复方法**:
确保 CSV 数据类型正确，或使用 `Option` 处理空值。

### 错误 3: 文件不存在

```rust
let mut reader = Reader::from_path("nonexistent.csv")?;
// ❌ 文件不存在
```

**错误信息**:
```
No such file or directory (os error 2)
```

**修复方法**:
```rust
use std::path::Path;

let path = "data.csv";
if !Path::new(path).exists() {
    eprintln!("文件不存在：{}", path);
    return;
}

let mut reader = Reader::from_path(path)?;
```

---

## 动手练习

### 练习 1: 定义员工结构体

```rust
use serde::{Deserialize, Serialize};

// TODO: 定义 Employee 结构体
// 字段：id (u32), name (String), department (String), salary (f64)
// 使用 serde 属性映射 CSV 列名
```

<details>
<summary>点击查看答案</summary>

```rust
#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Department")]
    department: String,
    #[serde(rename = "Salary")]
    salary: f64,
}
```
</details>

### 练习 2: 读取和打印员工

```rust
use csv::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("employees.csv")?;
    
    // TODO: 遍历所有员工
    // TODO: 打印每个员工信息
}
```

<details>
<summary>点击查看答案</summary>

```rust
for result in reader.deserialize() {
    let employee: Employee = result?;
    println!("{} - {} - {}", employee.name, employee.department, employee.salary);
}
```
</details>

### 练习 3: 过滤和写入

```rust
// TODO: 读取员工数据
// TODO: 过滤薪资 > 5000 的员工
// TODO: 写入到新 CSV 文件
```

<details>
<summary>点击查看答案</summary>

```rust
let mut reader = Reader::from_path("employees.csv")?;
let employees: Vec<Employee> = reader.deserialize().collect()?;

let filtered: Vec<_> = employees
    .into_iter()
    .filter(|e| e.salary > 5000.0)
    .collect();

let mut writer = Writer::from_path("high_salary.csv")?;
for employee in filtered {
    writer.serialize(employee)?;
}
writer.flush()?;
```
</details>

---

## 故障排查 (FAQ)

### Q: 如何处理空值？

**A**: 使用 `Option<T>`:

```rust
#[derive(Deserialize)]
struct Record {
    name: String,
    age: Option<u8>,  // 可以是空值
}
```

### Q: 如何处理不同的分隔符？

**A**: 
```rust
let mut reader = ReaderBuilder::new()
    .delimiter(b';')  // 分号
    .from_path("data.csv")?;
```

### Q: 如何处理大文件？

**A**: 逐行处理，不要一次性加载：

```rust
for result in reader.deserialize() {
    let record: Record = result?;
    // 逐条处理
}
```

---

## 知识扩展

### 自定义解析器

```rust
use csv::{ReaderBuilder, StringRecord};

let mut reader = ReaderBuilder::new()
    .has_headers(false)
    .from_path("data.csv")?;

for result in reader.records() {
    let record = result?;
    println!("Column 1: {}", record.get(0).unwrap());
    println!("Column 2: {}", record.get(1).unwrap());
}
```

### 并行处理

```rust
use rayon::prelude::*;

let employees: Vec<Employee> = reader
    .deserialize()
    .par_bridge()  // 并行处理
    .collect::<Result<_, _>>()?;
```

### 性能优化

```rust
// 预分配容量
let mut employees = Vec::with_capacity(1000);

for result in reader.deserialize() {
    employees.push(result?);
}
```

---

## 小结

**核心要点**：

1. **csv crate**: CSV 处理标准库
2. **Serde 集成**: 自动映射到结构体
3. **Reader/Writer**: 读写 CSV
4. **错误处理**: 使用 Result 处理解析错误
5. **大文件处理**: 逐行处理避免内存溢出

**关键术语**：

- **CSV**: 逗号分隔值
- **Deserialize**: 反序列化
- **Serialize**: 序列化
- **Reader**: CSV 读取器
- **Writer**: CSV 写入器

---

## 术语表

| English | 中文 |
| ------- | ---- |
| CSV | 逗号分隔值 |
| Deserialize | 反序列化 |
| Serialize | 序列化 |
| Reader | 读取器 |
| Writer | 写入器 |
| Delimiter | 分隔符 |

---

## 知识检查

**快速测验**（答案在下方）：

1. CSV 和 JSON 序列化有什么区别？

2. 如何处理 CSV 中不同类型的列？

3. `csv::Reader` 和 `csv::StringRecord` 的区别？

<details>
<summary>点击查看答案与解析</summary>

1. CSV 是表格格式（行/列），JSON 是树形格式
2. 使用 Serde 反序列化到结构体，自动类型转换
3. `Reader` 是迭代器，`StringRecord` 是单行数据

**关键理解**: CSV 适合表格数据，JSON 适合嵌套数据。
</details>

## 继续学习

**前一章**: [JSON 序列化](json.md)  
**下一章**: [零拷贝序列化](rkyv.md)

**相关章节**:
- [JSON 序列化](json.md)
- [Serde 序列化](serialization.md)
- 结构体

**返回**: 高级进阶

---

**完整示例**: [csv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/csv_sample.rs)
