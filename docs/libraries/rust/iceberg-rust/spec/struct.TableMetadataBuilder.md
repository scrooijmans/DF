# Struct TableMetadataBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/table_metadata_builder.rs.html#49-58" class="src">Source</a>

``` rust
pub struct TableMetadataBuilder { /* private fields */ }
```

Expand description

Manipulating table metadata.

For this builder the order of called functions matters. Functions are applied in-order. All operations applied to the `TableMetadata` are tracked in `changes` as a chronologically ordered vec of `TableUpdate`. If an operation does not lead to a change of the `TableMetadata`, the corresponding update is omitted from `changes`.

Unlike a typical builder pattern, the order of function calls matters. Some basic rules:

- `add_schema` must be called before `set_current_schema`.
- If a new partition spec and schema are added, the schema should be added first.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#impl-TableMetadataBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>

#### pub const <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#associatedconstant.LAST_ADDED" class="constant">LAST_ADDED</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> = -1i32

Proxy id for “last added” items, including schema, partition spec, sort order.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.new" class="fn">new</a>( schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>, spec: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>\>, sort_order: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>, location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, format_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion">FormatVersion</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create a `TableMetadata` object from scratch.

This method re-assign ids of fields in the schema, schema.id, sort_order.id and spec.id. It should only be used to create new table metadata from scratch.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.new_from_metadata" class="fn">new_from_metadata</a>( previous: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>, current_file_location: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Creates a new table metadata builder from the given metadata to modify it. `current_file_location` is the location where the current version of the metadata file is stored. This is used to update the metadata log. If `current_file_location` is `None`, the metadata log will not be updated. This should only be used to stage-create tables.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.from_table_creation" class="fn">from_table_creation</a>(table_creation: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html" class="struct" title="struct iceberg::TableCreation">TableCreation</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a new table metadata builder from the given table creation.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.assign_uuid" class="fn">assign_uuid</a>(self, uuid: <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>) -\> Self

Changes uuid of table metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.upgrade_format_version" class="fn">upgrade_format_version</a>( self, format_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion">FormatVersion</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Upgrade `FormatVersion`. Downgrades are not allowed.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors" class="doc-anchor">§</a>Errors

- Cannot downgrade to older format versions.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_properties" class="fn">set_properties</a>(self, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set properties. If a property already exists, it will be overwritten.

If a reserved property is set, the corresponding action is performed and the property is not persisted. Currently the following reserved properties are supported:

- format-version: Set the format version of the table.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-1" class="doc-anchor">§</a>Errors

- If properties contains a reserved property

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_properties" class="fn">remove_properties</a>(self, properties: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Remove properties from the table metadata. Does nothing if the key is not present.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-2" class="doc-anchor">§</a>Errors

- If properties to remove contains a reserved property

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_location" class="fn">set_location</a>(self, location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the location of the table, stripping any trailing slashes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_snapshot" class="fn">add_snapshot</a>(self, snapshot: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a snapshot to the table metadata.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-3" class="doc-anchor">§</a>Errors

- Snapshot id already exists.
- For format version \> 1: the sequence number of the snapshot is lower than the highest sequence number specified so far.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_branch_snapshot" class="fn">set_branch_snapshot</a>( self, snapshot: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>, branch: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Append a snapshot to the specified branch. Retention settings from the `branch` are re-used.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-4" class="doc-anchor">§</a>Errors

- Any of the preconditions of `self.add_snapshot` are not met.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_snapshots" class="fn">remove_snapshots</a>(self, snapshot_ids: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]) -\> Self

Remove snapshots by its ids from the table metadata. Does nothing if a snapshot id is not present. Keeps as changes only the snapshots that were actually removed.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_ref" class="fn">set_ref</a>( self, ref_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, reference: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotReference.html" class="struct" title="struct iceberg::spec::SnapshotReference">SnapshotReference</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set a reference to a snapshot.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-5" class="doc-anchor">§</a>Errors

- The snapshot id is unknown.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_ref" class="fn">remove_ref</a>(self, ref_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Remove a reference

If `ref_name='main'` the current snapshot id is set to -1.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_statistics" class="fn">set_statistics</a>(self, statistics: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>) -\> Self

Set statistics for a snapshot

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_statistics" class="fn">remove_statistics</a>(self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Remove statistics for a snapshot

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_partition_statistics" class="fn">set_partition_statistics</a>( self, partition_statistics_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionStatisticsFile.html" class="struct" title="struct iceberg::spec::PartitionStatisticsFile">PartitionStatisticsFile</a>, ) -\> Self

Set partition statistics

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_partition_statistics" class="fn">remove_partition_statistics</a>(self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Remove partition statistics

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_schema" class="fn">add_schema</a>(self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a schema to the table metadata.

The provided `schema.schema_id` may not be used.

Important: Use this method with caution. The builder does not check if the added schema is compatible with the current schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_current_schema" class="fn">set_current_schema</a>(self, schema_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set the current schema id.

If `schema_id` is -1, the last added schema is set as the current schema.

Errors:

- provided `schema_id` is -1 but no schema has been added via `add_schema`.
- No schema with the provided `schema_id` exists.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_current_schema" class="fn">add_current_schema</a>(self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a schema and set it as the current schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_partition_spec" class="fn">add_partition_spec</a>( self, unbound_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a partition spec to the table metadata.

The spec is bound eagerly to the current schema. If a schema is added in the same set of changes, the schema should be added first.

Even if `unbound_spec.spec_id` is provided as `Some`, it may not be used.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-6" class="doc-anchor">§</a>Errors

- The partition spec cannot be bound to the current schema.
- The partition spec has non-sequential field ids and the table format version is 1.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_default_partition_spec" class="fn">set_default_partition_spec</a>(self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set the default partition spec.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-7" class="doc-anchor">§</a>Errors

- spec_id is -1 but no spec has been added via `add_partition_spec`.
- No partition spec with the provided `spec_id` exists.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_default_partition_spec" class="fn">add_default_partition_spec</a>( self, unbound_spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a partition spec and set it as the default

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_partition_specs" class="fn">remove_partition_specs</a>(self, spec_ids: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Remove partition specs by their ids from the table metadata. Does nothing if a spec id is not present. Active partition specs should not be removed.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-8" class="doc-anchor">§</a>Errors

- Cannot remove the default partition spec.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.add_sort_order" class="fn">add_sort_order</a>(self, sort_order: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a sort order to the table metadata.

The spec is bound eagerly to the current schema and must be valid for it. If a schema is added in the same set of changes, the schema should be added first.

Even if `sort_order.order_id` is provided, it may not be used.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-9" class="doc-anchor">§</a>Errors

- Sort order id to add already exists.
- Sort order is incompatible with the current schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.set_default_sort_order" class="fn">set_default_sort_order</a>(self, sort_order_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set the default sort order. If `sort_order_id` is -1, the last added sort order is set as default.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#errors-10" class="doc-anchor">§</a>Errors

- sort_order_id is -1 but no sort order has been added via `add_sort_order`.
- No sort order with the provided `sort_order_id` exists.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>\>

Build the table metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.remove_schemas" class="fn">remove_schemas</a>(self, schema_id_to_remove: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Remove schemas by their ids from the table metadata. Does nothing if a schema id is not present. Active schemas should not be removed.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#impl-Clone-for-TableMetadataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#impl-Debug-for-TableMetadataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html#blanket-implementations" class="anchor">§</a>
