# Trait FileSink Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/file_sink_config.rs.html#37" class="src">Source</a>

``` rust
pub trait FileSink: DataSink {
    // Required methods
    fn config(&self) -> &FileSinkConfig;
    fn spawn_writer_tasks_and_join<'life0, 'life1, 'async_trait>(
        &'life0 self,
        context: &'life1 Arc<TaskContext>,
        demux_task: SpawnedTask<Result<(), DataFusionError>>,
        file_stream_rx: UnboundedReceiver<(Path, Receiver<RecordBatch>)>,
        object_store: Arc<dyn ObjectStore>,
    ) -> Pin<Box<dyn Future<Output = Result<u64, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;

    // Provided method
    fn write_all<'life0, 'life1, 'async_trait>(
        &'life0 self,
        data: Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>,
        context: &'life1 Arc<TaskContext>,
    ) -> Pin<Box<dyn Future<Output = Result<u64, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: Sync + 'async_trait { ... }
}
```

Expand description

General behaviors for files that do `DataSink` operations

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#tymethod.config" class="fn">config</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

Retrieves the file sink configuration.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#tymethod.spawn_writer_tasks_and_join" class="fn">spawn_writer_tasks_and_join</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, demux_task: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>, file_stream_rx: <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/sync/mpsc/unbounded/struct.UnboundedReceiver.html" class="struct" title="struct tokio::sync::mpsc::unbounded::UnboundedReceiver">UnboundedReceiver</a>\<(<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/sync/mpsc/bounded/struct.Receiver.html" class="struct" title="struct tokio::sync::mpsc::bounded::Receiver">Receiver</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>)\>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Spawns writer tasks and joins them to perform file writing operations. Is a critical part of `FileSink` trait, since it’s the very last step for `write_all`.

This function handles the process of writing data to files by:

1.  Spawning tasks for writing data to individual files.
2.  Coordinating the tasks using a demuxer to distribute data among files.
3.  Collecting results using `tokio::join`, ensuring that all tasks complete successfully.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#parameters" class="doc-anchor">§</a>Parameters

- `context`: The execution context (`TaskContext`) that provides resources like memory management and runtime environment.
- `demux_task`: A spawned task that handles demuxing, responsible for splitting an input [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") into dynamically determined partitions. See `start_demuxer_task()`
- `file_stream_rx`: A receiver that yields streams of record batches and their corresponding file paths for writing. See `start_demuxer_task()`
- `object_store`: A handle to the object store where the files are written.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#returns" class="doc-anchor">§</a>Returns

- `Result<u64>`: Returns the total number of rows written across all files.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#method.write_all" class="fn">write_all</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, data: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait,

File sink implementation of the [`DataSink::write_all`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.write_all "method datafusion::datasource::sink::DataSink::write_all") method.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#impl-FileSink-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSink">FileSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#impl-FileSink-for-JsonSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSink">FileSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSink.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSink">JsonSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#impl-FileSink-for-ParquetSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSink">FileSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/parquet/struct.ParquetSink.html" class="struct" title="struct datafusion::datasource::file_format::parquet::ParquetSink">ParquetSink</a>
