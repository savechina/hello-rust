use futures::Stream;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

mod store {
    tonic::include_proto!("store");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}
use store::inventory_server::{Inventory, InventoryServer};
use store::{
    InventoryChangeResponse, InventoryUpdateResponse, Item, ItemIdentifier, PriceChangeRequest,
    QuantityChangeRequest,
};

const BAD_PRICE_ERR: &str = "provided PRICE was invalid";
const DUP_PRICE_ERR: &str = "item is already at this price";
const DUP_ITEM_ERR: &str = "item already exists in inventory";
const EMPTY_QUANT_ERR: &str = "invalid quantity of 0 provided";
const EMPTY_SKU_ERR: &str = "provided SKU was empty";
const NO_ID_ERR: &str = "no ID or SKU provided for item";
const NO_ITEM_ERR: &str = "the item requested was not found";
const NO_STOCK_ERR: &str = "no stock provided for item";
const UNSUFF_INV_ERR: &str = "not enough inventory for quantity change";

#[derive(Debug)]
pub struct StoreInventory {
    inventory: Arc<Mutex<HashMap<String, Item>>>,
}

impl Default for StoreInventory {
    fn default() -> Self {
        StoreInventory {
            inventory: Arc::new(Mutex::new(HashMap::<String, Item>::new())),
        }
    }
}

#[tonic::async_trait]
impl Inventory for StoreInventory {
    async fn add(
        &self,
        request: Request<Item>,
    ) -> Result<Response<InventoryChangeResponse>, Status> {
        let item = request.into_inner();

        // validate SKU, verify that it&#039;s present and not empty
        let sku = match item.identifier.as_ref() {
            Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.sku.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR)),
        };

        // validate stock, verify its present and price is not negative or $0.00
        match item.stock.as_ref() {
            Some(stock) if stock.price <= 0.00 => {
                return Err(Status::invalid_argument(BAD_PRICE_ERR))
            }
            Some(_) => {}
            None => return Err(Status::invalid_argument(NO_STOCK_ERR)),
        };

        // if the item is already present don&#039;t allow the duplicate
        let mut map = self.inventory.lock().await;
        if let Some(_) = map.get(&sku) {
            return Err(Status::already_exists(DUP_ITEM_ERR));
        }

        // add the item to the inventory
        map.insert(sku.into(), item);

        Ok(Response::new(InventoryChangeResponse {
            status: "success".into(),
        }))
    }

    async fn remove(
        &self,
        request: Request<ItemIdentifier>,
    ) -> Result<Response<InventoryChangeResponse>, Status> {
        let identifier = request.into_inner();

        // don&#039;t allow empty SKU
        if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        // remove the item (if present)
        let mut map = self.inventory.lock().await;
        let msg = match map.remove(&identifier.sku) {
            Some(_) => "success: item was removed",
            None => "success: item didn&#039;t exist",
        };

        Ok(Response::new(InventoryChangeResponse {
            status: msg.into(),
        }))
    }

    async fn get(&self, request: Request<ItemIdentifier>) -> Result<Response<Item>, Status> {
        let identifier = request.into_inner();

        // don&#039;t allow empty SKU
        if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        // retrieve the item if it exists
        let map = self.inventory.lock().await;
        let item = match map.get(&identifier.sku) {
            Some(item) => item,
            None => return Err(Status::not_found(NO_ITEM_ERR)),
        };

        Ok(Response::new(item.clone()))
    }

    async fn update_quantity(
        &self,
        request: Request<QuantityChangeRequest>,
    ) -> Result<Response<InventoryUpdateResponse>, Status> {
        let change = request.into_inner();

        // don&#039;t allow empty SKU
        if change.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        // quantity changes with no actual change don&#039;t make sense, inform user
        if change.change == 0 {
            return Err(Status::invalid_argument(EMPTY_QUANT_ERR));
        }

        // retrieve the current inventory item data
        let mut map = self.inventory.lock().await;
        let item = match map.get_mut(&change.sku) {
            Some(item) => item,
            None => return Err(Status::not_found(NO_ITEM_ERR)),
        };

        // retrieve the stock mutable so we can update the quantity
        let mut stock = match item.stock.borrow_mut() {
            Some(stock) => stock,
            None => return Err(Status::internal(NO_STOCK_ERR)),
        };

        // validate and then handle the quantity change
        stock.quantity = match change.change {
            // handle negative numbers as stock reduction
            change if change < 0 => {
                if change.abs() as u32 > stock.quantity {
                    return Err(Status::resource_exhausted(UNSUFF_INV_ERR));
                }
                stock.quantity - change.abs() as u32
            }
            // handle positive numbers as stock increases
            change => stock.quantity + change as u32,
        };

        Ok(Response::new(InventoryUpdateResponse {
            status: "success".into(),
            price: stock.price,
            quantity: stock.quantity,
        }))
    }

    async fn update_price(
        &self,
        request: Request<PriceChangeRequest>,
    ) -> Result<Response<InventoryUpdateResponse>, Status> {
        let change = request.into_inner();

        // don&#039;t allow empty SKU
        if change.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        // $0.00 disallowed and negatives don&#039;t make sense, inform the user
        if change.price <= 0.0 {
            return Err(Status::invalid_argument(BAD_PRICE_ERR));
        }

        // retrieve the current inventory item data
        let mut map = self.inventory.lock().await;
        let item = match map.get_mut(&change.sku) {
            Some(item) => item,
            None => return Err(Status::not_found(NO_ITEM_ERR)),
        };

        // retrieve the stock mutable so we can update the quantity
        let mut stock = match item.stock.borrow_mut() {
            Some(stock) => stock,
            None => return Err(Status::internal(NO_STOCK_ERR)),
        };

        // let the client know if they requested to change the price to the
        // price that is already currently set
        if stock.price == change.price {
            return Err(Status::invalid_argument(DUP_PRICE_ERR));
        }

        // update the item unit price
        stock.price = change.price;

        Ok(Response::new(InventoryUpdateResponse {
            status: "success".into(),
            price: stock.price,
            quantity: stock.quantity,
        }))
    }

    type WatchStream = Pin<Box<dyn Stream<Item = Result<Item, Status>> + Send>>;

    async fn watch(
        &self,
        request: Request<ItemIdentifier>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        // retrieve the relevant item and get a baseline
        let id = request.into_inner();
        let mut item = self.get(Request::new(id.clone())).await?.into_inner();

        // the channel will be our stream back to the client, we&#039;ll send copies
        // of the requested item any time we notice a change to it in the
        // inventory.
        let (tx, rx) = mpsc::unbounded_channel();

        // we&#039;ll loop and poll new copies of the item until either the client
        // closes the connection, or an error occurs.
        let inventory = self.inventory.clone();
        tokio::spawn(async move {
            loop {
                // it&#039;s somewhat basic, but for this demo we&#039;ll just check the
                // item every second for any changes.
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                // pull a fresh copy of the item in the inventory
                let map = inventory.lock().await;
                let item_refresh = match map.get(&id.sku) {
                    Some(item) => item,
                    // the item has been removed from the inventory. Let the
                    // client know, and stop the stream.
                    None => {
                        if let Err(err) = tx.send(Err(Status::not_found(NO_ITEM_ERR))) {
                            println!("ERROR: failed to update stream client: {:?}", err);
                        }
                        return;
                    }
                };

                // check to see if the item has changed since we last saw it,
                // and if it has inform the client via the stream.
                if item_refresh != &item {
                    if let Err(err) = tx.send(Ok(item_refresh.clone())) {
                        println!("ERROR: failed to update stream client: {:?}", err);
                        return;
                    }
                }

                // cache the most recent copy of the item
                item = item_refresh.clone()
            }
        });

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::WatchStream))
    }
}

#[tokio::main]
pub async fn store_server(host: &str, port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}:{}", host, port);
    let addr = url.parse()?;
    let inventory = StoreInventory::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(store::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    tonic::transport::Server::builder()
        .add_service(InventoryServer::new(inventory))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
