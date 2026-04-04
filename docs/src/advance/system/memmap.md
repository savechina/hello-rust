# 内存映射文件

## 开篇故事

想象你要读一本很厚的书。传统方式是：一页一页读 → 记住内容 → 合上书。内存映射就像是：把整本书摊开在桌子上，你可以直接翻阅任何一页，不需要逐页读取。memmap 就是这样的技术，特别适合大文件处理。

---

## 本章适合谁

如果你需要高效处理大文件（数据库文件、日志文件、大数据集），本章适合你。内存映射是高性能文件处理的关键技术。

---

## 你会学到什么

完成本章后，你可以：

1. 理解内存映射概念
2. 将文件映射到内存
3. 直接访问文件内容
4. 修改内存映射内容
5. 获取系统页面大小

---

## 前置要求

- [文件与目录操作](directory.md) - 文件操作基础
- [临时文件](tempfile.md) - 临时文件处理
- 不安全代码 - unsafe 基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add memmap2
cargo add tempfile
```

## 第一个例子

最简单的内存映射：

```rust,ignore
use std::io::{Write, Seek, SeekFrom};
use std::fs::File;
use memmap2::Mmap;
use tempfile::tempfile;

fn main() -> std::io::Result<()> {
    // 创建临时文件
    let mut file = tempfile()?;
    
    // 写入数据
    write!(file, "Hello World!")?;
    
    // 重置文件指针
    file.seek(SeekFrom::Start(0))?;
    
    // 映射文件到内存
    let mmap = unsafe { Mmap::map(&file)? };
    
    // 直接访问内存中的数据
    assert_eq!(b"Hello World!", &mmap[..]);
    
    println!("映射内容：{}", String::from_utf8_lossy(&mmap));
    
    Ok(())
}
```

**完整示例**: [memmap_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/memmap_sample.rs)

---

## 原理解析

### 什么是内存映射？

**传统文件读取**：
```
文件 → 系统调用 → 内核缓冲区 → 用户缓冲区 → 程序
      (慢)          (复制)        (复制)
```

**内存映射**：
```
文件 → 内存映射 → 程序直接访问
      (映射一次，后续直接访问)
```

### memmap2 特性

**memmap2 是内存映射库**：

- ✅ 零拷贝读取
- ✅ 高性能
- ✅ 支持读写映射
- ✅ 跨平台支持

### 只读映射

**使用 Mmap::map()**：

```rust,ignore
use memmap2::Mmap;
use std::fs::File;

let file = File::open("data.txt")?;
let mmap = unsafe { Mmap::map(&file)? };

// 直接访问
println!("内容：{}", String::from_utf8_lossy(&mmap));
```

### 可写映射

**使用 MmapMut**：

```rust,ignore
use memmap2::MmapMut;
use std::fs::File;

let file = File::create("data.txt")?;
file.set_len(4096)?;  // 设置文件大小

let mut mmap = unsafe { MmapMut::map_mut(&file)? };

// 写入数据
mmap[..12].copy_from_slice(b"Hello World!");

// 刷新到磁盘
mmap.flush()?;
```

### 使映射可写

**使用 make_mut()**：

```rust,ignore
use memmap2::Mmap;

let mmap = unsafe { Mmap::map(&file)? };

// 使映射可写（会创建私有副本）
let mut mmap_mut = mmap.make_mut()?;

// 修改内容
mmap_mut[..5].copy_from_slice(b"Hello");
```

### 获取系统页面大小

**使用 page_size::get()**：

```rust,ignore
use page_size;

let page_size = page_size::get();
println!("系统页面大小：{} bytes", page_size);

// 通常是 4096 bytes (4KB)
```

### 性能优势

**大文件处理**：

```rust,ignore
use memmap2::Mmap;
use std::fs::File;

// 打开大文件（例如 1GB）
let file = File::open("large_data.bin")?;

// 内存映射（几乎瞬间完成）
let mmap = unsafe { Mmap::map(&file)? };

// 随机访问任意位置（无需读取整个文件）
let offset = 1024 * 1024;  // 1MB 处
println!("数据：{:?}", &mmap[offset..offset+100]);
```

---

## 常见错误

### 错误 1: 忘记设置文件大小

```rust,ignore
use memmap2::MmapMut;
use std::fs::File;

let file = File::create("data.txt")?;
let mmap = unsafe { MmapMut::map_mut(&file)? };
// ❌ 文件为空，无法映射
```

**错误信息**:
```
Invalid argument (os error 22)
```

**修复方法**:
```rust,ignore
file.set_len(4096)?;  // ✅ 先设置文件大小
let mmap = unsafe { MmapMut::map_mut(&file)? };
```

### 错误 2: 忘记使用 unsafe

```rust,ignore
let mmap = Mmap::map(&file)?;  // ❌ 缺少 unsafe
```

**错误信息**:
```
unsafe fn `map` requires unsafe function or block
```

**修复方法**:
```rust,ignore
let mmap = unsafe { Mmap::map(&file)? };  // ✅ 使用 unsafe 块
```

### 错误 3: 访问已释放的映射

```rust,ignore
fn create_mmap() -> Mmap {
    let file = File::open("data.txt").unwrap();
    unsafe { Mmap::map(&file).unwrap() }  // ❌ file 已被释放
}
```

**修复方法**:
```rust,ignore
fn keep_mmap() -> std::io::Result<Mmap> {
    let file = File::open("data.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(mmap)  // ✅ file 和 mmap 一起返回
}
```

---

## 动手练习

### 练习 1: 创建只读映射

```rust,ignore
use memmap2::Mmap;
use std::fs::File;

fn main() -> std::io::Result<()> {
    // TODO: 打开文件
    // TODO: 创建内存映射
    // TODO: 打印前 100 字节
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let file = File::open("data.txt")?;
let mmap = unsafe { Mmap::map(&file)? };

println!("前 100 字节：{:?}", &mmap[..100.min(mmap.len())]);
```
</details>

### 练习 2: 创建可写映射

```rust,ignore
use memmap2::MmapMut;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("test.bin")?;
    
    // TODO: 设置文件大小为 4096
    // TODO: 创建可写映射
    // TODO: 写入 "Hello Memory Map!"
    // TODO: 刷新到磁盘
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
file.set_len(4096)?;

let mut mmap = unsafe { MmapMut::map_mut(&file)? };
mmap[..17].copy_from_slice(b"Hello Memory Map!");
mmap.flush()?;
```
</details>

### 练习 3: 获取系统页面大小

```rust,ignore
use page_size;

fn main() {
    // TODO: 获取系统页面大小
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
let page_size = page_size::get();
println!("系统页面大小：{} bytes", page_size);
```
</details>

---

## 故障排查 (FAQ)

### Q: 内存映射和普通读取有什么区别？

**A**: 
- **普通读取**: 需要系统调用，数据复制到用户空间
- **内存映射**: 一次映射，后续直接访问内存
- **性能**: 内存映射在大文件场景性能更优

### Q: 内存映射安全吗？

**A**: 
- **只读映射**: 安全，不会修改原文件
- **可写映射**: 需要 unsafe，修改会反映到文件
- **注意**: 确保文件在映射期间不被删除

### Q: 如何处理超大文件？

**A**: 
```rust,ignore
use memmap2::Mmap;

// 映射整个文件（可能很大）
let mmap = unsafe { Mmap::map(&file)? };

// 只访问需要的部分
let chunk = &mmap[offset..offset+size];
```

---

## 知识扩展

### 异步内存映射

```rust,ignore
use tokio::fs::File;
use memmap2::Mmap;

// 注意：memmap2 是同步的
// 在 tokio 中使用 spawn_blocking
let mmap = tokio::task::spawn_blocking(move || {
    unsafe { Mmap::map(&file) }
}).await??;
```

### 匿名映射

```rust,ignore
use memmap2::MmapMut;

// 创建不关联文件的内存映射
let mmap = unsafe { MmapMut::map_anon(4096)? };
```

### 性能对比

```rust,ignore
// 传统读取
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;

// 内存映射（更快）
let mmap = unsafe { Mmap::map(&file)? };
```

---

## 小结

**核心要点**：

1. **内存映射**: 将文件映射到内存地址空间
2. **Mmap**: 只读内存映射
3. **MmapMut**: 可写内存映射
4. **零拷贝**: 直接访问文件内容
5. **性能优势**: 大文件处理更高效

**关键术语**：

- **Memory Map (内存映射)**: 文件到内存的映射
- **Mmap**: 只读映射类型
- **MmapMut**: 可写映射类型
- **Zero-copy (零拷贝)**: 无需数据复制

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Memory Map | 内存映射 |
| Mmap | 内存映射（只读） |
| MmapMut | 内存映射（可写） |
| Zero-copy | 零拷贝 |
| Page Size | 页面大小 |

---

## 知识检查

**快速测验**（答案在下方）：

1. 内存映射和普通文件读写有什么区别？

2. 什么时候应该使用内存映射？

3. `MmapMut` 和 `Mmap` 的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 内存映射将文件映射到内存，直接访问无需系统调用
2. 大文件随机访问、进程间共享内存
3. `MmapMut` 可写，`Mmap` 只读

**关键理解**: 内存映射适合大文件，小文件用普通 I/O 更简单。
</details>

## 继续学习

**前一章**: [临时文件](tempfile.md)  
**下一章**: [环境变量](dotenv.md)

**相关章节**:
- [临时文件](tempfile.md)
- [文件与目录操作](directory.md)
- 不安全代码

**返回**: 高级进阶

---

**完整示例**: [memmap_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/memmap_sample.rs)
