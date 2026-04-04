# 字节处理

## 开篇故事

想象你要处理二进制数据（图片、音频、网络数据包）。直接操作字节数组就像用手抓沙子——容易散落。bytes 库就像容器——帮你安全、高效地管理字节数据。

---

## 本章适合谁

如果你需要处理二进制数据（网络编程、文件处理、加密），本章适合你。bytes 是高性能字节处理的标准库。

---

## 你会学到什么

完成本章后，你可以：

1. 创建 Bytes 和 BytesMut
2. 分割和合并字节
3. 实现 Buf 和 BufMut trait
4. 使用 Base64 编解码
5. 使用位向量 (BitVec)

---

## 前置要求

- [变量与表达式](../basic/expression.md) - 基础语法
- [数据类型](../basic/datatype.md) - 数据类型基础
- [不安全代码](../basic/pointer.md) - unsafe 基础（可选）

---

## 第一个例子

最简单的 Bytes 使用：

```rust
use bytes::{Bytes, BytesMut, BufMut};

fn main() {
    // 从字符串创建 Bytes
    let mut mem = Bytes::from("Hello world");
    
    // 切片操作（零拷贝）
    let a = mem.slice(0..5);
    assert_eq!(a, "Hello");
    
    // 分割操作
    let b = mem.split_to(6);
    assert_eq!(b, "Hello ");
    assert_eq!(mem, "world");
    
    println!("a: {}, b: {}, mem: {}", a, b, mem);
}
```

**完整示例**: [bytes_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/bytes_sample.rs)

---

## 原理解析

### bytes 特性

**bytes 是字节处理库**：

- ✅ 零拷贝操作
- ✅ 高性能
- ✅ Buf/BufMut trait
- ✅ 线程安全

### Bytes 和 BytesMut

**Bytes (不可变)**：

```rust
use bytes::Bytes;

// 从字符串创建
let bytes = Bytes::from("Hello");

// 从字节数组创建
let bytes = Bytes::from(&b"Hello"[..]);

// 切片（零拷贝）
let slice = bytes.slice(0..5);
```

**BytesMut (可变)**：

```rust
use bytes::BytesMut;

// 创建可变缓冲区
let mut buf = BytesMut::with_capacity(1024);

// 写入数据
buf.put(&b"Hello"[..]);
buf.put_u16(1234);

// 转换为 Bytes
let bytes = buf.freeze();
```

### 分割操作

**使用 split()**：

```rust
use bytes::BytesMut;

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);

// 分割并获取前半部分
let a = buf.split();
assert_eq!(a, b"hello world"[..]);
```

**使用 split_to()**：

```rust
let mut buf = BytesMut::from(&b"hello world"[..]);

// 分割前 6 个字节
let b = buf.split_to(6);
assert_eq!(b, b"hello "[..]);
assert_eq!(buf, b"world"[..]);
```

### Buf Trait

**读取数据**：

```rust
use bytes::{Buf, Bytes};

let mut buf = Bytes::from(&b"hello"[..]);

// 读取字节
let byte = buf.get_u8();

// 读取 u16 (大端)
let val = buf.get_u16();

// 检查剩余字节
if buf.has_remaining() {
    println!("还有 {} 字节", buf.remaining());
}
```

### Base64 编解码

**使用 base64 crate**：

```rust
use base64::{Engine as _, engine::general_purpose::STANDARD};

// 编码
let encoded = STANDARD.encode("Hello World!");
println!("Base64: {}", encoded);

// 解码
let decoded = STANDARD.decode(&encoded)?;
println!("解码：{}", String::from_utf8_lossy(&decoded));
```

**使用 URL 安全编码**：

```rust
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

let encoded = URL_SAFE.encode("Hello+World/");
println!("URL Safe: {}", encoded);
```

### BitVec 位向量

**使用 bitvec crate**：

```rust
use bitvec::prelude::*;

// 创建位向量
let mut bv = bitvec![0, 0, 1, 1, 0, 1, 0, 1];

// 访问位
if bv[2] {
    println!("第 3 位是 1");
}

// 修改位
bv.set(0, true);

// 转换为字节
let bytes: Vec<u8> = bv.as_bytes().to_vec();
```

---

## 常见错误

### 错误 1: 越界访问

```rust
use bytes::Bytes;

let bytes = Bytes::from(&b"hello"[..]);
let slice = bytes.slice(0..10);  // ❌ 越界
```

**错误信息**:
```
range out of bounds
```

**修复方法**:
```rust
let slice = bytes.slice(0..5.min(bytes.len()));  // ✅ 检查边界
```

### 错误 2: 忘记检查剩余字节

```rust
use bytes::Buf;

let mut buf = Bytes::from(&b"hello"[..]);

while buf.has_remaining() {
    let byte = buf.get_u8();
    // ❌ 如果数据不足会 panic
}
```

**修复方法**:
```rust
while buf.remaining() >= 1 {  // ✅ 检查足够字节
    let byte = buf.get_u8();
}
```

### 错误 3: Base64 解码错误

```rust
use base64::{Engine as _, engine::general_purpose::STANDARD};

let invalid = "Invalid!Base64";
let decoded = STANDARD.decode(invalid)?;  // ❌ 可能失败
```

**错误信息**:
```
Invalid last symbol
```

**修复方法**:
```rust
match STANDARD.decode(invalid) {
    Ok(decoded) => println!("解码：{}", String::from_utf8_lossy(&decoded)),
    Err(e) => eprintln!("解码失败：{}", e),
}
```

---

## 动手练习

### 练习 1: 创建和分割 Bytes

```rust
use bytes::{Bytes, BytesMut};

fn main() {
    // TODO: 创建 BytesMut
    // TODO: 写入 "Hello World"
    // TODO: 分割前 5 个字节
    // TODO: 打印结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"Hello World"[..]);

let hello = buf.split_to(5);
assert_eq!(hello, b"Hello"[..]);
assert_eq!(buf, b" World"[..]);
```
</details>

### 练习 2: Base64 编解码

```rust
use base64::{Engine as _, engine::general_purpose::STANDARD};

fn main() {
    let original = "Hello Rust!";
    
    // TODO: 编码为 Base64
    // TODO: 解码回原始字符串
    // TODO: 验证结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
let encoded = STANDARD.encode(original);
println!("Base64: {}", encoded);

let decoded = STANDARD.decode(&encoded).unwrap();
assert_eq!(original, String::from_utf8_lossy(&decoded));
```
</details>

### 练习 3: 位向量操作

```rust
use bitvec::prelude::*;

fn main() {
    // TODO: 创建 8 位位向量
    // TODO: 设置第 0、2、4、6 位为 1
    // TODO: 转换为字节
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut bv = bitvec![0; 8];
bv.set(0, true);
bv.set(2, true);
bv.set(4, true);
bv.set(6, true);

let bytes: Vec<u8> = bv.as_bytes().to_vec();
println!("字节：{:02x?}", bytes);
```
</details>

---

## 故障排查 (FAQ)

### Q: Bytes 和 Vec<u8> 有什么区别？

**A**: 
- **Bytes**: 零拷贝，适合网络/IO
- **Vec<u8>**: 可修改，适合一般用途
- **性能**: Bytes 在分割/切片时性能更优

### Q: 什么时候使用 Buf trait？

**A**: 
- 解析二进制协议
- 网络数据读取
- 文件解析

### Q: BitVec 和普通 Vec<bool> 有什么区别？

**A**: 
- **BitVec**: 每 bit 存储一个布尔值（节省空间）
- **Vec<bool>**: 每字节存储一个布尔值
- **性能**: BitVec 空间效率更高

---

## 知识扩展

### 字节序转换

```rust
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

let data = vec![0x00, 0x00, 0x03, 0xe8];
let mut cursor = Cursor::new(data);

let val = cursor.read_u32::<BigEndian>()?;
assert_eq!(val, 1000);
```

### 性能优化

```rust
use bytes::BytesMut;

// 预分配容量
let mut buf = BytesMut::with_capacity(4096);

// 避免多次分配
buf.reserve(1024);
```

### 网络编程应用

```rust
use bytes::{Bytes, BytesMut, BufMut};

// 构建网络消息
let mut msg = BytesMut::with_capacity(1024);
msg.put_u32(100);  // 消息长度
msg.put(&b"Hello"[..]);  // 消息内容

// 发送
socket.send(&msg.freeze()).await?;
```

---

## 小结

**核心要点**：

1. **Bytes**: 零拷贝字节容器
2. **BytesMut**: 可写字节缓冲区
3. **Buf/BufMut**: 字节读取/写入 trait
4. **Base64**: 二进制到文本编码
5. **BitVec**: 位级别操作

**关键术语**：

- **Bytes**: 字节类型
- **Buf**: 缓冲区 trait
- **Zero-copy**: 零拷贝
- **Base64**: Base64 编码
- **BitVec**: 位向量

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Bytes | 字节 |
| Buffer | 缓冲区 |
| Zero-copy | 零拷贝 |
| Base64 | Base64 编码 |
| BitVec | 位向量 |
| Endianness | 字节序 |

---

## 延伸阅读

学习完字节处理后，你可能还想了解：

- [Bytes crate](https://docs.rs/bytes) - 零拷贝字节缓冲区
- [零拷贝网络编程](https://tokio.rs/tokio/tutorial/streams) - 直接传递缓冲区
- [SIMD 字节处理](https://docs.rs/simd-json) - 加速解析

**选择建议**:
- 想学习写时复制 → 继续学习 [Cow 类型](cow.md)
- 想学习环境变量 → 跳到 [环境变量](dotenv.md)

## 继续学习

**前一章**: [环境变量](dotenv.md)  
**下一章**: [Cow 类型](cow.md)

**相关章节**:
- [环境变量](dotenv.md)
- [Cow 类型](cow.md)
- [临时文件](tempfile.md)

**返回**: [高级进阶](advance-overview.md)

---

**完整示例**: [bytes_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/bytes_sample.rs)
