/// # dotenv_sample
/// use dotenvy crate to load environment variables from .env file.
/// Fails if .env file not found, not readable or invalid.
///    dotenvy::dotenv()?;
///
use dotenvy;
use std::env;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotenv_sample() {
        dotenv_sample().unwrap();
    }
}