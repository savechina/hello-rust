# Hello Rust
 [English](README.md)| 简体中文

[**Rust简明教程**](https://renyan.org/hello/rust) | [**GitHub Pages**](https://savechina.github.io/hello-rust/)

这是学习 Rust 编程语言的综合性样例工程，包含从基础语法到高级应用的完整教程和可运行代码示例。

## 📖 在线教程

- 🌐 **官方教程**: [renyan.org/hello/rust](https://renyan.org/hello/rust)
- 📚 **GitHub Pages**: [savechina.github.io/hello-rust](https://savechina.github.io/hello-rust/)

## 🚀 快速上手

```bash
# 克隆项目
git clone https://github.com/savechina/hello-rust.git
cd hello-rust

# 运行基础示例
cargo run

# 运行特定二进制示例
cargo run --bin grpc_hello_server
```

## 📦 工程模块

### 基础入门 (Basic)

涵盖 Rust 核心语法和概念，适合初学者：

| 模块 | 内容 |
|------|------|
| [变量与表达式](docs/src/basic/expression.md) | 变量绑定、可变性、基本表达式 |
| [基础数据类型](docs/src/basic/datatype.md) | 整数、浮点数、布尔值、字符、集合类型、日期时间 |
| [了解所有权](docs/src/basic/ownership.md) | 所有权规则、移动语义、借用与引用 |
| [生命周期](docs/src/basic/lifetimes.md) | 生命周期标注、借用检查器、悬垂引用预防 |
| [结构体](docs/src/basic/struct.md) | 结构体定义、方法、关联函数 |
| [枚举](docs/src/basic/enums.md) | 枚举定义、模式匹配、Option/Result |
| [特征](docs/src/basic/trait.md) | Trait 定义、实现、多态 |
| [特征对象](docs/src/basic/trait-objects.md) | 动态分发、dyn Trait、虚表机制 |
| [泛型](docs/src/basic/generic.md) | 泛型函数、单态化、特征约束 |
| [闭包](docs/src/basic/closure.md) | 闭包语法、捕获环境、Fn/FnMut/FnOnce |
| [模块系统](docs/src/basic/module.md) | 模块组织、可见性、use 语句 |
| [线程与并发](docs/src/basic/threads.md) | 线程创建、通道、Mutex、Arc |

### 高级进阶 (Advance)

深入 Rust 高级特性和生态系统：

| 模块 | 内容 |
|------|------|
| [智能指针](docs/src/advance/smart-pointers.md) | Box, Rc, RefCell, Arc, Weak, 循环引用 |
| [错误处理](docs/src/advance/error-handling/error-handling.md) | Result, ?, thiserror, anyhow, 错误传播 |
| [高级特征](docs/src/advance/advanced-traits.md) | 关联类型、完全限定语法、Newtype 模式 |
| [迭代器](docs/src/advance/iterators.md) | map, filter, collect, fold, 自定义迭代器 |
| [异步编程](docs/src/advance/async/async.md) | Tokio 运行时、async/await、Future trait |
| [数据库](docs/src/advance/database/database.md) | SQLx 异步查询、Diesel ORM |
| [Web 开发](docs/src/advance/web/axum.md) | Axum Web 框架、Hyper HTTP、gRPC |
| [数据处理](docs/src/advance/data/serialization.md) | Serde 序列化、JSON、CSV、Rkyv 零拷贝 |
| [系统编程](docs/src/advance/system/directory.md) | 文件操作、内存映射、进程管理、原子类型 |
| [测试](docs/src/advance/testing/test.md) | 单元测试、集成测试、Mock、nextest |
| [宏编程](docs/src/advance/testing/macros.md) | 声明宏、过程宏、宏卫生 |

### 算法与实战

| 模块 | 内容 |
|------|------|
| [算法实现](docs/src/algo/algo.md) | 链表、π 计算 |
| [LeetCode 题解](docs/src/leetcode/leetcode.md) | Two Sum, Add Two Numbers |
| [精选实战](docs/src/awesome/awesome-overview.md) | 服务框架、消息队列、模板引擎、依赖注入 |

## 🛠️ 技术栈

- **Rust 2024 Edition**
- **异步运行时**: Tokio
- **Web 框架**: Axum, Hyper, Tonic (gRPC)
- **数据库**: SQLx, Diesel
- **序列化**: Serde, Rkyv
- **测试**: Mockall, rspec, nextest

## 📋 项目结构

```
hello-rust/
├── src/
│   ├── basic/          # 基础入门示例代码
│   ├── advance/        # 高级进阶示例代码
│   ├── algo/           # 算法实现
│   └── bin/            # 15 个可运行二进制示例 (gRPC, UDS, Stdio IPC)
├── crates/
│   ├── awesome/        # 生产级服务框架 (DI, Consul, gRPC)
│   ├── leetcode/       # LeetCode 题解
│   └── macros/         # 过程宏示例 (#[log])
└── docs/               # mdBook 教程文档 (中文)
```

## 📝 许可证

MIT License
