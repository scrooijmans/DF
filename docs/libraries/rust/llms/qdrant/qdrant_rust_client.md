# Qdrant Rust client

The [Qdrant](https://qdrant.tech/) - High-Performance Vector Search at Scale - client for Rust.

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Apache 2.0 licensed][apache2-badge]][apache2-url]

[crates-badge]: https://img.shields.io/crates/v/qdrant-client.svg

[crates-url]: https://crates.io/crates/qdrant-client

[docs-badge]: https://img.shields.io/docsrs/qdrant-client.svg

[docs-url]: https://docs.rs/qdrant-client

[apache2-badge]: https://img.shields.io/badge/license-apache2-blue.svg

[apache2-url]: https://github.com/qdrant/rust-client/blob/master/LICENSE

Documentation:
- Qdrant documentation: <https://qdrant.tech/documentation/>
- Crate documentation: <https://docs.rs/qdrant-client>

## Installation

```bash
cargo add qdrant-client
```

Package is available in [crates.io](https://crates.io/crates/qdrant-client)

## Examples
A list of example snippets can be found [here](https://github.com/qdrant/api-reference/tree/main/snippets/rust)

More examples can be found in the [examples folder](https://github.com/qdrant/rust-client/tree/master/examples)

## Dependencies

The client uses gRPC via the [Tonic](https://github.com/hyperium/tonic) library.

To change anything in the protocol buffer definitions, you need the `protoc` Protocol Buffers compiler, along with Protocol Buffers resource files.

Refer to the [Tonic installation guide](https://github.com/hyperium/tonic#dependencies) for more details.

## Usage

Run Qdrant with enabled gRPC interface:

```bash
# With env variable
docker run -p 6333:6333 -p 6334:6334 \
    -e QDRANT__SERVICE__GRPC_PORT="6334" \
    qdrant/qdrant
```

Or by updating the configuration file:

```yaml
service:
  grpc_port: 6334
```

More info about gRPC in [documentation](https://qdrant.tech/documentation/quick_start/#grpc).

### Making requests

Add necessary dependencies:

```bash
cargo add qdrant-client anyhow tonic tokio serde-json --features tokio/rt-multi-thread
```

Add search example from [`examples/search.rs`](./examples/search.rs) to your `src/main.rs`:

```rust
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant, QdrantError};

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);
    // collections_list = {
    //   "collections": [
    //     {
    //       "name": "test"
    //     }
    //   ]
    // }

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    let collection_info = client.collection_info(collection_name).await?;
    dbg!(collection_info);

    let payload: Payload = serde_json::json!(
        {
            "foo": "Bar",
            "bar": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points))
        .await?;

    let search_result = client
        .search_points(
            SearchPointsBuilder::new(collection_name, [11.; 10], 10)
                .filter(Filter::all([Condition::matches("bar", 12)]))
                .with_payload(true)
                .params(SearchParamsBuilder::default().exact(true)),
        )
        .await?;
    dbg!(&search_result);
    // search_result = [
    //   {
    //     "id": 0,
    //     "version": 0,
    //     "score": 1.0000001,
    //     "payload": {
    //       "bar": 12,
    //       "baz": {
    //         "qux": "quux"
    //       },
    //       "foo": "Bar"
    //     }
    //   }
    // ]

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();
    println!("baz: {}", baz_payload);
    // baz: {"qux":"quux"}

    Ok(())
}
```

Or run the example from this project directly:

```bash
cargo run --example search
```

## Qdrant Cloud

[Qdrant Cloud](https://cloud.qdrant.io) is a managed service for Qdrant.

The client needs to be configured properly to access the service.

- make sure to use the correct port (6334)
- make sure to pass your API KEY

```rust
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://xxxxxxxxxx.eu-central.aws.cloud.qdrant.io:6334")
    // Use an environment variable for the API KEY for example
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build()?;
```

Description: The Qdrant - High-Performance Vector Search at Scale - client for Rust.

Title: qdrant\_client - Rust

Docs.rs

*   qdrant-client-1.14.1

*   qdrant-client 1.14.1
*   Permalink
*   Docs.rs crate page
*   Apache-2.0

*   Links
*   Homepage
*   Repository
*   crates.io
*   Source

*   Owners
*   generall

*   Dependencies
*   *   anyhow ^1.0.89 _normal_
*   derive\_builder ^0.20.2 _normal_
*   futures ^0.3.31 _normal_
*   futures-util ^0.3.31 _normal_ _optional_
*   prost ^0.13.3 _normal_
*   prost-types ^0.13.3 _normal_
*   reqwest ^0.12.8 _normal_ _optional_
*   semver ^1.0.24 _normal_
*   serde ^1.0.210 _normal_ _optional_
*   serde\_json ^1.0.128 _normal_ _optional_
*   thiserror ^1.0.64 _normal_
*   tokio ^1.40.0 _normal_
*   tonic ^0.12.3 _normal_
*   uuid ^1.8.2 _normal_ _optional_
*   tonic-build ^0.12.3 _dev_

*   Versions

*   **64.7%** of the crate is documented

*   Platform
*   i686-unknown-linux-gnu
*   x86\_64-unknown-linux-gnu
*   Feature flags

*   docs.rs
*   About docs.rs
*   Privacy policy

*   Rust
*   Rust website
*   The Book
*   Standard Library API Reference
*   Rust by Example
*   The Cargo Guide
*   Clippy Documentation

Crate qdrant\_clientCopy item path
==================================

Source

Expand description

The Qdrant - High-Performance Vector Search at Scale - client for Rust.

This crate connects to your Qdrant server over gRPC and provides an easy to use API interface for it.

§Connect
--------

First you’ll need to set up a `Qdrant` client, used to connect to a Qdrant instance:

```
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334")
.api_key(std::env::var("QDRANT_API_KEY"))
.build()?;
```

§Create collection
------------------

Qdrant works with Collections ⧉ of Points ⧉ . To add vector data, you first create a collection:

```
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};

let response = client
.create_collection(
CreateCollectionBuilder::new("my_collection")
.vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
)
.await?;
```

The most interesting parts are the two arguments of `VectorParamsBuilder::new`. The first one (`512`) is the length of vectors to store and the second one (`Distance::Cosine`) is the Distance, which is the `Distance` measure to gauge similarity for the nearest neighbors search.

Documentation: https://qdrant.tech/documentation/concepts/collections/#create-a-collection

§Upsert points
--------------

Now we have a collection, we can insert (or rather upsert) points. Points have an id, one or more vectors and a payload. We can usually do that in bulk, but for this example, we’ll add a single point:

```
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};

let points = vec![
PointStruct::new(
42,                 // Unique point ID
vec![0.0_f32; 512], // Vector to upsert
// Attached payload
[
("great", true.into()),
("level", 9000.into()),
("text", "Hi Qdrant!".into()),
("list", vec![1.234f32, 0.815].into()),
],
),
];

let response = client
.upsert_points(UpsertPointsBuilder::new("my_collection", points))
.await?;
```

Documentation: https://qdrant.tech/documentation/concepts/points/#upload-points

§Search
-------

Finally, we can retrieve points in various ways, the common one being a plain similarity search:

```
use qdrant_client::qdrant::SearchPointsBuilder;

let search_request = SearchPointsBuilder::new(
"my_collection",    // Collection name
vec![0.0_f32; 512], // Search vector
4,                  // Search limit, number of results to return
).with_payload(true);

let response = client.search_points(search_request).await?;
```

The parameter for `SearchPointsBuilder::new()` constructor are pretty straightforward: name of the collection, the vector and how many top-k results to return. The `with_payload(true)` call tells qdrant to also return the (full) payload data for each point. You can also add a `filter()` call to the `SearchPointsBuilder` to filter the result. See the `Filter` documentation for details.

Documentation: https://qdrant.tech/documentation/concepts/search/

Modules§
--------

config

Client configuration

qdrant

API types

Structs§
--------

Payload

Point payload

Qdrant

API client to interact with a Qdrant server.

Enums§
------

QdrantError

Qdrant client error

Type Aliases§
-------------

QdrantBuilder

A builder for `Qdrant`
