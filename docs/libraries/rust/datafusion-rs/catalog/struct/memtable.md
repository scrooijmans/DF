# MemTable in datafusion::catalog - Rust

```
pub struct MemTable {
    pub batches: Vec<Arc<RwLock<Vec<RecordBatch>>>>,
    pub sort_order: Arc<Mutex<RawMutex, Vec<Vec<Sort>>>>,
    /* private fields */
}
```

Expand description

In-memory data source for presenting a `Vec<RecordBatch>` as a data source that can be queried by DataFusion. This allows data to be pre-loaded into memory and then repeatedly queried without incurring additional file I/O overhead.

Optional pre-known sort order(s). Must be `SortExpr`s. inserting data into this table removes the order

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#69)
[§](#impl-MemTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#75)

Create a new in-memory table from the provided schema and record batches.

Requires at least one partition. To construct an empty `MemTable`, pass `vec![vec![]]` as the `partitions` argument, this represents one partition with no batches.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#104)

Assign constraints

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#110-113)

Assign column defaults

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#128)

Specify an optional pre-known sort order(s). Must be `SortExpr`s.

If the data is not sorted by this order, DataFusion may produce incorrect results.

DataFusion may take advantage of this ordering to omit sorts or use more efficient algorithms.

Note that multiple sort orders are supported, if some are known to be equivalent,

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#134-138)

Create a mem table by reading from another data source

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#57)
[§](#impl-Debug-for-MemTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#206)
[§](#impl-TableProvider-for-MemTable)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#205)
[§](#method.insert_into)

Returns an ExecutionPlan that inserts the execution results of a given [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") into this [`MemTable`](../datasource/struct.MemTable.html "struct datafusion::datasource::MemTable").

The [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") must have the same schema as this [`MemTable`](../datasource/struct.MemTable.html "struct datafusion::datasource::MemTable").

##### [§](#arguments)Arguments

- `state` - The [`SessionState`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html) containing the context for executing the plan.
- `input` - The [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to execute and insert.

##### [§](#returns)Returns

- A plan that returns the number of rows written.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#207)
[§](#method.as_any)

Returns the table provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#211)
[§](#method.schema)

Get a reference to the schema for this table

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#215)
[§](#method.constraints)

Get a reference to the constraints of the table. Returns: [Read more](about:blank/datasource/trait.TableProvider.html#method.constraints)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#219)
[§](#method.table_type)

Get the type of this table for metadata/catalog purposes.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#205)
[§](#method.scan)

Create an [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for scanning the table with optionally specified `projection`, `filter` and `limit`, described below. [Read more](about:blank/datasource/trait.TableProvider.html#tymethod.scan)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/table.rs.html#295)
[§](#method.get_column_default)

Get the default value for a column, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#73)
[§](#method.get_table_definition)

Get the create statement used to create this table, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#78)
[§](#method.get_logical_plan)

Get the [`LogicalPlan`](../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") of this table, if available.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#254-257)
[§](#method.supports_filters_pushdown)

Specify if DataFusion should provide filter expressions to the TableProvider to apply _during_ the scan. [Read more](about:blank/datasource/trait.TableProvider.html#method.supports_filters_pushdown)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#268)
[§](#method.statistics)

Get statistics for this table, if available Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules to perform operations such as re-ordering of joins.

[§](#impl-Freeze-for-MemTable)

[§](#impl-RefUnwindSafe-for-MemTable)

[§](#impl-Send-for-MemTable)

[§](#impl-Sync-for-MemTable)

[§](#impl-Unpin-for-MemTable)

[§](#impl-UnwindSafe-for-MemTable)
