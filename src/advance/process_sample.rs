use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// 获取当前进程 ID
/// 演示基本的进程信息获取
pub fn process_getpid_sample() -> io::Result<()> {
    let current_pid = std::process::id();
    println!("当前进程 ID: {}", current_pid);

    // 获取父进程 ID (Unix 系统)
    #[cfg(unix)]
    {
        let ppid = unsafe { libc::getppid() };
        println!("父进程 ID: {}", ppid);
    }

    Ok(())
}

/// 执行外部命令
/// 演示 Command 的基本用法
pub fn process_command_sample() -> io::Result<()> {
    // 执行 ls -l 命令
    let output = Command::new("ls").arg("-l").arg("/tmp").output()?;

    println!("命令执行成功：");
    println!("状态码：{}", output.status);
    println!("标准输出：\n{}", String::from_utf8_lossy(&output.stdout));

    if !output.stderr.is_empty() {
        println!("标准错误：\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

/// 带标准输入的命令
/// 演示如何向子进程发送数据
pub fn process_with_stdin_sample() -> io::Result<()> {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // 向子进程发送数据
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(b"Hello from parent process!\n")?;
    }

    // 等待子进程完成
    let output = child.wait_with_output()?;
    println!("子进程输出：{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// 进程池示例
/// 演示如何管理多个子进程
pub fn process_pool_sample() -> io::Result<()> {
    let mut children = vec![];

    // 创建 3 个子进程
    for i in 0..3 {
        let child = Command::new("echo")
            .arg(format!("Hello from child {}", i))
            .spawn()?;
        children.push(child);
    }

    // 等待所有子进程完成
    for (i, mut child) in children.into_iter().enumerate() {
        let status = child.wait()?;
        println!("子进程 {} 退出状态：{}", i, status);
    }

    Ok(())
}

/// 进程环境变量
/// 演示如何设置子进程的环境变量
pub fn process_env_sample() -> io::Result<()> {
    let output = Command::new("env")
        .env("MY_VAR", "my_value")
        .env("ANOTHER_VAR", "another_value")
        .env_remove("PATH") // 移除特定环境变量
        .output()?;

    println!("子进程环境变量：");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// 进程工作目录
/// 演示如何设置子进程的工作目录
pub fn process_cwd_sample() -> io::Result<()> {
    let output = Command::new("pwd").current_dir("/tmp").output()?;

    println!("子进程工作目录：");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// 进程超时控制
/// 演示如何设置子进程超时
pub fn process_timeout_sample() -> io::Result<()> {
    let mut child = Command::new("sleep").arg("10").spawn()?;

    // 等待最多 2 秒
    let timeout = Duration::from_secs(2);
    let start = std::time::Instant::now();

    loop {
        if let Some(status) = child.try_wait()? {
            println!("子进程完成：{}", status);
            break;
        }

        if start.elapsed() > timeout {
            println!("超时！终止子进程");
            child.kill()?;
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_getpid() {
        let result = process_getpid_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_command() {
        let result = process_command_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_with_stdin() {
        let result = process_with_stdin_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_pool() {
        let result = process_pool_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_env() {
        let result = process_env_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_cwd() {
        let result = process_cwd_sample();
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_timeout() {
        let result = process_timeout_sample();
        assert!(result.is_ok());
    }
}
