# Dotenv 环境变量配置

## 开篇故事

想象你在不同的环境中工作：家里、办公室、咖啡厅。每个地方有不同的 WiFi 密码、打印机设置。你不会把这些配置写死在脑子里，而是记在一个便携的小本子上。Dotenv 就像这个本子——把配置放在 `.env` 文件里，程序启动时自动读取。

---

## 本章适合谁

如果你正在开发需要在不同环境（开发、测试、生产）运行的 Rust 程序，需要管理数据库连接、API 密钥等配置，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 创建和使用 `.env` 文件管理环境变量
2. 使用 `dotenvy` crate 加载环境配置
3. 读取和操作环境变量
4. 处理不同环境的配置切换

---

## 前置要求

学习本章前，你需要理解：

- Result 类型的基本使用
- 基本的文件路径概念
- Cargo.toml 配置

---

## 第一个例子

最简单的 dotenv 使用：

```rust
use dotenvy;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    println!("数据库地址: {}", database_url);
    Ok(())
}
```

完整示例：src/advance/dotenv_sample.rs

---

## 原理解析

### 环境变量加载顺序

```
系统环境变量 → .env 文件 → 程序显式设置
```

### .env 文件格式

```bash
DATABASE_URL=postgres://user:pass@localhost/mydb
API_KEY=sk_live_xxxxxxxx
PORT=8080
```

---

## 常见错误

### 错误 1: .env 文件不存在

创建 .env 文件或使用 dotenvy::from_filename(".env.local")

### 错误 2: 环境变量不存在

使用 unwrap_or 提供默认值

### 错误 3: .env 提交到版本控制

添加到 .gitignore，使用 .env.example 作为模板

---

## 知识检查

**问题 1**: 生产环境应该用 .env 文件吗？

答案: 不推荐，使用系统环境变量或密钥管理服务

**问题 2**: 环境变量优先级？

答案: 系统变量 > .env 文件 > 程序默认值

---

## 小结

核心要点：.env文件、dotenvy加载、环境变量优先级、不要提交到Git

完整示例：src/advance/dotenv_sample.rs
