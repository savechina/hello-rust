# IPC 与分布式示例

本部分涵盖项目中 `src/bin/` 目录下的 15 个二进制示例，包括 gRPC 服务、Unix Domain Socket 通信、标准输入输出 IPC 和进程控制。

---

## gRPC 示例

### Hello gRPC 服务

**文件**: [`grpc_hello_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_server.rs) (18 行)

```bash
cargo run --bin grpc_hello_server
```

**学习目标**:
- 使用 tonic 构建 gRPC 服务器
- 定义 Protocol Buffer 服务
- 实现 RPC 方法

### Hello gRPC 客户端

**文件**: [`grpc_hello_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_hello_client.rs) (13 行)

```bash
cargo run --bin grpc_hello_client
```

**学习目标**:
- 连接 gRPC 服务器
- 发送 RPC 请求
- 处理响应

### Greeter 服务

**文件**: [`greeter_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/greeter_server.rs) (98 行)

**文件**: [`greeter_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/greeter_client.rs) (59 行)

```bash
# 先启动服务器
cargo run --bin greeter_server

# 再启动客户端
cargo run --bin greeter_client
```

**学习目标**:
- 完整的 gRPC 服务实现
- 请求/响应模式
- 错误处理

### gRPC Store 服务

**文件**: [`grpc_store_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_store_server.rs) (19 行)

**文件**: [`grpc_store_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/grpc_store_client.rs) (145 行)

```bash
cargo run --bin grpc_store_server
cargo run --bin grpc_store_client
```

**学习目标**:
- 状态ful gRPC 服务
- 键值存储模式
- 复杂 RPC 交互

---

## Unix Domain Socket (UDS) 示例

### UDS 服务器

**文件**: [`uds_server.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_server.rs) (62 行)

```bash
cargo run --bin uds_server
```

**学习目标**:
- Unix Domain Socket 服务器
- 本地进程间通信
- 异步接受连接

### UDS 客户端

**文件**: [`uds_client.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_client.rs) (42 行)

```bash
cargo run --bin uds_client
```

**学习目标**:
- 连接 UDS 服务器
- 发送/接收消息

### UDS 父进程

**文件**: [`uds_parent.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/uds_parent.rs) (40 行)

```bash
cargo run --bin uds_parent
```

**学习目标**:
- 父进程创建 UDS
- 管理子进程通信

---

## 标准输入输出 IPC 示例

### Stdio 父进程

**文件**: [`stdio_parent.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_parent.rs) (38 行)

```bash
cargo run --bin stdio_parent
```

**学习目标**:
- 生成子进程
- 通过 stdin/stdout 通信
- 等待子进程完成

### Stdio 子进程

**文件**: [`stdio_child.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/stdio_child.rs) (20 行)

```bash
# 通常由 stdio_parent 启动
cargo run --bin stdio_child
```

**学习目标**:
- 从 stdin 读取
- 向 stdout 写入
- 作为子进程运行

---

## 进程控制示例

### 系统控制 (SysCtl)

**文件**: [`app_sys_ctl.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/app_sys_ctl.rs) (286 行)

```bash
cargo run --bin app_sys_ctl
```

**学习目标**:
- 系统管理工具
- 进程生命周期管理
- 信号处理

### Nix 控制 (NixCtl)

**文件**: [`app_nix_ctl.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/app_nix_ctl.rs) (282 行)

```bash
cargo run --bin app_nix_ctl
```

**学习目标**:
- 使用 nix crate 进行系统调用
- 进程组和会话管理
- 守护进程模式

---

## 基础演示

### Basic 演示

**文件**: [`basic.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/basic.rs) (22 行)

```bash
cargo run --bin basic
```

**学习目标**:
- 运行第一个 Rust 程序
- 理解 main 函数

### Advance 演示

**文件**: [`advance.rs`](https://github.com/savechina/hello-rust/blob/main/src/bin/advance.rs) (90 行)

```bash
cargo run --bin advance
```

**学习目标**:
- 调用基础/进阶示例
- 理解模块调用

---

## 运行所有示例

```bash
# 列出所有可用的二进制
cargo build --bins

# 运行特定二进制
cargo run --bin <name>

# 运行所有测试
cargo test --workspace
```

---

## 相关章节

- [gRPC 服务](../advance/web/axum.md) - Web 框架
- [进程管理](../advance/system/process.md) - 进程控制
- [异步编程](../advance/async/async.md) - 异步基础

> 💡 **提示**: UDS 示例仅在 Unix 系统上可用。Windows 用户可以使用 stdio IPC 示例。
