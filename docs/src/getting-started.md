# Getting Started

## 安装 Rust

首先，你需要安装 Rust。你可以从 [Rust 官方网站](https://www.rust-lang.org/) 下载并安装。

Windows 上，你可以从 [Rust 官方网站](https://www.rust-lang.org/) 下 载并安装。

Linux 上，你可以使用包管理器来安装 Rust：

```bash
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    $ rustup update
    # rustup toolchain install stable 
    $ rustup default stable 
```

或者 Ubuntu 下使用以下命令来安装 Rust：

```bash
    $ sudo apt-get install rustup
    $ rustup update
    # rustup toolchain install stable 
    $ rustup default stable 
```

MacOS 上，你可以使用 Homebrew 来安装 Rust：
```bash
    $ brew install rustup
    $ rustup update
    # rustup toolchain install stable 
    $ rustup default stable 
```

## 创建项目

安装完成后，你可以使用以下命令来创建一个新的 Rust项目：

```bash
    $ cargo new hello-rust
```

这将创建一个名为 `hello-rust` 的项目目录，并在其中创建一个 `Cargo.toml` 文件，其中包含了项目的依赖信息。
进入项目目录：  
```bash
    $ cd hello-rust
```

你应该会看到以下目录结构：
```bash
.
├── Cargo.toml
└── src
    └── main.rs
```

现在你可以编辑 `src/main.rs` 文件来编写你的 Rust 代码。cargo 会默认生成一个 `main.rs` 文件，并在其中包含以下代码：
```rust
fn main() {
    println!("Hello, world!");
}
```
`main.rs` 是一个 Rust 程序的入口点。

或者你可以使用`cargo new -lib hello-rust`命令，创建的是一个crates库。Cargo 会创建一个`lib.rs`文件，它包含库的入口点，并且可以被其他 Rust 项目导入和使用。

Cargo.toml 文件内容如下：
```toml
[package]
name = "hello-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```
Cargo.toml 文件中，
* `[package]` 部分定义了项目的名称、版本和 Rust 版本。
* `[dependencies]` 部分定义了项目的依赖。在这个例子中，我们没有添加任何依赖，所以我们不需要添加任何依赖。

## 编译和运行
你可以使用以下命令来编译和运行项目
```bash
    $ cargo build
    $ cargo run
```

这将编译项目并运行 `main.rs` 文件。运行后，你会看到输出 `Hello, world!`。

## 一个完整的 Rust 项目结构

Cargo 推荐的目录结构，如下：
* Cargo.toml 和 Cargo.lock 保存在 package 根目录下
* 源代码放在 src 目录下
* Crate子模块源代码放在 crates 目录下
* 默认的 lib 包根是 src/lib.rs
* 默认的二进制包根是 src/main.rs
    * 其它二进制包根放在 src/bin/ 目录下
* 基准测试 benchmark 放在 benches 目录下
* 示例代码放在 examples 目录下
* 集成测试代码放在 tests 目录下

## 测试你的代码

> [!TIP]
> 良好的编程习惯，一定要写单元测试。下面先认识下，如何编写一个简单的单元测试，后面会有单独的章节来详细介绍如何编写单元测试。可以先使用Copy的技能，照着样例去写，然后慢慢深入理解。接下来去测试你的第一个 Rust 程序吧。

### 单元测试的结构

单元测试通常包含以下部分：
1. **导入模块**：使用 `use` 语句导入需要的模块。
2. **定义测试函数**：使用 `#[test]` 注解定义测试函数。测试函数应该以 `fn` 开头，并且返回 `Result` 或 `Option` 类型。
3. **编写测试代码**：在测试函数中编写实际的测试代码。你可以使用断言来验证函数的行为。
4. **运行测试**：使用 `cargo test` 命令来运行测试。

### 示例：单元测试

```rust

fn main() {
    println!("Hello, world!");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

```

`cargo test` 会运行 `tests` 目录下的所有测试文件。你可以使用以下命令来编译和运行测试，运行上述命令后，你会看到以下输出：
```rust
running 1 test
test tests::it_works ... ok

successes:

successes:
    tests::it_works

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
说明你的单元测试通过了。

在这个例子中，`add` 函数的测试通过了。如果 `add` 函数返回的值不是 4，测试将会失败。你可以通过修改 `add` 函数的返回值来验证这一点。

> [!CAUTION]
> 如果你的测试失败了，你可以通过查看 `test` 目录下的输出文件来找到具体的错误信息。

```rust
running 1 test
test tests::it_works ... FAILED

successes:

successes:

failures:

---- tests::it_works stdout ----

thread 'tests::it_works' panicked at src/main.rs:16:9:
assertion `left == right` failed
  left: 4
 right: 5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:695:5
   1: core::panicking::panic_fmt
             at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panicking.rs:75:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panicking.rs:380:5
   4: hello_rust::tests::it_works
             at ./src/main.rs:16:9
   5: hello_rust::tests::it_works::{{closure}}
             at ./src/main.rs:14:18
   6: core::ops::function::FnOnce::call_once
             at /Users/weirenyan/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


failures:
    tests::it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p hello-rust --bin hello-rust`
```

Rust 返回的测试失败结果信息，是很详细的，所以你一定要详细阅读错误信息，看清楚问题所在。最好的方法是通过错误问题，调试代码并解决这些问题，最终可以成功编译和运行项目，整体过程能快速提升代码能力。

> [!NOTE]  
> 经过上述简单的旅程，我们已经对 Rust 有了初步的了解。接下来，我们将深入探索 Rust 的核心概念和特性。那么，让我们继续前进吧！开始进入 Rust 的世界旅行吧！
