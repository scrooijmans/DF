# Enum TableUpdate Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#430-557" class="src">Source</a>

``` rust
pub enum TableUpdate {
Show 21 variants    UpgradeFormatVersion {
        format_version: FormatVersion,
    },
    AssignUuid {
        uuid: Uuid,
    },
    AddSchema {
        schema: Schema,
    },
    SetCurrentSchema {
        schema_id: i32,
    },
    AddSpec {
        spec: UnboundPartitionSpec,
    },
    SetDefaultSpec {
        spec_id: i32,
    },
    AddSortOrder {
        sort_order: SortOrder,
    },
    SetDefaultSortOrder {
        sort_order_id: i64,
    },
    AddSnapshot {
        snapshot: Snapshot,
    },
    SetSnapshotRef {
        ref_name: String,
        reference: SnapshotReference,
    },
    RemoveSnapshots {
        snapshot_ids: Vec<i64>,
    },
    RemoveSnapshotRef {
        ref_name: String,
    },
    SetLocation {
        location: String,
    },
    SetProperties {
        updates: HashMap<String, String>,
    },
    RemoveProperties {
        removals: Vec<String>,
    },
    RemovePartitionSpecs {
        spec_ids: Vec<i32>,
    },
    SetStatistics {
        statistics: StatisticsFile,
    },
    RemoveStatistics {
        snapshot_id: i64,
    },
    SetPartitionStatistics {
        partition_statistics: PartitionStatisticsFile,
    },
    RemovePartitionStatistics {
        snapshot_id: i64,
    },
    RemoveSchemas {
        schema_ids: Vec<i32>,
    },
}
```

Expand description

TableUpdate represents an update to a table in the catalog.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.UpgradeFormatVersion" class="anchor">§</a>

### UpgradeFormatVersion

Upgrade table’s format version

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.UpgradeFormatVersion.field.format_version" class="anchor field">§</a>`format_version: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion"><code>FormatVersion</code></a>

Target format upgrade to.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AssignUuid" class="anchor">§</a>

### AssignUuid

Assign a new UUID to the table

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AssignUuid.field.uuid" class="anchor field">§</a>`uuid: `<a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid"><code>Uuid</code></a>

The new UUID to assign.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSchema" class="anchor">§</a>

### AddSchema

Add a new schema to the table

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSchema.field.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema"><code>Schema</code></a>

The schema to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetCurrentSchema" class="anchor">§</a>

### SetCurrentSchema

Set table’s current schema

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetCurrentSchema.field.schema_id" class="anchor field">§</a>`schema_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Schema ID to set as current, or -1 to set last added schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSpec" class="anchor">§</a>

### AddSpec

Add a new partition spec to the table

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSpec.field.spec" class="anchor field">§</a>`spec: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec"><code>UnboundPartitionSpec</code></a>

The partition spec to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetDefaultSpec" class="anchor">§</a>

### SetDefaultSpec

Set table’s default spec

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetDefaultSpec.field.spec_id" class="anchor field">§</a>`spec_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Partition spec ID to set as the default, or -1 to set last added spec

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSortOrder" class="anchor">§</a>

### AddSortOrder

Add sort order to table.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSortOrder.field.sort_order" class="anchor field">§</a>`sort_order: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder"><code>SortOrder</code></a>

Sort order to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetDefaultSortOrder" class="anchor">§</a>

### SetDefaultSortOrder

Set table’s default sort order

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetDefaultSortOrder.field.sort_order_id" class="anchor field">§</a>`sort_order_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Sort order ID to set as the default, or -1 to set last added sort order

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSnapshot" class="anchor">§</a>

### AddSnapshot

Add snapshot to table.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.AddSnapshot.field.snapshot" class="anchor field">§</a>`snapshot: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot"><code>Snapshot</code></a>

Snapshot to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetSnapshotRef" class="anchor">§</a>

### SetSnapshotRef

Set table’s snapshot ref.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetSnapshotRef.field.ref_name" class="anchor field">§</a>`ref_name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Name of snapshot reference to set.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetSnapshotRef.field.reference" class="anchor field">§</a>`reference: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotReference.html" class="struct" title="struct iceberg::spec::SnapshotReference"><code>SnapshotReference</code></a>

Snapshot reference to set.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSnapshots" class="anchor">§</a>

### RemoveSnapshots

Remove table’s snapshots

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSnapshots.field.snapshot_ids" class="anchor field">§</a>`snapshot_ids: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`>`

Snapshot ids to remove.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSnapshotRef" class="anchor">§</a>

### RemoveSnapshotRef

Remove snapshot reference

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSnapshotRef.field.ref_name" class="anchor field">§</a>`ref_name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Name of snapshot reference to remove.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetLocation" class="anchor">§</a>

### SetLocation

Update table’s location

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetLocation.field.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

New location for table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetProperties" class="anchor">§</a>

### SetProperties

Update table’s properties

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetProperties.field.updates" class="anchor field">§</a>`updates: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Properties to update for table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveProperties" class="anchor">§</a>

### RemoveProperties

Remove table’s properties

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveProperties.field.removals" class="anchor field">§</a>`removals: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Properties to remove

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemovePartitionSpecs" class="anchor">§</a>

### RemovePartitionSpecs

Remove partition specs

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemovePartitionSpecs.field.spec_ids" class="anchor field">§</a>`spec_ids: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

Partition spec ids to remove.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetStatistics" class="anchor">§</a>

### SetStatistics

Set statistics for a snapshot

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetStatistics.field.statistics" class="anchor field">§</a>`statistics: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile"><code>StatisticsFile</code></a>

File containing the statistics

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveStatistics" class="anchor">§</a>

### RemoveStatistics

Remove statistics for a snapshot

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveStatistics.field.snapshot_id" class="anchor field">§</a>`snapshot_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Snapshot id to remove statistics for.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetPartitionStatistics" class="anchor">§</a>

### SetPartitionStatistics

Set partition statistics for a snapshot

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.SetPartitionStatistics.field.partition_statistics" class="anchor field">§</a>`partition_statistics: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionStatisticsFile.html" class="struct" title="struct iceberg::spec::PartitionStatisticsFile"><code>PartitionStatisticsFile</code></a>

File containing the partition statistics

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemovePartitionStatistics" class="anchor">§</a>

### RemovePartitionStatistics

Remove partition statistics for a snapshot

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemovePartitionStatistics.field.snapshot_id" class="anchor field">§</a>`snapshot_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Snapshot id to remove partition statistics for.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSchemas" class="anchor">§</a>

### RemoveSchemas

Remove schemas

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#variant.RemoveSchemas.field.schema_ids" class="anchor field">§</a>`schema_ids: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

Schema IDs to remove.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-TableUpdate" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.apply" class="fn">apply</a>( self, builder: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>\>

Applies the update to the table metadata builder.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-Clone-for-TableUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-Debug-for-TableUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-Deserialize%3C&#39;de%3E-for-TableUpdate" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-PartialEq-for-TableUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-Serialize-for-TableUpdate" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#impl-StructuralPartialEq-for-TableUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html#blanket-implementations" class="anchor">§</a>
