# 高级进阶 (Advance)

## 开篇故事

想象你已经学会了 Rust 的基础：变量、所有权、结构体、枚举。现在你想建造一座真正的房子——不是玩具模型，而是能住人的。这就需要更强大的工具：电钻、锯子、水平仪。Rust 的高级特性就是你的"电动工具"——它们让复杂任务变得简单，让高性能代码成为可能。

本部分涵盖 Rust 生态系统的核心工具：异步编程、数据库操作、Web 开发、数据处理、系统编程、测试与模拟、宏编程。掌握这些，你就能构建生产级应用。

---

## 本章适合谁

如果你已经完成了 [基础入门](../basic/basic-overview.md)，现在想学习 Rust 在实际项目中的应用，本部分适合你。

---

## 你会学到什么

完成高级进阶后，你可以：

1. **异步编程** - 使用 Tokio 编写高并发网络服务
2. **数据库操作** - 使用 SQLx 和 Diesel 操作数据库
3. **Web 开发** - 使用 Axum 和 Hyper 构建 REST API
4. **数据处理** - 序列化/反序列化 JSON、CSV 等格式
5. **系统编程** - 文件操作、内存映射、进程管理
6. **测试与模拟** - 编写单元测试和集成测试
7. **宏编程** - 使用声明宏和过程宏减少代码重复

---

## 前置要求

- ✅ [基础入门](../basic/basic-overview.md) 全部章节
- ✅ 理解所有权和借用
- ✅ 理解结构体和特征
- ✅ 基本的 Rust 项目结构知识

---

## 学习路径

### 阶段 1: 异步编程（核心）

```
异步编程
├── 异步编程基础 (async.md)
├── Futures 异步编程 (futures.md)
├── 并行计算 (rayon.md)
├── MIO 底层 I/O (mio.md)
└── 循环引用 (cyclerc.md)
```

**为什么先学这个？** 现代 Rust 服务几乎都是异步的。Tokio 是事实上的异步运行时。

### 阶段 2: 数据持久化

```
数据库操作
├── 数据库操作概览 (database/database.md)
├── SQLx 异步查询 (database/sqlx.md)
└── Diesel ORM (database/diesel.md)
```

**为什么学这个？** 几乎所有应用都需要存储和检索数据。

### 阶段 3: Web 服务

```
Web 开发
├── Axum Web 框架 (web/axum.md)
├── Hyper HTTP 底层 (web/hyper.md)
└── Ollama AI 集成 (web/ollama.md)
```

**为什么学这个？** 构建 API 和 Web 服务是 Rust 的主要应用场景。

### 阶段 4: 数据处理

```
数据处理
├── JSON 序列化 (data/json.md)
├── CSV 处理 (data/csv.md)
├── 零拷贝序列化 (data/rkyv.md)
└── 序列化基础 (data/serialization.md)
```

**为什么学这个？** 数据交换格式是系统间通信的基础。

### 阶段 5: 系统编程

```
系统编程
├── 文件与目录操作 (system/directory.md)
├── 临时文件 (system/tempfile.md)
├── 内存映射 (system/memmap.md)
├── 环境变量 (system/dotenv.md)
├── 字节处理 (system/bytes.md)
├── Cow 类型 (system/cow.md)
├── 进程管理 (system/process.md)
├── 系统信息 (system/sysinfo.md)
└── 资源嵌入 (system/includedir.md)
```

**为什么学这个？** 系统编程是 Rust 的核心优势。

### 阶段 6: 测试与模拟

```
测试与模拟
├── 测试基础 (testing/test.md)
├── 模拟测试 (testing/mock.md)
└── 测试框架 (testing/rspec.md)
```

**为什么学这个？** 可靠的代码需要可靠的测试。

### 阶段 7: 宏编程（独立）

```
宏编程
├── 声明宏和过程宏 (testing/macros.md)
└── 派生宏 (testing/getset.md)
```

**为什么学这个？** 宏让你成为"元程序员"，编写生成代码的代码。

---

## 代码示例

本部分每个章节都配有可运行的示例代码：

```
src/advance/
├── async_sample.rs       - 异步编程示例
├── futures_sample.rs     - Futures 示例
├── rayon_sample.rs       - 并行计算示例
├── mio_sample.rs         - MIO 示例
├── cyclerc_sample.rs     - 循环检测示例
├── sqlx_sample.rs        - SQLx 示例
├── diesel_sample.rs      - Diesel 示例
├── axum_sample.rs        - Axum 示例
├── hyper_sample.rs       - Hyper 示例
├── json_sample.rs        - JSON 序列化示例
├── csv_sample.rs         - CSV 处理示例
├── rkyv_sample.rs        - 零拷贝序列化示例
├── bytes_sample.rs       - 字节处理示例
├── cow_sample.rs         - Cow 类型示例
├── dotenv_sample.rs      - 环境变量示例
├── memmap_sample.rs      - 内存映射示例
├── process_sample.rs     - 进程管理示例
├── sysinfo_sample.rs     - 系统信息示例
├── includedir_sample.rs  - 资源嵌入示例
├── mock_sample.rs        - 模拟测试示例
├── getset_sample.rs      - 派生宏示例
├── macros_sample.rs      - 宏编程示例
└── typealias_sample.rs   - 类型别名示例
```

**运行示例**:
```bash
# 编译并运行特定示例
cargo run --bin async_sample
cargo run --bin macros_sample
```

---

## 项目结构

```
hello-rust/
├── src/
│   ├── basic/            # 基础入门
│   ├── advance/          # 高级进阶（本部分）
│   └── bin/              # 可运行二进制
├── crates/
│   ├── awesome/          # 生产级框架
│   ├── leetcode/         # 算法题解
│   └── macros/           # 过程宏
└── docs/                 # 文档
```

---

## 下一步

完成高级进阶后，继续学习 **[实战精选](../awesome/awesome-overview.md)** 部分，你将学习：

- 数据库高级应用
- 微服务架构
- 消息队列
- 依赖注入
- 插件系统

---

> 💡 **提示**：高级进阶是 Rust 从"玩具"到"工具"的关键一步！

---

## 继续学习

**前一章**: [基础入门](../basic/basic-overview.md)  
**下一章**: [实战精选](../awesome/awesome-overview.md)

**相关章节**:
- [异步编程](async/async.md)
- [数据库操作](database/database.md)
- [Web 开发](web/axum.md)

**返回**: [目录](../SUMMARY.md)
