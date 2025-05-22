use awesome::services::tonic_hello_server;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "127.0.0.1", long)]
    host: String,
    #[arg(default_value = "9001", short, long)]
    port: u32,
}
fn main() {
    println!("Hello,Tonic Hollo Server!");
    let opts = Args::parse();
    println!("Host: {}", opts.host);
    println!("Port: {}", opts.port);
    println!("This is a gRPC server for the Tonic Hollo client.");
    tonic_hello_server::hello_server(&opts.host, opts.port);
}
