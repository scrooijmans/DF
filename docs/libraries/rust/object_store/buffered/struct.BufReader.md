# Struct BufReader Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/buffered.rs.html#55-68" class="src">Source</a>

``` rust
pub struct BufReader { /* private fields */ }
```

Expand description

An async-buffered reader compatible with the tokio IO traits

Internally this maintains a buffer of the requested size, and uses [`ObjectStore::get_range`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range "method object_store::ObjectStore::get_range") to populate its internal buffer once depleted. This buffer is cleared on seek.

Whilst simple, this interface will typically be outperformed by the native [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") methods that better map to the network APIs. This is because most object stores have very [high first-byte latencies](https://docs.aws.amazon.com/AmazonS3/latest/userguide/optimizing-performance.html), on the order of 100-200ms, and so avoiding unnecessary round-trips is critical to throughput.

Systems looking to sequentially scan a file should instead consider using [`ObjectStore::get`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get "method object_store::ObjectStore::get"), or [`ObjectStore::get_opts`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.get_opts "method object_store::ObjectStore::get_opts"), or [`ObjectStore::get_range`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range "method object_store::ObjectStore::get_range") to read a particular range.

Systems looking to read multiple ranges of a file should instead consider using [`ObjectStore::get_ranges`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_ranges "method object_store::ObjectStore::get_ranges"), which will optimise the vectored IO.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#impl-BufReader" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html" class="struct" title="struct object_store::buffered::BufReader">BufReader</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.new" class="fn">new</a>(store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, meta: &<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>) -\> Self

Create a new [`BufReader`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html "struct object_store::buffered::BufReader") from the provided [`ObjectMeta`](https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html "struct object_store::ObjectMeta") and [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.with_capacity" class="fn">with_capacity</a>( store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, meta: &<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> Self

Create a new [`BufReader`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html "struct object_store::buffered::BufReader") from the provided [`ObjectMeta`](https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html "struct object_store::ObjectMeta"), [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore"), and `capacity`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#impl-AsyncBufRead-for-BufReader" class="anchor">§</a>

### impl <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_buf_read/trait.AsyncBufRead.html" class="trait" title="trait tokio::io::async_buf_read::AsyncBufRead">AsyncBufRead</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html" class="struct" title="struct object_store::buffered::BufReader">BufReader</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.poll_fill_buf" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_buf_read/trait.AsyncBufRead.html#tymethod.poll_fill_buf" class="fn">poll_fill_buf</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>

Attempts to return the contents of the internal buffer, filling it with more data from the inner reader if it is empty. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_buf_read/trait.AsyncBufRead.html#tymethod.poll_fill_buf)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.consume" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_buf_read/trait.AsyncBufRead.html#tymethod.consume" class="fn">consume</a>(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, amt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Tells this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to [`poll_read`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_read/trait.AsyncRead.html#tymethod.poll_read "method tokio::io::async_read::AsyncRead::poll_read"). [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_buf_read/trait.AsyncBufRead.html#tymethod.consume)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#impl-AsyncRead-for-BufReader" class="anchor">§</a>

### impl <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_read/trait.AsyncRead.html" class="trait" title="trait tokio::io::async_read::AsyncRead">AsyncRead</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html" class="struct" title="struct object_store::buffered::BufReader">BufReader</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.poll_read" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_read/trait.AsyncRead.html#tymethod.poll_read" class="fn">poll_read</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, out: &mut <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/read_buf/struct.ReadBuf.html" class="struct" title="struct tokio::io::read_buf::ReadBuf">ReadBuf</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Attempts to read from the `AsyncRead` into `buf`. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_read/trait.AsyncRead.html#tymethod.poll_read)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#impl-AsyncSeek-for-BufReader" class="anchor">§</a>

### impl <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_seek/trait.AsyncSeek.html" class="trait" title="trait tokio::io::async_seek::AsyncSeek">AsyncSeek</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html" class="struct" title="struct object_store::buffered::BufReader">BufReader</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.start_seek" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_seek/trait.AsyncSeek.html#tymethod.start_seek" class="fn">start_seek</a>(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, position: <a href="https://doc.rust-lang.org/nightly/std/io/enum.SeekFrom.html" class="enum" title="enum std::io::SeekFrom">SeekFrom</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Attempts to seek to an offset, in bytes, in a stream. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_seek/trait.AsyncSeek.html#tymethod.start_seek)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.poll_complete" class="anchor">§</a>

#### fn <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_seek/trait.AsyncSeek.html#tymethod.poll_complete" class="fn">poll_complete</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, \_cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>

Waits for a seek operation to complete. [Read more](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/io/async_seek/trait.AsyncSeek.html#tymethod.poll_complete)

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#impl-Debug-for-BufReader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html" class="struct" title="struct object_store::buffered::BufReader">BufReader</a>

<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html#blanket-implementations" class="anchor">§</a>
