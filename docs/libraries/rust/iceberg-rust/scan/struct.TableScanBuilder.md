# Struct TableScanBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/scan/mod.rs.html#49-62" class="src">Source</a>

``` rust
pub struct TableScanBuilder<'a> { /* private fields */ }
```

Expand description

Builder to create table scan.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#impl-TableScanBuilder%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html" class="struct" title="struct iceberg::scan::TableScanBuilder">TableScanBuilder</a>\<'a\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> Self

Sets the desired size of batches in the response to something other than the default

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_case_sensitive" class="fn">with_case_sensitive</a>(self, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Sets the scan’s case sensitivity

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_filter" class="fn">with_filter</a>(self, predicate: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>) -\> Self

Specifies a predicate to use as a filter

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.select_all" class="fn">select_all</a>(self) -\> Self

Select all columns.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.select_empty" class="fn">select_empty</a>(self) -\> Self

Select empty columns.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.select" class="fn">select</a>( self, column_names: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>\>, ) -\> Self

Select some columns of the table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.snapshot_id" class="fn">snapshot_id</a>(self, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Set the snapshot to scan. When not set, it uses current snapshot.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_concurrency_limit" class="fn">with_concurrency_limit</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Sets the concurrency limit for both manifest files and manifest entries for this scan

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_data_file_concurrency_limit" class="fn">with_data_file_concurrency_limit</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Sets the data file concurrency limit for this scan

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_manifest_entry_concurrency_limit" class="fn">with_manifest_entry_concurrency_limit</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Sets the manifest entry concurrency limit for this scan

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_row_group_filtering_enabled" class="fn">with_row_group_filtering_enabled</a>( self, row_group_filtering_enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

Determines whether to enable row group filtering. When enabled, if a read is performed with a filter predicate, then the metadata for each row group in the parquet file is evaluated against the filter predicate and row groups that cant contain matching rows will be skipped entirely.

Defaults to enabled, as it generally improves performance or keeps it the same, with performance degradation unlikely.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.with_row_selection_enabled" class="fn">with_row_selection_enabled</a>(self, row_selection_enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Determines whether to enable row selection. When enabled, if a read is performed with a filter predicate, then (for row groups that have not been skipped) the page index for each row group in a parquet file is parsed and evaluated against the filter predicate to determine if ranges of rows within a row group can be skipped, based upon the page-level statistics for each column.

Defaults to being disabled. Enabling requires parsing the parquet page index, which can be slow enough that parsing the page index outweighs any gains from the reduced number of rows that need scanning. It is recommended to experiment with partitioning, sorting, row group size, page size, and page row limit Iceberg settings on the table being scanned in order to get the best performance from using row selection.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScan.html" class="struct" title="struct iceberg::scan::TableScan">TableScan</a>\>

Build the table scan.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html#blanket-implementations" class="anchor">§</a>
