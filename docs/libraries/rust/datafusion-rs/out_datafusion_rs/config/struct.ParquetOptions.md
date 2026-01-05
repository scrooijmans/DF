# Struct ParquetOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#501-670" class="src">Source</a>

``` rust
pub struct ParquetOptions {Show 30 fields
    pub enable_page_index: bool,
    pub pruning: bool,
    pub skip_metadata: bool,
    pub metadata_size_hint: Option<usize>,
    pub pushdown_filters: bool,
    pub reorder_filters: bool,
    pub schema_force_view_types: bool,
    pub binary_as_string: bool,
    pub coerce_int96: Option<String>,
    pub bloom_filter_on_read: bool,
    pub data_pagesize_limit: usize,
    pub write_batch_size: usize,
    pub writer_version: String,
    pub skip_arrow_metadata: bool,
    pub compression: Option<String>,
    pub dictionary_enabled: Option<bool>,
    pub dictionary_page_size_limit: usize,
    pub statistics_enabled: Option<String>,
    pub max_row_group_size: usize,
    pub created_by: String,
    pub column_index_truncate_length: Option<usize>,
    pub statistics_truncate_length: Option<usize>,
    pub data_page_row_count_limit: usize,
    pub encoding: Option<String>,
    pub bloom_filter_on_write: bool,
    pub bloom_filter_fpp: Option<f64>,
    pub bloom_filter_ndv: Option<u64>,
    pub allow_single_file_parallelism: bool,
    pub maximum_parallel_row_group_writers: usize,
    pub maximum_buffered_record_batches_per_stream: usize,
}
```

Expand description

Options for reading and writing parquet files

See also: [`SessionConfig`](https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.enable_page_index" class="anchor field">§</a>`enable_page_index: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, reads the Parquet data page level metadata (the Page Index), if present, to reduce the I/O and number of rows decoded.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.pruning" class="anchor field">§</a>`pruning: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, the parquet reader attempts to skip entire row groups based on the predicate in the query and the metadata (min/max values) stored in the parquet file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.skip_metadata" class="anchor field">§</a>`skip_metadata: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, the parquet reader skip the optional embedded metadata that may be in the file Schema. This setting can help avoid schema conflicts when querying multiple parquet files with schemas containing compatible types but different metadata

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.metadata_size_hint" class="anchor field">§</a>`metadata_size_hint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

(reading) If specified, the parquet reader will try and fetch the last `size_hint` bytes of the parquet file optimistically. If not specified, two reads are required: One read to fetch the 8-byte parquet footer and another to fetch the metadata length encoded in the footer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.pushdown_filters" class="anchor field">§</a>`pushdown_filters: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, filter expressions are be applied during the parquet decoding operation to reduce the number of rows decoded. This optimization is sometimes called “late materialization”.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.reorder_filters" class="anchor field">§</a>`reorder_filters: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, filter expressions evaluated during the parquet decoding operation will be reordered heuristically to minimize the cost of evaluation. If false, the filters are applied in the same order as written in the query

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.schema_force_view_types" class="anchor field">§</a>`schema_force_view_types: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, parquet reader will read columns of `Utf8/Utf8Large` with `Utf8View`, and `Binary/BinaryLarge` with `BinaryView`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.binary_as_string" class="anchor field">§</a>`binary_as_string: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) If true, parquet reader will read columns of `Binary/LargeBinary` with `Utf8`, and `BinaryView` with `Utf8View`.

Parquet files generated by some legacy writers do not correctly set the UTF8 flag for strings, causing string columns to be loaded as BLOB instead.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.coerce_int96" class="anchor field">§</a>`coerce_int96: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

(reading) If true, parquet reader will read columns of physical type int96 as originating from a different resolution than nanosecond. This is useful for reading data from systems like Spark which stores microsecond resolution timestamps in an int96 allowing it to write values with a larger date range than 64-bit timestamps with nanosecond resolution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.bloom_filter_on_read" class="anchor field">§</a>`bloom_filter_on_read: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(reading) Use any available bloom filters when reading parquet files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.data_pagesize_limit" class="anchor field">§</a>`data_pagesize_limit: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) Sets best effort maximum size of data page in bytes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.write_batch_size" class="anchor field">§</a>`write_batch_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) Sets write_batch_size in bytes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.writer_version" class="anchor field">§</a>`writer_version: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

(writing) Sets parquet writer version valid values are “1.0” and “2.0”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.skip_arrow_metadata" class="anchor field">§</a>`skip_arrow_metadata: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(writing) Skip encoding the embedded arrow metadata in the KV_meta

This is analogous to the `ArrowWriterOptions::with_skip_arrow_metadata`. Refer to <https://docs.rs/parquet/53.3.0/parquet/arrow/arrow_writer/struct.ArrowWriterOptions.html#method.with_skip_arrow_metadata>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.compression" class="anchor field">§</a>`compression: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

(writing) Sets default parquet compression codec. Valid values are: uncompressed, snappy, gzip(level), lzo, brotli(level), lz4, zstd(level), and lz4_raw. These values are not case sensitive. If NULL, uses default parquet writer setting

Note that this default setting is not the same as the default parquet writer setting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.dictionary_enabled" class="anchor field">§</a>`dictionary_enabled: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

(writing) Sets if dictionary encoding is enabled. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.dictionary_page_size_limit" class="anchor field">§</a>`dictionary_page_size_limit: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) Sets best effort maximum dictionary page size, in bytes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.statistics_enabled" class="anchor field">§</a>`statistics_enabled: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

(writing) Sets if statistics are enabled for any column Valid values are: “none”, “chunk”, and “page” These values are not case sensitive. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.max_row_group_size" class="anchor field">§</a>`max_row_group_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) Target maximum number of rows in each row group (defaults to 1M rows). Writing larger row groups requires more memory to write, but can get better compression and be faster to read.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.created_by" class="anchor field">§</a>`created_by: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

(writing) Sets “created by” property

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.column_index_truncate_length" class="anchor field">§</a>`column_index_truncate_length: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

(writing) Sets column index truncate length

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.statistics_truncate_length" class="anchor field">§</a>`statistics_truncate_length: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

(writing) Sets statistics truncate length. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.data_page_row_count_limit" class="anchor field">§</a>`data_page_row_count_limit: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) Sets best effort maximum number of rows in data page

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.encoding" class="anchor field">§</a>`encoding: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

(writing) Sets default encoding for any column. Valid values are: plain, plain_dictionary, rle, bit_packed, delta_binary_packed, delta_length_byte_array, delta_byte_array, rle_dictionary, and byte_stream_split. These values are not case sensitive. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.bloom_filter_on_write" class="anchor field">§</a>`bloom_filter_on_write: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(writing) Write bloom filters for all columns when creating parquet files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.bloom_filter_fpp" class="anchor field">§</a>`bloom_filter_fpp: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>`>`

(writing) Sets bloom filter false positive probability. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.bloom_filter_ndv" class="anchor field">§</a>`bloom_filter_ndv: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

(writing) Sets bloom filter number of distinct values. If NULL, uses default parquet writer setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.allow_single_file_parallelism" class="anchor field">§</a>`allow_single_file_parallelism: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

(writing) Controls whether DataFusion will attempt to speed up writing parquet files by serializing them in parallel. Each column in each row group in each output file are serialized in parallel leveraging a maximum possible core count of n_files*n_row_groups*n_columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.maximum_parallel_row_group_writers" class="anchor field">§</a>`maximum_parallel_row_group_writers: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) By default parallel parquet writer is tuned for minimum memory usage in a streaming execution plan. You may see a performance benefit when writing large parquet files by increasing maximum_parallel_row_group_writers and maximum_buffered_record_batches_per_stream if your system has idle cores and can tolerate additional memory usage. Boosting these values is likely worthwhile when writing out already in-memory data, such as from a cached data frame.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.maximum_buffered_record_batches_per_stream" class="anchor field">§</a>`maximum_buffered_record_batches_per_stream: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

(writing) By default parallel parquet writer is tuned for minimum memory usage in a streaming execution plan. You may see a performance benefit when writing large parquet files by increasing maximum_parallel_row_group_writers and maximum_buffered_record_batches_per_stream if your system has idle cores and can tolerate additional memory usage. Boosting these values is likely worthwhile when writing out already in-memory data, such as from a cached data frame.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-ParquetOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.into_writer_properties_builder" class="fn">into_writer_properties_builder</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterPropertiesBuilder.html" class="struct" title="struct parquet::file::properties::WriterPropertiesBuilder">WriterPropertiesBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert the global session options, [`ParquetOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions"), into a single write action’s [`WriterPropertiesBuilder`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterPropertiesBuilder.html "struct parquet::file::properties::WriterPropertiesBuilder").

The returned [`WriterPropertiesBuilder`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterPropertiesBuilder.html "struct parquet::file::properties::WriterPropertiesBuilder") can then be further modified with additional options applied per column; a customization which is not applicable for [`ParquetOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions").

Note that this method does not include the key_value_metadata from [`TableParquetOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html "struct datafusion::config::TableParquetOptions").

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-Clone-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-ConfigField-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-Debug-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-Default-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-PartialEq-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#impl-StructuralPartialEq-for-ParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#blanket-implementations" class="anchor">§</a>
