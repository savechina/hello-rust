# Hello Rust 源码 GitHub 链接表

**仓库**: https://github.com/savechina/hello-rust  
**当前 Commit**: `81f7441a4d9dc7912cc18d4170077653655d335d`  
**Branch**: `main` (生产) | `001-rust-tutorial-docs` (开发中)

---

## 基础章节 (Basic)

### 变量与表达式
```markdown
源码：[`src/basic/expression_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/expression_sample.rs)
核心函数：
- `variable_bind()` - 变量绑定
- `number_calc()` - 数值计算
- `add()` - 泛型加法
```

### 所有权
```markdown
源码：[`src/basic/ownership_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs)
核心函数：
- `ownership_sample()` - 所有权示例
- `gives_ownership()` - 转移所有权
- `takes_and_gives_back()` - 获取并返回
```

### 数据类型
```markdown
源码：[`src/basic/datatype_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs)
核心函数：
- `string_sample()` - 字符串处理
- 集合操作 (Vec, HashMap)
- chrono 日期时间
```

### 结构体
```markdown
源码：[`src/basic/rectangle.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/rectangle.rs)
核心代码：
- `struct Rectangle` - 矩形结构体
- `impl Rectangle` - 方法实现
- `RectangleBuilder` - Builder 模式
```

### 特征 (Traits)
```markdown
源码：[`src/basic/traits_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/traits_sample.rs)
核心代码：
- `trait Printable` - 可打印特征
- `struct Person` - 示例结构体
- 特征继承示例
```

### 模块
```markdown
源码：[`src/basic/module_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/module_sample.rs)
核心内容：
- 模块定义
- 可见性控制
- 模块组织
```

### 泛型
```markdown
源码：[`src/basic/generic_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/generic_sample.rs)
核心函数：
- `add<T>()` - 泛型加法
- `add_generic_sample()` - 示例
```

### 闭包
```markdown
源码：[`src/basic/closure_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/closure_sample.rs)
核心示例：
- 基本闭包
- Fn trait 使用
- 环境捕获
```

### 线程
```markdown
源码：[`src/basic/threads_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/threads_sample.rs)
⚠️ **包含 unsafe 代码，仅用于教学**
核心内容：
- `thread::spawn`
- 线程 join
- ⚠️ 危险模式示例
```

### 指针
```markdown
源码：[`src/basic/pointer_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/pointer_sample.rs)
⚠️ **包含不安全代码**
核心内容：
- 原始指针
- ⚠️ `from_utf8_unchecked`
```

### 条件编译
```markdown
源码：[`src/basic/cfg_if_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/cfg_if_sample.rs)
核心内容：
- `#[cfg()]` 属性
- 平台特定代码
```

### 日志
```markdown
源码：[`src/basic/logger_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/logger_sample.rs)
核心内容：
- 日志记录
- 日志级别
```

### 追踪
```markdown
源码：[`src/basic/tracing_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/tracing_sample.rs)
核心内容：
- 异步追踪
- 性能分析
```

### 可见性
```markdown
源码：[`src/basic/visiable_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/visiable_sample.rs)
核心内容：
- pub 控制
- 模块可见性
```

---

## 高级章节 (Advance)

### 异步编程 (Tokio)
```markdown
源码：[`src/advance/tokio_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/tokio_sample.rs)
核心内容：
- TCP 服务器
- 异步运行时
- `tokio::spawn`
```

### 数据库 (SQLx)
```markdown
源码：[`src/advance/sqlx_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/sqlx_sample.rs)
核心内容：
- PostgreSQL 连接
- CRUD 操作
- 异步查询
```

### ORM (Diesel)
```markdown
源码：[`src/advance/diesel_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/diesel_sample.rs)
核心内容：
- 类型安全查询
- Schema 定义
- 关联查询
```

### Web 服务 (Axum)
```markdown
源码：[`src/advance/axum_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/axum_sample.rs)
核心内容：
- REST API 路由
- 请求处理
- 响应格式化
```

### 序列化
```markdown
源码：
- JSON: [`src/advance/json_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/json_sample.rs)
- CSV: [`src/advance/csv_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/csv_sample.rs)
- Rkyv: [`src/advance/rkyv_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/advance/rkyv_sample.rs)
```

---

## 算法章节 (Algo)

### PI 计算
```markdown
源码：[`src/algo/calc_pi_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/algo/calc_pi_sample.rs)
算法：莱布尼茨公式
核心函数：`calculate_pi(steps: usize) -> f64`
```

### 链表
```markdown
源码：[`src/algo/linked_list.rs`](https://github.com/savechina/hello-rust/blob/main/src/algo/linked_list.rs)
内容：安全链表实现
```

---

## LeetCode 章节

### 两数之和
```markdown
源码：[`crates/leetcode/src/solution_0001.rs`](https://github.com/savechina/hello-rust/blob/main/crates/leetcode/src/solution_0001.rs)
算法：HashMap 查找
难度：简单
```

### 两数相加
```markdown
源码：[`crates/leetcode/src/solution_0002.rs`](https://github.com/savechina/hello-rust/blob/main/crates/leetcode/src/solution_0002.rs)
算法：链表操作
难度：中等
```

---

## 二进制示例 (src/bin/)

### gRPC 示例
```markdown
服务器：
- [`src/bin/grpc_hello_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_server.rs)
- [`src/bin/grpc_store_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_store_server.rs)

客户端：
- [`src/bin/grpc_hello_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_client.rs)
- [`src/bin/grpc_store_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_store_client.rs)
```

### IPC 示例
```markdown
Unix Domain Socket:
- 服务器：[`src/bin/uds_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_server.rs)
- 客户端：[`src/bin/uds_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_client.rs)

Stdio IPC:
- 父进程：[`src/bin/stdio_parent.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_parent.rs)
- 子进程：[`src/bin/stdio_child.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_child.rs)
```

---

## Awesome 框架 (crates/awesome/)

### 服务框架
```markdown
源码目录：[`crates/awesome/src/services/framework/`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/framework/)

核心文件：
- [`lifecycle.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/framework/lifecycle.rs) - 生命周期管理
- [`config.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/framework/config.rs) - 配置管理
- [`error.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/framework/error.rs) - 错误处理
```

### 依赖注入
```markdown
源码：
- [`dynmaic_injection_box_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/dynmaic_injection_box_sample.rs) (注意：文件名有 typo)
- [`inventory_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/inventory_sample.rs) - 插件系统
```

### 数据库
```markdown
源码目录：[`crates/awesome/src/database/`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/database/)

核心文件：
- [`surrealdb_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/database/surrealdb_sample.rs) - SurrealDB
- [`sqlite_vec_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/database/sqlite_vec_sample.rs) (⚠️ unsafe)
```

### 消息队列
```markdown
源码：[`crates/awesome/src/mq/rumqtt_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/mq/rumqtt_sample.rs)
内容：MQTT 客户端
```

### 模板引擎
```markdown
源码目录：[`crates/awesome/src/templates/`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/)

核心文件：
- [`tera_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/tera_sample.rs) - Tera
- [`pest_sample.rs`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/pest_sample.rs) - Pest
```

---

## 使用指南

### 在文档中添加链接

```markdown
完整源码：[`src/basic/ownership_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs)
```

### 链接到特定行

```markdown
测试代码：[`ownership_sample.rs#L89-L110`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs#L89-L110)
```

### 验证链接

发布前检查所有链接：
```bash
# 快速检查（示例）
curl -I https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs
```

---

**最后更新**: 2026-04-04  
**维护**: 确保所有链接与当前 commit 一致
