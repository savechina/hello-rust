use std::env::{self};
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    loop {
        // Read length prefix
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes) as usize;

        // Read payload
        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;
        let message = String::from_utf8(payload).expect("Invalid UTF-8");

        // Process: reverse string or handle "done"
        let response = if message == "done" {
            // Send a special response for "done" to signal closure
            String::from("ok")
        } else {
            message.chars().rev().collect()
        };

        // Send response
        let resp_payload = response.as_bytes();
        stream.write_all(&u32::to_be_bytes(resp_payload.len() as u32))?;
        stream.write_all(resp_payload)?;
        stream.flush()?;

        if message == "done" {
            break;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Server is starting...");
    // 获取临时目录
    let temp_home = env::temp_dir();
    let socket_path = temp_home.join("hello.socket");

    println!("Socket path: {}", socket_path.display());

    // Clean up existing socket
    let _ = fs::remove_file(&socket_path);

    // Create and bind socket
    let listener = UnixListener::bind(&socket_path)?;

    // Accept one client
    match listener.accept() {
        Ok((stream, _addr)) => handle_client(stream)?,
        Err(e) => eprintln!("Accept failed: {}", e),
    }

    // Cleanup
    fs::remove_file(socket_path)?;
    Ok(())
}
