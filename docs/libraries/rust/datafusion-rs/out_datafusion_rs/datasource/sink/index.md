# Module sink Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/mod.rs.html#42" class="src">Source</a>

Expand description

Execution plan for writing data to [`DataSink`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink")s

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/struct.DataSinkExec.html" class="struct" title="struct datafusion::datasource::sink::DataSinkExec">DataSinkExec</a>  
Execution plan for writing record batches to a [`DataSink`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html" class="trait" title="trait datafusion::datasource::sink::DataSink">DataSink</a>  
`DataSink` implements writing streams of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to user defined destinations.
