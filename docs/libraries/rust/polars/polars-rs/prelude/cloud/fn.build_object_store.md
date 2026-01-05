# Function build_object_store Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/object_store_setup.rs.html#224-232" class="src">Source</a>

``` rust
pub async fn build_object_store(
    url: &str,
    options: Option<&CloudOptions>,
    glob: bool,
) -> Result<(CloudLocation, PolarsObjectStore), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Build an [`ObjectStore`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the URL and passed in url. Return the cloud location and an implementation of the object store.
