# Struct SnapshotSummaryCollector Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/snapshot_summary.rs.html#55-61" class="src">Source</a>

``` rust
pub struct SnapshotSummaryCollector { /* private fields */ }
```

Expand description

`SnapshotSummaryCollector` collects and aggregates snapshot update metrics. It gathers metrics about added or removed data files and manifests, and tracks partition-specific updates.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#impl-SnapshotSummaryCollector" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html" class="struct" title="struct iceberg::spec::SnapshotSummaryCollector">SnapshotSummaryCollector</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Set properties for snapshot summary

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.set_partition_summary_limit" class="fn">set_partition_summary_limit</a>(&mut self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Sets the limit for including partition summaries. Summaries are not included if the number of partitions is exceeded.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.add_file" class="fn">add_file</a>( &mut self, data_file: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>, )

Adds a data file to the summary collector

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.remove_file" class="fn">remove_file</a>( &mut self, data_file: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>, )

Removes a data file from the summary collector

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.add_manifest" class="fn">add_manifest</a>(&mut self, manifest: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>)

Adds a manifest to the summary collector

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.update_partition_metrics" class="fn">update_partition_metrics</a>( &mut self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, partition_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>, data_file: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, is_add_file: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, )

Updates partition-specific metrics for a data file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.merge" class="fn">merge</a>(&mut self, summary: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html" class="struct" title="struct iceberg::spec::SnapshotSummaryCollector">SnapshotSummaryCollector</a>)

Merges another `SnapshotSummaryCollector` into the current one

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.build" class="fn">build</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Builds final map of summaries

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#impl-Default-for-SnapshotSummaryCollector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html" class="struct" title="struct iceberg::spec::SnapshotSummaryCollector">SnapshotSummaryCollector</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html" class="struct" title="struct iceberg::spec::SnapshotSummaryCollector">SnapshotSummaryCollector</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html#blanket-implementations" class="anchor">§</a>
