# 临时文件处理

## 开篇故事

想象你在写一份草稿，需要临时保存一下。你不会把它放进永久文件夹，而是放在桌面的临时区域，用完就清理。tempfile 库就像这个临时区域——帮你创建临时文件和目录，用完自动清理。

---

## 本章适合谁

如果你需要在 Rust 程序中创建临时文件（缓存数据、中间结果、测试数据），本章适合你。临时文件是处理大量数据或测试的常用技术。

---

## 你会学到什么

完成本章后，你可以：

1. 创建临时文件
2. 创建临时目录
3. 读写临时文件
4. 自动清理临时资源
5. 处理临时文件错误

---

## 前置要求

- [文件与目录操作](directory.md) - 文件操作基础
- 错误处理 - 错误处理基础
- [环境变量](dotenv.md) - 环境变量（可选）

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tempfile
```

## 第一个例子

最简单的临时文件创建：

```rust,ignore
use std::io::{Write, Read, Seek, SeekFrom};
use tempfile::tempfile;

fn main() -> std::io::Result<()> {
    // 创建临时文件
    let mut tmpfile = tempfile()?;
    
    // 写入数据
    write!(tmpfile, "Hello World!")?;
    
    // 重置文件指针
    tmpfile.seek(SeekFrom::Start(0))?;
    
    // 读取数据
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf)?;
    
    assert_eq!("Hello World!", buf);
    
    // 临时文件在离开作用域时自动删除
    Ok(())
}
```

**完整示例**: [tempfile_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/tempfile_sample.rs)

---

## 原理解析

### tempfile 特性

**tempfile 是一个临时文件处理库**：

- ✅ 自动清理临时文件
- ✅ 跨平台支持
- ✅ 安全创建（防竞争条件）
- ✅ 临时目录支持

### 创建临时文件

**使用 tempfile()**：

```rust,ignore
use tempfile::tempfile;

let mut file = tempfile()?;
// 使用临时文件
// 离开作用域时自动删除
```

**使用 NamedTempFile**：

```rust,ignore
use tempfile::NamedTempFile;

let file = NamedTempFile::new()?;
// 可以获取文件路径
let path = file.path();
println!("临时文件路径：{}", path.display());
```

### 创建临时目录

**使用 tempdir()**：

```rust,ignore
use tempfile::tempdir;

let dir = tempdir()?;
let file_path = dir.path().join("test.txt");

// 在临时目录中创建文件
std::fs::write(&file_path, "Hello!")?;

// 目录和文件在离开作用域时自动删除
drop(dir);
```

### 在指定目录创建

**使用 new_in()**：

```rust,ignore
use tempfile::NamedTempFile;
use std::env;

let home_dir = env::home_dir().unwrap();
let file = NamedTempFile::new_in(&home_dir)?;

println!("在 home 目录创建：{:?}", file);
```

### 重新打开临时文件

**使用 reopen()**：

```rust,ignore
use tempfile::NamedTempFile;
use std::io::{Write, Read};

let text = "Hello World!";

let mut file1 = NamedTempFile::new()?;

// 写入数据
file1.write_all(text.as_bytes())?;

// 重新打开（获得新的文件句柄）
let mut file2 = file1.reopen()?;

// 从第二个句柄读取
let mut buf = String::new();
file2.read_to_string(&mut buf)?;

assert_eq!(buf, text);
```

### 显式清理

**使用 close()**：

```rust,ignore
use tempfile::tempdir;

let dir = tempdir()?;

// ... 使用临时目录 ...

// 显式清理并检查是否成功
dir.close().unwrap();
```

---

## 常见错误

### 错误 1: 临时文件权限问题

```rust,ignore
let file = NamedTempFile::new_in("/root")?;
// ❌ 如果没有权限，会失败
```

**错误信息**:
```
Permission denied (os error 13)
```

**修复方法**:
```rust,ignore
// 使用有权限的目录
let file = NamedTempFile::new_in("/tmp")?;
```

### 错误 2: 忘记处理错误

```rust,ignore
let file = NamedTempFile::new();  // ❌ 返回 Result，需要处理
```

**错误信息**:
```
unused `Result` that must be used
```

**修复方法**:
```rust,ignore
let file = NamedTempFile::new()?;  // ✅ 使用 ? 处理错误
```

### 错误 3: 临时文件过早删除

```rust,ignore
fn create_temp() -> std::path::PathBuf {
    let file = NamedTempFile::new().unwrap();
    file.path().to_path_buf()  // ❌ 文件已被删除
}
```

**修复方法**:
```rust,ignore
fn keep_temp() -> tempfile::NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    file  // ✅ 返回文件，保持存活
}
```

---

## 动手练习

### 练习 1: 创建临时文件

```rust,ignore
use tempfile::tempfile;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    // TODO: 创建临时文件
    // TODO: 写入 "Hello Tempfile!"
    // TODO: 读取并打印内容
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let mut tmpfile = tempfile()?;
write!(tmpfile, "Hello Tempfile!")?;

tmpfile.seek(SeekFrom::Start(0))?;

let mut buf = String::new();
tmpfile.read_to_string(&mut buf)?;

println!("{}", buf);
```
</details>

### 练习 2: 创建临时目录

```rust,ignore
use tempfile::tempdir;
use std::fs;

fn main() -> std::io::Result<()> {
    // TODO: 创建临时目录
    // TODO: 在目录中创建文件
    // TODO: 写入内容并读取
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let dir = tempdir()?;
let file_path = dir.path().join("test.txt");

fs::write(&file_path, "Hello from tempdir!")?;

let content = fs::read_to_string(&file_path)?;
println!("{}", content);

dir.close().unwrap();
```
</details>

### 练习 3: 重新打开临时文件

```rust,ignore
use tempfile::NamedTempFile;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    let text = "Test data";
    
    // TODO: 创建命名临时文件
    // TODO: 写入数据
    // TODO: 重新打开并读取
    
    let mut file1 = NamedTempFile::new()?;
    // ... 完成练习
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let mut file1 = NamedTempFile::new()?;
file1.write_all(text.as_bytes())?;

let mut file2 = file1.reopen()?;

let mut buf = String::new();
file2.read_to_string(&mut buf)?;

assert_eq!(buf, text);
```
</details>

---

## 故障排查 (FAQ)

### Q: 临时文件什么时候被删除？

**A**: 
- **tempfile()**: 文件句柄关闭时自动删除
- **NamedTempFile**: 离开作用域或调用 close() 时删除
- **tempdir**: 离开作用域或调用 close() 时删除

### Q: 如何保持临时文件不删除？

**A**: 
```rust,ignore
use tempfile::NamedTempFile;
use std::fs::File;

let temp = NamedTempFile::new()?;

// 转换为永久文件
let path = temp.path().to_path_buf();
temp.persist(&path)?;  // 不删除
```

### Q: 如何指定临时文件扩展名？

**A**: 
```rust,ignore
use tempfile::NamedTempFile;

let file = NamedTempFile::new()
    .unwrap()
    .keep()  // 保持文件
    .1;  // 获取路径
```

---

## 知识扩展

### 临时文件与进程

```rust,ignore
use tempfile::tempfile;

// 临时文件对当前进程可见
let file1 = tempfile()?;
let file2 = tempfile()?;

// 每个临时文件都是独立的
```

### 临时目录嵌套

```rust,ignore
use tempfile::tempdir;

let parent = tempdir()?;
let child_path = parent.path().join("child");

std::fs::create_dir(&child_path)?;

// 删除父目录时，子目录也会被删除
drop(parent);
```

### 性能优化

```rust,ignore
use tempfile::tempfile_in;
use std::env;

// 在 RAM disk 创建（如果可用）
let tmp_dir = env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
let file = tempfile_in(&tmp_dir)?;
```

---

## 小结

**核心要点**：

1. **tempfile**: 自动清理临时文件
2. **NamedTempFile**: 可获取路径的临时文件
3. **tempdir**: 临时目录支持
4. **自动清理**: 离开作用域自动删除
5. **reopen()**: 重新打开临时文件

**关键术语**：

- **Temporary File (临时文件)**: 临时存储数据的文件
- **NamedTempFile**: 命名临时文件
- **TempDir**: 临时目录
- **Auto-cleanup**: 自动清理

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Temporary File | 临时文件 |
| NamedTempFile | 命名临时文件 |
| TempDir | 临时目录 |
| Auto-cleanup | 自动清理 |
| Scope | 作用域 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `tempfile()` 和 `NamedTempFile` 有什么区别？

2. 临时文件什么时候被删除？

3. `TempDir` 的作用是什么？

<details>
<summary>点击查看答案与解析</summary>

1. `tempfile()` 无名称（自动删除），`NamedTempFile` 有路径（可持久化）
2. `TempFile` 在 drop 时自动删除，`NamedTempFile` 可调用 `persist()` 保留
3. `TempDir` 创建临时目录，drop 时删除目录及其内容

**关键理解**: 临时文件是测试和临时数据存储的理想选择。
</details>

## 继续学习

**前一章**: [内存映射](memmap.md)  
**下一章**: [环境变量](dotenv.md)

**相关章节**:
- [文件与目录操作](directory.md)
- [内存映射](memmap.md)
- [环境变量](dotenv.md)

**返回**: 高级进阶

---

**完整示例**: [tempfile_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/tempfile_sample.rs)
