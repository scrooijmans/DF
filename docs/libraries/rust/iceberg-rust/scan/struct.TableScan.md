# Struct TableScan Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/scan/mod.rs.html#308-330" class="src">Source</a>

``` rust
pub struct TableScan { /* private fields */ }
```

Expand description

Table scan.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#impl-TableScan" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html" class="struct" title="struct iceberg::scan::TableScan">TableScan</a>

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#method.plan_files" class="fn">plan_files</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.FileScanTaskStream.html" class="type" title="type iceberg::scan::FileScanTaskStream">FileScanTaskStream</a>\>

Returns a stream of [`FileScanTask`](https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html "struct iceberg::scan::FileScanTask")s.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#method.to_arrow" class="fn">to_arrow</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html" class="type" title="type iceberg::scan::ArrowRecordBatchStream">ArrowRecordBatchStream</a>\>

Returns an [`ArrowRecordBatchStream`](https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html "type iceberg::scan::ArrowRecordBatchStream").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#method.column_names" class="fn">column_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>

Returns a reference to the column names of the table scan.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#method.snapshot" class="fn">snapshot</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>\>

Returns a reference to the snapshot of the table scan.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#impl-Debug-for-TableScan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html" class="struct" title="struct iceberg::scan::TableScan">TableScan</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html#blanket-implementations" class="anchor">§</a>
