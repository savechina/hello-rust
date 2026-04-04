# 对象存储

## 开篇故事

想象你要存储大量文件（图片、文档、备份）。传统方式是：买硬盘 → 管理文件系统 → 处理备份。对象存储就像是：云存储仓库——你只管存储和读取，其他都交给服务处理。

---

## 本章适合谁

如果你需要在 Rust 程序中存储和检索文件（本地或云存储），本章适合你。object_store 是统一的对象存储接口。

---

## 你会学到什么

完成本章后，你可以：

1. 理解对象存储概念
2. 使用 LocalFileSystem
3. 存储键值对
4. 检索对象数据
5. 列出对象列表

---

## 前置要求

- 异步编程 - async/await 基础
- Tokio 运行时 - Tokio 基础
- [错误处理](../error-handling/error-handling.md) - 错误处理基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add object_store
cargo add futures
```

## 第一个例子

最简单的对象存储使用：

```rust,ignore
use object_store::{ObjectStore, path::Path};
use object_store::local::LocalFileSystem;
use bytes::Bytes;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取临时目录
    let temp_dir = std::env::temp_dir().join("hello");
    
    // 创建本地文件系统存储
    let store = Arc::new(LocalFileSystem::new_with_prefix(temp_dir)?);
    
    // 存储数据
    let key = Path::from("my_key.txt");
    let value = Bytes::from("Hello, Object Store!");
    
    store.put(&key, value.clone().into()).await?;
    println!("存储成功！");
    
    // 检索数据
    let result = store.get(&key).await?;
    let bytes = result.bytes().await?;
    println!("检索到的值：{}", String::from_utf8_lossy(&bytes));
    
    Ok(())
}
```

**完整示例**: [objectstore_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/objectstore_sample.rs)

---

## 原理解析

### object_store 特性

**object_store 是对象存储库**：

- ✅ 统一接口
- ✅ 支持多种后端
- ✅ 异步支持
- ✅ 类型安全

### 初始化存储

**使用 LocalFileSystem**：

```rust,ignore
use object_store::local::LocalFileSystem;

// 创建本地存储
let store = LocalFileSystem::new_with_prefix("/tmp/storage")?;
```

**使用内存存储**：

```rust,ignore
use object_store::memory::InMemory;

// 创建内存存储（用于测试）
let store = InMemory::new();
```

### 存储对象

**使用 put()**：

```rust,ignore
use object_store::{ObjectStore, path::Path};
use bytes::Bytes;

let key = Path::from("data/file.txt");
let value = Bytes::from("Hello, World!");

store.put(&key, value.into()).await?;
```

### 检索对象

**使用 get()**：

```rust,ignore
use object_store::ObjectStore;

let key = Path::from("data/file.txt");

// 获取对象
let result = store.get(&key).await?;

// 读取字节
let bytes = result.bytes().await?;
println!("{}", String::from_utf8_lossy(&bytes));
```

### 列出对象

**使用 list()**：

```rust,ignore
use object_store::{ObjectStore, path::Path};
use futures::StreamExt;

let prefix = Path::from("data");

// 列出所有带前缀的对象
let mut stream = store.list(Some(&prefix));

while let Some(meta) = stream.next().await {
    println!("对象：{:?}", meta?.location);
}
```

---

## 常见错误

### 错误 1: 目录不存在

```rust,ignore
let store = LocalFileSystem::new_with_prefix("/nonexistent/path");
// ❌ 目录不存在
```

**错误信息**:
```
No such file or directory
```

**修复方法**:
```rust,ignore
// 先创建目录
tokio::fs::create_dir_all("/tmp/storage").await?;
let store = LocalFileSystem::new_with_prefix("/tmp/storage")?;
```

### 错误 2: 忘记 await

```rust,ignore
let result = store.get(&key);  // ❌ 忘记 .await
let bytes = result.bytes().await?;
```

**错误信息**:
```
no method named `bytes` on type `impl Future`
```

**修复方法**:
```rust,ignore
let result = store.get(&key).await?;  // ✅ 添加 .await
```

### 错误 3: 路径错误

```rust,ignore
let key = Path::from("wrong/path/file.txt");
let result = store.get(&key).await?;
// ❌ 文件不存在
```

**错误信息**:
```
Object not found
```

**修复方法**:
```rust,ignore
let key = Path::from("data/file.txt");  // ✅ 正确路径
```

---

## 动手练习

### 练习 1: 创建存储

```rust,ignore
use object_store::local::LocalFileSystem;

#[tokio::main]
async fn main() {
    // TODO: 创建临时目录
    // TODO: 创建 LocalFileSystem 实例
    // TODO: 打印存储路径
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let temp_dir = std::env::temp_dir().join("hello");
let store = LocalFileSystem::new_with_prefix(temp_dir).unwrap();
println!("存储路径：{:?}", store);
```
</details>

### 练习 2: 存储数据

```rust,ignore
use object_store::{ObjectStore, path::Path};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let store = /* 创建存储 */;
    
    // TODO: 创建键
    // TODO: 创建值
    // TODO: 存储数据
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let key = Path::from("test.txt");
let value = Bytes::from("Hello!");
store.put(&key, value.into()).await.unwrap();
```
</details>

### 练习 3: 检索数据

```rust,ignore
use object_store::ObjectStore;

#[tokio::main]
async fn main() {
    let store = /* 创建存储 */;
    
    // TODO: 获取对象
    // TODO: 读取字节
    // TODO: 打印内容
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let key = Path::from("test.txt");
let result = store.get(&key).await.unwrap();
let bytes = result.bytes().await.unwrap();
println!("{}", String::from_utf8_lossy(&bytes));
```
</details>

---

## 故障排查 (FAQ)

### Q: object_store 支持哪些后端？

**A**: 
- LocalFileSystem（本地文件）
- InMemory（内存）
- AWS S3
- Google Cloud Storage
- Azure Blob Storage

### Q: 如何切换到云存储？

**A**: 
```rust,ignore
// AWS S3
use object_store::aws::AmazonS3Builder;

let store = AmazonS3Builder::new()
    .with_bucket_name("my-bucket")
    .with_region("us-east-1")
    .build()?;
```

### Q: 如何处理大文件？

**A**: 
```rust,ignore
// 流式上传
let mut writer = store.put_multipart(&path).await?;
writer.write(&chunk1).await?;
writer.write(&chunk2).await?;
writer.shutdown().await?;
```

---

## 知识扩展

### 删除对象

```rust,ignore
use object_store::ObjectStore;

store.delete(&Path::from("old_file.txt")).await?;
```

### 复制对象

```rust,ignore
use object_store::ObjectStore;

store.copy(
    &Path::from("source.txt"),
    &Path::from("dest.txt")
).await?;
```

### 元数据

```rust,ignore
use object_store::ObjectStore;

let meta = store.head(&Path::from("file.txt")).await?;
println!("大小：{} bytes", meta.size);
println!("最后修改：{}", meta.last_modified);
```

---

## 小结

**核心要点**：

1. **ObjectStore**: 统一接口
2. **LocalFileSystem**: 本地存储
3. **put()**: 存储对象
4. **get()**: 检索对象
5. **list()**: 列出对象

**关键术语**：

- **Object Store**: 对象存储
- **Path**: 路径
- **Put**: 存储
- **Get**: 获取

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Object Store | 对象存储 |
| Path | 路径 |
| Put | 存储 |
| Get | 获取 |
| List | 列出 |

---

## 知识检查

**快速测验**（答案在下方）：

1. 对象存储和文件系统有什么区别？

2. 常见的对象存储服务有哪些？

3. `object_store` crate 提供什么抽象？

<details>
<summary>点击查看答案与解析</summary>

1. 对象存储是扁平的（key-value），文件系统是分层的（目录树）
2. AWS S3, GCP Cloud Storage, Azure Blob Storage
3. 统一的对象存储接口，支持多种后端

**关键理解**: 对象存储适合大规模数据存储和访问。
</details>

## 继续学习

**前一章**: 资源嵌入  
**下一章**: Ollama AI 集成

**相关章节**:
- 资源嵌入
- Ollama AI 集成
- 异步编程

**返回**: 高级进阶

---

**完整示例**: [objectstore_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/objectstore_sample.rs)
