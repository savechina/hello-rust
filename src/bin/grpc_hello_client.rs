use awesome::services::tonic_hello_client;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(default_value = "http://127.0.0.1:9001", long)]
    url: String,
}
fn main() {
    println!("Hello, Tonic Hello client!");
    let opts = Args::parse();
    tonic_hello_client::hello_client(opts.url);
}
