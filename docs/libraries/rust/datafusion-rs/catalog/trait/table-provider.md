# TableProvider in datafusion::catalog - Rust

## Trait TableProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#51)

```
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

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#54)

Returns the table provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#57)

Get a reference to the schema for this table

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#70)

Get the type of this table for metadata/catalog purposes.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#166-172)

Create an [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for scanning the table with optionally specified `projection`, `filter` and `limit`, described below.

The `ExecutionPlan` is responsible scanning the datasource’s partitions in a streaming, parallelized fashion.

##### [§](#projection)Projection

If specified, only a subset of columns should be returned, in the order specified. The projection is a set of indexes of the fields in [`Self::schema`](about:blank/datasource/trait.TableProvider.html#tymethod.schema "method datafusion_catalog::table::TableProvider::schema::schema").

DataFusion provides the projection to scan only the columns actually used in the query to improve performance, an optimization called “Projection Pushdown”. Some datasources, such as Parquet, can use this information to go significantly faster when only a subset of columns is required.

##### [§](#filters)Filters

A list of boolean filter [`Expr`](../prelude/enum.Expr.html "enum datafusion::prelude::Expr")s to evaluate _during_ the scan, in the manner specified by [`Self::supports_filters_pushdown`](about:blank/datasource/trait.TableProvider.html#method.supports_filters_pushdown "method datafusion_catalog::table::TableProvider::supports_filters_pushdown::supports_filters_pushdown"). Only rows for which _all_ of the `Expr`s evaluate to `true` must be returned (aka the expressions are `AND`ed together).

To enable filter pushdown you must override [`Self::supports_filters_pushdown`](about:blank/datasource/trait.TableProvider.html#method.supports_filters_pushdown "method datafusion_catalog::table::TableProvider::supports_filters_pushdown::supports_filters_pushdown") as the default implementation does not and `filters` will be empty.

DataFusion pushes filtering into the scans whenever possible (“Filter Pushdown”), and depending on the format and the implementation of the format, evaluating the predicate during the scan can increase performance significantly.

###### [§](#note-some-columns-may-appear-only-in-filters)Note: Some columns may appear _only_ in Filters

In certain cases, a query may only use a certain column in a Filter that has been completely pushed down to the scan. In this case, the projection will not contain all the columns found in the filter expressions.

For example, given the query `SELECT t.a FROM t WHERE t.b > 5`,

```
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

##### [§](#limit)Limit

If `limit` is specified, must only produce _at least_ this many rows, (though it may return more). Like Projection Pushdown and Filter Pushdown, DataFusion pushes `LIMIT`s as far down in the plan as possible, called “Limit Pushdown” as some sources can use this information to improve their performance. Note that if there are any Inexact filters pushed down, the LIMIT cannot be pushed down. This is because inexact filters do not guarantee that every filtered row is removed, so applying the limit could lead to too few rows being available to return as a final result.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#65)

Get a reference to the constraints of the table. Returns:

- `None` for tables that do not support constraints.
- `Some(&Constraints)` for tables supporting constraints. Therefore, a `Some(&Constraints::empty())` return value indicates that this table supports constraints, but there are no constraints.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#73)

Get the create statement used to create this table, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#78)

Get the [`LogicalPlan`](../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") of this table, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#83)

Get the default value for a column, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#254-257)

Specify if DataFusion should provide filter expressions to the TableProvider to apply _during_ the scan.

Some TableProviders can evaluate filters more efficiently than the `Filter` operator in DataFusion, for example by using an index.

##### [§](#parameters-and-return-value)Parameters and Return Value

The return `Vec` must have one element for each element of the `filters` argument. The value of each element indicates if the TableProvider can apply the corresponding filter during the scan. The position in the return value corresponds to the expression in the `filters` parameter.

If the length of the resulting `Vec` does not match the `filters` input an error will be thrown.

Each element in the resulting `Vec` is one of the following:

- [`Exact`](about:blank/logical_expr/enum.TableProviderFilterPushDown.html#variant.Exact "variant datafusion::logical_expr::TableProviderFilterPushDown::Exact") or [`Inexact`](about:blank/logical_expr/enum.TableProviderFilterPushDown.html#variant.Inexact "variant datafusion::logical_expr::TableProviderFilterPushDown::Inexact"): The TableProvider can apply the filter during scan
- [`Unsupported`](about:blank/logical_expr/enum.TableProviderFilterPushDown.html#variant.Unsupported "variant datafusion::logical_expr::TableProviderFilterPushDown::Unsupported"): The TableProvider cannot apply the filter during scan

By default, this function returns [`Unsupported`](about:blank/logical_expr/enum.TableProviderFilterPushDown.html#variant.Unsupported "variant datafusion::logical_expr::TableProviderFilterPushDown::Unsupported") for all filters, meaning no filters will be provided to [`Self::scan`](about:blank/datasource/trait.TableProvider.html#tymethod.scan "method datafusion_catalog::table::TableProvider::scan::scan").

##### [§](#example)Example

```
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

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#268)

Get statistics for this table, if available Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules to perform operations such as re-ordering of joins.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#50)

Return an [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to insert data into this table, if supported.

The returned plan should return a single row in a UInt64 column called “count” such as the following

```
+-------+,
| count |,
+-------+,
| 6     |,
+-------+,
```

##### [§](#see-also)See Also

See [`DataSinkExec`](../datasource/sink/struct.DataSinkExec.html "struct datafusion::datasource::sink::DataSinkExec") for the common pattern of inserting a streams of `RecordBatch`es as files to an ObjectStore.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/cte_worktable.rs.html#69)
[§](#impl-TableProvider-for-CteWorkTable)

[Source](about:blank/src/datafusion/datasource/empty.rs.html#59-85)
[§](#impl-TableProvider-for-EmptyTable)

[Source](about:blank/src/datafusion/datasource/listing/table.rs.html#1145-1350)
[§](#impl-TableProvider-for-ListingTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/stream.rs.html#305)
[§](#impl-TableProvider-for-StreamTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#206)
[§](#impl-TableProvider-for-MemTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/view.rs.html#85)
[§](#impl-TableProvider-for-ViewTable)

[Source](https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/generate_series.rs.html#454)
[§](#impl-TableProvider-for-GenerateSeriesTable)

[Source](about:blank/src/datafusion/test_util/mod.rs.html#204-226)
[§](#impl-TableProvider-for-TestTableProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/streaming.rs.html#84)
[§](#impl-TableProvider-for-StreamingTable)
