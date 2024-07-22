use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub(crate) async fn tokio_server_main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" tokio_server_main ...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => {
                        println!("Client disconnected!");
                        return;
                    }
                    Ok(n) => {
                        println!("received size:{}", n);
                        n
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Process the received data (replace with your logic)
                println!("Received data: {}", String::from_utf8_lossy(&buf[..n]));

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });

        println!(" tokio_server_main ...done");
    }
}

use tokio::net::TcpSocket;

use std::io;

#[tokio::main]
pub(crate) async fn tokio_client_main() -> io::Result<()> {
    println!(" tokio_client_main ...");
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;

    let message = "Hello from the client!";
    stream.write_all(message.as_bytes()).await?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&buf[..n])
    );

    println!(" tokio_client_main ... done");

    Ok(())
}
///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    use tokio::time::sleep;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_fetures_tokio() {
        tokio_server_main();

        println!(" tokio_client_main ... 1");
        println!(" tokio_client_main ... 2");

        println!(" tokio_client_main ... 3");

        tokio_client_main();
    }
}
