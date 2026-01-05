# Struct Reader Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/read/reader.rs.html#92-94" class="src">Source</a>

``` rust
pub struct Reader { /* private fields */ }
```

Expand description

Reader is designed to read data from given path in an asynchronous manner.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#usage" class="doc-anchor">Â§</a>Usage

[`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") provides multiple ways to read data from given reader.

`Reader` implements `Clone` so you can clone it and store in place where ever you want.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#direct" class="doc-anchor">Â§</a>Direct

[`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") provides public API including [`Reader::read`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.read "method opendal::Reader::read"). You can use those APIs directly without extra copy.

``` rust
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> Result<()> {
    let r = op.reader("path/to/file").await?;
    let bs = r.read(0..1024).await?;
    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#read-like-stream" class="doc-anchor">Â§</a>Read like `Stream`

``` rust
use anyhow::Result;
use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let s = op
        .reader("path/to/file")
        .await?
        .into_bytes_stream(1024..2048)
        .await?;
    let bs: Vec<Bytes> = s.try_collect().await?;
    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#read-like-asyncread-and-asyncbufread" class="doc-anchor">Â§</a>Read like `AsyncRead` and `AsyncBufRead`

``` rust
use anyhow::Result;
use bytes::Bytes;
use futures::AsyncReadExt;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let mut r = op
        .reader("path/to/file")
        .await?
        .into_futures_async_read(1024..2048)
        .await?;
    let mut bs = vec![];
    let n = r.read_to_end(&mut bs).await?;
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#impl-Reader" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.read" class="fn">read</a>(&self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read give range from reader into [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer").

This operation is zero-copy, which means it keeps the \[`bytes::Bytes`\] returned by underlying storage services without any extra copy or intensive memory allocations.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.read_into" class="fn">read_into</a>( &self, buf: &mut impl BufMut, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Read all data from reader into given \[`BufMut`\].

This operation will copy and write bytes into given \[`BufMut`\]. Allocation happens while \[`BufMut`\] doesnâ€™t have enough space.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.fetch" class="fn">fetch</a>(&self, ranges: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>

Fetch specific ranges from reader.

This operation try to merge given ranges into a list of non-overlapping ranges. Users may also specify a `gap` to merge close ranges.

The returning `Buffer` may share the same underlying memory without any extra copy.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_stream" class="fn">into_stream</a>( self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html" class="struct" title="struct opendal::BufferStream">BufferStream</a>\>

Create a buffer stream to read specific range from given reader.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#notes" class="doc-anchor">Â§</a>Notes

BufferStream is a zero-cost abstraction. It doesnâ€™t involve extra copy of data. It will return underlying [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") directly.

The [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") this stream yields can be seen as an iterator of \[`Bytes`\].

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#inputs" class="doc-anchor">Â§</a>Inputs

- `range`: The range of data to read. range like `..` it will read all data from reader.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#examples" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#basic-usage" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Buffer;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut s = op
        .reader("hello.txt")
        .await?
        .into_stream(1024..2048)
        .await?;

    let bs: Vec<Buffer> = s.try_collect().await?;
    // We can use those buffer as bytes if we want.
    let bytes_vec: Vec<Bytes> = bs.clone().into_iter().flatten().collect();
    // Or we can merge them into a single [`Buffer`] and later use it as [`bytes::Buf`].
    let new_buffer: Buffer = bs.into_iter().flatten().collect::<Buffer>();

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#concurrent-read" class="doc-anchor">Â§</a>Concurrent Read

The following example reads data in 256B chunks with 8 concurrent.

``` rust
use std::io;

use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Buffer;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let s = op
        .reader_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_stream(1024..2048)
        .await?;

    // Every buffer except the last one in the stream will be 256B.
    let bs: Vec<Buffer> = s.try_collect().await?;
    // We can use those buffer as bytes if we want.
    let bytes_vec: Vec<Bytes> = bs.clone().into_iter().flatten().collect();
    // Or we can merge them into a single [`Buffer`] and later use it as [`bytes::Buf`].
    let new_buffer: Buffer = bs.into_iter().flatten().collect::<Buffer>();

    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_futures_async_read" class="fn">into_futures_async_read</a>( self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncReader.html" class="struct" title="struct opendal::FuturesAsyncReader">FuturesAsyncReader</a>\>

Convert reader into [`FuturesAsyncReader`](https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncReader.html "struct opendal::FuturesAsyncReader") which implements \[`futures::AsyncRead`\], \[`futures::AsyncSeek`\] and \[`futures::AsyncBufRead`\].

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#notes-1" class="doc-anchor">Â§</a>Notes

FuturesAsyncReader is not a zero-cost abstraction. The underlying reader returns an owned [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer"), which involves an extra copy operation.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#inputs-1" class="doc-anchor">Â§</a>Inputs

- `range`: The range of data to read. range like `..` it will read all data from reader.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#examples-1" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#basic-usage-1" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use futures::io::AsyncReadExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut r = op
        .reader("hello.txt")
        .await?
        .into_futures_async_read(1024..2048)
        .await?;
    let mut bs = Vec::new();
    r.read_to_end(&mut bs).await?;

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#concurrent-read-1" class="doc-anchor">Â§</a>Concurrent Read

The following example reads data in 256B chunks with 8 concurrent.

``` rust
use std::io;

use futures::io::AsyncReadExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut r = op
        .reader_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_futures_async_read(1024..2048)
        .await?;
    let mut bs = Vec::new();
    r.read_to_end(&mut bs).await?;

    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_bytes_stream" class="fn">into_bytes_stream</a>( self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesStream.html" class="struct" title="struct opendal::FuturesBytesStream">FuturesBytesStream</a>\>

Convert reader into [`FuturesBytesStream`](https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesStream.html "struct opendal::FuturesBytesStream") which implements \[`futures::Stream`\].

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#inputs-2" class="doc-anchor">Â§</a>Inputs

- `range`: The range of data to read. range like `..` it will read all data from reader.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#examples-2" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#basic-usage-2" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut s = op
        .reader("hello.txt")
        .await?
        .into_bytes_stream(1024..2048)
        .await?;
    let bs: Vec<Bytes> = s.try_collect().await?;

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#concurrent-read-2" class="doc-anchor">Â§</a>Concurrent Read

The following example reads data in 256B chunks with 8 concurrent.

``` rust
use std::io;

use bytes::Bytes;
use futures::TryStreamExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut s = op
        .reader_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_bytes_stream(1024..2048)
        .await?;
    let bs: Vec<Bytes> = s.try_collect().await?;

    Ok(())
}
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#impl-Clone-for-Reader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#blanket-implementations" class="anchor">Â§</a>
