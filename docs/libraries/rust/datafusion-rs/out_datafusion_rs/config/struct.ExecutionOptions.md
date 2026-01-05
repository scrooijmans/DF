# Struct ExecutionOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#349-499" class="src">Source</a>

``` rust
pub struct ExecutionOptions {Show 24 fields
    pub batch_size: usize,
    pub coalesce_batches: bool,
    pub collect_statistics: bool,
    pub target_partitions: usize,
    pub time_zone: String,
    pub parquet: ParquetOptions,
    pub planning_concurrency: usize,
    pub skip_physical_aggregate_schema_check: bool,
    pub spill_compression: SpillCompression,
    pub sort_spill_reservation_bytes: usize,
    pub sort_in_place_threshold_bytes: usize,
    pub meta_fetch_concurrency: usize,
    pub minimum_parallel_output_files: usize,
    pub soft_max_rows_per_output_file: usize,
    pub max_buffered_batches_per_output_file: usize,
    pub listing_table_ignore_subdirectory: bool,
    pub enable_recursive_ctes: bool,
    pub split_file_groups_by_statistics: bool,
    pub keep_partition_by_columns: bool,
    pub skip_partial_aggregation_probe_ratio_threshold: f64,
    pub skip_partial_aggregation_probe_rows_threshold: usize,
    pub use_row_number_estimates_to_optimize_partitioning: bool,
    pub enforce_batch_size_in_joins: bool,
    pub objectstore_writer_buffer_size: usize,
}
```

Expand description

Options related to query execution

See also: [`SessionConfig`](https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.batch_size" class="anchor field">§</a>`batch_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Default batch size while creating new batches, it’s especially useful for buffer-in-memory batches since creating tiny batches would result in too much metadata memory consumption

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.coalesce_batches" class="anchor field">§</a>`coalesce_batches: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, record batches will be examined between each operator and small batches will be coalesced into larger batches. This is helpful when there are highly selective filters or joins that could produce tiny output batches. The target batch size is determined by the configuration setting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.collect_statistics" class="anchor field">§</a>`collect_statistics: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion collect statistics when first creating a table. Has no effect after the table is created. Applies to the default `ListingTableProvider` in DataFusion. Defaults to true.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.target_partitions" class="anchor field">§</a>`target_partitions: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Number of partitions for query execution. Increasing partitions can increase concurrency.

Defaults to the number of CPU cores on the system

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The default time zone

Some functions, e.g. `EXTRACT(HOUR from SOME_TIME)`, shift the underlying datetime according to this time zone, and then extract the hour

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.parquet" class="anchor field">§</a>`parquet: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions"><code>ParquetOptions</code></a>

Parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.planning_concurrency" class="anchor field">§</a>`planning_concurrency: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Fan-out during initial physical planning.

This is mostly use to plan `UNION` children in parallel.

Defaults to the number of CPU cores on the system

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.skip_physical_aggregate_schema_check" class="anchor field">§</a>`skip_physical_aggregate_schema_check: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, skips verifying that the schema produced by planning the input of `LogicalPlan::Aggregate` exactly matches the schema of the input plan.

When set to false, if the schema does not match exactly (including nullability and metadata), a planning error will be raised.

This is used to workaround bugs in the planner that are now caught by the new schema verification step.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.spill_compression" class="anchor field">§</a>`spill_compression: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.SpillCompression.html" class="enum" title="enum datafusion::config::SpillCompression"><code>SpillCompression</code></a>

Sets the compression codec used when spilling data to disk.

Since datafusion writes spill files using the Arrow IPC Stream format, only codecs supported by the Arrow IPC Stream Writer are allowed. Valid values are: uncompressed, lz4_frame, zstd. Note: lz4_frame offers faster (de)compression, but typically results in larger spill files. In contrast, zstd achieves higher compression ratios at the cost of slower (de)compression speed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.sort_spill_reservation_bytes" class="anchor field">§</a>`sort_spill_reservation_bytes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Specifies the reserved memory for each spillable sort operation to facilitate an in-memory merge.

When a sort operation spills to disk, the in-memory data must be sorted and merged before being written to a file. This setting reserves a specific amount of memory for that in-memory sort/merge process.

Note: This setting is irrelevant if the sort operation cannot spill (i.e., if there’s no `DiskManager` configured).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.sort_in_place_threshold_bytes" class="anchor field">§</a>`sort_in_place_threshold_bytes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

When sorting, below what size should data be concatenated and sorted in a single RecordBatch rather than sorted in batches and merged.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.meta_fetch_concurrency" class="anchor field">§</a>`meta_fetch_concurrency: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Number of files to read in parallel when inferring schema and statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.minimum_parallel_output_files" class="anchor field">§</a>`minimum_parallel_output_files: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Guarantees a minimum level of output files running in parallel. RecordBatches will be distributed in round robin fashion to each parallel writer. Each writer is closed and a new file opened once soft_max_rows_per_output_file is reached.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.soft_max_rows_per_output_file" class="anchor field">§</a>`soft_max_rows_per_output_file: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Target number of rows in output files when writing multiple. This is a soft max, so it can be exceeded slightly. There also will be one file smaller than the limit if the total number of rows written is not roughly divisible by the soft max

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.max_buffered_batches_per_output_file" class="anchor field">§</a>`max_buffered_batches_per_output_file: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

This is the maximum number of RecordBatches buffered for each output file being worked. Higher values can potentially give faster write performance at the cost of higher peak memory consumption

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.listing_table_ignore_subdirectory" class="anchor field">§</a>`listing_table_ignore_subdirectory: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should sub directories be ignored when scanning directories for data files. Defaults to true (ignores subdirectories), consistent with Hive. Note that this setting does not affect reading partitioned tables (e.g. `/table/year=2021/month=01/data.parquet`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.enable_recursive_ctes" class="anchor field">§</a>`enable_recursive_ctes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion support recursive CTEs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.split_file_groups_by_statistics" class="anchor field">§</a>`split_file_groups_by_statistics: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Attempt to eliminate sorts by packing & sorting files with non-overlapping statistics into the same file groups. Currently experimental

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.keep_partition_by_columns" class="anchor field">§</a>`keep_partition_by_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion keep the columns used for partition_by in the output RecordBatches

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.skip_partial_aggregation_probe_ratio_threshold" class="anchor field">§</a>`skip_partial_aggregation_probe_ratio_threshold: `<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>

Aggregation ratio (number of distinct groups / number of input rows) threshold for skipping partial aggregation. If the value is greater then partial aggregation will skip aggregation for further input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.skip_partial_aggregation_probe_rows_threshold" class="anchor field">§</a>`skip_partial_aggregation_probe_rows_threshold: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Number of input rows partial aggregation partition should process, before aggregation ratio check and trying to switch to skipping aggregation mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.use_row_number_estimates_to_optimize_partitioning" class="anchor field">§</a>`use_row_number_estimates_to_optimize_partitioning: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion use row number estimates at the input to decide whether increasing parallelism is beneficial or not. By default, only exact row numbers (not estimates) are used for this decision. Setting this flag to `true` will likely produce better plans. if the source of statistics is accurate. We plan to make this the default in the future.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.enforce_batch_size_in_joins" class="anchor field">§</a>`enforce_batch_size_in_joins: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion enforce batch size in joins or not. By default, DataFusion will not enforce batch size in joins. Enforcing batch size in joins can reduce memory usage when joining large tables with a highly-selective join filter, but is also slightly slower.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.objectstore_writer_buffer_size" class="anchor field">§</a>`objectstore_writer_buffer_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Size (bytes) of data buffer DataFusion uses when writing output files. This affects the size of the data chunks that are uploaded to remote object stores (e.g. AWS S3). If very large (\>= 100 GiB) output files are being written, it may be necessary to increase this size to avoid errors from the remote end point.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-Clone-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-ConfigField-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-Debug-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-Default-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-PartialEq-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#impl-StructuralPartialEq-for-ExecutionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#blanket-implementations" class="anchor">§</a>
