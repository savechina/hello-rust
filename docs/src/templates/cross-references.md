# 交叉引用基础设施 (Cross-Reference Infrastructure)

**Purpose**: Standard formatting for all links and references across documentation chapters

**Created**: 2026-04-03  
**Branch**: `001-rust-tutorial-docs`  
**Applies to**: All chapters in `docs/src/`

---

## 内部链接格式 (Internal Linking)

### 章节间链接

**格式**:
```markdown
章节标题
```

**示例**:
```markdown
继续之前，请先阅读 了解所有权。

这个模式在 特征 中有进一步扩展。

参考 结构体 章节了解更多细节。
```

**目录导航**:
```markdown
- [基础入门](../basic/basic-overview.md)
- [高级进阶](../advance/advance-overview.md)
- [精选实战](../awesome/awesome-overview.md)
```

### 前置知识链接

**位置**: 每章第 4 节"前置要求"

**格式**:
```markdown
学习本章前，你需要理解：
- 变量表达式 - 变量绑定和基础语法
- 数据类型 - 基本类型和复合类型
- 所有权 - 所有权系统和移动语义
```

---

## 外部引用格式 (External References)

### Rust 官方资源

**Rust Book**:
```markdown
[Rust 编程指南中文版](https://kaisery.github.io/trpl-zh-cn/)
[Rust Programming Language (English)](https://doc.rust-lang.org/book/)
```

**Standard Library**:
```markdown
[`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html)
[`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
[`Result<T, E>`](https://doc.rust-lang.org/std/result/enum.Result.html)
```

**Rust By Example**:
```markdown
[Rust By Example 中文版](https://rustwiki.org/zh-CN/rust-by-example/)
[Rust By Example (English)](https://doc.rust-lang.org/rust-by-example/)
```

### Crate 文档

**docs.rs 格式**:
```markdown
[`tokio`](https://docs.rs/tokio/latest/tokio/)
[`serde`](https://docs.rs/serde/latest/serde/)
[`axum`](https://docs.rs/axum/latest/axum/)
```

**带版本号** (如果需要锁定):
```markdown
[`tokio` 1.35.0](https://docs.rs/tokio/1.35.0/tokio/)
```

### 社区资源

**Chinese Rust Community**:
```markdown
[Chinese Rust User Group](https://rustcc.cn/)
[Rust 中文社区](https://rustcc.cn/)
```

**Forums**:
```markdown
[Rust Users Forum](https://users.rust-lang.org/)
[/r/rust - Reddit](https://www.reddit.com/r/rust/)
```

---

## 代码文件引用 (Code File References)

### 示例代码位置

**格式**:
```markdown
> **完整示例**: `文件路径`
```

**示例**:
```markdown
> **完整示例**: `src/basic/ownership_sample.rs`

> **完整示例**: `src/advance/tokio_sample.rs` (第 13-64 行)

> **完整示例**: `crates/awesome/src/services/framework/lifecycle.rs`
```

### 代码片段引用

**带行号**:
```markdown
如 `src/basic/ownership_sample.rs` 的第 11-29 行所示...

参见 `src/advance/sqlx_sample.rs` 第 45-78 行的数据库连接示例。
```

**多文件**:
```markdown
这个示例涉及两个文件:
- `src/bin/greeter_server.rs` - gRPC 服务器实现
- `src/bin/greeter_client.rs` - gRPC 客户端实现
```

---

## 术语表引用 (Glossary References)

**首次出现格式**:
```markdown
所有权 (ownership) - Rust 的核心概念，控制资源使用的规则
```

**后续出现**:
```markdown
所有权系统确保...
```

**链接到术语表**:
```markdown
关于 [所有权](../glossary.md#核心概念-core-concepts) 的更多细节...

参见 [ borrow](../glossary.md#核心概念-core-concepts) 章节。
```

---

## 章节结构引用

### 节内跳转

**使用锚点**:
```markdown
跳到 [常见错误](#常见错误-common-mistakes) 节。

参考 [小结](#小结-summary) 的快速回顾。
```

### 跨章节跳转

**完整路径**:
```markdown
查看 异步编程 的 Tokio 运行时部分。
```

---

## 警告和提示框

### 警告框格式

```markdown
> ⚠️ **警告**: 这是危险操作
> 
> 详细说明和替代方案...
```

### 提示框

```markdown
> 💡 **提示**: 这是一个有用的技巧
> 
> 具体内容...
```

### 注意框

```markdown
> 📝 **注意**: 重要但不紧急的信息
> 
> 详细说明...
```

### 安全警告

```markdown
> 🔴 **安全警告**: 这是不安全代码
> 
> 为什么危险，何时安全，替代方案...
```

---

## 脚注和尾注

**脚注格式**:
```markdown
这是一个技术细节。[^1]

[^1]: 脚注内容，提供额外信息但不打断主流程。
```

**尾注** (章节末尾):
```markdown
---

## 参考资料

1. Rust Programming Language, Chapter 4
2. Tokio Documentation, Runtime section
3. Chinese Rust Community Guidelines
```

---

## 图表引用

### 图表标题格式

```markdown
**图 1-1**: 所有权转移示意图

**图 2-3**: 异步执行器工作流程
```

### 图内引用

```markdown
如图 1-1 所示，所有权从 `s1` 转移到 `s2`...

参见图 3-2 了解 tokio 运行时架构。
```

---

## 表格引用

### 表格标题

```markdown
**表 1-1**: 基本数据类型对比

**表 4-3**: 异步运行时性能对比
```

### 表内引用

```markdown
如表 1-1 所示，`String` 和 `&str` 有以下区别...
```

---

## 一致性检查清单

发布章节前检查:

- [ ] 所有内部链接指向存在的文件
- [ ] 外部链接是最新版本 (检查 docs.rs 链接)
- [ ] 代码文件路径正确且文件存在
- [ ] 术语首次出现有中英文对照
- [ ] 警告/提示框格式一致
- [ ] 图表和表格有编号和标题
- [ ] 前置知识章节链接正确
- [ ] 前进链接到后续章节 (如果有)

---

## 自动化工具

**未来增强** (TODO):

```bash
# 检查断链
cargo install linkcheck
linkcheck docs/src/

# 验证代码示例可编译
cargo test --workspace

# 检查 mdBook 构建
mdbook build
```

---

## 维护指南

**更新外部链接**:
- 每季度检查一次官方文档链接
- crate 版本更新时更新 docs.rs 链接
- 记录链接失效报告到 GitHub issues

**添加新资源**:
- 新 crate 发布且质量优秀时添加到推荐列表
- 社区发现的好资源可以收录
- 保持链接列表精简，只收录最佳资源

---

**最后更新**: 2026-04-03  
**维护者**: Hello Rust Documentation Team
