use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("assets");

/// include_dir sample
pub fn include_dir_sample() {
    let data_file = ASSETS.get_file("data.txt").unwrap();
    let data_content = std::str::from_utf8(data_file.contents()).unwrap();
    println!("data.txt: {}", data_content);

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_include_dir_sample() {
        include_dir_sample();
    }
}
