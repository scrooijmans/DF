# Function multipart_race_condition Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/integration.rs.html#1117-1204" class="src">Source</a>

``` rust
pub async fn multipart_race_condition(
    storage: &dyn ObjectStore,
    last_writer_wins: bool,
)
```

Available on **crate feature `integration`** only.

Expand description

Tests a race condition where 2 threads are performing multipart writes to the same path
