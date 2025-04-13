use std::env;
use std::fs;
use std::io;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Ensure the server and client are built
    println!("Parent is Building server and client...");
    // 获取临时目录
    let temp_home = env::temp_dir();
    let socket_path = temp_home.join("hello.socket");
    println!("Socket path: {}", socket_path.display());

    // Ensure socket is clean
    let _ = fs::remove_file(socket_path.clone());

    // Spawn server
    let mut server = Command::new("./uds_server")
        .stdout(Stdio::inherit())
        .spawn()?;

    // Wait for server to bind
    thread::sleep(Duration::from_millis(500));

    // Spawn client
    let mut client = Command::new("./uds_client")
        .stdout(Stdio::inherit()) // Inherit stdout to see client output
        .spawn()?;

    // Wait for completion
    client.wait()?;
    server.wait()?;

    // Ensure cleanup (server should handle, but double-check)
    let _ = fs::remove_file(socket_path.clone());

    Ok(())
}
