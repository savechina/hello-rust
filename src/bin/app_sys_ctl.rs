use clap::{Parser, Subcommand};
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Body;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use hyper::service::HttpService;
use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::pin;
// use hyper::{Body, Request, Response};
use log::{error, info};
use rkyv::ser;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use sysinfo::{Pid, System};
use tokio::signal;
use tokio::sync::oneshot;

#[derive(Parser)]
#[command(name = "my-app")]
#[command(about = "A simple app with start and stop commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the application
    Start,
    /// Stop the application
    Stop,
}

const PID_FILE: &str = "my-app.pid";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 解析命令行参数
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => start_app().await?,
        Commands::Stop => stop_app()?,
    }

    Ok(())
}

async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    // 检查是否已经运行
    if Path::new(PID_FILE).exists() {
        error!("Application is already running. Check {}", PID_FILE);
        return Err("Application already running".into());
    }

    // 保存当前进程的 PID
    let pid = std::process::id();
    File::create(PID_FILE)?.write_all(pid.to_string().as_bytes())?;

    info!("Application started with PID: {}", pid);

    // 设置服务器地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 创建一个 oneshot 通道用于优雅关机
    let (tx, rx) = oneshot::channel::<()>();

    // 定义简单的 HTTP 服务

    // async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    //     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    // }

    let hello = |_: Request<Incoming>| async {
        Ok::<_, Infallible>(Response::new(Full::new(Bytes::from("Hello, World!"))))
    };

    // 启动 HTTP 服务器

    let listener = TcpListener::bind(addr).await?;

    info!("Server running on http://{}", addr);

    let connection_timeouts = vec![Duration::from_secs(5), Duration::from_secs(2)];

    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Clone the connection_timeouts so they can be passed to the new task.
        let connection_timeouts_clone = connection_timeouts.clone();

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Pin the connection object so we can use tokio::select! below.
            let conn = http1::Builder::new().serve_connection(io, service_fn(hello));
            pin!(conn);

            // Iterate the timeouts.  Use tokio::select! to wait on the
            // result of polling the connection itself,
            // and also on tokio::time::sleep for the current timeout duration.
            for (iter, sleep_duration) in connection_timeouts_clone.iter().enumerate() {
                println!("iter = {} sleep_duration = {:?}", iter, sleep_duration);

                tokio::select! {
                    res = conn.as_mut() => {
                        // Polling the connection returned a result.
                        // In this case print either the successful or error result for the connection
                        // and break out of the loop.
                        match res {
                            Ok(()) => println!("after polling conn, no error"),
                            Err(e) =>  println!("error serving connection: {:?}", e),
                        };
                        break;
                    }
                    _ = tokio::time::sleep(*sleep_duration) => {
                        // tokio::time::sleep returned a result.
                        // Call graceful_shutdown on the connection and continue the loop.
                        println!("iter = {} got timeout_interval, calling conn.graceful_shutdown", iter);
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }
        });
    }

    // // 保存服务器任务
    // let server_task = tokio::spawn(async move {
    //     if let Err(e) = server.await {
    //         error!("Server error: {}", e);
    //     }
    // });

    // 监听系统信号以实现优雅关机
    signal::ctrl_c().await?;
    info!("Received Ctrl+C, initiating graceful shutdown...");

    // 发送关机信号
    let _ = tx.send(());

    // 等待服务器任务完成
    // server_task.await?;

    // 删除 PID 文件
    fs::remove_file(PID_FILE)?;
    info!("Application stopped successfully");

    Ok(())
}

fn stop_app() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 PID 文件
    if !Path::new(PID_FILE).exists() {
        error!("No running application found. PID file does not exist.");
        return Err("No running application".into());
    }

    let pid_str = fs::read_to_string(PID_FILE)?;
    let pid: u32 = pid_str.trim().parse()?;

    // 使用 sysinfo 查找并终止进程
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(Pid::from(pid as usize)) {
        // 发送 SIGTERM 信号
        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid as NixPid;
            kill(NixPid::from_raw(pid as i32), Signal::SIGTERM)?;
        }

        #[cfg(not(unix))]
        {
            error!("Graceful shutdown not fully supported on non-Unix systems.");
            process.kill(); // 直接杀死进程（Windows 不支持 SIGTERM）
        }

        info!("Sent stop signal to PID: {}", pid);
        fs::remove_file(PID_FILE)?;
    } else {
        error!("No process found with PID: {}", pid);
        fs::remove_file(PID_FILE)?; // 清理无效的 PID 文件
        return Err("No process found".into());
    }

    Ok(())
}
