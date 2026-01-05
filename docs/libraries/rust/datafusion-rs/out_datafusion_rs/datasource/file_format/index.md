# Module file_format Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/file_format/mod.rs.html#18-137" class="src">Source</a>

Expand description

Module containing helper methods for the various file formats See write.rs for write related helper methods

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/arrow/index.html" class="mod" title="mod datafusion::datasource::file_format::arrow">arrow</a>  
[`ArrowFormat`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/arrow/struct.ArrowFormat.html "struct datafusion::datasource::file_format::arrow::ArrowFormat"): Apache Arrow [`FileFormat`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html "trait datafusion::datasource::file_format::FileFormat") abstractions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/avro/index.html" class="mod" title="mod datafusion::datasource::file_format::avro">avro</a>`avro`  
Re-exports the [`datafusion_datasource_avro::file_format`](https://docs.rs/datafusion-datasource-avro/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_avro/file_format/index.html "mod datafusion_datasource_avro::file_format") module, and contains tests for it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/index.html" class="mod" title="mod datafusion::datasource::file_format::csv">csv</a>  
Re-exports the [`datafusion_datasource_csv::file_format`](https://docs.rs/datafusion-datasource-csv/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_csv/file_format/index.html "mod datafusion_datasource_csv::file_format") module, and contains tests for it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/index.html" class="mod" title="mod datafusion::datasource::file_format::file_compression_type">file_compression_type</a>  
File Compression type abstraction

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/index.html" class="mod" title="mod datafusion::datasource::file_format::json">json</a>  
Re-exports the [`datafusion_datasource_json::file_format`](https://docs.rs/datafusion-datasource-json/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource_json/file_format/index.html "mod datafusion_datasource_json::file_format") module, and contains tests for it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/index.html" class="mod" title="mod datafusion::datasource::file_format::options">options</a>  
User facing options for the file formats readers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/parquet/index.html" class="mod" title="mod datafusion::datasource::file_format::parquet">parquet</a>`parquet`  
Re-exports the [`datafusion_datasource_parquet::file_format`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/file_format/index.html "mod datafusion::datasource::physical_plan::parquet::file_format") module, and contains tests for it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/index.html" class="mod" title="mod datafusion::datasource::file_format::write">write</a>  
Module containing helper methods/traits related to enabling write support for the various file formats

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/struct.DefaultFileType.html" class="struct" title="struct datafusion::datasource::file_format::DefaultFileType">DefaultFileType</a>  
A container of [FileFormatFactory](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory") which also implements [FileType](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html "trait datafusion::common::file_options::file_type::FileType"). This enables converting a dyn FileFormat to a dyn FileType. The former trait is a superset of the latter trait, which includes execution time relevant methods. [FileType](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html "trait datafusion::common::file_options::file_type::FileType") is only used in logical planning and only implements the subset of methods required during logical planning.

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/constant.DEFAULT_SCHEMA_INFER_MAX_RECORD.html" class="constant" title="constant datafusion::datasource::file_format::DEFAULT_SCHEMA_INFER_MAX_RECORD">DEFAULT_SCHEMA_INFER_MAX_RECORD</a>  
Default max records to scan to infer the schema

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html" class="trait" title="trait datafusion::datasource::file_format::FileFormat">FileFormat</a>  
This trait abstracts all the file format specific implementations from the [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html). This helps code re-utilization across providers that support the same file formats.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>  
Factory for creating [`FileFormat`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html "trait datafusion::datasource::file_format::FileFormat") instances based on session and command level options

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/fn.file_type_to_format.html" class="fn" title="fn datafusion::datasource::file_format::file_type_to_format">file_type_to_format</a>  
Converts a [FileType](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html "trait datafusion::common::file_options::file_type::FileType") to a [FileFormatFactory](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory"). Returns an error if the [FileType](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html "trait datafusion::common::file_options::file_type::FileType") cannot be downcasted to a [DefaultFileType](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/struct.DefaultFileType.html "struct datafusion::datasource::file_format::DefaultFileType").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/fn.format_as_file_type.html" class="fn" title="fn datafusion::datasource::file_format::format_as_file_type">format_as_file_type</a>  
Converts a [FileFormatFactory](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory") to a [FileType](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html "trait datafusion::common::file_options::file_type::FileType")
