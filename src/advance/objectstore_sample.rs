use object_store;

#[tokio::main]
async fn objectstore_simple(){
    // create an ObjectStore
    let object_store: Arc<dyn ObjectStore> = get_object_store();

    // Recursively list all files below the 'data' path.
    // 1. On AWS S3 this would be the 'data/' prefix
    // 2. On a local filesystem, this would be the 'data' directory
    let prefix = Path::from("data");

    // Get an `async` stream of Metadata objects:
    let mut list_stream = object_store.list(Some(&prefix));

    // Print a line about each object
    while let Some(meta) = list_stream.next().await.transpose().unwrap() {
        println!("Name: {}, size: {}", meta.location, meta.size);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_objectstore_simple() {
        objectstore_simple();
    }
}
