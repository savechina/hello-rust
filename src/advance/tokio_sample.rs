use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpSocket;
use tokio::sync::mpsc;
use tokio::task;

use std::io;
// #[tokio::main]
pub(crate) async fn tokio_server_main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" tokio_server_main ...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        // if !status.load(Ordering::Relaxed) {
        //     break;
        // }

        let (mut socket, _) = listener.accept().await?;

        println!(" tokio_server_main ...connection accept");
        println!("New connection established!");

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

        println!(" tokio_server_main ...connection done");
    }

    Ok(())
}

// #[tokio::main]
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

#[tokio::main]
pub(crate) async fn tokio_client_sample() -> io::Result<()> {
    let mut running = AtomicBool::new(true);

    let server_task = tokio::spawn(async move {
        tokio_server_main();
    });

    println!(" tokio_client_main ... 1");
    println!(" tokio_client_main ... 2");

    println!(" tokio_client_main ... 3");

    while running.load(Ordering::Relaxed) {
        tokio_client_main();

        // Simulate a short delay between client connections
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    // Set running flag to false to stop the server
    running.store(false, Ordering::Relaxed);

    server_task.await?;

    Ok(())
}

#[tokio::main]
async fn tokio_mpsc_sample() {
    // 创建一个异步通道，并指定缓冲区大小（例如 100）
    let (tx, mut rx) = mpsc::channel(100);

    // 使用 tokio::spawn 创建一个异步任务
    task::spawn(async move {
        let val = String::from("hello from tokio");
        // 使用 .await 将发送操作转换为异步操作
        if let Err(_) = tx.send(val).await {
            println!("send error")
        }
    });

    // 在主任务中使用 .await 接收消息
    if let Some(received) = rx.recv().await {
        println!("Got: {}", received);
    }
}

#[tokio::main]
async fn tokio_mpsc_multitask_sample() {
    // 创建一个异步通道，并指定缓冲区大小（例如 100）
    let (tx, mut rx) = mpsc::channel(100);

    // 使用 tokio::spawn 创建多个异步任务
    for i in 0..50 {
        let tx_clone = tx.clone();

        task::spawn(async move {
            let val = String::from("hello from tokio, task: ") + &i.to_string();
            println!("task {} sending:{}", i, val);

            // 使用 .await 将发送操作转换为异步操作
            if let Err(e) = tx_clone.send(val).await {
                println!("task {} send error:{}", i, e)
            }
        });
    }

    // 手动关闭发送端，避免接收端一直等待
    drop(tx); // 非常重要！

    // 在主任务中使用 .await 接收消息
    while let Some(received) = rx.recv().await {
        println!("Received: {}", received);
    }
    println!("End of Task.");
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

    #[ignore]
    #[test]
    fn test_fetures_tokio() {
        tokio_server_main();
    }

    #[test]
    fn test_fetures_tokio_client() {
        // tokio_client_sample();
        tokio_client_main();
    }

    #[test]
    fn test_fetures_tokio_mpsc() {
        tokio_mpsc_sample();
    }

    #[test]
    fn test_fetures_tokio_mpsc_multitask() {
        tokio_mpsc_multitask_sample();
    }
}
