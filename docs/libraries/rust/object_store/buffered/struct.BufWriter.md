# Struct BufWriter Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/buffered.rs.html#220-228" class="src">Source</a>

``` rust
pub struct BufWriter { /* private fields */ }
```

Expand description

An async buffered writer compatible with the tokio IO traits

This writer adaptively uses [`ObjectStore::put`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put "method object_store::ObjectStore::put") or [`ObjectStore::put_multipart`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") depending on the amount of data that has been written.

Up to `capacity` bytes will be buffered in memory, and flushed on shutdown using [`ObjectStore::put`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put "method object_store::ObjectStore::put"). If `capacity` is exceeded, data will instead be streamed using [`ObjectStore::put_multipart`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#impl-BufWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html" class="struct" title="struct object_store::buffered::BufWriter">BufWriter</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.new" class="fn">new</a>(store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, path: <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> Self

Create a new [`BufWriter`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html "struct object_store::buffered::BufWriter") from the provided [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") and [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.with_capacity" class="fn">with_capacity</a>( store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, path: <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> Self

Create a new [`BufWriter`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html "struct object_store::buffered::BufWriter") from the provided [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore"), [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") and `capacity`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.with_max_concurrency" class="fn">with_max_concurrency</a>(self, max_concurrency: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Override the maximum number of in-flight requests for this writer

Defaults to 8

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.with_attributes" class="fn">with_attributes</a>(self, attributes: <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>) -\> Self

Set the attributes of the uploaded object

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.with_tags" class="fn">with_tags</a>(self, tags: <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>) -\> Self

Set the tags of the uploaded object

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.with_extensions" class="fn">with_extensions</a>(self, extensions: <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>) -\> Self

Set the extensions of the uploaded object

Implementation-specific extensions. Intended for use by [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.put" class="fn">put</a>(&mut self, bytes: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write data to the writer in [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes").

Unlike [`AsyncWrite::poll_write`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_write "method tokio::io::async_write::AsyncWrite::poll_write"), `put` can write data without extra copying.

This API is recommended while the data source generates [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes").

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.abort" class="fn">abort</a>(&mut self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort this writer, cleaning up any partially uploaded state

##### <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#panic" class="doc-anchor">§</a>Panic

Panics if this writer has already been shutdown or aborted

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#impl-AsyncWrite-for-BufWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html" class="trait" title="trait tokio::io::async_write::AsyncWrite">AsyncWrite</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html" class="struct" title="struct object_store::buffered::BufWriter">BufWriter</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.poll_write" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_write" class="fn">poll_write</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>\>

Attempt to write bytes from `buf` into the object. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_write)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.poll_flush" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_flush" class="fn">poll_flush</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>\>

Attempts to flush the object, ensuring that any buffered data reach their destination. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_flush)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.poll_shutdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_shutdown" class="fn">poll_shutdown</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>\>

Initiates or attempts to shut down this writer, returning success when the I/O connection has completely shut down. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_shutdown)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.poll_write_vectored" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#method.poll_write_vectored" class="fn">poll_write_vectored</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>\>

Like [`poll_write`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#tymethod.poll_write "method tokio::io::async_write::AsyncWrite::poll_write"), except that it writes from a slice of buffers. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#method.poll_write_vectored)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.is_write_vectored" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determines if this writer has an efficient [`poll_write_vectored`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#method.poll_write_vectored "method tokio::io::async_write::AsyncWrite::poll_write_vectored") implementation. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html#method.is_write_vectored)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#impl-Debug-for-BufWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html" class="struct" title="struct object_store::buffered::BufWriter">BufWriter</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html#blanket-implementations" class="anchor">§</a>
