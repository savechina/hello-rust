# 资源嵌入

## 开篇故事

想象你要打包应用程序的资源文件（图片、配置文件）。传统方式是：读取文件 → 打包 → 运行时加载。include_dir 就像是：时间胶囊——在编译时就把文件打包进程序，运行时直接使用。

---

## 本章适合谁

如果你想在 Rust 程序中嵌入静态资源（配置文件、图片、数据），本章适合你。include_dir 是编译时资源嵌入的标准库。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 include_dir 概念
2. 嵌入目录到二进制
3. 访问嵌入的文件
4. 遍历目录结构
5. 处理二进制文件

---

## 前置要求

- [文件与目录操作](directory.md) - 文件路径基础
- 宏编程 - 宏基础

---

## 第一个例子

最简单的 include_dir 使用：

```rust
use include_dir::{include_dir, Dir};

// 在编译时嵌入 assets 目录
static ASSETS: Dir = include_dir!("assets");

fn main() {
    // 获取文件
    let data_file = ASSETS.get_file("data.txt").unwrap();
    
    // 读取文件内容
    let content = std::str::from_utf8(data_file.contents()).unwrap();
    println!("data.txt: {}", content);
    
    // 遍历所有文件
    for file in ASSETS.files() {
        println!("文件：{:?}", file.path());
    }
}
```

**完整示例**: [includedir_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/includedir_sample.rs)

---

## 原理解析

### include_dir 特性

**include_dir 是资源嵌入库**：

- ✅ 编译时嵌入
- ✅ 零运行时开销
- ✅ 类型安全
- ✅ 支持目录遍历

### 嵌入目录

**使用 include_dir! 宏**：

```rust
use include_dir::include_dir;

// 嵌入整个目录
static ASSETS: Dir = include_dir!("assets");
```

**编译时检查**：
- 目录必须存在
- 路径相对于 Cargo.toml

### 访问文件

**使用 get_file()**：

```rust
use include_dir::Dir;

static ASSETS: Dir = include_dir!("assets");

// 获取文件
let file = ASSETS.get_file("config.json").unwrap();

// 读取内容（字节）
let bytes = file.contents();

// 读取内容（字符串）
let content = std::str::from_utf8(bytes).unwrap();
```

### 遍历目录

**遍历文件**：

```rust
use include_dir::Dir;

static ASSETS: Dir = include_dir!("assets");

// 遍历当前目录
for file in ASSETS.files() {
    println!("文件：{:?}", file.path());
}
```

**递归遍历**：

```rust
fn traverse(dir: &Dir) {
    // 遍历文件
    for file in dir.files() {
        println!("文件：{:?}", file.path());
    }
    
    // 递归遍历子目录
    for subdir in dir.dirs() {
        traverse(subdir);
    }
}

traverse(&ASSETS);
```

### 包含字节

**使用 include_bytes!**:

```rust
let data = include_bytes!("assets/data.txt");
println!("字节：{:?}", data);

// 转换为字符串
let content = std::str::from_utf8(data).unwrap();
println!("内容：{}", content);
```

---

## 常见错误

### 错误 1: 路径错误

```rust
static ASSETS: Dir = include_dir!("wrong_path");
// ❌ 目录不存在
```

**错误信息**:
```
No such file or directory
```

**修复方法**:
```rust
static ASSETS: Dir = include_dir!("assets");  // ✅ 正确路径
```

### 错误 2: 忘记 unwrap

```rust
let file = ASSETS.get_file("data.txt");
let content = file.contents();  // ❌ file 是 Option
```

**错误信息**:
```
no method named `contents` on type `Option`
```

**修复方法**:
```rust
let file = ASSETS.get_file("data.txt").unwrap();  // ✅ 解包
```

### 错误 3: UTF-8 转换错误

```rust
let file = ASSETS.get_file("image.png").unwrap();
let content = std::str::from_utf8(file.contents()).unwrap();
// ❌ 二进制文件不是 UTF-8
```

**修复方法**:
```rust
// 二进制文件直接处理字节
let bytes = file.contents();
// 或检查是否是文本文件
if let Some(content) = file.contents_utf8() {
    println!("{}", content);
} else {
    println!("二进制文件，大小：{} bytes", file.contents().len());
}
```

---

## 动手练习

### 练习 1: 嵌入目录

```rust
use include_dir::{include_dir, Dir};

// TODO: 嵌入 "data" 目录
// static ASSETS: Dir = ...

fn main() {
    // TODO: 打印嵌入的文件数量
}
```

<details>
<summary>点击查看答案</summary>

```rust
static ASSETS: Dir = include_dir!("data");

fn main() {
    println!("嵌入了 {} 个文件", ASSETS.files().count());
}
```
</details>

### 练习 2: 读取嵌入文件

```rust
use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("assets");

fn main() {
    // TODO: 获取 "config.txt" 文件
    // TODO: 打印文件内容
}
```

<details>
<summary>点击查看答案</summary>

```rust
let file = ASSETS.get_file("config.txt").unwrap();
let content = std::str::from_utf8(file.contents()).unwrap();
println!("内容：{}", content);
```
</details>

### 练习 3: 递归遍历

```rust
use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("assets");

// TODO: 实现递归遍历函数
fn traverse(dir: &Dir) {
    // TODO: 遍历文件
    // TODO: 递归遍历子目录
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn traverse(dir: &Dir) {
    for file in dir.files() {
        println!("文件：{:?}", file.path());
    }
    
    for subdir in dir.dirs() {
        traverse(subdir);
    }
}

traverse(&ASSETS);
```
</details>

---

## 故障排查 (FAQ)

### Q: include_dir 和 include_str 有什么区别？

**A**: 
- **include_str**: 嵌入单个文本文件
- **include_bytes**: 嵌入单个二进制文件
- **include_dir**: 嵌入整个目录

### Q: 嵌入的文件会增加二进制大小吗？

**A**: 
- 会，文件内容直接嵌入二进制
- 适合小文件（配置、模板）
- 大文件考虑运行时加载

### Q: 如何更新嵌入的文件？

**A**: 
- 重新编译即可
- 文件内容变化会自动重新编译

---

## 知识扩展

### 条件编译

```rust
#[cfg(debug_assertions)]
static ASSETS: Dir = include_dir!("assets/dev");

#[cfg(not(debug_assertions))]
static ASSETS: Dir = include_dir!("assets/prod");
```

### 嵌入特定文件

```rust
use include_dir::{include_dir, Dir, File};

static CONFIG: &str = include_str!("config.json");
static LOGO: &[u8] = include_bytes!("images/logo.png");
```

### 性能优化

```rust
// 懒加载
lazy_static::lazy_static! {
    static ref ASSETS: Dir<'static> = include_dir!("assets");
}
```

---

## 小结

**核心要点**：

1. **include_dir**: 嵌入目录
2. **编译时嵌入**: 零运行时开销
3. **类型安全**: 编译时检查
4. **遍历支持**: 递归遍历目录
5. **二进制支持**: 支持文本和二进制

**关键术语**：

- **Include Dir**: 包含目录
- **Embed**: 嵌入
- **Compile-time**: 编译时
- **Runtime**: 运行时

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Include Dir | 包含目录 |
| Embed | 嵌入 |
| Compile-time | 编译时 |
| Runtime | 运行时 |
| Binary | 二进制 |

---

## 延伸阅读

学习完资源嵌入后，你可能还想了解：

- [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html) - 嵌入二进制
- [rust-embed crate](https://docs.rs/rust-embed) - 更强大的嵌入
- [Phf (完美哈希)](https://docs.rs/phf) - 编译时查找表

**选择建议**:
- 想学习系统信息 → 继续学习 [系统信息](sysinfo.md)
- 想学习临时文件 → 跳到 [临时文件](tempfile.md)

## 知识检查

**快速测验**（答案在下方）：

1. `include_str!` 和 `include_bytes!` 的区别？

2. 编译时嵌入的优缺点？

3. `rust-embed` crate 的作用？

<details>
<summary>点击查看答案与解析</summary>

1. `include_str!` 返回 `&str`，`include_bytes!` 返回 `&[u8]`
2. 优点：无需运行时加载，缺点：增加二进制大小
3. 更强大的嵌入，支持目录和 glob 模式

**关键理解**: 编译时嵌入适合小型静态资源。
</details>

## 继续学习

**前一章**: [系统信息](sysinfo.md)  
**下一章**: 对象存储

**相关章节**:
- [系统信息](sysinfo.md)
- 对象存储
- 宏编程

**返回**: 高级进阶

---

**完整示例**: [includedir_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/includedir_sample.rs)
