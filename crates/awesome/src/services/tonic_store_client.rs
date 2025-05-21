mod store {
    tonic::include_proto!("store");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}
use store::inventory_client::InventoryClient;
use store::{
    Item, ItemIdentifier, ItemInformation, ItemStock, PriceChangeRequest, QuantityChangeRequest,
};
use tokio_stream::StreamExt;

pub struct AddRequest {
    pub sku: String,
    pub price: f32,
    pub quantity: u32,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn add(url: String, opts: AddRequest) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let id = ItemIdentifier { sku: opts.sku };

    let stock = ItemStock {
        price: opts.price,
        quantity: opts.quantity,
    };

    let info = ItemInformation {
        name: opts.name,
        description: opts.description,
    };

    let item = Item {
        identifier: Some(id),
        stock: Some(stock),
        information: Some(info),
    };

    let request = tonic::Request::new(item);
    let response = client.add(request).await?;
    assert_eq!(response.into_inner().status, "success");
    println!("success: item was added to the inventory.");

    Ok(())
}

pub struct RemoveRequest {
    pub sku: String,
}

pub async fn remove(url: String, opts: RemoveRequest) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let request = tonic::Request::new(ItemIdentifier { sku: opts.sku });
    let response = client.remove(request).await?;
    let msg = response.into_inner().status;
    assert!(msg.starts_with("success"));
    println!("{}", msg);

    Ok(())
}

pub struct GetRequest {
    pub sku: String,
}

pub async fn get(url: String, opts: GetRequest) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let request = tonic::Request::new(ItemIdentifier { sku: opts.sku });
    let item = client.get(request).await?.into_inner();
    println!("found item: {:?}", item);

    Ok(())
}

pub struct UpdateQuantityRequest {
    pub sku: String,
    pub change: i32,
}

pub async fn update_quantity(
    url: String,
    opts: UpdateQuantityRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let request = tonic::Request::new(QuantityChangeRequest {
        sku: opts.sku,
        change: opts.change,
    });

    let message = client.update_quantity(request).await?.into_inner();
    assert_eq!(message.status, "success");
    println!(
        "success: quantity was updated. Quantity: {} Price: {}",
        message.quantity, message.price
    );

    Ok(())
}

pub struct UpdatePriceRequest {
    pub sku: String,
    pub price: f32,
}

pub async fn update_price(
    url: String,
    opts: UpdatePriceRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let request = tonic::Request::new(PriceChangeRequest {
        sku: opts.sku,
        price: opts.price,
    });

    let message = client.update_price(request).await?.into_inner();
    assert_eq!(message.status, "success");
    println!(
        "success: price was updated. Quantity: {} Price: {}",
        message.quantity, message.price
    );

    Ok(())
}

pub async fn watch(url: String, opts: GetRequest) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InventoryClient::connect(url).await?;

    let mut stream = client
        .watch(ItemIdentifier {
            sku: opts.sku.clone(),
        })
        .await?
        .into_inner();

    println!("streaming changes to item {}", opts.sku);
    while let Some(item) = stream.next().await {
        match item {
            Ok(item) => println!("item was updated: {:?}", item),
            Err(err) => {
                if err.code() == tonic::Code::NotFound {
                    println!("watched item has been removed from the inventory.");
                    break;
                } else {
                    return Err(err.into());
                }
            }
        };
    }
    println!("stream closed");

    Ok(())
}
