# 术语表 (Glossary)

**Purpose**: Bilingual terminology reference (中文 ↔ English) for consistent translation across all documentation chapters

**Created**: 2026-04-03  
**Branch**: `001-rust-tutorial-docs`  
**Usage**: Reference this glossary when writing chapters to ensure terminology consistency

---

## 核心概念 (Core Concepts)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Ownership | 所有权 | 所有权 (ownership) | Rust's core resource management concept |
| Borrowing | 借用 | 借用 (borrowing) | Temporary access to data without taking ownership |
| Lifetime | 生命周期 | 生命周期 (lifetime) | Scope for which a reference is valid |
| Reference | 引用 | 引用 (reference) | Pointer to data without ownership |
| Move | 移动 | 移动 (move) | Transfer of ownership |
| Copy | 复制 | 复制 (copy) | Bitwise copy of data (for Copy types) |
| Clone | 克隆 | 克隆 (clone) | Deep copy operation |
| Scope | 作用域 | 作用域 (scope) | Range of code where variable is valid |
| Variable | 变量 | 变量 (variable) | Named storage location |
| Binding | 绑定 | 绑定 (binding) | Association between name and value |
| Expression | 表达式 | 表达式 (expression) | Code that evaluates to a value |
| Statement | 语句 | 语句 (statement) | Code that performs an action |
| Function | 函数 | 函数 (function) | Reusable code block |
| Method | 方法 | 方法 (method) | Function associated with type |
| Parameter | 参数 | 参数 (parameter) | Function input variable |
| Argument | 实参 | 实参 (argument) | Actual value passed to function |

---

## 数据类型 (Data Types)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Type | 类型 | 类型 (type) | Classification of data |
| Scalar | 标量 | 标量 (scalar) | Single value type |
| Compound | 复合 | 复合 (compound) | Multiple values type |
| Integer | 整数 | 整数 (integer) | Whole number type |
| Floating Point | 浮点数 | 浮点数 (floating point) | Decimal number type |
| Boolean | 布尔值 | 布尔值 (boolean) | true/false type |
| Character | 字符 | 字符 (character) | Single Unicode character |
| String | 字符串 | 字符串 (string) | Text type |
| Array | 数组 | 数组 (array) | Fixed-size collection |
| Slice | 切片 | 切片 (slice) | Dynamic view into collection |
| Tuple | 元组 | 元组 (tuple) | Fixed-size heterogeneous collection |
| Struct | 结构体 | 结构体 (struct) | Custom composite type |
| Enum | 枚举 | 枚举 (enum) | Type with named variants |
| Union | 联合体 | 联合体 (union) | Overlapping data representation |
| Option | 选项类型 | Option<T> | Type representing optional value |
| Result | 结果类型 | Result<T, E> | Type representing success/error |
| Vector | 向量 | Vec<T> | Growable array |
| HashMap | 哈希映射 | HashMap<K, V> | Key-value map |
| Box | 盒子 | Box<T> | Heap pointer type |
| Rc | 引用计数 | Rc<T> | Reference-counted pointer |
| Arc | 原子引用计数 | Arc<T> | Atomic reference-counted pointer |

---

## 特征与泛型 (Traits & Generics)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Trait | 特征 | 特征 (trait) | Interface definition |
| Generic | 泛型 | 泛型 (generic) | Type parameterization |
| Type Parameter | 类型参数 | 类型参数 (type parameter) | Generic type placeholder |
| Implement | 实现 | 实现 (implement) | Provide trait definition |
| Inference | 推断 | 推断 (inference) | Automatic type deduction |
| Annotation | 注解 | 注解 (annotation) | Explicit type declaration |
| Constraint | 约束 | 约束 (constraint) | Generic type limitation |
| Bound | 边界 | 边界 (bound) | Trait requirement on generic |
| Default | 默认 | 默认 (default) | Fallback implementation |
| Derive | 派生 | 派生 (derive) | Automatic trait implementation |
| Macro | 宏 | 宏 (macro) | Code generation facility |
| Attribute | 属性 | 属性 (attribute) | Metadata annotation |
| Procedural Macro | 过程宏 | 过程宏 (procedural macro) | Compile-time code generation |
| Declarative Macro | 声明宏 | 声明宏 (declarative macro) | Pattern-based code generation |

---

## 内存管理 (Memory Management)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Stack | 栈 | 栈 (stack) | LIFO memory region |
| Heap | 堆 | 堆 (heap) | Dynamic memory region |
| Allocation | 分配 | 分配 (allocation) | Memory reservation |
| Deallocation | 释放 | 释放 (deallocation) | Memory return |
| Drop | 丢弃 | Drop trait | Resource cleanup trait |
| Destructor | 析构函数 | 析构函数 (destructor) | Cleanup function |
| Dangling | 悬垂 | 悬垂指针 (dangling pointer) | Invalid reference |
| Leak | 泄漏 | 内存泄漏 (memory leak) | Unreturned memory |
| Safe | 安全 | 安全 (safe) | Memory-safe operation |
| Unsafe | 不安全 | 不安全 (unsafe) | Requires manual safety proof |

---

## 并发与异步 (Concurrency & Async)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Concurrency | 并发 | 并发 (concurrency) | Multiple tasks in progress |
| Parallelism | 并行 | 并行 (parallelism) | Multiple tasks simultaneous |
| Thread | 线程 | 线程 (thread) | Execution unit |
| Async/Await | 异步/等待 | async/await | Asynchronous programming model |
| Future | 未来值 | Future | Async computation result |
| Executor | 执行器 | 执行器 (executor) | Async task scheduler |
| Runtime | 运行时 | 运行时 (runtime) | Execution environment |
| Tokio | Tokio | Tokio | Async runtime crate |
| Mutex | 互斥锁 | Mutex | Mutual exclusion primitive |
| RwLock | 读写锁 | RwLock | Read-write lock |
| Channel | 通道 | 通道 (channel) | Message passing conduit |
| Send | 发送 | Send trait | Thread-safe transfer trait |
| Sync | 同步 | Sync trait | Thread-shared trait |
| Atomic | 原子操作 | 原子操作 (atomic) | Indivisible operation |

---

## 错误处理 (Error Handling)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Error | 错误 | 错误 (error) | Failure condition |
| Panic | 恐慌 | panic! | Unrecoverable error |
| Unwind | 展开 | 栈展开 (stack unwind) | Stack cleanup on panic |
| Abort | 中止 | 中止 (abort) | Immediate termination |
| Expect | 预期 | expect() | Unwrap with message |
| Unwrap | 解包 | unwrap() | Extract value or panic |
| Match | 匹配 | match | Pattern matching expression |
| Propagate | 传播 | 传播 (propagate) | Pass error to caller |
| Handle | 处理 | 处理 (handle) | Manage error condition |
| Recoverable | 可恢复 | 可恢复错误 (recoverable error) | Handled failure |
| Unrecoverable | 不可恢复 | 不可恢复错误 (unrecoverable error) | Fatal failure |

---

## 工具与生态系统 (Tools & Ecosystem)

| English | 中文 | First Use Format | Notes |
|---------|------|-----------------|-------|
| Cargo | Cargo | Cargo | Rust package manager |
| Crate | 箱 | 箱 (crate) | Compilation unit |
| Package | 包 | 包 (package) | Distribution unit |
| Module | 模块 | 模块 (module) | Code organization unit |
| Workspace | 工作空间 | 工作空间 (workspace) | Multi-crate project |
| Dependency | 依赖 | 依赖 (dependency) | External crate requirement |
| Feature | 特性 | 特性 (feature) | Optional functionality |
| Build | 构建 | 构建 (build) | Compilation process |
| Test | 测试 | 测试 (test) | Verification process |
| Lint | 代码检查 | 代码检查 (lint) | Code quality check |
| Clippy | Clippy | Clippy | Rust linter tool |
| Rustfmt | Rustfmt | rustfmt | Code formatter |
| Doc | 文档 | 文档 (documentation) | Generated documentation |
| Benchmark | 基准测试 | 基准测试 (benchmark) | Performance measurement |
| Profile | 性能分析 | 性能分析 (profiling) | Performance analysis |

---

## 常用短语 (Common Phrases)

| English | 中文 | Usage Context |
|---------|------|---------------|
| Compile time | 编译时 | When code is compiled |
| Runtime | 运行时 | When code executes |
| Type safety | 类型安全 | Prevention of type errors |
| Memory safety | 内存安全 | Prevention of memory errors |
| Zero-cost abstraction | 零成本抽象 | No runtime overhead |
| Fearless concurrency | 无畏并发 | Safe concurrent programming |
| Data race | 数据竞争 | Concurrent memory access bug |
| Undefined behavior | 未定义行为 | Unpredictable program behavior |
| Borrow checker | 借用检查器 | Compiler ownership validator |

---

## Usage Guidelines

**For Chapter Writers**:

1. **First occurrence in chapter**: Always use format `中文 (English)`
   - ✅ Correct: "所有权 (ownership) 是 Rust 的核心概念"
   - ✅ Correct: "让我们了解 borrowing (借用) 的规则"
   - ❌ Wrong: "Ownership 是重要的" (English without translation)
   - ❌ Wrong: "所有权是重要的" (No English reference for searchability)

2. **Subsequent occurrences**: Use Chinese only
   - ✅ "所有权系统确保..."
   - ✅ "借用规则防止..."

3. **Code examples**: Keep English keywords
   - ✅ `let x = 5;` (not `让 X = 5;`)
   - ✅ `fn main()` (not `函数 主 ()`)

4. **Links to external docs**: Use English terms for URLs
   - ✅ Link to `std::string::String` (not `std::字符串::字符串`)

**Glossary Maintenance**:
- Add new terms as discovered during chapter writing
- Ensure all contributors use this single source of truth
- Update if Rust community adopts new Chinese translations

---

## References

- [Rust 指南中文版](https://kaisery.github.io/trpl-zh-cn/)
- [Rust By Example 中文版](https://rustwiki.org/zh-CN/rust-by-example/)
- [Rust 术语表 (官方)](https://doc.rust-lang.org/book/glossary.html)
- [Chinese Rust Community](https://rustcc.cn/)
