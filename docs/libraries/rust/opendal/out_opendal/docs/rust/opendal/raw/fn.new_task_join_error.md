# Function new_task_join_error Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/tokio_util.rs.html#22-24" class="src">Source</a>

``` rust
pub fn new_task_join_error(e: JoinError) -> Error
```

Available on **crate feature `internal-tokio-rt`** only.

Expand description

Parse tokio error into opendal::Error.
