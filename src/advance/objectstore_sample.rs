use bytes::Bytes;
use futures::StreamExt;
use object_store;
use object_store::local::LocalFileSystem;
use object_store::path::Path;
use object_store::ObjectStore;
use object_store::{GetResult, PutPayload, Result};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn objectstore_simple() -> Result<()> {
    // 1. Initialize an ObjectStore (using LocalFileSystem for this example)
    // 获取临时目录
    let temp_home = env::temp_dir();
    let storage_dir = temp_home.join("hello");

    // Ensure the root directory exists

    // Create the directory if it doesn't exist
    if !storage_dir.exists() {
        // Create the directory
        // let _ = std::fs::create_dir_all(&storage_dir);
        tokio::fs::create_dir_all(&storage_dir).await.unwrap();
    }

    // let storage_dir = "data";

    // Initialize LocalFileSystem
    let local_store = Arc::new(LocalFileSystem::new_with_prefix(storage_dir)?);

    // Sample key-value data
    let key = Path::from("my_key.txt");
    let value = Bytes::from("Hello, Object Store!");

    // Store the key-value pair
    local_store.put(&key, value.clone().into()).await?;

    println!("Stored key-value pair successfully!");

    // Retrieve and verify the stored value
    let retrieved = local_store.get(&key).await?;
    let retrieved_bytes = retrieved.bytes().await?;
    println!("Retrieved value successfully!");
    let retrieved_str = String::from_utf8(retrieved_bytes.to_vec()).unwrap();
    println!("Retrieved value: {}", retrieved_str);

    // Recursively list all files below the 'data' path.
    // 1. On AWS S3 this would be the 'data/' prefix
    // 2. On a local filesystem, this would be the 'data' directory
    let prefix = Path::from("data");

    // 2. Define a path for our object
    let object_path = Path::from("data/example.txt");

    // 3. Put (write) data to the object store
    let data_to_write = Bytes::from("Hello, ObjectStore in Rust!");

    local_store
        .put(&object_path, data_to_write.clone().into())
        .await?;
    println!("Successfully wrote data to {:?}", object_path);

    // 4. Get (read) data from the object store
    let get_result: GetResult = local_store.get(&object_path).await?;
    let mut reader = get_result.into_stream();
    let mut contents = String::new();
    while let Some(chunk) = reader.next().await {
        contents.push_str(&String::from_utf8_lossy(&chunk?));
    }
    println!("Read data: {}", contents);

    // 5. List objects with a prefix
    let prefix = Path::from("data");
    // Get an `async` stream of Metadata objects:
    let mut list_stream = local_store.list(Some(&prefix));
    println!("Objects with prefix '{}':", prefix);
    // Print a line about each object
    while let Some(meta) = list_stream.next().await.transpose().unwrap() {
        println!("Name: {}, size: {}", meta.location, meta.size);
    }

    // 6. Get the metadata of an object
    let metadata = local_store.head(&object_path).await?;
    println!("Metadata for {:?}: {:?}", object_path, metadata);

    // 7. Delete an object
    local_store.delete(&object_path).await?;
    println!("Successfully deleted {:?}", object_path);

    // Clean up the directory (optional)
    // std::fs::remove_dir_all(root).unwrap_or_default();

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_objectstore_simple() {
        objectstore_simple().unwrap();
    }
}
