# Struct FileStreamProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/stream.rs.html#134" class="src">Source</a>

``` rust
pub struct FileStreamProvider {
    pub schema: Arc<Schema>,
    /* private fields */
}
```

Expand description

Stream data from the file at `location`

- Data will be read sequentially from the provided `location`
- New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html#method.with_encoding "method datafusion::datasource::stream::FileStreamProvider::with_encoding") and defaults to [`StreamEncoding::Csv`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html#variant.Csv "variant datafusion::datasource::stream::StreamEncoding::Csv")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

Get a reference to the schema for this file stream

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#impl-FileStreamProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.new_file" class="fn">new_file</a>(schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, location: <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

Stream data from the file at `location`

- Data will be read sequentially from the provided `location`
- New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html#method.with_encoding "method datafusion::datasource::stream::FileStreamProvider::with_encoding") and defaults to [`StreamEncoding::Csv`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html#variant.Csv "variant datafusion::datasource::stream::StreamEncoding::Csv")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

Set the batch size (the number of rows to load at one time)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.with_header" class="fn">with_header</a>(self, header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

Specify whether the file has a header (only applicable for [`StreamEncoding::Csv`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html#variant.Csv "variant datafusion::datasource::stream::StreamEncoding::Csv"))

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.with_encoding" class="fn">with_encoding</a>(self, encoding: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

Specify an encoding for the stream

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#impl-Debug-for-FileStreamProvider" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#impl-StreamProvider-for-FileStreamProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html" class="trait" title="trait datafusion::datasource::stream::StreamProvider">StreamProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this stream

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.reader" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html#tymethod.reader" class="fn">reader</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchReader">RecordBatchReader</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Provide `RecordBatchReader`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.writer" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html#method.writer" class="fn">writer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchWriter">RecordBatchWriter</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Provide `RecordBatchWriter`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#method.stream_write_display" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html#tymethod.stream_write_display" class="fn">stream_write_display</a>( &self, \_t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Display implementation when using as a DataSink

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.FileStreamProvider.html#blanket-implementations" class="anchor">§</a>
