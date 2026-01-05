# Trait FileOpener Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/file_stream.rs.html#366" class="src">Source</a>

``` rust
pub trait FileOpener:
    Unpin
    + Send
    + Sync {
    // Required method
    fn open(
        &self,
        file_meta: FileMeta,
        file: PartitionedFile,
    ) -> Result<Pin<Box<dyn Future<Output = Result<Pin<Box<dyn Stream<Item = Result<RecordBatch, DataFusionError>> + Send>>, DataFusionError>> + Send>>, DataFusionError>;
}
```

Expand description

Generic API for opening a file using an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") and resolving to a stream of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html#tymethod.open" class="fn">open</a>( &self, file_meta: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileMeta.html" class="struct" title="struct datafusion::datasource::physical_plan::FileMeta">FileMeta</a>, file: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.PartitionedFile.html" class="struct" title="struct datafusion::datasource::listing::PartitionedFile">PartitionedFile</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Asynchronously open the specified file and return a stream of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html#impl-FileOpener-for-CsvOpener" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.CsvOpener.html" class="struct" title="struct datafusion::datasource::physical_plan::CsvOpener">CsvOpener</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html#impl-FileOpener-for-JsonOpener" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.JsonOpener.html" class="struct" title="struct datafusion::datasource::physical_plan::JsonOpener">JsonOpener</a>
