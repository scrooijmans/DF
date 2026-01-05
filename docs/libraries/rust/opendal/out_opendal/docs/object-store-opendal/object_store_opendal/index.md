# Crate object_store_opendal Copy item path

<a href="https://opendal.apache.org/docs/object-store-opendal/src/object_store_opendal/lib.rs.html#18-111" class="src">Source</a>

Expand description

object_store_opendal is an object store implementation using opendal.

This crate can help you to access 30 more storage services with the same object_store API.

``` rust
use std::sync::Arc;

use bytes::Bytes;
use object_store::path::Path;
use object_store::ObjectStore;
use object_store_opendal::OpendalStore;
use opendal::services::S3;
use opendal::{Builder, Operator};

#[tokio::main]
async fn main() {
   let builder = S3::default()
    .access_key_id("my_access_key")
    .secret_access_key("my_secret_key")
    .endpoint("my_endpoint")
    .region("my_region");

    // Create a new operator
    let operator = Operator::new(builder).unwrap().finish();

    // Create a new object store
    let object_store = Arc::new(OpendalStore::new(operator));

    let path = Path::from("data/nested/test.txt");
    let bytes = Bytes::from_static(b"hello, world! I am nested.");

    object_store.put(&path, bytes.clone().into()).await.unwrap();

    let content = object_store
        .get(&path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    assert_eq!(content, bytes);
}
```

## Structs<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>  
ObjectStore backend builder

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html" class="struct" title="struct object_store_opendal::ObjectStoreService">ObjectStoreService</a>  
ObjectStore backend

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>  
OpendalStore implements ObjectStore trait by using opendal.
