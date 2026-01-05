# Function get_nonexistent_object Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/integration.rs.html#965-975" class="src">Source</a>

``` rust
pub async fn get_nonexistent_object(
    storage: &DynObjectStore,
    location: Option<Path>,
) -> Result<Bytes>
```

Available on **crate feature `integration`** only.

Expand description

Tests fetching a non-existent object returns a not found error
