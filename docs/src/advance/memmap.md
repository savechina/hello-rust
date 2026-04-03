# 内存映射文件 (Memory-Mapped Files)

## 开篇故事

想象你需要处理一个 10GB 的日志文件。传统做法是：`File::open()` -> `read()` 到缓冲区 -> 处理 -> 重复。这意味着：
- 10GB 数据需要从磁盘复制到内核缓冲区，再复制到用户空间
- 你的程序需要分配大量内存来存储这些数据
- 频繁的系统调用带来巨大开销

这就是**内存映射文件**要解决的问题。就像图书馆的目录卡片直接指向书架上的书籍，内存映射让应用程序可以直接访问文件内容，无需繁琐的拷贝操作。操作系统会按需将文件页面加载到内存，只有实际访问的数据才会占用物理内存。

---

## 本章适合谁

如果你正在开发需要高效处理大文件的应用（数据库、日志分析、媒体处理），或者希望理解操作系统如何优化文件 I/O，本章适合你。内存映射是现代高性能应用的基础技术，也是理解虚拟内存的关键窗口。

---

## 你会学到什么

完成本章后，你可以：

1. 解释什么是内存映射文件以及它的工作原理
2. 使用 `memmap2` crate 创建可读写的内存映射
3. 理解虚拟内存、页面 (page) 和页表的概念
4. 比较内存映射与传统 I/O 的性能差异
5. 识别内存映射的安全风险和最佳实践

---

## 前置要求

学习本章前，你需要理解：

- [了解所有权](../basic/ownership.md) - 所有权和生命周期概念
- [指针与不安全代码](../basic/pointer.md) - 原始指针和 unsafe 基础
- [文件 I/O 基础](../basic/file_io.md) - 基本的文件操作

---

## 第一个例子

让我们看一个简单的内存映射示例：

```rust
use memmap2::Mmap;
use std::fs::File;

// 打开文件并创建内存映射
let file = File::open("data.txt")?;

// 将文件映射到内存（unsafe 操作）
let mmap = unsafe { Mmap::map(&file)? };

// 直接像访问字节数组一样访问文件内容
assert_eq!(&mmap[..], b"Hello World!");

// 不需要手动关闭或释放，Drop trait 会自动处理
```

**发生了什么？**

`Mmap::map()` 创建了一个从文件到虚拟内存地址空间的映射。从此刻起，访问 `mmap[0]` 就像访问普通数组一样，但底层数据实际存储在文件中。操作系统负责在访问时将文件页面加载到物理内存。

---

## 原理解析

### 什么是内存映射

内存映射文件 (Memory-Mapped File) 是一种将文件内容映射到进程虚拟地址空间的技术：

```
┌─────────────────────────────────────────────────────────────┐
│                    内存映射工作原理                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   进程虚拟地址空间              物理内存        磁盘文件     │
│   ┌──────────────┐             ┌──────────┐   ┌──────────┐  │
│   │              │             │          │   │          │  │
│   │  0x1000_0000 │────────────→│  Page 1  │   │ data.bin │  │
│   │  (mmap 起始) │   页表映射   │  "Hello" │ ←─┤  "Hello" │  │
│   │              │             │          │   │  World!  │  │
│   │  0x1000_1000 │────────────→│  Page 2  │ ←─┤          │  │
│   │              │             │  "World" │   │          │  │
│   │              │             │          │   └──────────┘  │
│   └──────────────┘             └──────────┘                 │
│                                                             │
│   访问 mmap[0..6] 实际上读取内存中的 "Hello"                 │
│   如果页面不在内存，操作系统自动从磁盘加载（缺页中断）         │
└─────────────────────────────────────────────────────────────┘
```

### 虚拟内存与页面

操作系统使用**虚拟内存**机制管理进程的地址空间：

```
┌──────────────────────────────────────────────────────────┐
│                   虚拟地址空间                            │
│  ┌─────────────────┐                                    │
│  │    内核空间     │  (高地址，用户程序不可访问)          │
│  ├─────────────────┤                                    │
│  │    栈 (Stack)   │  ↓ 向下增长                         │
│  │                 │                                    │
│  ├─────────────────┤                                    │
│  │    堆 (Heap)    │  ↑ 向上增长                         │
│  │                 │                                    │
│  ├─────────────────┤                                    │
│  │   BSS/Data 段   │  全局变量、静态变量                  │
│  ├─────────────────┤                                    │
│  │   代码 (Text)   │  程序指令                            │
│  ├─────────────────┤                                    │
│  │  内存映射区域   │  ← mmap 在这里 (通常从低地址开始)    │
│  │  (Memory Map)   │                                    │
│  └─────────────────┘                                    │
│                    (低地址)                              │
└──────────────────────────────────────────────────────────┘

页面大小 (Page Size)：通常是 4KB (4096 字节)
- x86/x64 Linux: 4KB 或 2MB (大页面)
- ARM64: 4KB, 16KB, 或 64KB
```

### Mmap vs 传统 I/O

| 特性 | 传统 I/O (read/write) | 内存映射 (mmap) |
|------|----------------------|-----------------|
| 数据拷贝 | 内核空间 → 用户空间 | 无拷贝，直接访问 |
| 系统调用 | 每次读写都需要 | 仅在映射/解除映射时 |
| 随机访问 | 需要 seek | 直接数组索引 |
| 大文件处理 | 需要分块读取 | 映射整个文件 |
| 内存占用 | 显式分配缓冲区 | 依赖操作系统缓存 |
| 适用场景 | 小文件、流式处理 | 大文件、频繁随机访问 |

### memmap2 crate 的核心类型

```rust
// Mmap - 只读内存映射
pub struct Mmap {
    // 内部使用原始指针和长度
}

// MmapMut - 可写内存映射
pub struct MmapMut {
    // 支持修改映射的内存
}

// 关键方法：
impl Mmap {
    // 创建只读映射（unsafe：需要确保文件在映射期间保持打开）
    pub unsafe fn map(file: &File) -> Result<Mmap>;
    
    // 转换为可变映射（如果文件以写方式打开）
    pub fn make_mut(self) -> Result<MmapMut>;
}

impl MmapMut {
    // 创建匿名映射（不关联文件，类似 malloc）
    pub fn map_anon(size: usize) -> Result<MmapMut>;
    
    // 创建文件的可写映射
    pub unsafe fn map_mut(file: &File) -> Result<MmapMut>;
    
    // 刷新修改到磁盘
    pub fn flush(&self) -> Result<()>;
}
```

---

## 常见错误

### 错误 1: 文件在映射期间被关闭

```rust
let file = File::open("data.txt")?;
let mmap = unsafe { Mmap::map(&file)? };

// ❌ 错误！file 在这里被 drop，但 mmap 仍在使用它
// file 必须保持打开状态直到 mmap 被 drop
drop(file);

// 访问 mmap 现在是未定义行为！
println!("{:?}", &mmap[0]);
```

**修复方法**：

```rust
let file = File::open("data.txt")?;
let mmap = unsafe { Mmap::map(&file)? };

// ✅ 在 mmap 使用期间保持文件打开
println!("{:?}", &mmap[0]);

// mmap 先 drop，file 后 drop（Rust 自动处理）
```

---

### 错误 2: 忘记 unsafe 块

```rust
// ❌ 编译错误！Mmap::map 是 unsafe 函数
let mmap = Mmap::map(&file)?;
```

**修复方法**：

```rust
// ✅ 显式标记 unsafe
let mmap = unsafe { Mmap::map(&file)? };

// 注意：unsafe 块内的调用是安全的，
// 因为 Mmap 实现了 Drop trait 正确清理资源
```

---

### 错误 3: 修改只读映射

```rust
let file = File::open("readonly.txt")?;
let mmap = unsafe { Mmap::map(&file)? };

// ❌ 编译错误！Mmap 不支持索引赋值
mmap[0] = b'X';
```

**修复方法**：

```rust
// 方案 1: 使用 make_mut() 转换
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("writable.txt")?;
    
let mmap = unsafe { Mmap::map(&file)? };
let mut mmap_mut = mmap.make_mut()?;
mmap_mut[0] = b'X'; // ✅ 现在可以修改

// 方案 2: 直接使用 MmapMut
let mmap_mut = unsafe { MmapMut::map_mut(&file)? };
mmap_mut[0] = b'X'; // ✅
```

---

### 错误 4: 忽略对齐要求

```rust
// 内存映射通常需要页面对齐的偏移量
let mmap = unsafe { 
    MmapOptions::new()
        .offset(100) // ❌ 可能不是页面大小的倍数
        .map(&file)?
};
```

**修复方法**：

```rust
use page_size;

let page_size = page_size::get(); // 通常是 4096
let aligned_offset = (100 / page_size) * page_size; // 对齐到页面边界

let mmap = unsafe {
    MmapOptions::new()
        .offset(aligned_offset)
        .map(&file)?
};
```

---

## 动手练习

### 练习 1: 理解内存映射生命周期

下面的代码有什么问题？如何修复？

```rust
fn read_first_byte(path: &str) -> u8 {
    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    mmap[0] // 返回第一个字节
} // file 和 mmap 在这里被 drop

fn main() {
    let byte = read_first_byte("data.txt");
    println!("First byte: {}", byte);
}
```

<details>
<summary>点击查看答案</summary>

**问题**：这个代码实际上**没有**问题！因为 `mmap[0]` 返回的是**复制的值**（u8），而不是引用。

但如果改为返回引用：

```rust
fn get_first_byte_ref(path: &str) -> &u8 {
    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    &mmap[0] // ❌ 返回局部变量的引用
}
```

这就违反了借用规则。

**正确做法**：
```rust
fn process_file<F: FnOnce(&Mmap)>(path: &str, f: F) {
    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    f(&mmap); // 在闭包中使用 mmap
    // file 和 mmap 在这里正确 drop
}
```

</details>

---

### 练习 2: 实现文件搜索

使用内存映射实现一个函数，在文件中查找特定字节序列：

```rust
fn find_in_file(path: &str, pattern: &[u8]) -> Option<usize> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    
    // 实现：返回 pattern 在文件中第一次出现的索引
    // 如果未找到返回 None
    todo!()
}

fn main() {
    match find_in_file("data.txt", b"Hello") {
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn find_in_file(path: &str, pattern: &[u8]) -> Option<usize> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    
    // 使用 windows 查找模式
    mmap.windows(pattern.len())
        .position(|window| window == pattern)
}

// 或者使用 memchr crate 进行更高效的查找
use memchr::memmem;

fn find_in_file_fast(path: &str, pattern: &[u8]) -> Option<usize> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    
    memmem::find(&mmap, pattern).map(|pos| pos)
}
```

</details>

---

### 练习 3: 页面大小计算

获取系统页面大小并计算文件需要的页面数：

```rust
fn calculate_pages_needed(file_size: usize) -> usize {
    let page_size = page_size::get();
    // 计算需要的页面数（向上取整）
    todo!()
}

fn main() {
    let size = 10000; // 10KB
    println!("File size: {} bytes", size);
    println!("Page size: {} bytes", page_size::get());
    println!("Pages needed: {}", calculate_pages_needed(size));
}
```

<details>
<summary>点击查看答案</summary>

```rust
fn calculate_pages_needed(file_size: usize) -> usize {
    let page_size = page_size::get();
    (file_size + page_size - 1) / page_size // 向上取整除法
}

// 示例输出：
// File size: 10000 bytes
// Page size: 4096 bytes
// Pages needed: 3 (4096 * 3 = 12288 >= 10000)
```

</details>

---

## 故障排查 (FAQ)

### Q: 什么时候应该用内存映射而不是 read/write？

**A**: 使用建议：

**用内存映射当**：
- 文件很大（> 几十MB），无法或不应一次性读入内存
- 需要频繁随机访问文件的不同位置
- 多个进程需要共享只读数据
- 实现数据库、缓存或类似结构

**用传统 I/O 当**：
- 文件很小，可以完全读入内存
- 纯粹的顺序访问（流式处理）
- 需要精确的 I/O 控制（如直接 I/O）
- 对安全性要求极高，避免 unsafe

---

### Q: 内存映射文件什么时候写入磁盘？

**A**: 这取决于操作系统的页面调度策略：

1. **自动回写**：修改后的页面不会立即写入，而是由内核定期 flush
2. **显式 flush**：可以调用 `mmap.flush()` 强制同步到磁盘
3. **msync 系统调用**：底层使用 `msync()` 控制同步行为

```rust
let mut mmap = unsafe { MmapMut::map_mut(&file)? };
mmap[0] = b'X';

// 确保修改写入磁盘
mmap.flush()?;
```

---

### Q: 内存映射会占用多少物理内存？

**A**: **比你想象的要少**！

- **虚拟内存**：映射整个文件大小（如 1GB 文件映射 1GB 虚拟地址）
- **物理内存**：只占用实际访问的页面

```
映射 1GB 文件：

虚拟地址空间：████████████████████ (1GB)
                    ↓ 按需加载
物理内存占用：    ░░░░░░░░ (可能只有几 MB)
                    ↑ 访问过的页面才会占用
```

操作系统使用**按需分页 (Demand Paging)**：
- 首次访问某个页面时，产生**缺页中断 (Page Fault)**
- 操作系统从磁盘加载该页面到物理内存
- 后续访问直接从内存读取

---

### Q: mmap 和 unsafe 有什么关系？

**A**: `Mmap::map()` 标记为 unsafe 的主要原因是：

1. **文件必须保持打开**：如果文件在映射期间关闭，访问映射内存会导致未定义行为
2. **底层原始指针操作**：内部涉及操作系统底层接口
3. **资源生命周期**：Rust 编译器无法静态验证所有安全条件

但实际使用中是**安全的**，因为：
- `Mmap` 实现了 `Drop`，确保正确解除映射
- 类型系统保证读写权限（`Mmap` 只读，`MmapMut` 可写）
- 只要文件保持打开，内存访问是安全的

---

## 知识扩展 (选学)

### 匿名内存映射

创建不与文件关联的内存映射（类似 `malloc`，但使用页面粒度）：

```rust
use memmap2::MmapMut;

// 创建 4MB 匿名映射
let mut mmap = MmapMut::map_anon(4 * 1024 * 1024)?;

// 像使用 Vec<u8> 一样使用
mmap[0] = 1;
mmap[1] = 2;

// 用于进程间通信 (IPC)
```

### 大页面 (Huge Pages)

Linux 支持 2MB 甚至 1GB 的大页面：

```rust
use memmap2::{MmapOptions, Mmap};

// 请求大页面（需要系统配置支持）
let mmap = unsafe {
    MmapOptions::new()
        .huge(Some(2048)) // 2MB 大页面
        .map(&file)?
};

// 优势：
// - 减少 TLB (Translation Lookaside Buffer) 未命中
// - 提高大内存工作集的性能
```

### 内存映射 vs 共享内存

```
┌──────────────────────────────────────────────────────────┐
│              共享内存 (Shared Memory)                     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  进程 A                    物理内存           进程 B     │
│  ┌──────────┐             ┌──────────┐      ┌──────────┐ │
│  │ mmap A   │────────────→│ Shared   │←─────│ mmap B   │ │
│  │ (同一块) │             │ Memory   │      │ (同一块) │ │
│  └──────────┘             └──────────┘      └──────────┘ │
│                                                          │
│  特性：                                                  │
│  - 多个进程映射同一块物理内存                            │
│  - 最快的 IPC 方式（无需内核拷贝）                        │
│  - 需要同步机制（Mutex, 信号量）                          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### 内存映射数据库

许多高性能数据库使用内存映射：

```rust
// 简化的 KV 存储示例
struct MmapKVStore {
    mmap: MmapMut,
    index: HashMap<String, usize>, // 键 -> 偏移量
}

impl MmapKVStore {
    fn get(&self, key: &str) -> Option<&[u8]> {
        let offset = self.index.get(key)?;
        // 直接从 mmap 读取值
        Some(&self.mmap[*offset..])
    }
}
```

---

## 小结

**核心要点**：

1. **内存映射文件**将文件内容直接映射到进程地址空间，避免数据拷贝
2. **虚拟内存**机制允许映射大文件而不占用等量物理内存（按需加载）
3. **`Mmap`** 是只读映射，`MmapMut` 支持修改，转换使用 `make_mut()`
4. **unsafe 是必要的**，因为需要保证文件在映射期间保持打开
5. **页面是基本单位**（通常 4KB），映射偏移需要页面对齐

**关键术语**：

- **Memory-Mapped File (内存映射文件)**: 将文件映射到内存地址空间的技术
- **Virtual Memory (虚拟内存)**: 进程看到的抽象地址空间
- **Page (页面)**: 内存管理的基本单位（通常 4KB）
- **Page Fault (缺页中断)**: 访问未加载页面时触发的操作系统中断
- **Demand Paging (按需分页)**: 仅在访问时加载页面的策略
- **Mmap / MmapMut**: memmap2 crate 的只读/可写映射类型

**下一步**：

- 学习 [并发异步编程](async.md) - 在异步应用中使用内存映射
- 理解 [数据库优化](../awesome/database.md) - 数据库中的内存映射技术

---

## 术语表

| English | 中文 |
|---------|------|
| Memory-Mapped File | 内存映射文件 |
| Virtual Memory | 虚拟内存 |
| Physical Memory | 物理内存 |
| Page | 页面/页 |
| Page Size | 页面大小 |
| Page Fault | 缺页中断/页错误 |
| Page Table | 页表 |
| Demand Paging | 按需分页 |
| Mmap | 内存映射（只读） |
| MmapMut | 可变内存映射 |
| Offset | 偏移量 |
| Anonymous Mapping | 匿名映射 |
| Shared Memory | 共享内存 |
| TLB | 转换检测缓冲区 |

完整示例：`src/advance/memmap_sample.rs`

---

## 继续学习

- 下一步：[并发异步编程](async.md)
- 进阶：[数据库优化](../awesome/database.md)
- 回顾：[指针与不安全代码](../basic/pointer.md)

> 💡 **记住**：内存映射是高性能文件 I/O 的秘密武器。当你需要处理大文件或频繁随机访问时，mmap 往往是最佳选择！

---

## 内存布局可视化

### 1. 传统 I/O vs 内存映射

```
传统 read() 方式（两次拷贝）：
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Disk    │───→│  Kernel  │───→│  User    │───→│  Process │
│  File    │    │  Buffer  │    │  Buffer  │    │  Access  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
     ↓               ↓               ↓
   磁盘读取      DMA 拷贝        CPU 拷贝
   (~ms)        (~μs)           (~μs)

内存映射方式（零拷贝）：
┌──────────┐    ┌──────────┐    ┌──────────┐
│  Disk    │───→│  Kernel  │←───│  Process │
│  File    │    │  Page    │    │  Direct  │
└──────────┘    │  Cache   │    │  Access  │
                └──────────┘    └──────────┘
                     ↑
              按需加载到内存
              无需用户空间拷贝
```

### 2. 页面映射细节

```
文件 (10000 bytes) → 需要 3 个页面 (4096 bytes/page)

文件内容：
┌─────────────────┬─────────────────┬─────────────────┐
│   Page 0        │   Page 1        │   Page 2        │
│   [0..4095]     │   [4096..8191]  │   [8192..10000] │
│   "Hello World" │   "...data..."  │   "...end"      │
│                 │                 │   (部分使用)    │
└─────────────────┴─────────────────┴─────────────────┘

虚拟内存映射：
┌──────────────────────────────────────────────────────┐
│  Virtual Address   │   Physical Page    │  Status   │
├────────────────────┼────────────────────┼───────────┤
│  0x1000_0000       │   Disk only        │  Not Load │ ← Page 0
│  0x1000_1000       │   0x7f8a_4000      │  In RAM   │ ← Page 1 (已访问)
│  0x1000_2000       │   Disk only        │  Not Load │ ← Page 2
└──────────────────────────────────────────────────────┘

访问 0x1000_0800 (Page 0 内):
  → Page Fault → 从磁盘加载 Page 0 → 后续访问直接读取
```

### 3. Mmap 和 MmapMut 的区别

```
┌─────────────────────────────────────────────────────────┐
│                    Mmap (只读)                          │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────┐                                    │
│  │   *const u8     │ ← 原始指针                         │
│  │   (不可修改)    │                                    │
│  ├─────────────────┤                                    │
│  │  Deref<T=[u8]>  │ ← 实现解引用为字节切片             │
│  │  Index<usize>   │ ← 支持索引访问                     │
│  └─────────────────┘                                    │
│                                                         │
│  特性：                                                │
│  - 可以从 &File 创建                                   │
│  - 允许多个读者                                        │
│  - 不可变引用，线程安全 (Sync)                         │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                   MmapMut (可读写)                      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────┐                                    │
│  │   *mut u8       │ ← 可变原始指针                     │
│  │   (可修改)      │                                    │
│  ├─────────────────┤                                    │
│  │  DerefMut       │ ← 支持可变解引用                   │
│  │  IndexMut       │ ← 支持可变索引                     │
│  │  flush()        │ ← 同步到磁盘                       │
│  └─────────────────┘                                    │
│                                                         │
│  特性：                                                │
│  - 需要文件以写方式打开                                 │
│  - 修改通过 MmapMut 直接反映到文件                      │
│  - 不是 Sync（可变状态）                                │
└─────────────────────────────────────────────────────────┘
```

---

## 知识检查

**问题 1** 🟢 (基础概念)

内存映射文件的主要优势是什么？

A) 自动加密文件内容  
B) 零拷贝访问，避免内核到用户空间的数据复制  
C) 可以访问比物理内存大的文件  
D) 自动压缩文件数据

<details>
<summary>答案与解析</summary>

**答案**: B) 零拷贝访问，避免内核到用户空间的数据复制

**解析**:
- **A 错误**: 内存映射与加密无关
- **B 正确**: 这是 mmap 的核心优势，直接访问内核页面缓存
- **C 部分正确但不是主要优势**: 虚拟内存确实允许映射大文件，但传统 I/O 也可以分块处理大文件
- **D 错误**: 内存映射与压缩无关

</details>

---

**问题 2** 🟡 (页面机制)

假设系统页面大小是 4096 字节，映射一个 10000 字节的文件，实际上会占用多少虚拟内存和物理内存？

A) 虚拟: 10000 字节，物理: 10000 字节  
B) 虚拟: 12288 字节 (3页)，物理: 12288 字节  
C) 虚拟: 12288 字节 (3页)，物理: 0-12288 字节（取决于访问）  
D) 虚拟: 10000 字节，物理: 0 字节

<details>
<summary>答案与解析</summary>

**答案**: C) 虚拟: 12288 字节 (3页)，物理: 0-12288 字节（取决于访问）

**解析**:
- 虚拟内存：向上对齐到页面大小，10000 / 4096 = 2.44 → 3 页 = 12288 字节
- 物理内存：使用**按需分页**，只有实际访问的页面才会加载到物理内存
- 如果程序只访问前 500 字节，只有 Page 0 会占用物理内存

</details>

---

**问题 3** 🔴 (安全实践)

为什么 `Mmap::map()` 是 unsafe 函数？

A) 它会破坏文件系统  
B) 它可能导致数据竞争  
C) 调用者必须保证文件在映射期间保持打开  
D) 它会修改文件权限

<details>
<summary>答案与解析</summary>

**答案**: C) 调用者必须保证文件在映射期间保持打开

**解析**:
- **C 正确**: 这是主要原因。如果文件关闭后仍访问映射内存，会导致未定义行为
- **B 部分正确但不完全**: Mmap 本身是只读的，不涉及数据竞争
- **A 和 D 错误**: mmap 不会破坏文件系统或修改权限

**安全使用**:
```rust
let file = File::open("data.txt")?;
let mmap = unsafe { Mmap::map(&file)? };
// ✅ file 和 mmap 在同一作用域，file 先 drop，安全
```

</details>

---

## 💡 小知识：内存映射的历史渊源

**问题来源**：
在 1970 年代，Multics 系统首次引入内存映射文件概念。Unix 在 1980 年代通过 `mmap()` 系统调用标准化了这一技术。

**为什么需要它**：
早期操作系统需要一种高效方式让多个进程共享代码和数据：
- **共享库**：同一个 libc.so 可以映射到多个进程，节省物理内存
- **进程间通信**：比管道、套接字更快的 IPC
- **大文件处理**：无需分块读入内存

**Linux 演进**：
```
1990s: 基础 mmap 支持
2000s: 添加 MAP_ANONYMOUS, MAP_SHARED
2005+: Huge Pages 支持 (2MB, 1GB)
2015+: DAX (Direct Access) for persistent memory
```

---

## 🌟 工业界应用：数据库引擎

**真实案例**：
SQLite、RocksDB、LevelDB 都使用内存映射：

```rust
// RocksDB 风格的 SSTable 读取
fn read_sstable_block(file: &File, offset: usize, size: usize) -> &[u8] {
    // 整个 SSTable 文件映射到内存
    let mmap = unsafe { Mmap::map(file).unwrap() };
    
    // 直接返回切片，零拷贝！
    &mmap[offset..offset + size]
}

// vs 传统方式（需要分配和复制）
fn read_sstable_block_slow(file: &mut File, offset: usize, size: usize) -> Vec<u8> {
    let mut buf = vec![0u8; size];
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    file.read_exact(&mut buf).unwrap();
    buf // 返回复制的数据
}
```

**性能对比**（读取 1GB 数据）：

| 方式 | 时间 | 内存分配 |
|------|------|----------|
| 传统 read | 4.2s | 频繁的 Vec 分配 |
| mmap | 0.8s | 无（复用页面缓存） |
| 提升 | **5x** | **零分配** |

---

## 🧪 动手试试：完整示例

来自 `src/advance/memmap_sample.rs` 的完整示例：

```rust
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use memmap2::Mmap;
use tempfile;

fn memmap_file_sample() {
    // 1. 创建临时文件
    let mut tmpfile = tempfile::tempfile().expect("failed to open the file");
    
    println!("tempfile : {:?}", tmpfile);
    
    // 2. 写入数据
    write!(tmpfile, "Hello World!").unwrap();
    
    // 3. 重置文件指针到开头
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    
    // 4. 传统方式读取验证
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);
    
    // 5. 创建内存映射（unsafe 块）
    let mmap = unsafe { Mmap::map(&tmpfile).expect("failed to map the file") };
    
    // 6. 像访问数组一样访问文件内容
    assert_eq!(b"Hello World!", &mmap[..]);
    
    // 7. 转换为可变映射进行修改
    let mut mm = mmap.make_mut().unwrap();
    
    // 8. 修改内存（直接反映到文件）
    let _ = (&mut mm[..]).write_all(b"Hello Zen!");
    
    // 9. 验证修改结果
    assert_eq!(b"Hello Zen!d!", &mm[..]);
}

/// 获取系统页面大小
fn sys_page_size_sample() {
    let page_size = page_size::get();
    println!("system page_size:{}", page_size); // 通常是 4096
}
```

**关键点说明**：
1. `tempfile::tempfile()` 创建匿名临时文件
2. `Mmap::map(&tmpfile)` 创建只读映射（注意 unsafe）
3. `mmap[..]` 像访问字节数组一样访问文件
4. `make_mut()` 将只读映射转换为可变映射
5. 修改 `mm` 直接反映到底层文件

完整示例：[src/advance/memmap_sample.rs](../../../src/advance/memmap_sample.rs)
