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
`main.rs` 是一个 Rust 程序的入口点。或者创建的是一个crates库，它是一个独立的 Rust 项目，可以被其他 Rust 项目导入和使用。Cargo 会创建一个`lib.rs`文件，它包含库的入口点，并且可以被其他 Rust 项目导入和使用。

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

## 项目结构

Cargo 推荐的目录结构，解释如下：
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

cargo test 会运行所有测试文件。你可以使用以下命令来编译和运行测试：

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

`cargo test` 会运行 `tests` 目录下的所有测试文件。你可以使用以下命令来编译和运行测试：
```
running 1 test
test tests::it_works ... ok

successes:

successes:
    tests::it_works

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

在这个例子中，`add` 函数的测试通过了。如果 `add` 函数返回的值不是 4，测试将会失败。你可以通过修改 `add` 函数的返回值来验证这一点。



> [!NOTE]  
> Highlights information that users should take into account, even when skimming.



> [!TIP]
> Optional information to help a user be more successful.

> [!IMPORTANT]  
> Crucial information necessary for users to succeed.

> [!WARNING]  
> Critical content demanding immediate user attention due to potential risks.

> [!CAUTION]
> Negative potential consequences of an action.