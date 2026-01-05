# Struct ManifestsTable Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/inspect/manifests.rs.html#36-38" class="src">Source</a>

``` rust
pub struct ManifestsTable<'a> { /* private fields */ }
```

Expand description

Manifests table.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#impl-ManifestsTable%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html" class="struct" title="struct iceberg::inspect::ManifestsTable">ManifestsTable</a>\<'a\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#method.new" class="fn">new</a>(table: &'a <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>) -\> Self

Create a new Manifests table instance.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

Returns the iceberg schema of the manifests table.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#method.scan" class="fn">scan</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html" class="type" title="type iceberg::scan::ArrowRecordBatchStream">ArrowRecordBatchStream</a>\>

Scans the manifests table.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.ManifestsTable.html#blanket-implementations" class="anchor">§</a>
