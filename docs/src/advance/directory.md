# 文件目录操作 



## 获取 HOME 目录

使用`std::env` 模块来获取 HOME 目录。也可以使用`dotenvy` 从 `.env` 文件中加载环境变量。

安装：
```bash
    cargo add home
    cargo add dotenvy
```
或者配置`Cargo.toml`
```toml
dotenvy = "0.15.7"
home = "0.5.11"
```


以下示例代码，主要实现通过dotenv 加载环境变量，获取当前环境变量全部列表配置，并打印出来。获取 HOME 环境变量。获取 CARGO_MANIFEST_DIR 环境变量。获取 Cargo.toml 所在的目录。


样例代码：

```rust
/// # dotenv_sample
/// use dotenvy crate to load environment variables from .env file.
/// Fails if .env file not found, not readable or invalid.
///    dotenvy::dotenv()?;
///
use dotenvy;
use home;
use std::env::{self, home_dir};
use std::error::Error;

/// dotenv_sample
/// use dotenvy crate to load environment variables from .env file.
fn dotenv_sample() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    // Iterate over all environment variables
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    // 获取 HOME 环境变量
    let home = dotenvy::var("HOME")?;
    println!("HOME: {}", home);

    // 获取 CARGO_MANIFEST_DIR 环境变量
    let cargo_home = env::var("CARGO_MANIFEST_DIR")?;
    println!("CARGO_MANIFEST_DIR: {}", cargo_home);

    // 获取 Cargo.toml 所在的目录
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("Cargo manifest directory (Project root): {}", manifest_dir);

    // 构建相对于 Cargo.toml 的路径
    let data_path = format!("{}/data/data.txt", manifest_dir);
    println!("Data file path: {}", data_path);

    // 使用 PathBuf 构建路径（更推荐）
    use std::path::PathBuf;
    let data_path_buf = PathBuf::from(manifest_dir).join("data").join("data.txt");
    println!("Data file path (PathBuf): {}", data_path_buf.display());

    Ok(())
}

fn current_dir_sample() -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前工作目录
    let current_dir = env::current_dir()?;
    println!("Current directory: {}", current_dir.display());

    // 获取可执行文件的完整路径
    let exe_path = env::current_exe()?;

    // 从路径中提取目录部分
    let exe_dir = exe_path
        .parent()
        .ok_or("Could not get executable directory")?;

    println!("Executable directory: {}", exe_dir.display());

    // 获取 HOME 目录
    env::home_dir().map(|home| println!("Home directory: {}", home.display()));

    // 获取 HOME 目录
    home::home_dir().map(|home| println!("Home directory: {}", home.display()));

    // 获取 HOME 目录
    // 优先使用 home crate
    home::cargo_home().map(|home| println!("Cargo home directory: {}", home.display()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotenv_sample() {
        dotenv_sample().unwrap();
    }

    #[test]
    fn test_current_dir_sample() {
        current_dir_sample().unwrap();
    }
}


```


## 临时文件

在 Rust 中，你可以使用 `std::fs` 模块来创建和管理临时文件。


安装：
```bash
    cargo add tempfile
```
或者配置`Cargo.toml`
```toml
tempfile = "3.10.0"
```

样例代码：

```rust
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use tempfile::{tempdir, NamedTempFile};

/**
 * 临时文件样例，
 * 此创建一个系统临时文件，并进行写入内容，读取临时文件内容。
 */
pub(crate) fn tempfile_sample() {
    // 系统临时目录
    let tmpdir = std::env::temp_dir();
    println!("temp dir location: {:?}", tmpdir);

    let currdir = std::env::current_dir().unwrap();

    println!("current dir: {:?}", currdir);

    // Write
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    println!("tempfile : {:?}", tmpfile);

    write!(tmpfile, "Hello World!").unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);
}

/**
 *
 */
pub(crate) fn temp_namedfile_sample() {
    let text = "Brian was here. Briefly.";

    let home_dir: std::path::PathBuf = env::home_dir().expect("Failed to get home directory");

    // Create a file inside of path  by `NamedTempFile::new_in(paht)`.
    let mut file1 = NamedTempFile::new_in(home_dir).unwrap();
    println!("tempfile : {:?}", { &file1 });

    // Re-open it.
    let mut file2 = file1.reopen().unwrap();

    // Write some test data to the first handle.
    file1.write_all(text.as_bytes()).unwrap();

    // Read the test data using the second handle.
    let mut buf = String::new();
    file2.read_to_string(&mut buf).unwrap();

    assert_eq!(buf, text);
}

/**
 * 临时目录创建，临时文件
 */
pub(crate) fn tempdir_addfile() {
    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("my-temporary-note.txt");

    let mut file = File::create(file_path).unwrap();

    writeln!(file, "Brian was here. Briefly.").unwrap();

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    drop(file);
    dir.close().unwrap();
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_tempfile() {
        tempfile_sample();

        tempdir_addfile();

        temp_namedfile_sample();
    }
}

```