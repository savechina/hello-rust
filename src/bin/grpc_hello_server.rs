use awesome::services::tonic_hello_server;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}
fn main() {
    println!("Hello,Tonic Hollo Server!");
    tonic_hello_server::hello_server("[::1]", 50051);
}
