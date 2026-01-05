# Function sync_on_close Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/sync_on_close.rs.html#17" class="src">Source</a>

``` rust
pub fn sync_on_close(
    sync_on_close: SyncOnCloseType,
    file: &mut File,
) -> Result<(), Error>
```

Available on **crate feature `polars-io`** only.
