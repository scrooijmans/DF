- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/integrations/object_store/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/integrations/" class="breadcrumbs__link">Integrations</a>
- Object Store

On this page

# Object Store

## Apache OpenDALâ„¢ object_store integration

[<img src="out_opendal/integrations/object_store/index_media/49073adb969e94cba3d1c470073bbb2ae9fb8754.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Build Status" />](https://github.com/apache/opendal/actions?query=branch%3Amain) [<img src="out_opendal/integrations/object_store/index_media/1b610f4f7a0682a2b8e99de8878cb5246ae3692d.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Latest Version" />](https://crates.io/crates/object_store_opendal) [<img src="out_opendal/integrations/object_store/index_media/ef3346b973194923109ce335657f4966844df06e.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate Downloads" />](https://crates.io/crates/object_store_opendal) [<img src="out_opendal/integrations/object_store/index_media/8344f1bc1d55e7813afa8d864a46394b41605b73.svg" class="img_KBPg" decoding="async" loading="lazy" alt="chat" />](https://opendal.apache.org/discord)

`object_store_opendal` is an [`object_store`](https://crates.io/crates/object_store) implementation using [`opendal`](https://github.com/apache/opendal).

This crate can help you to access 30 more storage services with the same object_store API.

## Useful Links<a href="https://opendal.apache.org/integrations/object_store/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- Documentation: [release](https://docs.rs/object_store_opendal/) \| [dev](https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/)

## Examples<a href="https://opendal.apache.org/integrations/object_store/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

`opendal_store_opendal` depends on the `opendal` crate. Please make sure to always use the latest versions of both.

latest `object_store_opendal` <img src="out_opendal/integrations/object_store/index_media/1b610f4f7a0682a2b8e99de8878cb5246ae3692d.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate" />

latest `opendal` <img src="out_opendal/integrations/object_store/index_media/1b610f4f7a0682a2b8e99de8878cb5246ae3692d.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate" />

### 1. using `object_store` API to access S3<a href="https://opendal.apache.org/integrations/object_store/#1-using-object_store-api-to-access-s3" class="hash-link" aria-label="Direct link to 1-using-object_store-api-to-access-s3" translate="no" title="Direct link to 1-using-object_store-api-to-access-s3">â€‹</a>

Add the following dependencies to your `Cargo.toml` with correct version:

``` prism-code
[dependencies]
object_store = "0.11.0"
object_store_opendal =  "xxx"   # see the latest version above
opendal = { version = "xxx", features = ["services-s3"] }  # see the latest version above
```

Build `OpendalStore` via `opendal::Operator`:

``` prism-code
use std::sync::Arc;

use bytes::Bytes;
use object_store::path::Path;
use object_store::ObjectStore;
use object_store_opendal::OpendalStore;
use opendal::services::S3;
use opendal::{Builder, Operator};

#[tokio::main]
async fn main() {
    let builder = S3::from_map(
        vec![
            ("access_key".to_string(), "my_access_key".to_string()),
            ("secret_key".to_string(), "my_secret_key".to_string()),
            ("endpoint".to_string(), "my_endpoint".to_string()),
            ("region".to_string(), "my_region".to_string()),
        ]
        .into_iter()
        .collect(),
    ).unwrap();

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

### 2. querying data in a S3 bucket using DataFusion<a href="https://opendal.apache.org/integrations/object_store/#2-querying-data-in-a-s3-bucket-using-datafusion" class="hash-link" aria-label="Direct link to 2. querying data in a S3 bucket using DataFusion" translate="no" title="Direct link to 2. querying data in a S3 bucket using DataFusion">â€‹</a>

Add the following dependencies to your `Cargo.toml` with correct version:

``` prism-code
[dependencies]
object_store = "0.11.0"
object_store_opendal = "xxx" # see the latest version above
opendal = { version = "xxx", features = ["services-s3"] } # see the latest version above
datafusion = "44.0.0"
url = "2.5.2"
```

Build `OpendalStore` via `opendal::Operator` and register it to `DataFusion`:

``` prism-code
use datafusion::error::DataFusionError;
use datafusion::error::Result;
use datafusion::prelude::*;
use opendal::services::S3;
use opendal::Operator;
use std::sync::Arc;
use url::Url;


#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();

    // Configure OpenDAL for S3
    let region = "my_region";
    let bucket_name = "my_bucket";
    let builder = S3::default()
        .endpoint("my_endpoint")
        .bucket(bucket_name)
        .region(region)
        .access_key_id("my_access_key")
        .secret_access_key("my_secret_key");
    let op = Operator::new(builder)
        .map_err(|err| DataFusionError::External(Box::new(err)))?
        .finish();
    let store = object_store_opendal::OpendalStore::new(op);

    // Register the object store
    let path = format!("s3://{bucket_name}");
    let s3_url = Url::parse(&path).unwrap();
    ctx.register_object_store(&s3_url, Arc::new(store));

    // Register CSV file as a table
    let path = format!("s3://{bucket_name}/csv/data.csv");
    ctx.register_csv("trips", &path, CsvReadOptions::default())
        .await?;

    // Execute the query
    let df = ctx.sql("SELECT * FROM trips LIMIT 10").await?;
    // Print the results
    df.show().await?;

    // Dynamic query using the file path directly
    let ctx = ctx.enable_url_table();
    let df = ctx
        .sql(format!(r#"SELECT * FROM '{}' LIMIT 10"#, &path).as_str())
        .await?;
    // Print the results
    df.show().await?;

    Ok(())
}
```

## WASM support<a href="https://opendal.apache.org/integrations/object_store/#wasm-support" class="hash-link" aria-label="Direct link to WASM support" translate="no" title="Direct link to WASM support">â€‹</a>

To build with `wasm32-unknown-unknown` target, you need to enable the `send_wrapper` feature:

``` prism-code
cargo build --target wasm32-unknown-unknown --features send_wrapper
```

## Branding<a href="https://opendal.apache.org/integrations/object_store/#branding" class="hash-link" aria-label="Direct link to Branding" translate="no" title="Direct link to Branding">â€‹</a>

The first and most prominent mentions must use the full form: **Apache OpenDALâ„¢** of the name for any individual usage (webpage, handout, slides, etc.) Depending on the context and writing style, you should use the full form of the name sufficiently often to ensure that readers clearly understand the association of both the OpenDAL project and the OpenDAL software product to the ASF as the parent organization.

For more details, see the [Apache Product Name Usage Guide](https://www.apache.org/foundation/marks/guide).

## License and Trademarks<a href="https://opendal.apache.org/integrations/object_store/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/30-integrations/object_store.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/integrations/object_store/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/integrations/fuse3/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Fuse3 (removed)

<a href="https://opendal.apache.org/integrations/parquet/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Parquet

- <a href="https://opendal.apache.org/integrations/object_store/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/integrations/object_store/#examples" class="table-of-contents__link toc-highlight">Examples</a>
  - <a href="https://opendal.apache.org/integrations/object_store/#1-using-object_store-api-to-access-s3" class="table-of-contents__link toc-highlight">1. using <code>object_store</code> API to access S3</a>
  - <a href="https://opendal.apache.org/integrations/object_store/#2-querying-data-in-a-s3-bucket-using-datafusion" class="table-of-contents__link toc-highlight">2. querying data in a S3 bucket using DataFusion</a>
- <a href="https://opendal.apache.org/integrations/object_store/#wasm-support" class="table-of-contents__link toc-highlight">WASM support</a>
- <a href="https://opendal.apache.org/integrations/object_store/#branding" class="table-of-contents__link toc-highlight">Branding</a>
- <a href="https://opendal.apache.org/integrations/object_store/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
