use awesome::rpcs::tonic_store_server;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}
fn main() {
    println!("Hello,Tonic Store server!");
    tonic_store_server::store_server("127.0.0.1", 9001);
}
