# Trait TableProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#51" class="src">Source</a>

``` rust
pub trait TableProvider:
    Debug
    + Sync
    + Send {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn schema(&self) -> Arc<Schema>;
    fn table_type(&self) -> TableType;
    fn scan<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        state: &'life1 dyn Session,
        projection: Option<&'life2 Vec<usize>>,
        filters: &'life3 [Expr],
        limit: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn ExecutionPlan>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             'life3: 'async_trait,
             Self: 'async_trait;

    // Provided methods
    fn constraints(&self) -> Option<&Constraints> { ... }
    fn get_table_definition(&self) -> Option<&str> { ... }
    fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>> { ... }
    fn get_column_default(&self, _column: &str) -> Option<&Expr> { ... }
    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>, DataFusionError> { ... }
    fn statistics(&self) -> Option<Statistics> { ... }
    fn insert_into<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _state: &'life1 dyn Session,
        _input: Arc<dyn ExecutionPlan>,
        _insert_op: InsertOp,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn ExecutionPlan>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait { ... }
}
```

Expand description

A table which can be queried and modified.

Please see [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") for details of implementing a custom catalog.

[`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") represents a source of data which can provide data as Apache Arrow [`RecordBatch`](https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html)es. Implementations of this trait provide important information for planning such as:

1.  [`Self::schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.schema "method datafusion_catalog::table::TableProvider::schema::schema"): The schema (columns and their types) of the table
2.  [`Self::supports_filters_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown "method datafusion_catalog::table::TableProvider::supports_filters_pushdown::supports_filters_pushdown"): Should filters be pushed into this scan
3.  [`Self::scan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan "method datafusion_catalog::table::TableProvider::scan::scan"): An [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") that can read data

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the table provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this table

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan" class="fn">scan</a>\<'life0, 'life1, 'life2, 'life3, 'async_trait\>( &'life0 self, state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life2 <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, filters: &'life3 \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, Self: 'async_trait,

Create an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for scanning the table with optionally specified `projection`, `filter` and `limit`, described below.

The `ExecutionPlan` is responsible scanning the datasource’s partitions in a streaming, parallelized fashion.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#projection" class="doc-anchor">§</a>Projection

If specified, only a subset of columns should be returned, in the order specified. The projection is a set of indexes of the fields in [`Self::schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.schema "method datafusion_catalog::table::TableProvider::schema::schema").

DataFusion provides the projection to scan only the columns actually used in the query to improve performance, an optimization called “Projection Pushdown”. Some datasources, such as Parquet, can use this information to go significantly faster when only a subset of columns is required.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#filters" class="doc-anchor">§</a>Filters

A list of boolean filter [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s to evaluate *during* the scan, in the manner specified by [`Self::supports_filters_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown "method datafusion_catalog::table::TableProvider::supports_filters_pushdown::supports_filters_pushdown"). Only rows for which *all* of the `Expr`s evaluate to `true` must be returned (aka the expressions are `AND`ed together).

To enable filter pushdown you must override [`Self::supports_filters_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown "method datafusion_catalog::table::TableProvider::supports_filters_pushdown::supports_filters_pushdown") as the default implementation does not and `filters` will be empty.

DataFusion pushes filtering into the scans whenever possible (“Filter Pushdown”), and depending on the format and the implementation of the format, evaluating the predicate during the scan can increase performance significantly.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#note-some-columns-may-appear-only-in-filters" class="doc-anchor">§</a>Note: Some columns may appear *only* in Filters

In certain cases, a query may only use a certain column in a Filter that has been completely pushed down to the scan. In this case, the projection will not contain all the columns found in the filter expressions.

For example, given the query `SELECT t.a FROM t WHERE t.b > 5`,

``` text
┌────────────────────┐
│  Projection(t.a)   │
└────────────────────┘
           ▲
           │
           │
┌────────────────────┐     Filter     ┌────────────────────┐   Projection    ┌────────────────────┐
│  Filter(t.b > 5)   │────Pushdown──▶ │  Projection(t.a)   │ ───Pushdown───▶ │  Projection(t.a)   │
└────────────────────┘                └────────────────────┘                 └────────────────────┘
           ▲                                     ▲                                      ▲
           │                                     │                                      │
           │                                     │                           ┌────────────────────┐
┌────────────────────┐                ┌────────────────────┐                 │        Scan        │
│        Scan        │                │        Scan        │                 │  filter=(t.b > 5)  │
└────────────────────┘                │  filter=(t.b > 5)  │                 │  projection=(t.a)  │
                                      └────────────────────┘                 └────────────────────┘

Initial Plan                  If `TableProviderFilterPushDown`           Projection pushdown notes that
                              returns true, filter pushdown              the scan only needs t.a
                              pushes the filter into the scan
                                                                         BUT internally evaluating the
                                                                         predicate still requires t.b
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#limit" class="doc-anchor">§</a>Limit

If `limit` is specified, must only produce *at least* this many rows, (though it may return more). Like Projection Pushdown and Filter Pushdown, DataFusion pushes `LIMIT`s as far down in the plan as possible, called “Limit Pushdown” as some sources can use this information to improve their performance. Note that if there are any Inexact filters pushed down, the LIMIT cannot be pushed down. This is because inexact filters do not guarantee that every filtered row is removed, so applying the limit could lead to too few rows being available to return as a final result.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get a reference to the constraints of the table. Returns:

- `None` for tables that do not support constraints.
- `Some(&Constraints)` for tables supporting constraints. Therefore, a `Some(&Constraints::empty())` return value indicates that this table supports constraints, but there are no constraints.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_table_definition" class="fn">get_table_definition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the create statement used to create this table, if available.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") of this table, if available.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_column_default" class="fn">get_column_default</a>(&self, \_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filters: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Specify if DataFusion should provide filter expressions to the TableProvider to apply *during* the scan.

Some TableProviders can evaluate filters more efficiently than the `Filter` operator in DataFusion, for example by using an index.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#parameters-and-return-value" class="doc-anchor">§</a>Parameters and Return Value

The return `Vec` must have one element for each element of the `filters` argument. The value of each element indicates if the TableProvider can apply the corresponding filter during the scan. The position in the return value corresponds to the expression in the `filters` parameter.

If the length of the resulting `Vec` does not match the `filters` input an error will be thrown.

Each element in the resulting `Vec` is one of the following:

- [`Exact`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html#variant.Exact "variant datafusion::logical_expr::TableProviderFilterPushDown::Exact") or [`Inexact`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html#variant.Inexact "variant datafusion::logical_expr::TableProviderFilterPushDown::Inexact"): The TableProvider can apply the filter during scan
- [`Unsupported`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html#variant.Unsupported "variant datafusion::logical_expr::TableProviderFilterPushDown::Unsupported"): The TableProvider cannot apply the filter during scan

By default, this function returns [`Unsupported`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html#variant.Unsupported "variant datafusion::logical_expr::TableProviderFilterPushDown::Unsupported") for all filters, meaning no filters will be provided to [`Self::scan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan "method datafusion_catalog::table::TableProvider::scan::scan").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#example" class="doc-anchor">§</a>Example

``` rust
// Define a struct that implements the TableProvider trait
#[derive(Debug)]
struct TestDataSource {}

#[async_trait]
impl TableProvider for TestDataSource {
        todo!()
    // Override the supports_filters_pushdown to evaluate which expressions
    // to accept as pushdown predicates.
    fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>> {
        // Process each filter
        let support: Vec<_> = filters.iter().map(|expr| {
          match expr {
            // This example only supports a between expr with a single column named "c1".
            Expr::Between(between_expr) => {
                between_expr.expr
                .try_as_col()
                .map(|column| {
                    if column.name == "c1" {
                        TableProviderFilterPushDown::Exact
                    } else {
                        TableProviderFilterPushDown::Unsupported
                    }
                })
                // If there is no column in the expr set the filter to unsupported.
                .unwrap_or(TableProviderFilterPushDown::Unsupported)
            }
            _ => {
                // For all other cases return Unsupported.
                TableProviderFilterPushDown::Unsupported
            }
        }
    }).collect();
    Ok(support)
    }
}
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>

Get statistics for this table, if available Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules to perform operations such as re-ordering of joins.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into" class="fn">insert_into</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, \_state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, \_input: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, \_insert_op: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp">InsertOp</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Return an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to insert data into this table, if supported.

The returned plan should return a single row in a UInt64 column called “count” such as the following

``` text
+-------+,
| count |,
+-------+,
| 6     |,
+-------+,
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#see-also" class="doc-anchor">§</a>See Also

See [`DataSinkExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/struct.DataSinkExec.html "struct datafusion::datasource::sink::DataSinkExec") for the common pattern of inserting a streams of `RecordBatch`es as files to an ObjectStore.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-StreamingTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/streaming/struct.StreamingTable.html" class="struct" title="struct datafusion::catalog::streaming::StreamingTable">StreamingTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-GenerateSeriesTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.GenerateSeriesTable.html" class="struct" title="struct datafusion::functions_table::generate_series::GenerateSeriesTable">GenerateSeriesTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-TestTableProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableProvider.html" class="struct" title="struct datafusion::test_util::TestTableProvider">TestTableProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-CteWorkTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-EmptyTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/empty/struct.EmptyTable.html" class="struct" title="struct datafusion::datasource::empty::EmptyTable">EmptyTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-ListingTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-StreamTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html" class="struct" title="struct datafusion::datasource::stream::StreamTable">StreamTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-MemTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.MemTable.html" class="struct" title="struct datafusion::datasource::MemTable">MemTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#impl-TableProvider-for-ViewTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.ViewTable.html" class="struct" title="struct datafusion::datasource::ViewTable">ViewTable</a>
