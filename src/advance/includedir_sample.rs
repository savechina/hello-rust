use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("assets");

/// include_dir sample
pub fn include_dir_sample() {
    // 读取data.txt文件
    let data_file = ASSETS.get_file("data.txt").unwrap();
    let data_content = std::str::from_utf8(data_file.contents()).unwrap();
    println!("data.txt: {}", data_content);
    // 读取images/logo.png文件
    let logo_file = ASSETS.get_file("images/logo.png").unwrap();
    println!("logo.png size: {} bytes", logo_file.contents().len());

    // 遍历所有文件
    for file in ASSETS.files() {
        println!("File path: {:?}", file.path());
    }

    // 遍历所有文件（包括子目录中的文件）
    for file in ASSETS.dirs() {
        println!("Recursive File path: {:?}", file.path());

        file.files().for_each(|f| {
            println!("Recursive File path: {:?}", f.path());
        });
    }
}
/**
includedir_all sample
*/
fn includedir_all_sample() {
    // 使用 traverse_recursively 函数遍历所有文件
    fn traverse_recursively(dir: &Dir) {
        // 遍历当前目录下的所有文件
        for file in dir.files() {
            println!("File path: {:?}", file.path());

            // 区分文本文件和二进制文件
            if let Some(content) = file.contents_utf8() {
                println!("File content:\n{}", content);
            } else {
                println!("File is binary, size: {} bytes", file.contents().len());
                // 这里可以进行二进制文件的处理，例如保存到磁盘
                // std::fs::write(file.path(), file.contents()).unwrap();
            }
            println!("---");
        }

        // 递归遍历子目录
        for subdir in dir.dirs() {
            traverse_recursively(subdir);
        }
    }
    // 递归遍历所有文件
    traverse_recursively(&ASSETS);
}

fn include_bytes_sample() {
    let current_dir = std::env::current_dir().unwrap();
    println!("current dir: {:?}", current_dir);

    let data = include_bytes!("../../assets/data.txt");
    println!("include_bytes File content (as bytes): {:?}", data);

    // 将字节数组转换为字符串（需要处理可能的 UTF-8 编码错误）
    let content = std::str::from_utf8(data).unwrap();
    println!("include_bytes File content (as string): {}", content);

    let data_path = current_dir.join("assets/data.txt");

    if let Some(s) = data_path.to_str() {
        println!("Data path: {}", s);
    } else {
        println!("路径包含无效的 UTF-8 字符");
    }

    fn os_str_to_str(s: &OsStr) -> Option<&str> {
        std::str::from_utf8(s.as_encoded_bytes()).ok()
    }

    let os_str = data_path.into_os_string();
    match os_str_to_str(&os_str) {
        Some(s) => println!("Data path: {}", s),
        None => println!("路径包含无效的 UTF-8 字符"),
    }

    let data = include_str!("../../assets/data.txt");
    println!("include_str File content (as str): {:?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_include_dir_sample() {
        include_dir_sample();
    }

    #[test]
    fn test_include_alldir_sample() {
        includedir_all_sample();
    }

    #[test]
    fn test_include_bytes_sample() {
        include_bytes_sample();
    }
}
