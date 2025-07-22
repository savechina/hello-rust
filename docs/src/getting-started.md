# Getting Started

## 安装 Rust

首先，你需要安装 Rust。你可以从 [Rust 官方网站](https://www.rust-lang.org/) 下载并安装。

Windows 上，你可以从 [Rust 官方网站](https://www.rust-lang.org/) 下 载并安装。

Linux 上，你可以使用包管理器来安装 Rust：

```bash
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    $ rustup update
```

或者

```bash
    $ sudo apt-get install rustup
```

MacOS 上，你可以使用 Homebrew 来安装 Rust：
```bash
    $ brew install rustup
    $ rustup update
```

## 创建项目

安装完成后，你可以使用以下命令来创建一个新的 Rust项目：

```bash
    $ cargo new hello-rust
```


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