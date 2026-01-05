# Struct TableMetadata Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/table_metadata.rs.html#143-213" class="src">Source</a>

``` rust
pub struct TableMetadata { /* private fields */ }
```

Expand description

Fields for the version 2 of the table metadata.

We assume that this data structure is always valid, so we will panic when invalid error happens. We check the validity of this data structure when constructing.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-TableMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.into_builder" class="fn">into_builder</a>( self, current_file_location: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>

Convert this Table Metadata into a builder for modification.

`current_file_location` is the location where the current version of the metadata file is stored. This is used to update the metadata log. If `current_file_location` is `None`, the metadata log will not be updated. This should only be used to stage-create tables.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.format_version" class="fn">format_version</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion">FormatVersion</a>

Returns format version of this metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.uuid" class="fn">uuid</a>(&self) -\> <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>

Returns uuid of current table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.location" class="fn">location</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns table location.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.last_sequence_number" class="fn">last_sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Returns last sequence number.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.next_sequence_number" class="fn">next_sequence_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Returns the next sequence number for the table.

For format version 1, it always returns the initial sequence number. For other versions, it returns the last sequence number incremented by 1.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.last_column_id" class="fn">last_column_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the last column id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.last_partition_id" class="fn">last_partition_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns the last partition_id

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.last_updated_timestamp" class="fn">last_updated_timestamp</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>\>

Returns last updated time.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.last_updated_ms" class="fn">last_updated_ms</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Returns last updated time in milliseconds.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.schemas_iter" class="fn">schemas_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Returns schemas

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.schema_by_id" class="fn">schema_by_id</a>(&self, schema_id: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>

Lookup schema by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.current_schema" class="fn">current_schema</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>

Get current schema

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.current_schema_id" class="fn">current_schema_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>

Get the id of the current schema

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.partition_specs_iter" class="fn">partition_specs_iter</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>\>

Returns all partition specs.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.partition_spec_by_id" class="fn">partition_spec_by_id</a>(&self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>\>

Lookup partition spec by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.default_partition_spec" class="fn">default_partition_spec</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>

Get default partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.default_partition_type" class="fn">default_partition_type</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>

Return the partition type of the default partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.default_partition_spec_id" class="fn">default_partition_spec_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns spec id of the “current” partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.snapshots" class="fn">snapshots</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>\>

Returns all snapshots

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.snapshot_by_id" class="fn">snapshot_by_id</a>(&self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>\>

Lookup snapshot by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.history" class="fn">history</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotLog.html" class="struct" title="struct iceberg::spec::SnapshotLog">SnapshotLog</a>\]

Returns snapshot history.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.metadata_log" class="fn">metadata_log</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MetadataLog.html" class="struct" title="struct iceberg::spec::MetadataLog">MetadataLog</a>\]

Returns the metadata log.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.current_snapshot" class="fn">current_snapshot</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>\>

Get current snapshot

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.current_snapshot_id" class="fn">current_snapshot_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Get the current snapshot id

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.snapshot_for_ref" class="fn">snapshot_for_ref</a>(&self, ref_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>\>

Get the snapshot for a reference Returns an option if the `ref_name` is not found

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.sort_orders_iter" class="fn">sort_orders_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SortOrderRef.html" class="type" title="type iceberg::spec::SortOrderRef">SortOrderRef</a>\>

Return all sort orders.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.sort_order_by_id" class="fn">sort_order_by_id</a>(&self, sort_order_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SortOrderRef.html" class="type" title="type iceberg::spec::SortOrderRef">SortOrderRef</a>\>

Lookup sort order by id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.default_sort_order" class="fn">default_sort_order</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SortOrderRef.html" class="type" title="type iceberg::spec::SortOrderRef">SortOrderRef</a>

Returns default sort order id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.default_sort_order_id" class="fn">default_sort_order_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Returns default sort order id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns properties of table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.statistics_iter" class="fn">statistics_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>\>

Return location of statistics files.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.partition_statistics_iter" class="fn">partition_statistics_iter</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionStatisticsFile.html" class="struct" title="struct iceberg::spec::PartitionStatisticsFile">PartitionStatisticsFile</a>\>

Return location of partition statistics files.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.statistics_for_snapshot" class="fn">statistics_for_snapshot</a>( &self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>\>

Get a statistics file for a snapshot id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.partition_statistics_for_snapshot" class="fn">partition_statistics_for_snapshot</a>( &self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionStatisticsFile.html" class="struct" title="struct iceberg::spec::PartitionStatisticsFile">PartitionStatisticsFile</a>\>

Get a partition statistics file for a snapshot id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.encryption_keys_iter" class="fn">encryption_keys_iter</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = (&<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>

Iterate over all encryption keys

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.encryption_key" class="fn">encryption_key</a>(&self, key_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get the encryption key for a given key id

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.read_from" class="fn">read_from</a>( file_io: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, metadata_location: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>\>

Read table metadata from the given location.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.write_to" class="fn">write_to</a>( &self, file_io: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, metadata_location: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write table metadata to the given location.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-Clone-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-Debug-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-Deserialize%3C&#39;de%3E-for-TableMetadata" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-From%3CTableMetadataBuildResult%3E-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(result: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-PartialEq-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-Serialize-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-Eq-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#impl-StructuralPartialEq-for-TableMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html#blanket-implementations" class="anchor">§</a>
