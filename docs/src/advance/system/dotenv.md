# 环境变量配置

## 开篇故事

想象你要配置一个应用程序。硬编码配置（如数据库密码）就像把密码写在代码里——不安全且难以修改。环境变量就像配置文件——可以随时修改而不需要重新编译。.env 文件就是这样的配置文件。

---

## 本章适合谁

如果你需要在 Rust 程序中管理配置（数据库连接、API 密钥、环境设置），本章适合你。环境变量是管理配置的标准方法。

---

## 你会学到什么

完成本章后，你可以：

1. 从 .env 文件加载环境变量
2. 读取环境变量
3. 获取系统目录路径
4. 构建相对路径
5. 处理环境变量错误

---

## 前置要求

- 错误处理 - 错误处理基础
- [文件与目录操作](directory.md) - 文件路径基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add dotenvy
cargo add home
```

## 第一个例子

最简单的 .env 加载：

```rust,ignore
use dotenvy;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从 .env 文件加载环境变量
    dotenvy::dotenv()?;
    
    // 读取环境变量
    let database_url = env::var("DATABASE_URL")?;
    println!("数据库：{}", database_url);
    
    // 遍历所有环境变量
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    
    Ok(())
}
```

**完整示例**: [dotenv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/dotenv_sample.rs)

---

## 原理解析

### dotenvy 特性

**dotenvy 是环境变量加载库**：

- ✅ 从 .env 文件加载
- ✅ 覆盖现有环境变量
- ✅ 错误处理
- ✅ 跨平台支持

### .env 文件格式

**标准格式**：

```bash
# 注释
DATABASE_URL=postgres://localhost/mydb
API_KEY=your_api_key_here
DEBUG=true
PORT=3000
```

**带空格的值**：

```bash
MESSAGE="Hello World"
```

### 加载环境变量

**使用 dotenv()**：

```rust,ignore
use dotenvy;

// 加载当前目录的 .env 文件
dotenvy::dotenv()?;

// 读取环境变量
let api_key = std::env::var("API_KEY")?;
```

**使用 dotenv() 的 Result**：

```rust,ignore
// 如果 .env 文件不存在，忽略错误
let _ = dotenvy::dotenv();

// 或明确处理
match dotenvy::dotenv() {
    Ok(_) => println!("配置已加载"),
    Err(_) => println!("使用默认配置"),
}
```

### 读取环境变量

**使用 env::var()**：

```rust,ignore
use std::env;

// 返回 Result
let db_url = env::var("DATABASE_URL")?;

// 或使用 unwrap_or
let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
```

**使用 env::var_os()**：

```rust,ignore
// 返回 Option<OsString>
if let Some(home) = env::var_os("HOME") {
    println!("HOME: {:?}", home);
}
```

### 获取系统目录

**使用 env::home_dir()**：

```rust,ignore
use std::env;

if let Some(home) = env::home_dir() {
    println!("HOME: {}", home.display());
}
```

**使用 home crate**：

```rust,ignore
use home;

if let Some(home) = home::home_dir() {
    println!("HOME: {}", home.display());
}

// 获取 Cargo home
if let Some(cargo_home) = home::cargo_home() {
    println!("Cargo HOME: {}", cargo_home.display());
}
```

### 构建路径

**使用 env!("CARGO_MANIFEST_DIR")**：

```rust,ignore
// 编译时获取项目根目录
let manifest_dir = env!("CARGO_MANIFEST_DIR");

// 构建相对路径
let data_path = format!("{}/data/data.txt", manifest_dir);
```

**使用 PathBuf**：

```rust,ignore
use std::path::PathBuf;
use std::env;

let manifest_dir = env!("CARGO_MANIFEST_DIR");

// 推荐方式
let data_path = PathBuf::from(manifest_dir)
    .join("data")
    .join("data.txt");

println!("数据文件：{}", data_path.display());
```

### 获取当前目录

**使用 env::current_dir()**：

```rust,ignore
use std::env;

let current_dir = env::current_dir()?;
println!("当前目录：{}", current_dir.display());
```

**获取可执行文件目录**：

```rust,ignore
use std::env;

let exe_path = env::current_exe()?;
let exe_dir = exe_path.parent().unwrap();
println!("可执行文件目录：{}", exe_dir.display());
```

---

## 常见错误

### 错误 1: .env 文件不存在

```rust,ignore
dotenvy::dotenv()?;  // ❌ 如果 .env 不存在会报错
```

**错误信息**:
```
path not found
```

**修复方法**:
```rust,ignore
let _ = dotenvy::dotenv();  // ✅ 忽略错误
```

### 错误 2: 环境变量未设置

```rust,ignore
let db_url = env::var("DATABASE_URL")?;
// ❌ 如果未设置会报错
```

**错误信息**:
```
environment variable not found
```

**修复方法**:
```rust,ignore
// 提供默认值
let db_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "sqlite::memory:".to_string());
```

### 错误 3: 路径拼接错误

```rust,ignore
let path = env!("CARGO_MANIFEST_DIR") + "/data/data.txt";
// ❌ 字符串拼接，不跨平台
```

**修复方法**:
```rust,ignore
use std::path::PathBuf;

let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("data")
    .join("data.txt");  // ✅ 使用 PathBuf
```

---

## 动手练习

### 练习 1: 加载 .env 文件

```rust,ignore
use dotenvy;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 加载 .env 文件
    // TODO: 读取 API_KEY 环境变量
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
dotenvy::dotenv()?;

let api_key = env::var("API_KEY")?;
println!("API Key: {}", api_key);
```
</details>

### 练习 2: 获取系统目录

```rust,ignore
use std::env;

fn main() {
    // TODO: 获取 HOME 目录
    // TODO: 获取当前目录
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
if let Some(home) = env::home_dir() {
    println!("HOME: {}", home.display());
}

let current = env::current_dir().unwrap();
println!("当前目录：{}", current.display());
```
</details>

### 练习 3: 构建相对路径

```rust,ignore
use std::path::PathBuf;

fn main() {
    // TODO: 使用 CARGO_MANIFEST_DIR
    // TODO: 构建 data/config.json 路径
    // TODO: 打印路径
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let manifest = env!("CARGO_MANIFEST_DIR");

let config_path = PathBuf::from(manifest)
    .join("data")
    .join("config.json");

println!("配置文件：{}", config_path.display());
```
</details>

---

## 故障排查 (FAQ)

### Q: .env 文件应该放在哪里？

**A**: 
- **开发环境**: 项目根目录
- **生产环境**: 使用系统环境变量
- **不要提交**: .env 文件应该加入 .gitignore

### Q: 如何在测试中使用 .env？

**A**: 
```rust,ignore
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_env() {
        let _ = dotenvy::dotenv();
        // 测试代码
    }
}
```

### Q: 如何覆盖 .env 中的值？

**A**: 
```bash
# 系统环境变量优先级更高
export DATABASE_URL="postgres://prod/db"
# 会覆盖 .env 中的值
```

---

## 知识扩展

### .env 文件示例

```bash
# 数据库配置
DATABASE_URL=postgres://localhost/mydb
DATABASE_POOL_SIZE=10

# API 配置
API_KEY=your_secret_key
API_TIMEOUT=30

# 应用配置
RUST_LOG=debug
PORT=8080
DEBUG=true
```

### 条件加载

```rust,ignore
// 根据环境加载不同配置
if cfg!(debug_assertions) {
    let _ = dotenvy::from_filename(".env.development");
} else {
    let _ = dotenvy::from_filename(".env.production");
}
```

### 最佳实践

```rust,ignore
// config.rs
use dotenvy;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = dotenvy::dotenv();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite::memory:".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}
```

---

## 小结

**核心要点**：

1. **dotenvy**: 从 .env 文件加载环境变量
2. **env::var()**: 读取环境变量
3. **home_dir()**: 获取用户主目录
4. **CARGO_MANIFEST_DIR**: 项目根目录
5. **PathBuf**: 构建跨平台路径

**关键术语**：

- **.env File**: 环境变量配置文件
- **Environment Variable**: 环境变量
- **PathBuf**: 路径缓冲区
- **Manifest Dir**: 清单目录（项目根目录）

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Environment Variable | 环境变量 |
| .env File | .env 文件 |
| PathBuf | 路径缓冲区 |
| Manifest Directory | 清单目录 |
| Home Directory | 主目录 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `.env` 文件应该在什么时候加载？

2. 环境变量和配置文件的区别？

3. 如何处理不同环境的配置？

<details>
<summary>点击查看答案与解析</summary>

1. 程序启动时，在读取环境变量之前
2. 环境变量是进程级别的，配置文件是文件持久化的
3. 使用 `.env.development`, `.env.production` 或配置管理库

**关键理解**: `.env` 文件不应提交到版本控制（包含敏感信息）。
</details>

## 继续学习

**前一章**: [内存映射](memmap.md)  
**下一章**: [字节处理](bytes.md)

**相关章节**:
- [内存映射](memmap.md)
- [临时文件](tempfile.md)
- [文件与目录操作](directory.md)

**返回**: 高级进阶

---

**完整示例**: [dotenv_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/dotenv_sample.rs)
