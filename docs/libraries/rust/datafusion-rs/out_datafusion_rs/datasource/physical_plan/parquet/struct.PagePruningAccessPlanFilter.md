# Struct PagePruningAccessPlanFilter Copy item path

<a href="https://docs.rs/datafusion-datasource-parquet/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_parquet/page_filter.rs.html#113" class="src">Source</a>

``` rust
pub struct PagePruningAccessPlanFilter { /* private fields */ }
```

Available on **crate feature `parquet`** only.

Expand description

Filters a [`ParquetAccessPlan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.ParquetAccessPlan.html "struct datafusion::datasource::physical_plan::parquet::ParquetAccessPlan") based on the [Parquet PageIndex](https://github.com/apache/parquet-format/blob/master/PageIndex.md), if present

It does so by evaluating statistics from the [`ParquetColumnIndex`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/metadata/type.ParquetColumnIndex.html "type parquet::file::metadata::ParquetColumnIndex") and [`ParquetOffsetIndex`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/metadata/type.ParquetOffsetIndex.html "type parquet::file::metadata::ParquetOffsetIndex") and converting them to [`RowSelection`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/selection/struct.RowSelection.html "struct parquet::arrow::arrow_reader::selection::RowSelection").

For example, given a row group with two column (chunks) for `A` and `B` with the following with page level statistics:

``` text
┏━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━
   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   ┃
┃     ┌──────────────┐  │     ┌──────────────┐  │  ┃
┃  │  │              │     │  │              │     ┃
┃     │              │  │     │     Page     │  │
   │  │              │     │  │      3       │     ┃
┃     │              │  │     │   min: "A"   │  │  ┃
┃  │  │              │     │  │   max: "C"   │     ┃
┃     │     Page     │  │     │ first_row: 0 │  │
   │  │      1       │     │  │              │     ┃
┃     │   min: 10    │  │     └──────────────┘  │  ┃
┃  │  │   max: 20    │     │  ┌──────────────┐     ┃
┃     │ first_row: 0 │  │     │              │  │
   │  │              │     │  │     Page     │     ┃
┃     │              │  │     │      4       │  │  ┃
┃  │  │              │     │  │   min: "D"   │     ┃
┃     │              │  │     │   max: "G"   │  │
   │  │              │     │  │first_row: 100│     ┃
┃     └──────────────┘  │     │              │  │  ┃
┃  │  ┌──────────────┐     │  │              │     ┃
┃     │              │  │     └──────────────┘  │
   │  │     Page     │     │  ┌──────────────┐     ┃
┃     │      2       │  │     │              │  │  ┃
┃  │  │   min: 30    │     │  │     Page     │     ┃
┃     │   max: 40    │  │     │      5       │  │
   │  │first_row: 200│     │  │   min: "H"   │     ┃
┃     │              │  │     │   max: "Z"   │  │  ┃
┃  │  │              │     │  │first_row: 250│     ┃
┃     └──────────────┘  │     │              │  │
   │                       │  └──────────────┘     ┃
┃   ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘  ┃
┃       ColumnChunk            ColumnChunk         ┃
┃            A                      B
 ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━┛

  Total rows: 300
```

Given the predicate `A > 35 AND B = 'F'`:

Using `A > 35`: can rule out all of values in Page 1 (rows 0 -\> 199)

Using `B = 'F'`: can rule out all values in Page 3 and Page 5 (rows 0 -\> 99, and 250 -\> 299)

So we can entirely skip rows 0-\>199 and 250-\>299 as we know they can not contain rows that match the predicate.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#implementation-notes" class="doc-anchor">§</a>Implementation notes

Single column predicates are evaluated using the PageIndex information for that column to determine which row ranges can be skipped based.

The resulting [`RowSelection`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/selection/struct.RowSelection.html "struct parquet::arrow::arrow_reader::selection::RowSelection")’s are combined into a final row selection that is added to the [`ParquetAccessPlan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.ParquetAccessPlan.html "struct datafusion::datasource::physical_plan::parquet::ParquetAccessPlan").

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#impl-PagePruningAccessPlanFilter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html" class="struct" title="struct datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter">PagePruningAccessPlanFilter</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#method.new" class="fn">new</a>( expr: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html" class="struct" title="struct datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter">PagePruningAccessPlanFilter</a>

Create a new [`PagePruningAccessPlanFilter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html "struct datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter") from a physical expression.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#method.prune_plan_with_page_index" class="fn">prune_plan_with_page_index</a>( &self, access_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.ParquetAccessPlan.html" class="struct" title="struct datafusion::datasource::physical_plan::parquet::ParquetAccessPlan">ParquetAccessPlan</a>, arrow_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, parquet_schema: &<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/schema/types/struct.SchemaDescriptor.html" class="struct" title="struct parquet::schema::types::SchemaDescriptor">SchemaDescriptor</a>, parquet_metadata: &<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/metadata/struct.ParquetMetaData.html" class="struct" title="struct parquet::file::metadata::ParquetMetaData">ParquetMetaData</a>, file_metrics: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetFileMetrics.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetFileMetrics">ParquetFileMetrics</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.ParquetAccessPlan.html" class="struct" title="struct datafusion::datasource::physical_plan::parquet::ParquetAccessPlan">ParquetAccessPlan</a>

Returns an updated [`ParquetAccessPlan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.ParquetAccessPlan.html "struct datafusion::datasource::physical_plan::parquet::ParquetAccessPlan") by applying predicates to the parquet page index, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#method.filter_number" class="fn">filter_number</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of filters in the [`PagePruningAccessPlanFilter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html "struct datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#impl-Debug-for-PagePruningAccessPlanFilter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html" class="struct" title="struct datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter">PagePruningAccessPlanFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/parquet/struct.PagePruningAccessPlanFilter.html#blanket-implementations" class="anchor">§</a>
