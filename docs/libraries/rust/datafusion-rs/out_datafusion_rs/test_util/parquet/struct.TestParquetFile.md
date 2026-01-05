# Struct TestParquetFile Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/parquet.rs.html#49-54" class="src">Source</a>

``` rust
pub struct TestParquetFile { /* private fields */ }
```

Available on **crate feature `parquet`** only.

Expand description

a ParquetFile that has been created for testing.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#impl-TestParquetFile" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html" class="struct" title="struct datafusion::test_util::parquet::TestParquetFile">TestParquetFile</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#method.try_new" class="fn">try_new</a>( path: <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>, props: <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterProperties.html" class="struct" title="struct parquet::file::properties::WriterProperties">WriterProperties</a>, batches: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Creates a new parquet file at the specified location with the given properties

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#impl-TestParquetFile-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html" class="struct" title="struct datafusion::test_util::parquet::TestParquetFile">TestParquetFile</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#method.create_scan" class="fn">create_scan</a>( &self, ctx: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>, maybe_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Return a `DataSourceExec` with the specified options.

If `maybe_filter` is non-None, the DataSourceExec will be filtered using the given expression, and this method will return the same plan that DataFusion will make with a pushed down predicate followed by a filter:

``` text
(FilterExec)
  (DataSourceExec)
```

Otherwise if `maybe_filter` is None, return just a `DataSourceExec`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#method.parquet_metrics" class="fn">parquet_metrics</a>(plan: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Retrieve metrics from the parquet exec returned from `create_scan`

Recursively searches for DataSourceExec and returns the metrics on the first one it finds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>

The schema of this parquet file

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#method.path" class="fn">path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>

The path to the parquet file

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/struct.TestParquetFile.html#blanket-implementations" class="anchor">§</a>
