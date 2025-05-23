use parking_lot;
use rand::rngs::SmallRng;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::io;
use std::sync::atomic::{self, AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::task;

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

/// tokio client 示例
#[tokio::main]
pub(crate) async fn tokio_client_sample() -> io::Result<()> {
    let mut running = AtomicBool::new(true);

    let server_task = tokio::spawn(async move {
        tokio_server_main();
    });

    println!(" tokio_client_main ... 1");
    println!(" tokio_client_main ... 2");

    println!(" tokio_client_main ... 3");

    // Run the client in a loop
    while running.load(Ordering::Relaxed) {
        // Simulate a short delay between client connections
        tokio_client_main();

        // Simulate a short delay between client connections
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    // Set running flag to false to stop the server
    running.store(false, Ordering::Relaxed);

    server_task.await?;

    Ok(())
}

/// tokio::sync::mpsc 示例
/// 通过创建单个异步任务，实现并发处理
#[tokio::main]
async fn tokio_mpsc_sample() {
    // 创建一个异步通道，并指定缓冲区大小（例如 100）
    let (tx, mut rx) = mpsc::channel(100);

    // 使用 tokio::spawn 创建一个异步任务
    let task = task::spawn(async move {
        let val = String::from("hello from tokio");
        // 使用 .await 将发送操作转换为异步操作
        if let Err(_) = tx.send(val).await {
            println!("send error")
        }
        drop(tx); // 非常重要！
    });

    // 等待异步任务完成
    tokio::join!(task);

    // 在主任务中使用 .await 接收消息
    if let Some(received) = rx.recv().await {
        println!("Got: {}", received);
    }
}

/// 多任务并发处理示例
/// 通过创建多个异步任务，实现并发处理
#[tokio::main]
async fn tokio_mpsc_multitask_sample() {
    // 创建一个异步通道，并指定缓冲区大小（例如 100）
    let (tx, mut rx) = mpsc::channel(10);

    // 使用 tokio::spawn 创建多个异步任务
    for i in 0..50 {
        // 创建一个通道的克隆副本
        let tx_clone = tx.clone();

        // 使用 task::spawn 创建一个异步任务
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

/// tokio::sync::oneshot 示例
/// 通过创建单个异步任务，实现并发处理
#[tokio::main]
async fn tokio_oneshot_sample() {
    // 创建一个异步通道，并指定缓冲区大小（例如 100）
    let (tx, mut rx) = oneshot::channel();

    // 使用 tokio::spawn 创建一个异步任务
    let task = task::spawn(async move {
        let val = String::from("hello from tokio");
        // 使用将发送操作转换为异步操作
        if let Err(_) = tx.send(val) {
            println!("send error")
        }
    });

    // 等待异步任务完成
    tokio::join!(task);

    // 在主任务中使用 .await 接收消息
    if let Ok(received) = rx.await {
        println!("Got: {}", received);
    }
}

/// tokio::sync::RwLock 示例
#[tokio::main]
async fn tokio_rwlock_basic_sample() {
    // 创建一个 RwLock 包装的共享数据
    let data = Arc::new(RwLock::new(0));

    // 创建多个读任务
    let mut read_tasks = Vec::new();
    // 读取数据
    for _ in 0..5 {
        // 使用 Arc 和 clone 来共享数据
        let data_clone = data.clone();
        // 使用 spawn 创建异步任务
        read_tasks.push(tokio::spawn(async move {
            // 获取读锁
            let read_guard = data_clone.read().await; // 获取读锁
            println!("读取数据：{}", *read_guard);
            // 读锁在 guard drop 时自动释放
        }));
    }

    // 创建一个写任务
    let data_clone2 = data.clone();

    // 使用 spawn 创建异步任务
    let write_task = tokio::spawn(async move {
        let mut write_guard = data_clone2.write().await; // 获取写锁

        // 修改数据
        *write_guard += 1;

        println!("写入数据：{}", *write_guard);
        // 写锁在 guard drop 时自动释放
    });

    // 等待所有任务完成
    for read_task in read_tasks {
        read_task.await.unwrap();
    }
    write_task.await.unwrap();

    // 再次读取以验证写入
    let read_guard = data.read().await;
    println!("最终数据：{}", *read_guard);
}

/// tokio::sync::RwLock 示例 , 读写锁的复杂示例，使用rand生成随机数,
/// 模拟多个线程同时读写数据并发操作。
#[tokio::main]
async fn tokio_rwlock_complex_sample() {
    let data = Arc::new(RwLock::new(0));
    let num_tasks = 10;

    let mut handles = Vec::new();

    for i in 0..num_tasks {
        // 创建一个任务来修改数据
        // 使用 Arc 和 clone 来共享数据
        let data_clone = data.clone();

        handles.push(tokio::spawn(async move {
            // 使用 rand::thread_rng() 随机数生成器，是非线程安全的，ThreadRng 缺少Send trait，因此不能跨线程共享。
            // let mut rng = rand::thread_rng(); // 错误用法
            //建议使用StdRng,SmallRng等，它是线程安全的
            let mut rng = StdRng::from_entropy();

            // 使用 Arc 和 RwLock 来实现线程安全的读写操作。
            loop {
                let operation = rng.gen_range(0..2); // 0: 读，1: 写

                if operation == 0 {
                    let read_guard = data_clone.read().await;
                    println!("Task {} 读取：{}", i, *read_guard);
                } else {
                    let mut write_guard = data_clone.write().await;
                    *write_guard += 1;
                    println!("Task {} 写入：{}", i, *write_guard);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }));
    }

    // 让任务运行一段时间
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // 终止所有任务 (在实际应用中需要更优雅的终止方式)
    for handle in handles {
        handle.abort();
    }
}

/// tokio::sync::RwLock 示例 , 读写锁的复杂示例，使用atomic进行原子操作计数取模，进行随机打散模拟读写操作。
#[tokio::main]
async fn tokio_rwlock_complex_atomic_sample() {
    let data = Arc::new(RwLock::new(0));
    let num_tasks = 10;

    let mut handles = Vec::new();
    for i in 0..num_tasks {
        let data_clone = data.clone();

        let atomic_count = Arc::new(AtomicU32::new(i));

        handles.push(tokio::spawn(async move {
            loop {
                let num = atomic_count.fetch_add(1, Ordering::Relaxed);

                let operation = num % 2; // 0: 读，1: 写

                if operation == 0 {
                    let read_guard = data_clone.read().await;
                    println!("Task {} 读取：{}", i, *read_guard);
                } else {
                    let mut write_guard = data_clone.write().await;
                    *write_guard += 1;
                    println!("Task {} 写入：{}", i, *write_guard);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }));
    }

    // 让任务运行一段时间
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 终止所有任务 (在实际应用中需要更优雅的终止方式)
    for handle in handles {
        handle.abort();
    }
}

/// tokio::task::spawn random numbers and print them in a separate task
#[tokio::main]
async fn tokio_random_sample() {
    // 在闭包外部创建 rng，这是错误的！
    // let mut rng = rand::thread_rng; // 移到闭包内部！！！

    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(task::spawn(async move {
            let mut rng = rand::thread_rng(); // 每个任务都有自己的 rng 实例

            let random_number = rng.gen_range(0..100);
            println!("Task {}: Random number = {}", i, random_number);
        }));

        // SmallRng 是线程安全的，可以跨线程共享
        handles.push(task::spawn(async move {
            let mut rng = SmallRng::from_entropy(); // 生成随机数种子

            let random_number = rng.gen_range(0..100);
            println!("Task {}: Random number = {}", i, random_number);

            // 模拟任务执行时间
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }));
    }
    // 模拟任务执行时间
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    for handle in handles {
        handle.await.unwrap();
    }
}

/// tokio::task::spawn_blocking 示例. 使用Parking_lot库的RwLock来实现线程安全的读写操作.
#[tokio::main]
async fn tokio_task_blocking_example() {
    // 创建一个 RwLock 包装的共享数据结构体
    let data = Arc::new(parking_lot::RwLock::new(0));

    let mut handles = Vec::new();

    // 创建多个任务来模拟对共享数据的读操作
    for i in 0..10 {
        // 创建一个任务来模拟对共享数据的写操作
        let data = data.clone();

        // 使用 spawn 创建异步任务
        handles.push(tokio::spawn(async move {
            // 使用 spawn_blocking 来执行阻塞任务
            let result = task::spawn_blocking(move || {
                // 使用 spawn_blocking RwLock 的读锁
                let read_guard = data.read();
                *read_guard
            })
            .await
            .unwrap();
            println!("任务:{},读取数据：{}", i, result);
        }));
    }

    // 等待所有读任务完成,然后读取数据
    let data_read_clone = data.clone();

    // 等待所有读任务完成,然后读取数据
    let data_writer_clone = data.clone();

    // 启动一个写任务
    let write_handle = tokio::spawn(async move {
        task::spawn_blocking(move || {
            // 使用 spawn_blocking RwLock 的写锁
            let mut write_guard = data_writer_clone.write();

            // 写入数据 +1
            *write_guard += 1;

            println!("任务：{},写入数据：{}", "write_handle", *write_guard);

            *write_guard
        })
        .await
        .unwrap();
    });

    // 等待所有读任务完成
    for handle in handles {
        handle.await.unwrap();
    }

    // 等待写任务完成
    write_handle.await.unwrap();

    // 读取数据 并打印结果
    let result = task::spawn_blocking(move || {
        // 使用 spawn_blocking，获取读锁，读取数据
        let read_guard = data_read_clone.read();
        *read_guard
    })
    .await
    .unwrap();

    println!("最终数据：{}", result);
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

    #[test]
    fn test_fetures_tokio_mpsc_multitask2() {
        tokio_mpsc_multitask_sample();
    }

    #[test]
    fn test_fetures_tokio_oneshot() {
        tokio_oneshot_sample();
    }

    #[test]
    fn test_features_rwlock_basic() {
        tokio_rwlock_basic_sample();
    }

    #[test]
    fn test_features_rwlock_complex() {
        tokio_rwlock_complex_sample();
    }

    #[test]
    fn test_features_rwlock_complex_atomic() {
        tokio_rwlock_complex_atomic_sample();
    }

    #[test]
    fn test_features_random_sample() {
        tokio_random_sample();
    }

    /// .
    #[test]
    fn test_features_task_blocking_example() {
        tokio_task_blocking_example();
    }
}
