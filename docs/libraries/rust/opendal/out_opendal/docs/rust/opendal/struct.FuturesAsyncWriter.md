# Struct FuturesAsyncWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/write/futures_async_writer.rs.html#35-38" class="src">Source</a>

``` rust
pub struct FuturesAsyncWriter { /* private fields */ }
```

Expand description

FuturesIoAsyncWriter is the adapter of \[`AsyncWrite`\] for [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer").

Users can use this adapter in cases where they need to use \[`AsyncWrite`\] related trait.

FuturesIoAsyncWriter also implements [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") and [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#impl-AsyncWrite-for-FuturesAsyncWriter" class="anchor">Â§</a>

### impl AsyncWrite for <a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html" class="struct" title="struct opendal::FuturesAsyncWriter">FuturesAsyncWriter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#method.poll_write" class="anchor">Â§</a>

#### fn poll_write( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

Attempt to write bytes from `buf` into the object. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#method.poll_flush" class="anchor">Â§</a>

#### fn poll_flush(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Attempt to flush the object, ensuring that any buffered data reach their destination. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#method.poll_close" class="anchor">Â§</a>

#### fn poll_close(self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Attempt to close the object. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#method.poll_write_vectored" class="anchor">Â§</a>

#### fn poll_write_vectored( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>\>

Attempt to write bytes from `bufs` into the object using vectored IO operations. Read more

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html#blanket-implementations" class="anchor">Â§</a>
