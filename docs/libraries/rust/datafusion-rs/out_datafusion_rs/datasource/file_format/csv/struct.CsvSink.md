# Struct CsvSink Copy item path

<a href="https://docs.rs/datafusion-datasource-csv/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_csv/file_format.rs.html#656" class="src">Source</a>

``` rust
pub struct CsvSink { /* private fields */ }
```

Expand description

Implements [`DataSink`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink") for writing to a CSV file.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#impl-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.new" class="fn">new</a>(config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>, writer_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/csv_writer/struct.CsvWriterOptions.html" class="struct" title="struct datafusion::common::file_options::csv_writer::CsvWriterOptions">CsvWriterOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

Create from config.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.writer_options" class="fn">writer_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/csv_writer/struct.CsvWriterOptions.html" class="struct" title="struct datafusion::common::file_options::csv_writer::CsvWriterOptions">CsvWriterOptions</a>

Retrieve the writer options

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#impl-DataSink-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the data sink as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the sink schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.write_all-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.write_all" class="fn">write_all</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, data: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>: 'async_trait,

Writes the data to the sink, returns the number of values written [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.write_all)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the [MetricsSet](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html "struct datafusion::physical_plan::metrics::MetricsSet") for this [DataSink](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#method.metrics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#impl-Debug-for-CsvSink" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#impl-DisplayAs-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.fmt_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format according to `DisplayFormatType`, used when verbose representation looks different from the default one [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#impl-FileSink-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSink">FileSink</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.config" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#tymethod.config" class="fn">config</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

Retrieves the file sink configuration.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.spawn_writer_tasks_and_join" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#tymethod.spawn_writer_tasks_and_join" class="fn">spawn_writer_tasks_and_join</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, demux_task: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>, file_stream_rx: <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/sync/mpsc/unbounded/struct.UnboundedReceiver.html" class="struct" title="struct tokio::sync::mpsc::unbounded::UnboundedReceiver">UnboundedReceiver</a>\<(<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/sync/mpsc/bounded/struct.Receiver.html" class="struct" title="struct tokio::sync::mpsc::bounded::Receiver">Receiver</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>)\>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>: 'async_trait,

Spawns writer tasks and joins them to perform file writing operations. Is a critical part of `FileSink` trait, since it’s the very last step for `write_all`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#tymethod.spawn_writer_tasks_and_join)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#method.write_all" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html#method.write_all" class="fn">write_all</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, data: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, context: &'life1 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait,

File sink implementation of the [`DataSink::write_all`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html#tymethod.write_all "method datafusion::datasource::sink::DataSink::write_all") method.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html#blanket-implementations" class="anchor">§</a>
