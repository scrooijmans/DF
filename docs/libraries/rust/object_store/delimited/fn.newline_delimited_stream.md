# Function newline_delimited_stream Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/delimited.rs.html#152-183" class="src">Source</a>

``` rust
pub fn newline_delimited_stream<S>(s: S) -> impl Stream<Item = Result<Bytes>>where
    S: Stream<Item = Result<Bytes>> + Unpin,
```

Expand description

Given a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") returns a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") where each yielded [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") contains a whole number of new line delimited records accounting for `\` style escapes and `"` quotes
