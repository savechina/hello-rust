# 代码样例质量参考

**Purpose**: Comprehensive guide to code sample quality, documenting which samples are excellent teaching tools, which require warnings, and how to handle unsafe patterns

**Created**: 2026-04-03  
**Branch**: `001-rust-tutorial-docs`  
**Based on**: research.md findings

---

## 本章适合谁

- **文档撰写者**: 需要知道哪些样例可以直接使用，哪些需要警告
- **技术审核者**: 验证文档是否正确处理了 unsafe 代码
- **高级学习者**: 想了解 Rust 安全边界的读者

---

## 你会学到什么

1. 识别高质量教学样例的特征
2. 了解哪些样例包含 unsafe 模式
3. 知道如何为 unsafe 代码添加适当警告
4. 掌握安全替代方案的推荐方式

---

## 样例质量分级

### ✅ 高质量教学样例 (HIGH QUALITY)

这些样例是优秀的教学工具，可以直接用于章节，无需额外警告。

#### 1. `ownership_sample.rs`

**位置**: `src/basic/ownership_sample.rs`  
**质量**: ⭐⭐⭐⭐⭐  
**适合章节**: 所有权 (ownership)

**优点**:
- ✅ 清晰展示所有权转移
- ✅ 包含注释的错误示例（悬垂指针预防）
- ✅ 生命周期注解示例
- ✅ 有正确和错误两种模式

**如何使用**:
- 作为所有权章节的核心示例
- 展示 `moves ownership` 的注释代码
- 引用第 55-70 行的注释示例

**注意事项**:
- 无需特殊警告
- 可以直接作为"正确模式"展示

---

#### 2. `datatype_sample.rs`

**位置**: `src/basic/datatype_sample.rs` (1017 行)  
**质量**: ⭐⭐⭐⭐⭐  
**适合章节**: 数据类型、字符串、集合

**优点**:
- ✅ 全面的类型演示
- ✅ String 与&str对比
- ✅ 集合类型完整 (Vec, HashMap, BTreeMap)
- ✅ chrono 集成示例
- ✅ BigDecimal 精度数学

**如何使用**:
- **注意**: 文件很大 (1017 行)，应该拆分为多个专注章节
- 建议拆分:
  - 字符串处理 (行 1-200)
  - 集合类型 (行 201-500)
  - 日期时间 (行 501-700)
  - 数值精度 (行 701-1017)

**注意事项**:
- 不要一次展示整个文件
- 按主题选择相关片段

---

#### 3. `tokio_sample.rs`

**位置**: `src/advance/tokio_sample.rs` (509 行)  
**质量**: ⭐⭐⭐⭐⭐  
**适合章节**: 异步编程、TCP 网络

**优点**:
- ✅ 完整的异步 TCP 服务器/客户端
- ✅ 正确的错误处理
- ✅ 展示 tokio::spawn
- ✅ 包含实际网络代码

**如何使用**:
- 建议拆分为两个章节:
  - "异步基础" (前 200 行)
  - "网络编程" (后 309 行)

**注意事项**:
- 无需特殊警告
- 适合展示生产级异步代码

---

### ⚠️ 需要上下文的样例 (REQUIRES CONTEXT)

这些样例包含可教学的模式，但需要额外说明或警告。

#### 1. `dynmaic_injection_box_sample.rs`

**位置**: `crates/awesome/src/services/dynmaic_injection_box_sample.rs`  
**质量**: ⭐⭐⭐⭐  
**适合章节**: 依赖注入、特征对象

**优点**:
- ✅ 清晰的特征为基础架构
- ✅ 类型安全的容器设计
- ✅ 展示 Box 和 Arc 用法

**⚠️ 注意事项**:
1. **文件名拼写错误**: `dynmaic` 应该是 `dynamic`
   - 文档中应该注明这个拼写错误
   - 聚焦于模式而不是文件名

2. **复杂度较高**:
   - 适合高级章节
   - 需要前置知识（特征、泛型、类型系统）

**如何使用**:
```markdown
> **注意**: 文件名有拼写错误 (`dynmaic` → `dynamic`)，但代码展示了正确的依赖注入模式。
```

---

#### 2. `threads_sample.rs`

**位置**: `src/basic/threads_sample.rs`  
**质量**: ⭐⭐⭐  
**适合章节**: 并发、线程

**优点**:
- ✅ 基础线程创建示例
- ✅ 展示 `thread::spawn`
- ✅ 包含 `join()` 用法

**⚠️ CRITICAL Warnings Required**:

1. **Unsafe Pattern** (line 243):
   ```rust
   static mut VAL: i32 = 0; // ⚠️ 不安全！
   ```
   
   **文档必须添加警告**:
   ```markdown
   > ⚠️ **安全警告**: 此示例使用 `static mut` 来演示线程间共享状态。
   > 
   > **为什么这是问题**:
   > - `static mut` 需要 `unsafe` 访问
   > - 没有编译器检查数据竞争
   > - 可能导致未定义行为
   > 
   > **安全替代方案**:
   > - 使用 `Mutex` 保护共享状态
   > - 使用 `Arc<Mutex<T>>` 多线程共享
   > - 使用通道 (channel) 传递消息
   > 
   > 此示例仅用于教学目的。生产代码应使用安全原语。
   ```

2. **死锁示例** (line 231-234, 注释):
   ```rust
   // 以下代码会阻塞发生死锁
   ```
   
   **文档说明**:
   ```markdown
   > 这个注释示例展示了死锁情况。虽然被注释掉了，但理解为什么这会死锁很重要...
   ```

**如何使用**:
- 必须包含上述安全警告
- 优先展示安全替代方案
- 仅在"危险模式"章节展示 unsafe 版本

---

### ✗ 关键问题样例 (CRITICAL ISSUES)

这些样例包含严重的反模式或不安全代码，**必须**添加明确警告。

#### 1. `pointer_sample.rs:17`

**位置**: `src/basic/pointer_sample.rs` line 17  
**问题**: `str::from_utf8_unchecked`  
**风险等级**: 🔴 **HIGH**

**问题代码**:
```rust
let s = str::from_utf8_unchecked(bytes); // ⚠️ 没有验证！
```

**为什么危险**:
- ❌ 不验证 UTF-8 有效性
- ❌ 可能产生未定义行为
- ❌ 如果字节不是有效 UTF-8，程序崩溃
- ❌ 学习者可能误用

**文档必须包含**:

```markdown
> 🔴 **高危警告**: `from_utf8_unchecked` 是不安全函数
> 
> **发生了什么**:
> 这个函数假设你提供的字节是有效的 UTF-8，但**不进行检查**。
> 如果假设错误，程序会有未定义行为（崩溃、数据损坏）。
> 
> **何时安全**:
> - 你已经手动验证了字节是 UTF-8
> - 性能关键路径，且你有其他保证
> - 你理解并接受了风险
> 
> **安全替代方案**:
> ```rust
> // ✅ 安全版本 - 返回 Result
> let s = std::str::from_utf8(bytes)?;
> 
> // ✅ 安全版本 - 返回 Option
> let s = std::str::from_utf8(bytes).ok()?;
> 
> // ✅ 安全版本 - panic 但有明确错误
> let s = std::str::from_utf8(bytes).unwrap();
> ```
> 
> **规则**: 除非你有性能剖析数据证明需要，否则总是使用安全版本。
```

---

#### 2. `sqlite_vec_sample.rs:7`

**位置**: `crates/awesome/src/database/sqlite_vec_sample.rs` line 7  
**问题**: `std::mem::transmute` on function pointers  
**风险等级**: 🔴 **CRITICAL**

**问题代码**:
```rust
let func = std::mem::transmute::<_, fn()>(ptr); // ⚠️ 极度危险！
```

**为什么危险**:
- ❌ `transmute` 是最危险的 Rust 函数之一
- ❌ 绕过了所有类型检查
- ❌ 函数指针转换极易出错
- ❌ 学习者绝对不应该使用

**文档必须包含**:

```markdown
> 🔴 **极度危险**: 此示例使用 `std::mem::transmute`
> 
> **警告**: 这是本教程中最危险的代码。`transmute` 会:
> - 完全绕过类型系统
> - 不检查转换是否有效
> - 导致未定义行为如果出错
> 
> **为什么示例中存在**:
> 这是一个高级 FFI (Foreign Function Interface) 示例，展示如何与动态加载的库交互。
> 
> **你不应该**:
> - ❌ 在生产代码中使用 `transmute`
> - ❌ 除非你有 10 年以上 Rust 经验
> - ❌ 除非你有完整的测试覆盖
> - ❌ 除非你有替代方案的性能数据
> 
> **安全替代方案**:
> 1. 使用 `cast()` 函数 (仍然需要 unsafe 但有检查)
> 2. 使用 `bindgen` 生成 FFI 绑定
> 3. 使用 `libloading` crate 安全封装
> 
> **如果必须使用**:
> - 封装在安全的抽象层后面
> - 提供详细的安全不变量文档
> - 进行彻底的测试和代码审查
> 
> 本示例仅供理解 Rust 的 unsafe 能力边界。**不要复制此代码。**
```

---

#### 3. `tonic_store_server.rs:34`

**位置**: `crates/awesome/src/services/tonic_store_server.rs` line 34  
**问题**: `std::sync::Mutex` in async context  
**风险等级**: 🟡 **MEDIUM** (反模式)

**问题代码**:
```rust
use std::sync::Mutex; // ⚠️ 在异步代码中会阻塞执行器

struct Store {
    data: Mutex<HashMap<String, String>>, // 会阻塞 async executor
}
```

**为什么是问题**:
- ❌ `std::sync::Mutex` 会阻塞整个 async executor
- ❌ 如果锁被持有，其他任务无法运行
- ❌ 可能导致死锁
- ❌ 违背异步编程原则

**文档必须包含**:

```markdown
> ⚠️ **反模式警告**: 在异步代码中使用 `std::sync::Mutex`
> 
> **发生了什么**:
> `std::sync::Mutex` 是同步原语。在异步上下文中获取锁时:
> - 如果锁不可用，会阻塞**整个 executor**
> - 其他异步任务无法运行
> - 可能导致性能问题或死锁
> 
> **正确做法**:
> ```rust
> use tokio::sync::Mutex; // ✅ 异步安全的 Mutex
> 
> struct Store {
>     data: Mutex<HashMap<String, String>>,
> }
> 
> // 使用时:
> let mut data = self.data.lock().await; // 不会阻塞 executor
> ```
> 
> **规则**: 
> - ✅ 在 `async fn` 中总是使用 `tokio::sync::Mutex`
> - ✅ 在同步代码中使用 `std::sync::Mutex`
> - ❌ 不要在异步代码中使用同步锁
> 
> **为什么示例中有这个问题**:
> 这是一个反模式示例，展示什么不应该做。文档会说明正确方式。
```

---

## 硬编码值警告

以下样例包含硬编码的配置值，**不应该**在生产代码中复制。

### 硬编码 IP 地址

**样例**: `greeter_service.rs`, `greeter_consume.rs`  
**值**: `"192.168.2.6"`, `"192.168.2.7"`

**文档说明**:
```markdown
> ⚠️ **配置警告**: 此示例使用硬编码的 IP 地址
> 
> **生产代码应该**:
> - 从环境变量读取: `std::env::var("SERVICE_IP")`
> - 使用配置文件 (TOML/YAML)
> - 使用服务发现 (Consul, Kubernetes DNS)
> 
> **示例**:
> ```rust
> let ip = std::env::var("SERVICE_IP")
>     .unwrap_or_else(|_| "127.0.0.1".to_string());
> ```
```

### 硬编码 Socket 路径

**样例**: UDS binaries (`uds_server.rs`, `uds_parent.rs`)  
**值**: `"/tmp/hello.socket"`

**文档说明**:
```markdown
> ⚠️ **平台特定**: 此示例使用 Unix 风格的 socket 路径
> 
> **限制**:
> - 只能在 Unix 系统 (Linux, macOS) 运行
> - Windows 不支持此路径格式
> 
> **跨平台替代**:
> - Windows: 使用命名管道 (Named Pipes)
> - 跨平台: 使用 TCP socket 或抽象 Unix socket
> ```

---

## 文档撰写检查清单

为每个样例编写章节时，检查:

### 通用检查
- [ ] 所有代码示例可以编译 (`cargo build --workspace`)
- [ ] 标明了完整示例的文件路径
- [ ] 有前置知识章节链接
- [ ] 包含预期输出

### 安全关键检查 (如果涉及 unsafe)
- [ ] 🔴 有红色警告框说明危险
- [ ] 解释了为什么不安全
- [ ] 提供了安全替代方案
- [ ] 说明了何时（如果有的话）可以使用

### 反模式检查
- [ ] ⚠️ 标注了反模式
- [ ] 解释了为什么是反模式
- [ ] 提供了正确模式示例
- [ ] 链接到相关最佳实践章节

### 硬编码值检查
- [ ] ⚠️ 标注了硬编码值
- [ ] 说明了生产环境应该如何配置
- [ ] 提供了环境变量示例

---

## 总结

**高质量样例** (可以直接使用):
- `ownership_sample.rs`
- `datatype_sample.rs` (拆分使用)
- `tokio_sample.rs` (拆分使用)

**需要上下文的样例** (需要说明):
- `dynmaic_injection_box_sample.rs` (注明拼写错误)
- `threads_sample.rs` (⚠️ unsafe 警告)

**关键问题样例** (🔴 必须警告):
- `pointer_sample.rs:17` (from_utf8_unchecked)
- `sqlite_vec_sample.rs:7` (transmute)
- `tonic_store_server.rs:34` (std::sync::Mutex in async)

**硬编码值** (⚠️ 配置警告):
- IP 地址: "192.168.2.6/7"
- Socket 路径: /tmp/hello.socket

---

## 下一步

**文档撰写者**:
1. 使用本参考确定样例的质量等级
2. 根据等级添加相应警告或说明
3. 始终提供安全替代方案

**后续更新**:
- 发现新的 unsafe 模式时更新本文档
- 收集学习者的困惑点，增加对应说明
- 定期审核样例代码是否与最新 Rust 兼容
