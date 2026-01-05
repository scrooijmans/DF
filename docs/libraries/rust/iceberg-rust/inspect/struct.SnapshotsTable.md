# Struct SnapshotsTable Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/inspect/snapshots.rs.html#37-39" class="src">Source</a>

``` rust
pub struct SnapshotsTable<'a> { /* private fields */ }
```

Expand description

Snapshots table.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#impl-SnapshotsTable%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html" class="struct" title="struct iceberg::inspect::SnapshotsTable">SnapshotsTable</a>\<'a\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#method.new" class="fn">new</a>(table: &'a <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>) -\> Self

Create a new Snapshots table instance.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

Returns the iceberg schema of the snapshots table.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#method.scan" class="fn">scan</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html" class="type" title="type iceberg::scan::ArrowRecordBatchStream">ArrowRecordBatchStream</a>\>

Scans the snapshots table.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.SnapshotsTable.html#blanket-implementations" class="anchor">§</a>
