use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;
use std::{env, thread};

fn main() -> io::Result<()> {
    println!("Client is starting...");
    // 获取临时目录
    let temp_home = env::temp_dir();
    let socket_path = temp_home.join("hello.socket");

    println!("Socket path: {}", socket_path.display());

    let messages = vec!["hello", "world", "rust", "done"];

    // Connect to server
    let mut stream = UnixStream::connect(socket_path)?;

    for msg in messages {
        // Send binary: length prefix + payload
        let payload = msg.as_bytes();
        stream.write_all(&u32::to_be_bytes(payload.len() as u32))?;
        stream.write_all(payload)?;
        stream.flush()?;

        // Read response
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)?;

        let length = u32::from_be_bytes(length_bytes) as usize;
        let mut resp_payload = vec![0u8; length];

        stream.read_exact(&mut resp_payload)?;

        let resp_str = String::from_utf8(resp_payload).expect("Invalid UTF-8");
        println!("Client got: {}", resp_str);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
