# Module physical_plan Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/physical_plan/mod.rs.html#18-190" class="src">Source</a>

Expand description

Execution plans that read file formats

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/avro/index.html" class="mod" title="mod datafusion::datasource::physical_plan::avro">avro</a>`avro`  
Reexports the [`datafusion_datasource_json::source`](https://docs.rs/datafusion-datasource-json/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_json/source/index.html "mod datafusion_datasource_json::source") module, containing [Avro](https://avro.apache.org/) based [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/csv/index.html" class="mod" title="mod datafusion::datasource::physical_plan::csv">csv</a>  
Reexports the [`datafusion_datasource_json::source`](https://docs.rs/datafusion-datasource-json/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_json/source/index.html "mod datafusion_datasource_json::source") module, containing CSV based [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/json/index.html" class="mod" title="mod datafusion::datasource::physical_plan::json">json</a>  
Reexports the [`datafusion_datasource_json::source`](https://docs.rs/datafusion-datasource-json/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_json/source/index.html "mod datafusion_datasource_json::source") module, containing JSON based [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/index.html" class="mod" title="mod datafusion::datasource::physical_plan::parquet">parquet</a>`parquet`  
Reexports the [`datafusion_datasource_parquet`](https://docs.rs/datafusion-datasource-parquet/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_parquet/index.html "mod datafusion_datasource_parquet") crate, containing Parquet based [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource").

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ArrowSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ArrowSource">ArrowSource</a>  
Arrow configuration struct that is given to DataSourceExec Does not hold anything special, since [`FileScanConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html "struct datafusion::datasource::physical_plan::FileScanConfig") is sufficient for arrow

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>`avro`  
AvroSource holds the extra configuration that is necessary for opening avro files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.CsvOpener.html" class="struct" title="struct datafusion::datasource::physical_plan::CsvOpener">CsvOpener</a>  
A [`FileOpener`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html "trait datafusion::datasource::physical_plan::FileOpener") that opens a CSV file and yields a [`FileOpenFuture`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/type.FileOpenFuture.html "type datafusion::datasource::physical_plan::FileOpenFuture")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.CsvSource.html" class="struct" title="struct datafusion::datasource::physical_plan::CsvSource">CsvSource</a>  
A Config for [`CsvOpener`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.CsvOpener.html "struct datafusion::datasource::physical_plan::CsvOpener")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroup.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroup">FileGroup</a>  
Represents a group of partitioned files that’ll be processed by a single thread. Maintains optional statistics across all files in the group.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>  
Repartition input files into `target_partitions` partitions, if total file size exceed `repartition_file_min_size`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileMeta.html" class="struct" title="struct datafusion::datasource::physical_plan::FileMeta">FileMeta</a>  
A single file or part of a file that should be read, along with its schema, statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>  
The base configurations for a [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec"), the a physical plan for any given file format.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfigBuilder.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfigBuilder">FileScanConfigBuilder</a>  
A builder for [`FileScanConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html "struct datafusion::datasource::physical_plan::FileScanConfig")’s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>  
The base configurations to provide when creating a physical plan for writing to any given file format.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileStream.html" class="struct" title="struct datafusion::datasource::physical_plan::FileStream">FileStream</a>  
A stream that iterates record batch by record batch, file over file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.JsonOpener.html" class="struct" title="struct datafusion::datasource::physical_plan::JsonOpener">JsonOpener</a>  
A [`FileOpener`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html "trait datafusion::datasource::physical_plan::FileOpener") that opens a JSON file and yields a [`FileOpenFuture`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/type.FileOpenFuture.html "type datafusion::datasource::physical_plan::FileOpenFuture")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.JsonSource.html" class="struct" title="struct datafusion::datasource::physical_plan::JsonSource">JsonSource</a>  
JsonSource holds the extra configuration that is necessary for [`JsonOpener`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.JsonOpener.html "struct datafusion::datasource::physical_plan::JsonOpener")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetFileMetrics.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetFileMetrics">ParquetFileMetrics</a>`parquet`  
Stores metrics about the parquet execution for a particular parquet file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>`parquet`  
Execution plan for reading one or more Parquet files.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/enum.OnError.html" class="enum" title="enum datafusion::datasource::physical_plan::OnError">OnError</a>  
Describes the behavior of the `FileStream` if file opening or scanning fails

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a>  
Generic API for opening a file using an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") and resolving to a stream of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSink.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSink">FileSink</a>  
General behaviors for files that do `DataSink` operations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>  
file format specific behaviors for elements in [`DataSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html "trait datafusion::datasource::source::DataSource")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html" class="trait" title="trait datafusion::datasource::physical_plan::ParquetFileReaderFactory">ParquetFileReaderFactory</a>`parquet`  
Interface for reading parquet files.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/fn.wrap_partition_type_in_dict.html" class="fn" title="fn datafusion::datasource::physical_plan::wrap_partition_type_in_dict">wrap_partition_type_in_dict</a>  
Convert type to a type suitable for use as a `ListingTable` partition column. Returns `Dictionary(UInt16, val_type)`, which is a reasonable trade off between a reasonable number of partition values and space efficiency.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/fn.wrap_partition_value_in_dict.html" class="fn" title="fn datafusion::datasource::physical_plan::wrap_partition_value_in_dict">wrap_partition_value_in_dict</a>  
Convert a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") of partition columns to a type, as described in the documentation of [`wrap_partition_type_in_dict`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/fn.wrap_partition_type_in_dict.html "fn datafusion::datasource::physical_plan::wrap_partition_type_in_dict"), which can wrap the types.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/type.FileOpenFuture.html" class="type" title="type datafusion::datasource::physical_plan::FileOpenFuture">FileOpenFuture</a>  
A fallible future that resolves to a stream of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")
