# Hello Rust
 English | [简体中文](README_zh.md)

[**Hello Rust Tutorial**](https://renyan.org/hello/rust) | [**GitHub Pages**](https://savechina.github.io/hello-rust/)

A comprehensive sample project and tutorial for learning the Rust programming language, from basic syntax to advanced production-grade applications.

## 📖 Online Tutorial

- 🌐 **Official Tutorial**: [renyan.org/hello/rust](https://renyan.org/hello/rust)
- 📚 **GitHub Pages**: [savechina.github.io/hello-rust](https://savechina.github.io/hello-rust/)

## 🚀 Quick Start

```bash
# Clone the project
git clone https://github.com/savechina/hello-rust.git
cd hello-rust

# Run basic examples
cargo run

# Run specific binary examples
cargo run --bin grpc_hello_server
```

## 📦 Project Modules

### Basic

Core Rust syntax and concepts for beginners:

| Module | Content |
|--------|---------|
| [Variables & Expressions](docs/src/basic/expression.md) | Variable binding, mutability, basic expressions |
| [Data Types](docs/src/basic/datatype.md) | Integers, floats, booleans, chars, collections, date/time |
| [Ownership](docs/src/basic/ownership.md) | Ownership rules, move semantics, borrowing & references |
| [Lifetimes](docs/src/basic/lifetimes.md) | Lifetime annotations, borrow checker, dangling references |
| [Structs](docs/src/basic/struct.md) | Struct definitions, methods, associated functions |
| [Enums](docs/src/basic/enums.md) | Enum definitions, pattern matching, Option/Result |
| [Traits](docs/src/basic/trait.md) | Trait definitions, implementations, polymorphism |
| [Trait Objects](docs/src/basic/trait-objects.md) | Dynamic dispatch, dyn Trait, vtable mechanism |
| [Generics](docs/src/basic/generic.md) | Generic functions, monomorphization, trait bounds |
| [Closures](docs/src/basic/closure.md) | Closure syntax, environment capture, Fn/FnMut/FnOnce |
| [Module System](docs/src/basic/module.md) | Module organization, visibility, use statements |
| [Threads & Concurrency](docs/src/basic/threads.md) | Thread spawning, channels, Mutex, Arc |

### Advance

Deep dive into advanced Rust features and ecosystem:

| Module | Content |
|--------|---------|
| [Smart Pointers](docs/src/advance/smart-pointers.md) | Box, Rc, RefCell, Arc, Weak, reference cycles |
| [Error Handling](docs/src/advance/error-handling/error-handling.md) | Result, ?, thiserror, anyhow, error propagation |
| [Advanced Traits](docs/src/advance/advanced-traits.md) | Associated types, fully qualified syntax, Newtype pattern |
| [Iterators](docs/src/advance/iterators.md) | map, filter, collect, fold, custom iterators |
| [Async Programming](docs/src/advance/async/async.md) | Tokio runtime, async/await, Future trait |
| [Database](docs/src/advance/database/database.md) | SQLx async queries, Diesel ORM |
| [Web Development](docs/src/advance/web/axum.md) | Axum web framework, Hyper HTTP, gRPC |
| [Data Processing](docs/src/advance/data/serialization.md) | Serde serialization, JSON, CSV, Rkyv zero-copy |
| [System Programming](docs/src/advance/system/directory.md) | File operations, memory mapping, process management, atomics |
| [Testing](docs/src/advance/testing/test.md) | Unit tests, integration tests, Mock, nextest |
| [Macros](docs/src/advance/testing/macros.md) | Declarative macros, procedural macros, hygiene |

### Algorithm & Practice

| Module | Content |
|--------|---------|
| [Algorithms](docs/src/algo/algo.md) | Linked list, pi calculation |
| [LeetCode Solutions](docs/src/leetcode/leetcode.md) | Two Sum, Add Two Numbers |
| [Awesome Examples](docs/src/awesome/awesome-overview.md) | Service framework, message queues, template engines, DI |

## 🛠️ Tech Stack

- **Rust 2024 Edition**
- **Async Runtime**: Tokio
- **Web Frameworks**: Axum, Hyper, Tonic (gRPC)
- **Databases**: SQLx, Diesel
- **Serialization**: Serde, Rkyv
- **Testing**: Mockall, rspec, nextest

## 📋 Project Structure

```
hello-rust/
├── src/
│   ├── basic/          # Basic tutorial example code
│   ├── advance/        # Advanced example code
│   ├── algo/           # Algorithm implementations
│   └── bin/            # 15 runnable binary examples (gRPC, UDS, Stdio IPC)
├── crates/
│   ├── awesome/        # Production-grade service framework (DI, Consul, gRPC)
│   ├── leetcode/       # LeetCode solutions
│   └── macros/         # Procedural macro examples (#[log])
└── docs/               # mdBook tutorial documentation (Chinese)
```

## 📝 License

MIT License
