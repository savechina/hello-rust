use awesome::services::tonic_hello_client;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}
fn main() {
    println!("Hello, Tonic Hello client!");
    let url = "http://[::1]:50051";
    tonic_hello_client::hello_client(url);
}
