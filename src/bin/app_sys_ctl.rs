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
use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use std::convert::Infallible;

use std::net::SocketAddr;
use std::process;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::pin;
use tokio::signal::unix::{signal, SignalKind};
// use hyper::{Body, Request, Response};
use env_logger;
use log::{error, info};
use rkyv::ser;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use sysinfo::{Pid, System};

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
    /// Status the application
    Status,
}

const PID_FILE: &str = "my-app.pid";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .try_init()
        .unwrap_or_else(|_| {
            eprintln!("Failed to initialize logger");
        });

    // 解析命令行参数
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => start_app().await?,
        Commands::Stop => stop_app()?,
        Commands::Status => status_app()?,
    }

    Ok(())
}

async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting application...");
    // 检查是否已经运行
    if Path::new(PID_FILE).exists() {
        error!("Application is already running. Check {}", PID_FILE);
        return Err("Application already running".into());
    }

    // 保存当前进程的 PID
    let pid = write_pid()?;

    info!("Application started with PID: {}", pid);

    // 设置服务器地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3600));

    // 创建一个 oneshot 通道用于优雅关机
    let (tx, rx) = oneshot::channel::<()>();

    // 定义简单的 HTTP 服务

    // async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    //     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    // }

    let hello = |_: Request<Incoming>| async {
        Ok::<_, Infallible>(Response::new(Full::new(Bytes::from("Hello, World!"))))
    };

    async fn shutdown_signal() {
        // Wait for the CTRL+C signal
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    }

    // 启动 HTTP 服务器

    let listener = TcpListener::bind(addr).await?;

    info!("Server running on http://{}", addr);
    // specify our HTTP settings (http1, http2, auto all work)
    let mut http = http1::Builder::new();
    // the graceful watcher
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();
    // when this signal completes, start shutdown
    let mut signal = std::pin::pin!(shutdown_signal());

    // 监听 SIGINT (Ctrl+C)
    // let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())?;

    // 监听 SIGTERM (kill 默认发送的信号)
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    // Our server accept loop
    loop {
        tokio::select! {
            Ok((stream, _addr)) = listener.accept() => {
                let io = TokioIo::new(stream);
                let conn = http.serve_connection(io, service_fn(hello));
                // watch this connection
                let fut = graceful.watch(conn);
                tokio::spawn(async move {
                    if let Err(e) = fut.await {
                        eprintln!("Error serving connection: {:?}", e);
                    }
                });
            },

            _ = &mut signal => {
                info!("Received Ctrl+C, initiating graceful shutdown...");
                drop(listener);
                info!("graceful shutdown signal received");
                // stop the accept loop
                break;
            },
            _ = sigterm.recv() => {
                info!("Received SIGTERM (kill), initiating graceful shutdown...");

                drop(listener);
                break;
            }
        }
    }

    // Now start the shutdown and wait for them to complete
    // Optional: start a timeout to limit how long to wait.

    tokio::select! {
        _ = graceful.shutdown() => {
            eprintln!("all connections gracefully closed");
        },
        // _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
        //     eprintln!("timed out wait for all connections to close");
        // },
    }

    // tokio::select! {
    //     _ = sigint.recv() => {
    //                 println!("✅ 收到 SIGINT (Ctrl+C)，开始退出...");
    //             },
    //     _ = sigterm.recv() => {
    //         println!("✅ 收到 SIGTERM (kill)，开始退出...");
    //     }
    // }

    // 删除 PID 文件
    if Path::new(PID_FILE).exists() {
        fs::remove_file(PID_FILE)?;
        info!("PID file removed");
    } else {
        info!("PID file not found, skipping removal");
    }

    info!("Application stopped successfully");

    Ok(())
}

fn write_pid() -> Result<u32, Box<dyn std::error::Error>> {
    let pid = std::process::id();
    File::create(PID_FILE)?.write_all(pid.to_string().as_bytes())?;
    Ok(pid)
}

fn stop_app() -> Result<(), Box<dyn std::error::Error>> {
    info!("Stopping application...");
    // 检查 PID 文件是否存在

    let pid = get_pid()?;

    info!("start Sent stop signal to PID: {}", pid);
    // 使用 sysinfo 查找并终止进程
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(Pid::from(pid as usize)) {
        // 发送 SIGTERM 信号
        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid as NixPid;
            info!("Sent SIGTERM to process with PID: {}", pid);
            kill(NixPid::from_raw(pid as i32), Signal::SIGTERM)?;
        }

        #[cfg(not(unix))]
        {
            error!("Graceful shutdown not fully supported on non-Unix systems.");
            process.kill(); // 直接杀死进程（Windows 不支持 SIGTERM）
        }

        fs::remove_file(PID_FILE)?;
    } else {
        error!("No process found with PID: {}", pid);
        fs::remove_file(PID_FILE)?; // 清理无效的 PID 文件
        return Err("No process found".into());
    }

    info!("Application stopped successfully");
    Ok(())
}

fn get_pid() -> Result<u32, Box<dyn std::error::Error>> {
    if !Path::new(PID_FILE).exists() {
        error!("No running application found. PID file does not exist.");
        return Err("No running application".into());
    }
    let pid_str = fs::read_to_string(PID_FILE)?;
    let pid: u32 = pid_str.trim().parse()?;
    Ok(pid)
}

/// app run status
fn status_app() -> Result<(), Box<dyn std::error::Error>> {
    let pid = get_pid()?;

    print_process_stats(pid);
    Ok(())
}

fn print_process_stats(pid: u32) {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(Pid::from_u32(pid)) {
        println!("id: {:?}", process.pid());
        println!("name:{:?}", process.name());
        println!("cpu: {:?}", process.cpu_usage());
        println!("memory: {:?}", process.memory());
        println!("start time:{:?}", process.start_time());
        println!("run time: {:?}", process.run_time());
    }
}
