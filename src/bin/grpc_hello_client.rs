use awesome::rpcs::tonic_hello_client;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}
fn main() {
    println!("Hello, Awesome!");
    tonic_hello_client::hello_client();
}
