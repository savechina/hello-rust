# Mio 异步 I/O 库

## 开篇故事

想象你是一位邮局管理员，负责处理成千上万封信件。传统方式是依次处理每封信（**阻塞 I/O**），但这样效率低下。更聪明的做法是同时观察所有信箱，哪个有新信件就处理哪个（**非阻塞 I/O**）。Mio 就是 Rust 中的"智能邮局管理员"——它提供低级的异步 I/O 原语，是 Tokio 等高级运行时的基石。

---

## 本章适合谁

如果你想深入理解 Rust 异步 I/O 的底层实现原理，或者需要构建高性能的网络服务，本章适合你。Mio 是 Tokio、Hyper 等库的核心依赖，理解它能让你更好地掌握 Rust 异步生态。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Mio 的设计理念：为什么它是"底层"的
2. 使用 Poll、Token、Event 实现事件循环
3. 构建非阻塞 TCP 服务器
4. 理解 epoll/kqueue 等系统调用抽象

---

## 前置要求

学习本章前，你需要理解：

- async/await 基础
- TCP 网络编程基础
- 所有权和借用规则

---

## 第一个例子

最简单的 Mio TCP 服务器：

```rust
use mio::{Poll, Token, Event, Events};
use mio::net::TcpListener;

let poll = Poll::new()?;
let listener = TcpListener::bind(addr)?;

poll.registry().register(
    &listener,
    Token(0),
    Interest::READABLE,
)?;
```

完整示例：src/advance/mio_sample.rs

---

## 原理解析

### 事件循环架构

```
┌─────────────┐
│   Poll      │ ← 系统调用 (epoll/kqueue)
│   (注册)    │
└──────┬──────┘
       │
       v
┌─────────────┐
│   Events    │ ← 事件集合
│   (轮询)    │
└──────┬──────┘
       │
       v
┌─────────────┐
│   Handler   │ ← 处理就绪事件
│   (分发)    │
└─────────────┘
```

### 核心概念

- **Poll**: 事件轮询器，封装系统调用
- **Token**: 自定义标识符，关联注册的源
- **Event**: 就绪事件，包含 Token 和就绪状态
- **Interest**: 关注的事件类型（READABLE/WRITABLE）

---

## 常见错误

### 错误 1: 忘记注册事件源

必须先调用 poll.registry().register()

### 错误 2: Token 冲突

确保每个源使用唯一的 Token

### 错误 3: 阻塞操作

在事件循环中执行阻塞操作会降低性能

---

## 知识检查

**问题 1**: Mio 和 Tokio 的关系？

答案: Mio 提供底层 I/O 原语，Tokio 在其上构建运行时

**问题 2**: Interest::READABLE 表示什么？

答案: 关注源的可读事件（有数据到达）

---

## 小结

核心要点：非阻塞I/O、事件循环、Poll/Token/Event、系统调用抽象

完整示例：src/advance/mio_sample.rs
