# Struct MetadataTable Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/inspect/metadata_table.rs.html#28" class="src">Source</a>

``` rust
pub struct MetadataTable<'a>(/* private fields */);
```

Expand description

Metadata table is used to inspect a table’s history, snapshots, and other metadata as a table.

References:

- <https://github.com/apache/iceberg/blob/ac865e334e143dfd9e33011d8cf710b46d91f1e5/core/src/main/java/org/apache/iceberg/MetadataTableType.java#L23-L39>
- <https://iceberg.apache.org/docs/latest/spark-queries/#querying-with-sql>
- <https://py.iceberg.apache.org/api/#inspecting-tables>

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#impl-MetadataTable%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html" class="struct" title="struct iceberg::inspect::MetadataTable">MetadataTable</a>\<'a\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#method.new" class="fn">new</a>(table: &'a <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>) -\> Self

Creates a new metadata scan.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#method.snapshots" class="fn">snapshots</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html" class="struct" title="struct iceberg::inspect::SnapshotsTable">SnapshotsTable</a>\<'\_\>

Get the snapshots table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#method.manifests" class="fn">manifests</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html" class="struct" title="struct iceberg::inspect::ManifestsTable">ManifestsTable</a>\<'\_\>

Get the manifests table.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#impl-Debug-for-MetadataTable%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html" class="struct" title="struct iceberg::inspect::MetadataTable">MetadataTable</a>\<'a\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html#blanket-implementations" class="anchor">§</a>
