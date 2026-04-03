# 代码示例使用规范

## 原则

**所有文档中的代码示例必须对应项目中的实际实现代码**

## 示例代码映射表

| 章节 | 对应的源代码文件 | 说明 |
|------|-----------------|------|
| 变量与表达式 | `src/basic/expression_sample.rs` | variable_bind(), number_calc(), add() |
| 所有权 | `src/basic/ownership_sample.rs` | ownership_sample(), gives_ownership() |
| 数据类型 | `src/basic/datatype_sample.rs` | string_sample(), bigdecimal_example() |
| 结构体 | `src/basic/rectangle.rs` | Rectangle 结构体实现，area() 方法 |
| 枚举 | `src/basic/traits_sample.rs` | trait 实现中的 enum 使用 |
| 特征 | `src/basic/traits_sample.rs` | traits_sample() 中的 trait 定义 |
| 模块 | `src/basic/module_sample.rs` | 模块组织结构示例 |
| 泛型 | `src/basic/generic_sample.rs` | 泛型函数和结构体 |
| 闭包 | `src/basic/closure_sample.rs` | 闭包定义和 use 关键字 |
| 线程 | `src/basic/threads_sample.rs` | thread::spawn 和 join() |
| 指针 | `src/basic/pointer_sample.rs` | 原始指针示例（含 unsafe 警告） |
| 条件编译 | `src/basic/cfg_if_sample.rs` | cfg 属性使用 |
| 实用工具 | `src/basic/logger_sample.rs` | 日志记录 |
| 异步 | `src/advance/tokio_sample.rs` | Tokio 运行时和 async/await |
| 数据库 | `src/advance/sqlx_sample.rs` | SQLx 异步数据库操作 |
| 宏 | `src/advance/macros_sample.rs` | 宏定义和使用 |

## 更新指南

当你编写文档章节时：

1. **先查看源码**：找到对应的 `*_sample.rs` 文件
2. **直接引用代码**：使用真实代码，不要编造
3. **提取核心逻辑**：如果源码太长，提取最核心的部分
4. **保持注释一致**：如果源码有注释，文档中保留
5. **标明出处**：每章末尾添加"完整示例：`路径/文件名.rs`"

## 示例

### ❌ 错误：编造的代码

```rust
// 不要这样做
struct Person {
    name: String,
    age: u32,
}
```

### ✅ 正确：引用实际代码

```rust
// src/basic/rectangle.rs
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
```

## 实际项目示例

### 算法示例

- **PI 计算**: `src/algo/calc_pi_sample.rs`
- **链表**: `src/algo/linked_list.rs`

### LeetCode 示例

- **两数之和**: `crates/leetcode/src/solution_0001.rs`
- **两数相加**: `crates/leetcode/src/solution_0002.rs`

### IPC 示例

- **gRPC 服务器**: `src/bin/grpc_hello_server.rs`
- **Unix Domain Socket**: `src/bin/uds_server.rs`
- **标准输入输出 IPC**: `src/bin/stdio_parent.rs`

---

**目标**: 让读者学习文档后，可以直接阅读和运行项目中的实际代码！
