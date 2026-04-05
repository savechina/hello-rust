# 常用操作速查

本章节提供 Rust 开发中最常用操作的代码片段,方便快速查找和复制使用。每个类别包含 3-5 个实用示例。

---

## 文件操作

### 读取文件

```rust,ignore
use std::fs;
use std::io::{self, BufRead};

// 一次性读取整个文件
fn read_entire_file() -> io::Result<String> {
    let content = fs::read_to_string("data.txt")?;
    Ok(content)
}

// 按行读取文件
fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect();
    Ok(lines)
}

// 读取二进制文件
fn read_binary_file() -> io::Result<Vec<u8>> {
    let data = fs::read("image.png")?;
    Ok(data)
}
```

### 写入文件

```rust,ignore
use std::fs::{self, File};
use std::io::Write;

// 写入字符串到文件
fn write_to_file(content: &str) -> io::Result<()> {
    fs::write("output.txt", content)?;
    Ok(())
}

// 追加内容到文件
fn append_to_file(content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.txt")?;
    
    writeln!(file, "{}", content)?;
    Ok(())
}

// 写入二进制数据
fn write_binary_data(data: &[u8]) -> io::Result<()> {
    fs::write("binary.dat", data)?;
    Ok(())
}
```

### 文件与目录操作

```rust,ignore
use std::fs;
use std::path::Path;

// 创建目录
fn create_directory() -> io::Result<()> {
    fs::create_dir_all("path/to/directory")?;
    Ok(())
}

// 复制文件
fn copy_file() -> io::Result<()> {
    fs::copy("source.txt", "destination.txt")?;
    Ok(())
}

// 删除文件或目录
fn delete_file_or_dir() -> io::Result<()> {
    fs::remove_file("file.txt")?;
    fs::remove_dir_all("directory")?;
    Ok(())
}

// 检查路径是否存在
fn check_path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
```

---

## 集合操作

### Vec 向量操作

```rust,ignore
// 创建和初始化
let mut vec: Vec<i32> = Vec::new();
let vec = vec![1, 2, 3, 4, 5];
let vec: Vec<i32> = (0..10).collect();

// 常用操作
vec.push(6);              // 添加元素
vec.pop();                // 移除末尾元素
vec.insert(0, 0);         // 在索引 0 插入
vec.remove(0);            // 移除索引 0 的元素
vec.len();                // 获取长度
vec.is_empty();           // 判断是否为空
vec.clear();              // 清空

// 查找和过滤
let first = vec.first();          // 获取第一个元素
let last = vec.last();            // 获取最后一个元素
let found = vec.iter().find(|&&x| x > 3);  // 查找元素
let filtered: Vec<&i32> = vec.iter().filter(|&&x| x > 2).collect();  // 过滤

// 排序和反转
vec.sort();               // 排序
vec.sort_by(|a, b| b.cmp(a));  // 自定义排序
vec.reverse();            // 反转
vec.dedup();             // 去重
```

### HashMap 哈希表操作

```rust,ignore
use std::collections::HashMap;

// 创建和初始化
let mut map: HashMap<&str, i32> = HashMap::new();
let map = HashMap::from([("apple", 5), ("banana", 3)]);

// 基本操作
map.insert("orange", 10);          // 插入
map.remove("apple");                // 移除
map.get("banana");                  // 获取值 (Option<&i32>)
map.len();                          // 获取长度
map.is_empty();                     // 判断是否为空
map.clear();                        // 清空

// 安全访问
let value = map.get("key").unwrap_or(&0);  // 带默认值
let value = map.entry("key").or_insert(0); // 不存在则插入
*map.entry("count").or_insert(0) += 1;     // 计数器

// 遍历
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// 只遍历键或值
for key in map.keys() { /* ... */ }
for value in map.values() { /* ... */ }
```

### HashSet 哈希集合操作

```rust,ignore
use std::collections::HashSet;

// 创建和初始化
let mut set: HashSet<i32> = HashSet::new();
let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

// 基本操作
set.insert(4);              // 插入元素
set.remove(&1);             // 移除元素
set.contains(&2);            // 检查是否存在
set.len();                   // 获取长度
set.is_empty();              // 判断是否为空

// 集合运算
let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let b: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

let union: HashSet<&i32> = a.union(&b).collect();        // 并集 {1, 2, 3, 4, 5}
let intersection: HashSet<&i32> = a.intersection(&b).collect();  // 交集 {3}
let difference: HashSet<&i32> = a.difference(&b).collect();      // 差集 {1, 2}
```

---

## 字符串操作

### 创建与转换

```rust,ignore
// 创建字符串
let s1 = String::new();
let s2 = String::from("hello");
let s3 = "world".to_string();
let s4 = format!("{} {}", s2, s3);  // 格式化创建

// 从其他类型转换
let num_str = 42.to_string();
let bool_str = true.to_string();
let parsed: i32 = "42".parse().unwrap();  // 字符串转数字

// String 和 &str 转换
let string: String = "hello".to_string();
let slice: &str = &string;
let owned: String = slice.to_owned();
```

### 字符串修改

```rust,ignore
let mut s = String::from("hello");

// 追加内容
s.push_str(" world");    // 追加字符串
s.push('!');             // 追加字符

// 插入内容
s.insert(0, 'H');        // 在索引 0 插入字符
s.insert_str(6, "Rust ");  // 在索引 6 插入字符串

// 替换内容
s = s.replace("hello", "hi");
s = s.replacen("l", "L", 1);  // 只替换第一个

// 删除内容
s.remove(0);             // 删除指定位置的字符
s.truncate(5);           // 截断到指定长度
s.clear();               // 清空字符串

// 弹出字符
let last_char = s.pop();  // 弹出最后一个字符
```

### 字符串查询

```rust,ignore
let s = "Hello, Rust!";

// 长度和判空
let len = s.len();               // 长度
let is_empty = s.is_empty();     // 是否为空

// 查找
s.contains("Rust");              // 是否包含
s.starts_with("Hello");          // 是否以...开头
s.ends_with("!");                // 是否以...结尾
s.find("Rust");                  // 查找位置 (Option<usize>)
s.rfind("l");                    // 从右边查找

// 分割
let parts: Vec<&str> = "a,b,c".split(',').collect();
let lines: Vec<&str> = "line1\nline2".lines().collect();
let words: Vec<&str> = "hello world".split_whitespace().collect();

// 截取
let slice = &s[0..5];            // 切片 [0, 5)
let first_n = s.chars().take(5).collect::<String>();  // 前5个字符
```

---

## 错误处理

### Result 处理

```rust,ignore
use std::fs::File;
use std::io::{self, Read};

// 基本错误处理
fn read_file_basic() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 使用 match 处理
fn handle_with_match() {
    let result = File::open("data.txt");
    match result {
        Ok(file) => println!("文件打开成功"),
        Err(error) => println!("错误: {}", error),
    }
}

// unwrap 和 expect
let file = File::open("data.txt").unwrap();  // 失败时 panic
let file = File::open("data.txt").expect("无法打开文件");  // 带 panic 信息

// 提供默认值
let content = read_file_basic().unwrap_or_default();
let content = read_file_basic().unwrap_or_else(|err| {
    println!("读取失败: {}", err);
    String::new()
});
```

### Option 处理

```rust,ignore
// 基本用法
let some_value: Option<i32> = Some(42);
let none_value: Option<i32> = None;

// 检查和提取
some_value.is_some();            // 是否有值
some_value.is_none();            // 是否无值
let value = some_value.unwrap(); // 提取值 (None 会 panic)

// 安全提取
let value = some_value.unwrap_or(0);        // 提供默认值
let value = some_value.unwrap_or_else(|| 0); // 闭包提供默认值
let value = some_value.unwrap_or_default(); // 类型默认值

// map 和 and_then
let doubled = some_value.map(|x| x * 2);          // Some(84)
let result = some_value.and_then(|x| Some(x + 1)); // Some(43)

// 使用 match
match some_value {
    Some(value) => println!("值为: {}", value),
    None => println!("无值"),
}

// if let 语法糖
if let Some(value) = some_value {
    println!("值为: {}", value);
}
```

### 自定义错误类型

```rust,ignore
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO错误: {}", e),
            MyError::ParseError(e) => write!(f, "解析错误: {}", e),
            MyError::CustomError(msg) => write!(f, "自定义错误: {}", msg),
        }
    }
}

impl Error for MyError {}

// 使用 thiserror 简化 (需要添加依赖)
// use thiserror::Error;
// 
// #[derive(Error, Debug)]
// enum MyError {
//     #[error("IO错误: {0}")]
//     Io(#[from] std::io::Error),
//     
//     #[error("解析错误: {0}")]
//     Parse(#[from] std::num::ParseIntError),
// }
```

---

## 异步基础

### async/await 基本用法

```rust,ignore
use tokio::time::{sleep, Duration};

// 定义异步函数
async fn say_hello() {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World");
}

// 异步函数返回值
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let body = response.text().await?;
    Ok(body)
}

// 运行异步函数
#[tokio::main]
async fn main() {
    say_hello().await;
    match fetch_data().await {
        Ok(data) => println!("数据: {}", data),
        Err(e) => println!("错误: {}", e),
    }
}
```

### 并发执行

```rust,ignore
use tokio;

// 顺序执行
async fn sequential() {
    let r1 = async_function1().await;
    let r2 = async_function2().await;
}

// 并发执行
async fn concurrent() {
    let (r1, r2) = tokio::join!(
        async_function1(),
        async_function2(),
    );
}

// 使用 tokio::spawn 创建任务
async fn spawn_tasks() {
    let handle1 = tokio::spawn(async {
        // 任务 1
        println!("Task 1");
    });
    
    let handle2 = tokio::spawn(async {
        // 任务 2
        println!("Task 2");
    });
    
    let _ = tokio::join!(handle1, handle2);
}

// 使用 join_all 等待多个任务
async fn join_all_tasks() {
    let tasks: Vec<_> = (0..5).map(|i| {
        tokio::spawn(async move {
            println!("Task {}", i);
        })
    }).collect();
    
    let results = futures::future::join_all(tasks).await;
}
```

### 异步通道

```rust,ignore
use tokio::sync::mpsc;

// 多生产者单消费者通道
async fn mpsc_example() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // 生产者
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    
    // 消费者
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("收到: {}", value);
        }
    });
    
    let _ = tokio::join!(producer, consumer);
}

// oneshot 通道 (单次发送)
async fn oneshot_example() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    tokio::spawn(async move {
        tx.send("Hello").unwrap();
    });
    
    if let Ok(msg) = rx.await {
        println!("收到: {}", msg);
    }
}
```

---

## 网络编程

### TCP 客户端

```rust,ignore
use std::io::{self, Write};
use std::net::TcpStream;

// TCP 客户端
fn tcp_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    
    // 发送数据
    stream.write_all(b"Hello, Server!")?;
    
    // 读取响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    println!("收到响应: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}

// 异步 TCP 客户端 (使用 tokio)
async fn async_tcp_client() -> io::Result<()> {
    use tokio::net::TcpStream;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"Hello, Server!").await?;
    
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("收到响应: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}
```

### TCP 服务器

```rust,ignore
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// TCP 服务器
fn tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("服务器启动,监听 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => println!("连接失败: {}", e),
        }
    }
    
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 { break; }
        
        // 回显
        stream.write_all(&buffer[..n]).unwrap();
    }
}
```

### HTTP 客户端

```rust,ignore
// 使用 reqwest (需要添加依赖)
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    println!("GET 响应: {}", body);
    
    // POST 请求
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&serde_json::json!({
            "name": "Rust",
            "version": "1.70"
        }))
        .send()
        .await?;
    
    println!("POST 响应: {}", response.text().await?);
    
    Ok(())
}
```

---

## 序列化

### JSON 序列化/反序列化

```rust,ignore
use serde::{Deserialize, Serialize};

// 定义结构体
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

// 序列化为 JSON 字符串
fn serialize_json() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
    };
    
    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {}", json);
    // {"id":1,"name":"Alice","email":"alice@example.com","active":true}
    
    // 美化输出
    let pretty = serde_json::to_string_pretty(&user).unwrap();
    println!("Pretty JSON:\n{}", pretty);
}

// 从 JSON 反序列化
fn deserialize_json() {
    let json = r#"{
        "id": 1,
        "name": "Alice",
        "email": "alice@example.com",
        "active": true
    }"#;
    
    let user: User = serde_json::from_str(json).unwrap();
    println!("User: {:?}", user);
}

// 处理泛型 JSON
fn parse_generic_json() {
    let json = r#"{"name": "Alice", "age": 30}"#;
    let value: serde_json::Value = serde_json::from_str(json).unwrap();
    
    let name = value["name"].as_str().unwrap();
    let age = value["age"].as_i64().unwrap();
    println!("Name: {}, Age: {}", name, age);
}
```

### 其他序列化格式

```rust,ignore
// YAML 序列化 (需要添加 serde_yaml 依赖)
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    database_url: String,
    port: u16,
}

fn yaml_example() {
    let config = Config {
        database_url: "localhost:5432".to_string(),
        port: 8080,
    };
    
    // 序列化为 YAML
    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("YAML:\n{}", yaml);
    
    // 从 YAML 反序列化
    let config: Config = serde_yaml::from_str(&yaml).unwrap();
    println!("Config: {:?}", config);
}

// TOML 序列化 (需要添加 toml 依赖)
fn toml_example() {
    let config = Config {
        database_url: "localhost:5432".to_string(),
        port: 8080,
    };
    
    // 序列化为 TOML
    let toml = toml::to_string(&config).unwrap();
    println!("TOML:\n{}", toml);
    
    // 从 TOML 反序列化
    let config: Config = toml::from_str(&toml).unwrap();
    println!("Config: {:?}", config);
}

// 二进制序列化 (需要添加 bincode 依赖)
fn binary_example() {
    let config = Config {
        database_url: "localhost:5432".to_string(),
        port: 8080,
    };
    
    // 序列化为二进制
    let bytes = bincode::serialize(&config).unwrap();
    println!("Binary length: {} bytes", bytes.len());
    
    // 从二进制反序列化
    let config: Config = bincode::deserialize(&bytes).unwrap();
    println!("Config: {:?}", config);
}
```

---

## 测试

### 单元测试

```rust,ignore
// 在同一文件中编写测试
#[cfg(test)]
mod tests {
    // 需要导入父模块的内容
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_string_concatenation() {
        let s1 = "Hello";
        let s2 = "World";
        assert_eq!(format!("{}, {}", s1, s2), "Hello, World");
    }
    
    #[test]
    fn test_with_assertions() {
        let value = Some(42);
        
        // 各种断言
        assert!(value.is_some());           // 布尔断言
        assert_eq!(value.unwrap(), 42);      // 相等断言
        assert_ne!(value, None);            // 不等断言
    }
    
    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_panic() {
        panic!("division by zero");
    }
    
    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }
}
```

### 集成测试

```rust,ignore
// tests/integration_test.rs

// 导入外部 crate 的公共 API
use my_crate::function_to_test;

#[test]
fn test_external_function() {
    assert_eq!(function_to_test(2), 4);
}

// 测试异步函数
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}

// 测试错误情况
#[test]
fn test_error_case() {
    let result = fallible_function(-1);
    assert!(result.is_err());
}
```

### 测试辅助工具

```rust,ignore
// 测试前初始化
#[cfg(test)]
mod tests {
    use super::*;
    
    // 每个测试前运行
    fn setup() -> TestContext {
        TestContext::new()
    }
    
    #[test]
    fn test_with_setup() {
        let ctx = setup();
        // 使用 ctx 进行测试
    }
}

// 使用测试属性宏
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]  // 默认跳过此测试
    fn expensive_test() {
        // 运行: cargo test -- --ignored
    }
    
    // 测试特定功能
    #[test]
    #[cfg(feature = "advanced")]
    fn test_advanced_feature() {
        // 仅在 feature 启用时运行
    }
}

// 基准测试 (需要 nightly)
// #[bench]
// fn bench_addition(b: &mut test::Bencher) {
//     b.iter(|| 2 + 2);
// }
```

---

## 命令行参数

### std::env::args

```rust,ignore
use std::env;

fn main() {
    // 获取所有参数
    let args: Vec<String> = env::args().collect();
    
    // args[0] 是程序名
    println!("程序名: {}", args[0]);
    
    // 遍历参数
    for (i, arg) in args.iter().enumerate() {
        println!("参数 {}: {}", i, arg);
    }
    
    // 简单参数解析
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "help" => println!("显示帮助"),
            "version" => println!("版本 1.0.0"),
            _ => println!("未知命令"),
        }
    }
}
```

### 使用 clap (推荐)

```rust,ignore
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .about("我的应用")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("输入文件")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("输出文件"))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("详细输出")
            .action(clap::ArgAction::SetTrue))
        .get_matches();
    
    // 获取参数值
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");
    
    println!("输入: {}", input);
    if let Some(o) = output {
        println!("输出: {}", o);
    }
    if verbose {
        println!("详细模式");
    }
}
```

### 使用 derive 宏 (更简洁)

```rust,ignore
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "myapp")]
#[command(about = "我的应用", long_about = None)]
struct Args {
    /// 输入文件
    #[arg(short, long)]
    input: String,
    
    /// 输出文件
    #[arg(short, long)]
    output: Option<String>,
    
    /// 详细输出
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    /// 端口号
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    
    println!("输入: {}", args.input);
    if let Some(output) = args.output {
        println!("输出: {}", output);
    }
    println!("详细模式: {}", args.verbose);
    println!("端口: {}", args.port);
}
```

---

## 环境变量

### 读取环境变量

```rust,ignore
use std::env;

fn main() {
    // 获取单个环境变量
    match env::var("HOME") {
        Ok(home) => println!("HOME: {}", home),
        Err(e) => println!("无法获取 HOME: {}", e),
    }
    
    // 带默认值
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    println!("端口: {}", port);
    
    // 解析为特定类型
    let debug = env::var("DEBUG")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    println!("Debug: {}", debug);
    
    // 获取所有环境变量
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```

### 设置环境变量

```rust,ignore
use std::env;

fn main() {
    // 设置环境变量 (仅当前进程)
    env::set_var("MY_VAR", "my_value");
    println!("MY_VAR: {}", env::var("MY_VAR").unwrap());
    
    // 移除环境变量
    env::remove_var("MY_VAR");
    
    // 临时设置环境变量
    {
        let _guard = env::var("PATH").unwrap();
        env::set_var("PATH", "/custom/path");
        // 在此作用域内 PATH 为 /custom/path
    }
    // 作用域结束后原值恢复
}
```

### 使用 dotenv 加载 .env 文件

```rust,ignore
// 需要添加 dotenv 依赖
use dotenv::dotenv;
use std::env;

fn main() {
    // 从 .env 文件加载环境变量
    dotenv().ok();
    
    // 现在可以读取 .env 中的变量
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("数据库 URL: {}", database_url);
    
    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set");
    println!("API Key: {}", api_key);
}

// .env 文件示例:
// DATABASE_URL=postgres://user:pass@localhost/db
// API_KEY=your-api-key-here
// DEBUG=true
```

---

## 线程与并发

### 创建线程

```rust,ignore
use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 主线程
    for i in 1..=5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    // 等待线程结束
    handle.join().unwrap();
}
```

### 线程间通信

```rust,ignore
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建通道
    let (tx, rx) = mpsc::channel();
    
    // 发送者线程
    let sender = thread::spawn(move || {
        let messages = vec![
            "Hello",
            "from",
            "thread",
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 主线程接收
    for received in rx {
        println!("收到: {}", received);
    }
    
    sender.join().unwrap();
}
```

### 共享状态

```rust,ignore
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 使用 Arc 和 Mutex 实现共享可变状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    // 等待所有线程
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("计数器: {}", *counter.lock().unwrap());
}
```

### 线程池

```rust,ignore
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// 简单线程池实现
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        
        Worker { id, thread }
    }
}

// 使用线程池
fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 0..10 {
        pool.execute(move || {
            println!("任务 {} 在执行", i);
        });
    }
}
```

---

## 时间与日期

### 时间测量

```rust,ignore
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // 测量代码执行时间
    let start = Instant::now();
    
    // 执行一些操作
    let mut sum = 0;
    for i in 0..1000000 {
        sum += i;
    }
    
    let duration = start.elapsed();
    println!("耗时: {:?}", duration);
    println!("微秒: {}", duration.as_micros());
    println!("毫秒: {}", duration.as_millis());
    println!("秒: {}", duration.as_secs());
    
    // 获取当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("Unix 时间戳: {}", timestamp);
    
    // 创建 Duration
    let five_seconds = Duration::from_secs(5);
    let hundred_millis = Duration::from_millis(100);
    let one_microsecond = Duration::from_micros(1);
}
```

### 使用 chrono 库

```rust,ignore
// 需要添加 chrono 依赖
use chrono::{DateTime, Local, Utc, TimeZone, NaiveDate, NaiveTime, Duration};

fn main() {
    // 当前时间
    let now = Local::now();
    println!("当前时间: {}", now);
    println!("格式化: {}", now.format("%Y-%m-%d %H:%M:%S"));
    
    // UTC 时间
    let utc_now = Utc::now();
    println!("UTC 时间: {}", utc_now);
    
    // 从字符串解析
    let dt = NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap();
    println!("日期: {}", dt);
    
    // 创建日期时间
    let dt = Local.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
    println!("创建的日期时间: {}", dt);
    
    // 时间运算
    let tomorrow = Local::now() + Duration::days(1);
    let last_week = Local::now() - Duration::weeks(1);
    println!("明天: {}", tomorrow.format("%Y-%m-%d"));
    println!("上周: {}", last_week.format("%Y-%m-%d"));
    
    // 时间比较
    let dt1 = Local.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
    let dt2 = Local.with_ymd_and_hms(2024, 1, 15, 11, 0, 0).unwrap();
    
    if dt1 < dt2 {
        println!("dt1 更早");
    }
    
    let diff = dt2.signed_duration_since(dt1);
    println!("时间差: {} 小时", diff.num_hours());
}
```

---

## 正则表达式

### 基本匹配

```rust,ignore
// 需要添加 regex 依赖
use regex::Regex;

fn main() {
    // 创建正则表达式
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    // 测试是否匹配
    let date = "2024-01-15";
    if re.is_match(date) {
        println!("{} 是有效的日期格式", date);
    }
    
    // 查找匹配
    let text = "我的邮箱是 example@test.com 和 test@example.com";
    let email_re = Regex::new(r"\w+@\w+\.\w+").unwrap();
    
    for cap in email_re.captures_iter(text) {
        println!("邮箱: {}", &cap[0]);
    }
    
    // 查找所有匹配
    let phone_re = Regex::new(r"\d{3}-\d{4}").unwrap();
    let text = "电话: 123-4567, 987-6543";
    
    for mat in phone_re.find_iter(text) {
        println!("电话: {}", mat.as_str());
    }
}
```

### 替换和分组

```rust,ignore
use regex::Regex;

fn main() {
    // 替换
    let re = Regex::new(r"\d+").unwrap();
    let text = "我有 100 个苹果和 200 个橘子";
    let result = re.replace_all(text, "X");
    println!("替换后: {}", result);
    
    // 使用捕获组
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "日期: 2024-01-15";
    
    if let Some(caps) = re.captures(text) {
        println!("年: {}", &caps[1]);
        println!("月: {}", &caps[2]);
        println!("日: {}", &caps[3]);
    }
    
    // 命名捕获组
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let text = "2024-01-15";
    
    if let Some(caps) = re.captures(text) {
        println!("年: {}", &caps["year"]);
        println!("月: {}", &caps["month"]);
        println!("日: {}", &caps["day"]);
    }
    
    // 使用捕获组替换
    let result = re.replace(text, "$month/$day/$year");
    println!("美式日期: {}", result);
}
```

---

## 宏

### 声明宏

```rust,ignore
// 基本宏定义
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 带参数的宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {:?} called", stringify!($func_name));
        }
    };
}

// 多参数宏
macro_rules! vec_macro {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 带类型的宏
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    say_hello!();
    
    create_function!(my_func);
    my_func();
    
    let v = vec_macro![1, 2, 3, 4, 5];
    println!("Vec: {:?}", v);
    
    let map = hash_map!["a" => 1, "b" => 2, "c" => 3];
    println!("Map: {:?}", map);
}
```

### 过程宏

```rust,ignore
// 自定义派生宏 (需要 proc-macro crate)
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}

// 使用
// #[derive(HelloMacro)]
// struct MyStruct;
// 
// MyStruct::hello_macro();
```

---

## 常用迭代器方法

### 映射与过滤

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // map: 转换每个元素
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("加倍: {:?}", doubled);
    
    // filter: 过滤元素
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // filter_map: 过滤并转换
    let result: Vec<i32> = numbers.iter()
        .filter_map(|x| if x % 2 == 0 { Some(x * 2) } else { None })
        .collect();
    println!("偶数加倍: {:?}", result);
    
    // take: 取前 n 个
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("前三个: {:?}", first_three);
    
    // skip: 跳过前 n 个
    let skip_five: Vec<&i32> = numbers.iter().skip(5).collect();
    println!("跳过前五个: {:?}", skip_five);
}
```

### 聚合操作

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 求和
    let sum: i32 = numbers.iter().sum();
    println!("总和: {}", sum);
    
    // 求积
    let product: i32 = numbers.iter().product();
    println!("乘积: {}", product);
    
    // 最小/最大值
    let min = numbers.iter().min();
    let max = numbers.iter().max();
    println!("最小值: {:?}, 最大值: {:?}", min, max);
    
    // 计数
    let count = numbers.iter().count();
    println!("数量: {}", count);
    
    // 折叠 (fold)
    let sum_with_fold = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Fold 求和: {}", sum_with_fold);
    
    // reduce (需要非空)
    let reduced = numbers.iter().reduce(|a, b| a + b);
    println!("Reduce: {:?}", reduced);
    
    // 判断条件
    let all_positive = numbers.iter().all(|x| *x > 0);
    let any_even = numbers.iter().any(|x| *x % 2 == 0);
    println!("全部为正: {}, 有偶数: {}", all_positive, any_even);
}
```

### 查找与排序

```rust,ignore
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 查找第一个满足条件的元素
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);
    
    // 查找元素的索引
    let position = numbers.iter().position(|x| *x == 5);
    println!("5 的位置: {:?}", position);
    
    // 排序
    let mut unsorted = vec![3, 1, 4, 1, 5, 9, 2, 6];
    unsorted.sort();
    println!("排序后: {:?}", unsorted);
    
    // 自定义排序
    unsorted.sort_by(|a, b| b.cmp(a));  // 降序
    println!("降序: {:?}", unsorted);
    
    // 稳定排序
    let mut pairs = vec![(2, 'b'), (1, 'a'), (2, 'a')];
    pairs.sort_by_key(|k| k.0);
    println!("按键排序: {:?}", pairs);
    
    // 反转
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.reverse();
    println!("反转后: {:?}", vec);
    
    // 去重
    let mut duplicates = vec![1, 2, 2, 3, 3, 3, 4];
    duplicates.sort();
    duplicates.dedup();
    println!("去重后: {:?}", duplicates);
}
```

---

## 智能指针

### Box 堆分配

```rust,ignore
// Box: 堆分配
let b = Box::new(5);
println!("Box: {}", b);

// 递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
println!("{:?}", list);
```

### Rc 引用计数

```rust,ignore
use std::rc::Rc;

fn main() {
    // Rc: 多所有权
    let a = Rc::new(5);
    let b = Rc::clone(&a);  // 增加引用计数
    let c = Rc::clone(&a);  // 增加引用计数
    
    println!("引用计数: {}", Rc::strong_count(&a));  // 3
    println!("值: {}", *b);  // 5
}
```

### RefCell 内部可变性

```rust,ignore
use std::cell::RefCell;

fn main() {
    // RefCell: 运行时借用检查
    let value = RefCell::new(5);
    
    // 不可变借用
    let borrowed = value.borrow();
    println!("值: {}", *borrowed);
    drop(borrowed);  // 必须释放借用
    
    // 可变借用
    let mut mutable = value.borrow_mut();
    *mutable += 1;
    println!("修改后: {}", *mutable);
}
```

### Arc + Mutex 多线程共享

```rust,ignore
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc + Mutex: 多线程共享可变数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("计数器: {}", *counter.lock().unwrap());
}
```

---

## 实用工具

### 随机数生成

```rust,ignore
// 需要添加 rand 依赖
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    
    // 随机整数
    let n1: i32 = rng.gen();
    let n2: i32 = rng.gen_range(0..100);
    println!("随机数: {}, {}", n1, n2);
    
    // 随机浮点数
    let f: f64 = rng.gen();
    println!("随机浮点数: {}", f);
    
    // 随机布尔值
    let b: bool = rng.gen();
    println!("随机布尔值: {}", b);
    
    // 从数组随机选择
    let choices = [1, 2, 3, 4, 5];
    let choice = choices.choose(&mut rng);
    println!("随机选择: {:?}", choice);
    
    // 打乱数组
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("打乱后: {:?}", nums);
}
```

### 命令行颜色输出

```rust,ignore
// 需要添加 colored 依赖
use colored::Colorize;

fn main() {
    // 彩色文本
    println!("{}", "hello".red());
    println!("{}", "world".green().bold());
    println!("{}", "rust".blue().on_white());
    println!("{}", "colored".yellow().italic());
    println!("{}", "text".magenta().underline());
    
    // 组合样式
    let message = "Hello, Rust!";
    println!("{}", message.bright_cyan().on_black().bold());
}
```

### 进度条

```rust,ignore
// 需要添加 indicatif 依赖
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let bar = ProgressBar::new(100);
    
    for _ in 0..100 {
        bar.inc(1);
        thread::sleep(Duration::from_millis(50));
    }
    
    bar.finish();
}
```

---

## 完整示例代码

所有代码片段均可直接复制使用。更多完整示例请参考:

- [文件操作示例](../advance/system/directory.md)
- [异步编程示例](../advance/async/tokio.md)
- [Web 开发示例](../advance/web/axum.md)
- [序列化示例](../advance/data/json.md)

---

**返回**: [目录](../SUMMARY.md)