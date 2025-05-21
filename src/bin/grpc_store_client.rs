use awesome::services::tonic_store_client;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    command: Command,
    #[clap(default_value = "http://127.0.0.1:9001", long)]
    url: String,
}
/// A simple inventory management system
/// using gRPC and Tonic.
/// This program allows you to add, remove, get, update quantity, and update price of items in the inventory.
/// ./grpc_store_client add --sku TESTSKU --price 1.99 --quantity 20 --name bananas --description "yellow fruit"
/// ./grpc_store_client get --sku TESTSKU

#[derive(Debug, Parser)]
enum Command {
    /// Add an item to the inventory
    Add(AddOptions),
    /// Remove an item from the inventory
    Remove(RemoveOptions),
    /// Get an item from the inventory
    Get(GetOptions),
    /// Update the quantity of an item in the inventory
    UpdateQuantity(UpdateQuantityOptions),
    /// Update the price of an item in the inventory
    UpdatePrice(UpdatePriceOptions),
    /// Watch an item in the inventory
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello,Tonic Store Client!");
    println!("This is a gRPC client for the Tonic Store server.");

    let opts = Options::parse();

    use tonic_store_client::{
        AddRequest, GetRequest, RemoveRequest, UpdatePriceRequest, UpdateQuantityRequest,
    };
    use Command::*;

    // let client_url = String::from("http://127.0.0.1:9001");
    let client_url = opts.url;

    println!("Connecting to gRPC server at {}", client_url);

    match opts.command {
        //
        Add(opts) => {
            tonic_store_client::add(
                client_url,
                tonic_store_client::AddRequest {
                    sku: opts.sku,
                    price: opts.price,
                    quantity: opts.quantity,
                    name: opts.name,
                    description: opts.description,
                },
            )
            .await?
        }
        Remove(opts) => {
            tonic_store_client::remove(
                client_url,
                tonic_store_client::RemoveRequest { sku: opts.sku },
            )
            .await?
        }
        Get(opts) => {
            tonic_store_client::get(client_url, tonic_store_client::GetRequest { sku: opts.sku })
                .await?
        }
        UpdateQuantity(opts) => {
            tonic_store_client::update_quantity(
                client_url,
                tonic_store_client::UpdateQuantityRequest {
                    sku: opts.sku,
                    change: opts.change,
                },
            )
            .await?
        }
        UpdatePrice(opts) => {
            tonic_store_client::update_price(
                client_url,
                tonic_store_client::UpdatePriceRequest {
                    sku: opts.sku,
                    price: opts.price,
                },
            )
            .await?
        }
        Watch(opts) => {
            tonic_store_client::watch(client_url, tonic_store_client::GetRequest { sku: opts.sku })
                .await?
        }
    };

    Ok(())
}
