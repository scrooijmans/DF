# Trait DataSink Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/sink.rs.html#48" class="src">Source</a>

``` rust
pub trait DataSink:
    DisplayAs
    + Debug
    + Send
    + Sync {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn schema(&self) -> &Arc<Schema>;
    fn write_all<'life0, 'life1, 'async_trait>(
        &'life0 self,
        data: Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>,
        context: &'life1 Arc<TaskContext>,
    ) -> Pin<Box<dyn Future<Output = Result<u64, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;

    // Provided method
    fn metrics(&self) -> Option<MetricsSet> { ... }
}
```

Expand description

`DataSink` implements writing streams of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to user defined destinations.

The `Display` impl is used to format the sink for explain plan output.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the data sink as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the sink schema

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.write_all" class="fn">write_all</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, data: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Writes the data to the sink, returns the number of values written

This method will be called exactly once during each DML statement. Thus prior to return, the sink should do any commit or rollback required.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the [MetricsSet](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html "struct datafusion::physical_plan::metrics::MetricsSet") for this [DataSink](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink").

See [ExecutionPlan::metrics()](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics "method datafusion::physical_plan::ExecutionPlan::metrics") for more details

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#impl-DataSink-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#impl-DataSink-for-JsonSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSink.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSink">JsonSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#impl-DataSink-for-ParquetSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/parquet/struct.ParquetSink.html" class="struct" title="struct datafusion::datasource::file_format::parquet::ParquetSink">ParquetSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#impl-DataSink-for-MemSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a> for <a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource/memory/struct.MemSink.html" class="struct" title="struct datafusion_datasource::memory::MemSink">MemSink</a>
