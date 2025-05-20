use awesome::rpcs::tonic_store_server;
use clap::{Parser, Subcommand};
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,
// }

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Add(AddOptions),
    Remove(RemoveOptions),
    Get(GetOptions),
    UpdateQuantity(UpdateQuantityOptions),
    UpdatePrice(UpdatePriceOptions),
    Watch(GetOptions),
}

#[derive(Debug, Parser)]
struct AddOptions {
    #[clap(long)]
    sku: String,
    #[clap(long)]
    price: f32,
    #[clap(default_value = "0", long)]
    quantity: u32,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    description: Option<String>,
}

#[derive(Debug, Parser)]
struct RemoveOptions {
    #[clap(long)]
    sku: String,
}

#[derive(Debug, Parser)]
struct GetOptions {
    #[clap(long)]
    sku: String,
}

#[derive(Debug, Parser)]
struct UpdateQuantityOptions {
    #[clap(long)]
    sku: String,
    #[clap(allow_hyphen_values = true, long)]
    change: i32,
}

#[derive(Debug, Parser)]
struct UpdatePriceOptions {
    #[clap(long)]
    sku: String,
    #[clap(long)]
    price: f32,
}

fn main() {
    println!("Hello,Tonic Store server!");
    tonic_store_server::store_server("127.0.0.1", 9001);

    let opts = Options::parse();

    // use Command::*;
    // match opts.command {
    //     Add(opts) => add(opts).await?,
    //     Remove(opts) => remove(opts).await?,
    //     Get(opts) => get(opts).await?,
    //     UpdateQuantity(opts) => update_quantity(opts).await?,
    //     UpdatePrice(opts) => update_price(opts).await?,
    //     Watch(opts) => watch(opts).await?,
    // };
}
