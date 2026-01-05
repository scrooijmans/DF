# datafusion_objectstore_s3 - Rust

## Crate datafusion_objectstore_s3 

[Source](about:blank/src/datafusion_objectstore_s3/lib.rs.html#18-195)

Expand description

[DataFusion-ObjectStore-S3](https://github.com/datafusion-contrib/datafusion-objectstore-s3) provides a `TableProvider` interface for using `Datafusion` to query data in S3. This includes AWS S3 and services such as MinIO that implement the S3 API.

### [§](#examples)Examples

Examples for querying AWS and other implementors, such as MinIO, are shown below.

Load credentials from default AWS credential provider (such as environment or ~/.aws/credentials)

```
let s3_file_system = Arc::new(S3FileSystem::default().await);
```

`S3FileSystem::default()` is a convenience wrapper for `S3FileSystem::new(None, None, None, None, None, None)`.

Connect to implementor of S3 API (MinIO, in this case) using access key and secret.

```
use datafusion_objectstore_s3::object_store::s3::S3FileSystem;

use aws_types::credentials::SharedCredentialsProvider;
use aws_types::credentials::Credentials;
use aws_sdk_s3::Endpoint;
use http::Uri;

// Example credentials provided by MinIO
const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
const PROVIDER_NAME: &str = "Static";
const MINIO_ENDPOINT: &str = "http://91.99.166.223:9000";

let s3_file_system = S3FileSystem::new(
    Some(SharedCredentialsProvider::new(Credentials::new(
        MINIO_ACCESS_KEY_ID,
        MINIO_SECRET_ACCESS_KEY,
        None,
        None,
        PROVIDER_NAME,
    ))), // SharedCredentialsProvider
    None, //Region
    Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))), //Endpoint
    None, // RetryConfig
    None, // AsyncSleep
    None, // TimeoutConfig
)
.await;
```

Using DataFusion’s `ListingOtions` and `ListingTable` we register a table into a DataFusion `ExecutionContext` so that it can be queried.

```
use std::sync::Arc;

use datafusion::datasource::listing::*;
use datafusion::datasource::TableProvider;
use datafusion::prelude::SessionContext;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::error::Result;

use datafusion_objectstore_s3::object_store::s3::S3FileSystem;

use aws_types::credentials::SharedCredentialsProvider;
use aws_types::credentials::Credentials;
use aws_sdk_s3::Endpoint;
use http::Uri;


let filename = "s3://data/alltypes_plain.snappy.parquet";


let config = ListingTableConfig::new(s3_file_system, filename).infer().await?;

let table = ListingTable::try_new(config)?;

let mut ctx = SessionContext::new();

ctx.register_table("tbl", Arc::new(table))?;

let df = ctx.sql("SELECT * FROM tbl").await?;
df.show();
```

We can also register the `S3FileSystem` directly as an `ObjectStore` on an `ExecutionContext`. This provides an idiomatic way of creating `TableProviders` that can be queried.

```
use std::sync::Arc;

use datafusion::datasource::listing::*;
use datafusion::error::Result;

use datafusion_objectstore_s3::object_store::s3::S3FileSystem;

use aws_sdk_s3::Endpoint;
use aws_types::credentials::Credentials;
use aws_types::credentials::SharedCredentialsProvider;
use datafusion::prelude::SessionContext;
use http::Uri;

const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
const PROVIDER_NAME: &str = "Static";
const MINIO_ENDPOINT: &str = "http://91.99.166.223:9000";

#[tokio::main]
async fn main() -> Result<()> {
    let s3_file_system = Arc::new(
        S3FileSystem::new(
            Some(SharedCredentialsProvider::new(Credentials::new(
                MINIO_ACCESS_KEY_ID,
                MINIO_SECRET_ACCESS_KEY,
                None,
                None,
                PROVIDER_NAME,
            ))),
            None,
            Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
            None,
            None,
            None,
        )
        .await,
    );

    let ctx = SessionContext::new();

    let uri = "s3://data/alltypes_plain.snappy.parquet";

    let config = ListingTableConfig::new(s3_file_system, uri)
        .infer()
        .await?;

    let table = ListingTable::try_new(config)?;

    ctx.register_table("tbl", Arc::new(table))?;

    let df = ctx.sql("SELECT * FROM tbl").await?;
    df.show().await?;
    Ok(())
}
```

[error](error/index.html "mod datafusion_objectstore_s3::error")

Custom error type for `DataFusion-ObjectStore-S3`

[object_store](object_store/index.html "mod datafusion_objectstore_s3::object_store")

`ObjectStore` implementation for the Amazon S3 API
