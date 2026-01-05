# Module stream Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/lib.rs.html#39" class="src">Source</a>

Expand description

TableProvider for stream sources, such as FIFO files

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.FileStreamProvider.html" class="struct" title="struct datafusion::datasource::stream::FileStreamProvider">FileStreamProvider</a>  
Stream data from the file at `location`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>  
The configuration for a [`StreamTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html "struct datafusion::datasource::stream::StreamTable")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html" class="struct" title="struct datafusion::datasource::stream::StreamTable">StreamTable</a>  
A [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") for an unbounded stream source

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTableFactory.html" class="struct" title="struct datafusion::datasource::stream::StreamTableFactory">StreamTableFactory</a>  
A [`TableProviderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory") for [`StreamTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html "struct datafusion::datasource::stream::StreamTable")

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>  
The data encoding for [`StreamTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html "struct datafusion::datasource::stream::StreamTable")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html" class="trait" title="trait datafusion::datasource::stream::StreamProvider">StreamProvider</a>  
The StreamProvider trait is used as a generic interface for reading and writing from streaming data sources (such as FIFO, Websocket, Kafka, etc.). Implementations of the provider are responsible for providing a `RecordBatchReader` and optionally a `RecordBatchWriter`.
