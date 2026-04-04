# 项目实战指南

**更新日期**: 2026-04-04  
**理念**: 使用项目中实际存在的样例工程学习

---

## 项目原则

本系列教程使用 **Hello Rust 项目中已有的代码作为教学示例**，而不是虚构项目。

**为什么？**
- ✅ 真实可运行的代码
- ✅ 可以直接修改和试验
- ✅ 与章节内容无缝衔接
- ✅ 学完就能用在实际项目中

---

## 基础项目

### 1. Hello Rust 基础演示 🟢

**文件**: `src/bin/basic.rs`  
**代码量**: ~15 行  
**难度**: 🟢 入门

**运行**:
```bash
cargo run --bin basic
```

**学习目标**:
- 熟悉 Rust 项目结构
- 运行第一个 Rust 程序
- 使用 `cargo run` 命令

**动手试试**:
```rust
// 修改 basic.rs 中的输出
println!("Hello, [你的名字]!");
```

---

## 进阶项目

### 2. gRPC 服务器示例 🟡

**文件**: `src/bin/greeter_server.rs`  
**代码量**: ~100 行  
**难度**: 🟡 中级

**前置要求**:
- 理解异步编程
- 了解 gRPC 基本概念

**运行**:
```bash
# 先安装 protoc
# macOS: brew install protobuf
# Linux: apt-get install protobuf-compiler

cargo run --bin greeter_server
```

**学习重点**:
- gRPC 服务定义
- Protocol Buffers
- 异步服务实现

**动手试试**:
1. 添加一个新的 RPC 方法
2. 修改返回格式
3. 添加日志输出

**相关章节**:
- [gRPC 服务](../advance/web/axum.md)
- [异步编程](../advance/async/async.md)

---

### 3. gRPC 客户端 🟡

**文件**: `src/bin/greeter_client.rs`  
**代码量**: ~50 行  
**难度**: 🟡 中级

**运行**:
```bash
# 确保服务器在运行
cargo run --bin greeter_server &

# 运行客户端
cargo run --bin greeter_client
```

**学习重点**:
- gRPC 客户端调用
- 错误处理
- 连接管理

**相关章节**:
- [gRPC 客户端](../advance/web/axum.md)

---

### 4. Unix Domain Socket IPC 🟡

**文件**: `src/bin/uds_server.rs` + `src/bin/uds_client.rs`  
**代码量**: ~60 行  
**难度**: 🟡 中级

**运行**:
```bash
# 在终端 1
cargo run --bin uds_server

# 在终端 2
cargo run --bin uds_client
```

**学习重点**:
- Unix 套接字通信
- 进程间通信 (IPC)
- 错误处理

**相关章节**:
- [IPC 编程](../projects/binaries/README.md)

---

### 5. 标准输入输出 IPC 🟡

**文件**: `src/bin/stdio_parent.rs` + `src/bin/stdio_child.rs`  
**代码量**: ~40 行  
**难度**: 🟡 中级

**运行**:
```bash
cargo run --bin stdio_parent
```

**学习重点**:
- 子进程创建
- 管道通信
- Stdin/Stdout 处理

**相关章节**:
- [进程管理](../advance/system/process.md)

---

## 算法项目

### 6. PI 值计算 🟡

**文件**: `src/algo/calc_pi_sample.rs`  
**代码量**: ~100 行  
**难度**: 🟡 中级

**运行**:
```bash
cd src/algo
rustc calc_pi_sample.rs -o calc_pi
./calc_pi
```

**学习重点**:
- 数值算法
- 循环和迭代
- 精度计算

**相关章节**:
- [算法实战](../algo/algo.md)

---

### 7. LeetCode 题解 🟢

**文件**: `crates/leetcode/src/`  
**代码量**: ~50 行/题  
**难度**: 🟢 入门

**题目列表**:
1. [两数之和](../leetcode/leetcode.md) - HashMap 应用
2. [两数相加](../leetcode/leetcode.md) - 链表操作

**运行**:
```bash
cd crates/leetcode
cargo test
```

**学习重点**:
- 数据结构应用
- 算法实现
- 测试驱动

**相关章节**:
- [LeetCode 题解](../leetcode/leetcode.md)
- [LeetCode 题解](../leetcode/leetcode.md)

---

## 框架实战

### 8. Awesome 框架应用 🔴

**目录**: `crates/awesome/src/`  
**代码量**: ~2000 行  
**难度**: 🔴 高级

**包含**:
- 服务生命周期管理
- 依赖注入
- 数据库连接池
- gRPC 服务

**学习重点**:
- 生产级架构
- 设计模式
- 错误处理最佳实践

**相关章节**:
- [服务框架](../advance/tools/services.md)
- [依赖注入](../advance/tools/services.md)
- [数据库集成](../awesome/database.md)

---

## 项目完成清单

### 基础阶段
- [ ] 1. Hello Rust 基础演示
- [ ] 2. 运行所有示例

### 进阶阶段
- [ ] 3. gRPC 服务器
- [ ] 4. gRPC 客户端
- [ ] 5. UDS IPC
- [ ] 6. Stdio IPC

### 算法阶段
- [ ] 7. PI 值计算
- [ ] 8. LeetCode 两数之和
- [ ] 9. LeetCode 两数相加

### 框架阶段
- [ ] 10. Awesome 框架概览
- [ ] 11. 实现自定义服务
- [ ] 12. 数据库集成实战

---

## 学习建议

### 项目练习流程

1. **阅读相关章节** - 先学习理论知识
2. **运行示例代码** - 确认环境正常
3. **修改代码试验** - 试试改动有什么效果
4. **独立完成扩展** - 按练习建议实现功能

### 遇到问题时

1. 查看章节中的"常见错误"
2. 搜索错误信息
3. 在 [RustCN 论坛](https://rustcc.cn/) 提问
4. 查看其他项目示例

### 进阶路径

```
基础项目 → 进阶项目 → 算法项目 → 框架实战 → 贡献代码
```

---

## 贡献

欢迎贡献更多项目示例！

提交 PR 前确保：
- [ ] 代码可编译运行
- [ ] 添加相关文档
- [ ] 通过测试
- [ ] 符合项目风格

---

**下一步**: 选择一个项目开始吧！🎯
