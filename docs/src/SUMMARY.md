# Summary

[关于 Hello Rust](./about-hello.md)
[简介](./introduction.md)
[快速开始](./getting-started.md)

# 基础部分 (Basic)

- [基础入门](./basic/basic-overview.md)
    - [变量与表达式](./basic/expression.md)
    - [基础数据类型](./basic/datatype.md)
    - [了解所有权](./basic/ownership.md)
    - [结构体](./basic/struct.md)
        - [结构体字段](./basic/struct-fields.md)
        - [结构体方法](./basic/struct-methods.md)
    - [枚举](./basic/enums.md)
    - [特征 (Traits)](./basic/trait.md)
    - [泛型](./basic/generic.md)
    - [闭包](./basic/closure.md)
    - [模块系统](./basic/module.md)
    - [线程与并发](./basic/threads.md)
    - [条件编译](./basic/cfg_if.md)
    - [指针与不安全代码](./basic/pointer.md)
    - [日志记录](./basic/logger.md)
    - [追踪 (Tracing)](./basic/tracing.md)
    - [可见性控制](./basic/visiable.md)

# 进阶部分 (Advance)

<!-- 异步编程 -->
- [高级进阶](./advance/advance-overview.md)
  - [异步编程](./advance/async/async.md) - Tokio 运行时和 async/await 语法
    - [Futures 异步编程](./advance/async/futures.md) - Future trait 和组合子
    - [并行计算](./advance/async/rayon.md) - 数据并行处理
    - [MIO](./advance/async/mio.md) - 底层 I/O 库
    - [CycleRC](./advance/async/cyclerc.md) - 引用计数循环检测
  - [数据库操作](./advance/database/database.md) - SQLx 异步数据库编程
    - [数据库 ORM](./advance/database/diesel.md) - Diesel ORM 框架
    - [SQLx](./advance/database/sqlx.md) - SQLx 异步查询
  - [Web 开发](./advance/web/axum.md) - Web 开发
    - [Web 框架](./advance/web/axum.md) - Axum Web 服务构建
    - [HTTP 库](./advance/web/hyper.md) - Hyper HTTP 底层库
    - [AI 集成](./advance/web/ollama.md) - Ollama 本地 AI 集成
  - [数据处理](./advance/data/json.md) - 数据处理
    - [JSON 序列化](./advance/data/json.md) - JSON 序列化/反序列化
    - [CSV 处理](./advance/data/csv.md) - CSV 文件读写
    - [零拷贝序列化](./advance/data/rkyv.md) - Rkyv 零拷贝优化
    - [序列化基础](./advance/data/serialization.md) - 序列化概念
  - [系统编程](./advance/system/directory.md) - 系统编程
    - [文件与目录操作](./advance/system/directory.md) - 文件系统操作
    - [临时文件](./advance/system/tempfile.md) - 临时文件管理
    - [内存映射](./advance/system/memmap.md) - 内存映射文件
    - [环境变量](./advance/system/dotenv.md) - 环境变量配置
    - [字节处理](./advance/system/bytes.md) - 字节数据处理
    - [Cow 类型](./advance/system/cow.md) - Clone-on-Write 优化
    - [进程管理](./advance/system/process.md) - 进程控制和管理
    - [系统信息](./advance/system/sysinfo.md) - 系统信息获取
    - [资源嵌入](./advance/system/includedir.md) - 编译时资源嵌入
  - [测试与模拟](./advance/testing/test.md) - 测试与模拟
    - [测试基础](./advance/testing/test.md) - 测试基础
    - [模拟测试](./advance/testing/mock.md) - Mock 模拟测试
    - [测试框架](./advance/testing/rspec.md) - RSpec 行为驱动测试
    - [派生宏](./advance/testing/getset.md) - Getters/Setters 派生
    - [宏编程](./advance/testing/macros.md) - 声明宏和过程宏
    - [类型别名](./advance/testing/typealias.md) - 类型别名简化
  - [其他工具](./advance/tools/error-handling.md) - 其他工具
    - [错误处理](./advance/tools/error-handling.md) - 错误处理最佳实践
    - [对象存储](./advance/tools/objectstore.md) - 对象存储接口
    - [服务框架](./advance/tools/services.md) - 服务框架基础

# 实战精选 (Awesome)

- [实战精选](./awesome/awesome-overview.md)
    - [数据库高级应用](./awesome/database.md)
    - [微服务架构](./awesome/services.md)
        - [服务依赖注入](./awesome/dependency_injection.md)
        - [插件系统](./awesome/plugin.md)
    - [序列生成](./awesome/sequences.md)
    - [消息队列](./awesome/mq.md)
    - [模板引擎](./awesome/templates.md)

# 算法与练习

- [算法实现](./algo/algo.md)
- [LeetCode 题解](./leetcode/leetcode.md)

# 特色功能

- [代码片段速查](./quick_reference/snippets.md)
- [学习路径技能树](./learning_path.md)
- [知识检查题库](./quiz/index.md)
- [项目实战](./projects/README.md)
    - [命令行待办事项](./projects/todo-cli/README.md)
    - [简易 HTTP 服务器](./projects/http-server/README.md)
    - [多线程爬虫](./projects/web-scraper/README.md)

# 社区与贡献

- [贡献指南](./CONTRIBUTING.md)

# 附录

- [术语表](./glossary.md)
- [常见问题 FAQ](./faq.md)
- [更新日志](./CHANGELOG.md)
