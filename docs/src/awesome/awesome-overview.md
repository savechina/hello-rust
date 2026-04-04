# 精选实战 (Awesome)

## 📖 学习内容概览

太棒了！你已经完成了 **[基础入门](../basic/basic-overview.md)** 和 **[高级进阶](../advance/advance-overview.md)**！现在你已经掌握了 Rust 的核心概念和生态系统工具。在**精选实战**部分，你将学习如何构建生产级的应用程序，使用真实的框架和最佳实践。

---

## 🎯 你将学到什么

完成本部分学习后，你将能够：

1. **构建生产服务** - 使用服务框架和生命周期管理
2. **依赖注入** - 实现松耦合的模块化架构
3. **消息队列集成** - 使用 MQTT 进行异步通信
4. **模板引擎** - 使用 Tera、Liquid、Pest 生成动态内容
5. **数据处理** - 使用 Polars 进行数据分析
6. **插件系统** - 构建可扩展的插件架构

---

## 📚 章节列表

| 章节 | 说明 | 难度 | 预计时间 |
|------|------|------|---------|
| [服务框架](services.md) | 服务生命周期、注册、管理 | 🔴 困难 | 90 分钟 |
| [依赖注入](dependency_injection.md) | DI 模式、容器、特性 | 🔴 困难 | 90 分钟 |
| [数据库实战](database.md) | SurrealDB、SQLite 高级用法 | 🟡 中等 | 60 分钟 |
| [消息队列](mq.md) | MQTT Broker 和 Client | 🟡 中等 | 60 分钟 |
| [序列生成](sequences.md) | UUID、雪花算法 | 🟡 中等 | 45 分钟 |
| [模板引擎](templates.md) | Tera、Liquid、Pest | 🟡 中等 | 60 分钟 |
| [数据处理](data.md) | Polars 数据分析 | 🟡 中等 | 60 分钟 |
| [插件系统](plugin.md) | 插件架构、动态加载 | 🔴 困难 | 90 分钟 |

---

## 🔗 前置要求

**必须完成**:
- ✅ [基础入门](../basic/basic-overview.md) 所有章节
- ✅ [高级进阶](../advance/advance-overview.md) 核心章节
- ✅ 理解异步编程 (async/await)
- ✅ 熟悉数据库操作 (SQLx 或 Diesel)
- ✅ 了解 Web 框架基础 (Axum 或 Hyper)

**建议具备**:
- 微服务架构基础概念
- 消息队列基础概念
- 设计模式基础（尤其是依赖注入）

---

## 📈 学习路径

```
高级进阶完成
    ↓
服务框架 → 依赖注入
    ↓
数据库实战 → 序列生成 → 数据处理
    ↓
消息队列 → 模板引擎
    ↓
插件系统
    ↓
毕业项目
```

**推荐学习顺序**:

1. **架构基础** (必须先学):
   - 服务框架 → 依赖注入

2. **数据层** (核心技能):
   - 数据库实战 → 序列生成 → 数据处理

3. **通信层** (进阶):
   - 消息队列 → 模板引擎

4. **扩展能力** (深入):
   - 插件系统

---

## ✅ 学习检查点

完成本部分后，你应该能够：

- [ ] 设计服务生命周期管理
- [ ] 实现依赖注入容器
- [ ] 使用 SurrealDB 进行文档数据库操作
- [ ] 集成 MQTT 消息队列
- [ ] 生成唯一标识符 (UUID、雪花)
- [ ] 使用 Tera/Liquid 模板引擎
- [ ] 使用 Polars 进行数据分析
- [ ] 设计和实现插件系统
- [ ] 构建生产级 Rust 应用

---

## 🎓 实践项目

**毕业项目建议**:

1. **微服务架构**:
   - 使用服务框架构建多个微服务
   - 通过消息队列通信
   - 使用依赖注入管理依赖

2. **数据驱动应用**:
   - 使用 SurrealDB 存储数据
   - 使用 Polars 分析数据
   - 使用模板引擎生成报告

3. **可扩展平台**:
   - 实现插件系统
   - 支持动态加载插件
   - 提供服务注册和发现

---

## 📖 完整示例代码

本部分所有示例代码都来自真实项目：

| 样例 | GitHub 链接 |
|------|-----------|
| **Tera 模板** | [tera_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/tera_sample.rs) |
| **Liquid 模板** | [liquid_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/liquid_sample.rs) |
| **Pest 解析器** | [pest_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/pest_sample.rs) |

---

## 🏆 毕业认证

完成所有精选实战章节后，你将获得：

- ✅ Rust 高级编程能力
- ✅ 生产级应用架构经验
- ✅ 真实项目代码样例
- ✅ 完整的作品集项目

---

## ➡️ 下一步

完成精选实战后，你可以：

1. **继续深造**:
   - [算法实现](../algo/algo.md) - 链表、PI 计算
   - [LeetCode 题解](../leetcode/leetcode.md) - 面试准备

2. **实战项目**:
   - 构建完整的 Web 应用
   - 创建开源 Rust 库
   - 贡献 Rust 社区

3. **职业发展**:
   - Rust 后端工程师
   - 系统程序员
   - 区块链开发

---

**准备好了吗？让我们开始 [服务框架](services.md) 的学习！** 🚀
