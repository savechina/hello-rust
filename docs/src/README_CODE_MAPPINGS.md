# 代码示例映射表

**原则**：文档所有代码示例必须对应项目中的真实实现

## 基础章节 (src/basic/)

| 章节         | 源文件                              | 核心示例                           |
| ------------ | ----------------------------------- | ---------------------------------- |
| 变量与表达式 | `expression_sample.rs`              | `variable_bind()`, `number_calc()` |
| 数据类型     | `datatype_sample.rs`                | `string_sample()`, 集合操作         |
| 所有权       | `ownership_sample.rs`               | `gives_ownership()`, 移动语义       |
| 结构体       | `rectangle.rs`                      | `Rectangle`, `area()` 方法          |
| 结构体字段   | `rectangle.rs`                      | 字段访问，可见性                    |
| 结构体方法   | `rectangle.rs`                      | `new()`, `area()`, `double_width()` |
| 枚举         | `traits_sample.rs`                  | `Result`, `Option` 使用              |
| 特征         | `traits_sample.rs`                  | `Printable trait`, 特性继承         |
| 模块         | `module_sample.rs`                  | 模块组织，pub 可见性                 |
| 泛型         | `generic_sample.rs`                 | `add<T>()`, 泛型约束                |
| 闭包         | `closure_sample.rs`                 | `Fn trait`, 环境捕获                 |
| 线程         | `threads_sample.rs`                 | `thread::spawn`, ⚠️ unsafe 警告      |
| 指针         | `pointer_sample.rs`                 | ⚠️ unsafe 原始指针，UTF-8 转换       |
| 条件编译     | `cfg_if_sample.rs`                  | `#[cfg()]` 属性                     |
| 日志         | `logger_sample.rs`                  | 日志记录                            |
| 追踪         | `tracing_sample.rs`                 | 异步追踪                            |
| 可见性       | `visiable_sample.rs`                | pub 控制                             |

## 高级章节 (src/advance/)

| 章节     | 源文件              | 核心示例                      |
| -------- | ------------------- | ----------------------------- |
| 异步     | `tokio_sample.rs`   | TCP server, async/await       |
| 数据库   | `sqlx_sample.rs`    | PostgreSQL, CRUD 操作         |
| ORM      | `diesel_sample.rs`  | 类型安全查询                  |
| Web 服务 | `axum_sample.rs`    | REST API 路由                  |
| HTTP     | `hyper_sample.rs`   | 底层 HTTP 处理                  |
| 序列化   | `json_sample.rs`    | serde_json                    |
| CSV      | `csv_sample.rs`     | CSV 读写                        |
| 零拷贝   | `rkyv_sample.rs`    | 零拷贝序列化                  |

## 算法章节 (src/algo/)

| 章节     | 源文件              | 核心示例           |
| -------- | ------------------- | ------------------ |
| PI 计算 | `calc_pi_sample.rs` | 莱布尼茨公式       |
| 链表     | `linked_list.rs`    | 安全链表实现       |

## LeetCode 章节 (crates/leetcode/)

| 题目        | 源文件             | 算法              |
| ----------- | ------------------ | ----------------- |
| 两数之和    | `solution_0001.rs` | HashMap           |
| 两数相加    | `solution_0002.rs` | 链表操作          |

## 二进制示例 (src/bin/)

| 示例类型        | 源文件                         |
| --------------- | ------------------------------ |
| gRPC 服务器      | `grpc_hello_server.rs`         |
| gRPC 客户端      | `grpc_hello_client.rs`         |
| UDS 服务器       | `uds_server.rs`                |
| UDS 客户端       | `uds_client.rs`                |
| Stdio IPC       | `stdio_parent.rs`, `stdio_child.rs` |
| 进程控制        | `app_sys_ctl.rs`               |

---

## 文档编写指南

当你编写文档章节时：

### ✅ 正确做法

```markdown
// 1. 直接引用项目真实代码
let rect = Rectangle { width: 30, height: 50 };
println!("面积：{}", rect.area());

// 2. 文末标出处
完整示例：`src/basic/rectangle.rs`

// 3. 提取核心逻辑，保持注释
fn area(&self) -> u32 {
    self.width * self.height  // 计算矩形面积
}
```

### ❌ 错误做法

```markdown
// 1. 编造与项目无关的代码
struct FakeExample {
    foo: String,  // 项目中没有的代码
}

// 2. 不标明出处
// (无来源说明)

// 3. 脱离项目空谈概念
```

---

**目标**：读者学完文档后，能直接运行并理解项目的实际代码！
