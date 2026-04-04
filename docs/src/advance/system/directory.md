# 文件与目录操作

## 开篇故事

想象你在整理一个巨大的仓库。传统方式是：走进仓库 → 找到物品 → 拿出来 → 走回办公室记录。每次只能处理一件物品，效率极低。

更聪明的做法是：使用智能仓库管理系统——你可以一次性列出所有物品、批量移动、按类别搜索，甚至在不同房间之间建立快捷通道。Rust 的文件与目录操作就是你的"智能仓库管理系统"——它让你高效地管理文件系统。

---

## 本章适合谁

如果你需要在 Rust 程序中读写文件、遍历目录、处理路径，本章适合你。文件系统操作是几乎所有应用程序的基础需求。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 `std::fs` 读写文件
2. 使用 `std::path::Path` 和 `PathBuf` 处理路径
3. 遍历目录树
4. 创建和删除文件/目录
5. 获取文件和目录元数据
6. 处理跨平台路径差异

---

## 前置要求

- [错误处理](../error-handling/error-handling.md) - Result 和错误传播
- [所有权](../../basic/ownership.md) - 所有权基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add home
cargo add dotenvy
```

## 第一个例子

读取文件内容：

```rust,ignore
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取整个文件到字符串
    let content = fs::read_to_string("hello.txt")?;
    println!("文件内容：\n{}", content);
    Ok(())
}
```

**发生了什么？**

- `fs::read_to_string` - 读取文件并返回 `String`
- `?` - 错误传播（文件不存在或无权限时返回错误）

---

## 原理解析

### 1. 文件系统树形结构

```
/ (根目录)
├── home/
│   └── user/
│       ├── documents/
│       │   ├── report.txt
│       │   └── notes.md
│       └── pictures/
│           └── photo.jpg
├── etc/
│   └── config.ini
└── tmp/
    └── temp_file.txt
```

### 2. 路径处理

```rust,ignore
use std::path::{Path, PathBuf};

// Path - 借用路径（不拥有所有权）
let path = Path::new("hello.txt");
println!("文件名：{}", path.file_name().unwrap().to_str().unwrap());

// PathBuf - 拥有路径（可修改）
let mut path_buf = PathBuf::from("/home");
path_buf.push("user");
path_buf.push("documents");
println!("完整路径：{}", path_buf.display());

// 推荐：使用 join 构建路径
let path = Path::new("/home")
    .join("user")
    .join("documents")
    .join("report.txt");
```

### 3. 文件读写操作

```rust,ignore
use std::fs::{self, File};
use std::io::{Read, Write};

// 读取整个文件
let content = fs::read_to_string("file.txt")?;

// 读取为字节
let bytes = fs::read("file.txt")?;

// 写入整个文件（覆盖）
fs::write("output.txt", "Hello, World!")?;

// 追加写入
let mut file = File::options()
    .create(true)
    .append(true)
    .open("log.txt")?;
file.write_all(b"New log entry\n")?;
```

### 4. 目录遍历

```rust,ignore
use std::fs;

// 读取目录内容
let entries = fs::read_dir("/home/user")?;
for entry in entries {
    let entry = entry?;
    println!("文件名：{}", entry.file_name().to_str().unwrap());
    println!("路径：{}", entry.path().display());
    
    let metadata = entry.metadata()?;
    println!("是文件：{}", metadata.is_file());
    println!("是目录：{}", metadata.is_dir());
}
```

### 5. 递归目录遍历

```rust,ignore
use std::fs;
use std::path::Path;

fn list_dir_recursive(path: &Path, prefix: &str) -> std::io::Result<()> {
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        
        println!("{}{}", prefix, name.to_str().unwrap());
        
        if path.is_dir() {
            list_dir_recursive(&path, &format!("{}  ", prefix))?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    list_dir_recursive(Path::new("."), "")
}
```

### 6. 文件和目录操作

```rust,ignore
use std::fs;

// 创建目录
fs::create_dir("new_dir")?;
fs::create_dir_all("parent/child/grandchild")?;  // 递归创建

// 删除文件
fs::remove_file("file.txt")?;

// 删除空目录
fs::remove_dir("empty_dir")?;

// 删除目录及其内容
fs::remove_dir_all("dir_with_contents")?;

// 复制文件
fs::copy("source.txt", "dest.txt")?;

// 重命名/移动
fs::rename("old_name.txt", "new_name.txt")?;
```

### 7. 文件元数据

```rust,ignore
use std::fs;

let metadata = fs::metadata("file.txt")?;

println!("文件大小：{} 字节", metadata.len());
println!("是文件：{}", metadata.is_file());
println!("是目录：{}", metadata.is_dir());

// 权限（Unix 系统）
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let perms = metadata.permissions();
    println!("权限：{:o}", perms.mode());
}
```

---

## 常见错误

### 错误 1: 路径拼接使用字符串

```rust,ignore
// ❌ 错误：跨平台不兼容
let path = format!("{}/{}", dir, filename);

// ✅ 正确：使用 Path::join
let path = Path::new(dir).join(filename);
```

### 错误 2: 不处理文件不存在

```rust,ignore
// ❌ 错误：panic 如果文件不存在
let content = fs::read_to_string("missing.txt").unwrap();

// ✅ 正确：处理错误
match fs::read_to_string("missing.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("无法读取文件：{}", e),
}
```

---

## 动手练习

### 练习 1: 统计目录中文件类型

编写程序统计目录中各种文件类型的数量：

```rust,ignore
// TODO: 实现 count_file_types 函数
// 接受一个目录路径
// 返回 HashMap<扩展名，数量>
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn count_file_types(path: &Path) -> std::io::Result<HashMap<String, usize>> {
    let mut counts = HashMap::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("no_extension")
                .to_string();
            
            *counts.entry(ext).or_insert(0) += 1;
        }
    }
    
    Ok(counts)
}
```
</details>

---

## 故障排查

### Q: 如何获取 HOME 目录？

**A**: 
```rust,ignore
// 方法 1: std::env
std::env::home_dir()

// 方法 2: home crate (推荐)
home::home_dir()
```

### Q: 如何处理大文件？

**A**: 使用 `BufReader` 和 `BufWriter` 逐行/逐块处理：
```rust,ignore
use std::io::{BufReader, BufRead};
let file = File::open("large.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    // 处理每一行
}
```

### Q: 跨平台路径分隔符？

**A**: 永远使用 `Path::join` 或 `PathBuf::push`，不要硬编码 `/` 或 `\`。

---

## 小结

**核心要点**：

1. **std::fs**: 文件和目录操作
2. **Path/PathBuf**: 路径处理
3. **read_dir**: 目录遍历
4. **metadata**: 文件信息

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| File System       | 文件系统   |
| Path              | 路径       |
| Directory         | 目录       |
| Metadata          | 元数据     |
| Recursion         | 递归       |
| Cross-platform    | 跨平台     |

---

完整示例：`src/advance/system/directory_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `Path` 和 `PathBuf` 有什么区别？

2. 为什么不应该使用字符串拼接路径？

3. `fs::read_dir` 返回什么类型？

<details>
<summary>点击查看答案与解析</summary>

1. `Path` 是借用类型，`PathBuf` 是拥有类型
2. 跨平台不兼容（Windows 用 `\`，Unix 用 `/`）
3. `Result<ReadDir, io::Error>`，迭代返回 `Result<DirEntry, io::Error>`

**关键理解**: 始终使用 `Path::join` 构建路径。
</details>

## 继续学习

- 下一步：[临时文件](tempfile.md)
- 进阶：[内存映射](memmap.md)
- 回顾：[错误处理](../error-handling/error-handling.md)

> 💡 **记住**：始终使用 Path/PathBuf 处理路径，确保跨平台兼容！
