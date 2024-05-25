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
