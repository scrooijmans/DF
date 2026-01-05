# Struct ArrowFileReader Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/reader.rs.html#1325-1332" class="src">Source</a>

``` rust
pub struct ArrowFileReader<R: FileRead> { /* private fields */ }
```

Expand description

ArrowFileReader is a wrapper around a FileRead that impls parquets AsyncFileReader.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#impl-ArrowFileReader%3CR%3E" class="anchor">§</a>

### impl\<R: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html" class="trait" title="trait iceberg::io::FileRead">FileRead</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html" class="struct" title="struct iceberg::arrow::ArrowFileReader">ArrowFileReader</a>\<R\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.new" class="fn">new</a>(meta: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileMetadata.html" class="struct" title="struct iceberg::io::FileMetadata">FileMetadata</a>, r: R) -\> Self

Create a new ArrowFileReader

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.with_preload_column_index" class="fn">with_preload_column_index</a>(self, preload: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Enable or disable preloading of the column index

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.with_preload_offset_index" class="fn">with_preload_offset_index</a>(self, preload: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Enable or disable preloading of the offset index

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.with_preload_page_index" class="fn">with_preload_page_index</a>(self, preload: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Enable or disable preloading of the page index

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.with_metadata_size_hint" class="fn">with_metadata_size_hint</a>(self, hint: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Provide a hint as to the number of bytes to prefetch for parsing the Parquet metadata

This hint can help reduce the number of fetch requests. For more details see the [ParquetMetaDataReader documentation](https://docs.rs/parquet/latest/parquet/file/metadata/struct.ParquetMetaDataReader.html#method.with_prefetch_hint).

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#impl-AsyncFileReader-for-ArrowFileReader%3CR%3E" class="anchor">§</a>

### impl\<R: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html" class="trait" title="trait iceberg::io::FileRead">FileRead</a>\> <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/async_reader/trait.AsyncFileReader.html" class="trait" title="trait parquet::arrow::async_reader::AsyncFileReader">AsyncFileReader</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html" class="struct" title="struct iceberg::arrow::ArrowFileReader">ArrowFileReader</a>\<R\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.get_bytes" class="anchor">§</a>

#### fn <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/async_reader/trait.AsyncFileReader.html#tymethod.get_bytes" class="fn">get_bytes</a>(&mut self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/type.BoxFuture.html" class="type" title="type futures_core::future::BoxFuture">BoxFuture</a>\<'\_, <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/errors/type.Result.html" class="type" title="type parquet::errors::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\>

Retrieve the bytes in `range`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.get_metadata" class="anchor">§</a>

#### fn <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/async_reader/trait.AsyncFileReader.html#tymethod.get_metadata" class="fn">get_metadata</a>( &mut self, \_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/struct.ArrowReaderOptions.html" class="struct" title="struct parquet::arrow::arrow_reader::ArrowReaderOptions">ArrowReaderOptions</a>\>, ) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/type.BoxFuture.html" class="type" title="type futures_core::future::BoxFuture">BoxFuture</a>\<'\_, <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/errors/type.Result.html" class="type" title="type parquet::errors::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/file/metadata/struct.ParquetMetaData.html" class="struct" title="struct parquet::file::metadata::ParquetMetaData">ParquetMetaData</a>\>\>\>

Return a future which results in the [`ParquetMetaData`](https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/file/metadata/struct.ParquetMetaData.html "struct parquet::file::metadata::ParquetMetaData") for this Parquet file. [Read more](https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/async_reader/trait.AsyncFileReader.html#tymethod.get_metadata)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#method.get_byte_ranges" class="anchor">§</a>

#### fn <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/arrow/async_reader/trait.AsyncFileReader.html#method.get_byte_ranges" class="fn">get_byte_ranges</a>( &mut self, ranges: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>, <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + '\_\>\>

Retrieve multiple byte ranges. The default implementation will call `get_bytes` sequentially

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html#blanket-implementations" class="anchor">§</a>
