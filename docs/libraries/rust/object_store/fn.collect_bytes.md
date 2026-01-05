# Function collect_bytes Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/util.rs.html#52-75" class="src">Source</a>

``` rust
pub async fn collect_bytes<S, E>(
    stream: S,
    size_hint: Option<u64>,
) -> Result<Bytes, E>where
    E: Send,
    S: Stream<Item = Result<Bytes, E>> + Send + Unpin,
```

Expand description

Collect a stream into [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") avoiding copying in the event of a single chunk
