# Module delimited Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/delimited.rs.html#18-272" class="src">Source</a>

Expand description

Utility for streaming newline delimited files from object storage

## Functions<a href="https://docs.rs/object_store/latest/object_store/delimited/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/delimited/fn.newline_delimited_stream.html" class="fn" title="fn object_store::delimited::newline_delimited_stream">newline_delimited_stream</a>  
Given a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") returns a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") where each yielded [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") contains a whole number of new line delimited records accounting for `\` style escapes and `"` quotes
