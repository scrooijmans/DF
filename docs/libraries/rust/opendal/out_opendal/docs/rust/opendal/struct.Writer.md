# Struct Writer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/write/writer.rs.html#100-104" class="src">Source</a>

``` rust
pub struct Writer { /* private fields */ }
```

Expand description

Writer is designed to write data into given path in an asynchronous manner.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes" class="doc-anchor">Â§</a>Notes

Please make sure either `close` or `abort` has been called before dropping the writer otherwise the data could be lost.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#usage" class="doc-anchor">Â§</a>Usage

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#write-multiple-chunks" class="doc-anchor">Â§</a>Write Multiple Chunks

Some services support to write multiple chunks of data into given path. Services that doesnâ€™t support write multiple chunks will return [`ErrorKind::Unsupported`](https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unsupported "variant opendal::ErrorKind::Unsupported") error when calling `write` at the second time.

``` rust
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> Result<()> {
    let mut w = op.writer("path/to/file").await?;
    w.write(vec![1; 1024]).await?;
    w.write(vec![2; 1024]).await?;
    w.close().await?;
    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#write-like-sink" class="doc-anchor">Â§</a>Write like `Sink`

``` rust
use anyhow::Result;
use futures::SinkExt;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let mut w = op.writer("path/to/file").await?.into_bytes_sink();
    w.send(vec![1; 1024].into()).await?;
    w.send(vec![2; 1024].into()).await?;
    w.close().await?;
    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#write-like-asyncwrite" class="doc-anchor">Â§</a>Write like `AsyncWrite`

``` rust
use anyhow::Result;
use futures::AsyncWriteExt;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let mut w = op.writer("path/to/file").await?.into_futures_async_write();
    w.write(&vec![1; 1024]).await?;
    w.write(&vec![2; 1024]).await?;
    w.close().await?;
    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#write-with-append-enabled" class="doc-anchor">Â§</a>Write with append enabled

Writer also supports to write with append enabled. This is useful when users want to append some data to the end of the file.

- If file doesnâ€™t exist, it will be created and just like calling `write`.
- If file exists, data will be appended to the end of the file.

Possible Errors:

- Some services store normal file and appendable file in different way. Trying to append on non-appendable file could return [`ErrorKind::ConditionNotMatch`](https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.ConditionNotMatch "variant opendal::ErrorKind::ConditionNotMatch") error.
- Services that doesnâ€™t support append will return [`ErrorKind::Unsupported`](https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unsupported "variant opendal::ErrorKind::Unsupported") error when creating writer with `append` enabled.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#impl-Writer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html" class="struct" title="struct opendal::Writer">Writer</a>

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.write" class="fn">write</a>(&mut self, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") into writer.

This operation will write all data in given buffer into writer.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> Result<()> {
    let mut w = op.writer("hello.txt").await?;
    // Buffer can be created from continues bytes.
    w.write("hello, world").await?;
    // Buffer can also be created from non-continues bytes.
    w.write(vec![Bytes::from("hello,"), Bytes::from("world!")])
        .await?;

    // Make sure file has been written completely.
    w.close().await?;
    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.write_from" class="fn">write_from</a>(&mut self, bs: impl Buf) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write \[`bytes::Buf`\] into inner writer.

This operation will write all data in given buffer into writer.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#todo" class="doc-anchor">Â§</a>TODO

Optimize this function to avoid unnecessary copy.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the writer and clean up all written data.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes-1" class="doc-anchor">Â§</a>Notes

Abort should only be called when the writer is not closed or aborted, otherwise an unexpected error could be returned.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data have been committed.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes-2" class="doc-anchor">Â§</a>Notes

Close should only be called when the writer is not closed or aborted, otherwise an unexpected error could be returned.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.into_sink" class="fn">into_sink</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferSink.html" class="struct" title="struct opendal::BufferSink">BufferSink</a>

Convert writer into [`BufferSink`](https://opendal.apache.org/docs/rust/opendal/struct.BufferSink.html "struct opendal::BufferSink") which implements \[`Sink<Buffer>`\].

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes-3" class="doc-anchor">Â§</a>Notes

BufferSink is a zero-cost abstraction. The underlying writer will reuse the Bytes and wonâ€™t perform any copy operation over data.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#examples-1" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#basic-usage" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use bytes::Bytes;
use futures::SinkExt;
use opendal::Buffer;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut s = op.writer("hello.txt").await?.into_sink();
    let bs = "Hello, World!".as_bytes();
    s.send(Buffer::from(bs)).await?;
    s.close().await?;

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#concurrent-write" class="doc-anchor">Â§</a>Concurrent Write

``` rust
use std::io;

use bytes::Bytes;
use futures::SinkExt;
use opendal::Buffer;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut w = op
        .writer_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_sink();
    let bs = "Hello, World!".as_bytes();
    w.send(Buffer::from(bs)).await?;
    w.close().await?;

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.into_futures_async_write" class="fn">into_futures_async_write</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html" class="struct" title="struct opendal::FuturesAsyncWriter">FuturesAsyncWriter</a>

Convert writer into [`FuturesAsyncWriter`](https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html "struct opendal::FuturesAsyncWriter") which implements \[`futures::AsyncWrite`\],

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes-4" class="doc-anchor">Â§</a>Notes

FuturesAsyncWriter is not a zero-cost abstraction. The underlying writer requires an owned [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer"), which involves an extra copy operation.

FuturesAsyncWriter is required to call `close()` to make sure all data have been written to the storage.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#examples-2" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#basic-usage-1" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use futures::io::AsyncWriteExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut w = op.writer("hello.txt").await?.into_futures_async_write();
    let bs = "Hello, World!".as_bytes();
    w.write_all(bs).await?;
    w.close().await?;

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#concurrent-write-1" class="doc-anchor">Â§</a>Concurrent Write

``` rust
use std::io;

use futures::io::AsyncWriteExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut w = op
        .writer_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_futures_async_write();
    let bs = "Hello, World!".as_bytes();
    w.write_all(bs).await?;
    w.close().await?;

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.into_bytes_sink" class="fn">into_bytes_sink</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesSink.html" class="struct" title="struct opendal::FuturesBytesSink">FuturesBytesSink</a>

Convert writer into [`FuturesBytesSink`](https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesSink.html "struct opendal::FuturesBytesSink") which implements \[`futures::Sink<Bytes>`\].

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#notes-5" class="doc-anchor">Â§</a>Notes

FuturesBytesSink is a zero-cost abstraction. The underlying writer will reuse the Bytes and wonâ€™t perform any copy operation.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#examples-3" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#basic-usage-2" class="doc-anchor">Â§</a>Basic Usage

``` rust
use std::io;

use bytes::Bytes;
use futures::SinkExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut w = op.writer("hello.txt").await?.into_bytes_sink();
    let bs = "Hello, World!".as_bytes();
    w.send(Bytes::from(bs)).await?;
    w.close().await?;

    Ok(())
}
```

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#concurrent-write-2" class="doc-anchor">Â§</a>Concurrent Write

``` rust
use std::io;

use bytes::Bytes;
use futures::SinkExt;
use opendal::Operator;
use opendal::Result;

async fn test(op: Operator) -> io::Result<()> {
    let mut w = op
        .writer_with("hello.txt")
        .concurrent(8)
        .chunk(256)
        .await?
        .into_bytes_sink();
    let bs = "Hello, World!".as_bytes();
    w.send(Bytes::from(bs)).await?;
    w.close().await?;

    Ok(())
}
```

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#blanket-implementations" class="anchor">Â§</a>
