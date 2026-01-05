# SQLOptions in datafusion::execution::context - Rust

```
pub struct SQLOptions { /* private fields */ }
```

Expand description

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1845-1875)
[§](#impl-SQLOptions)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1847-1849)

Create a new `SQLOptions` with default values

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1852-1855)

Should DDL data definition commands (e.g. `CREATE TABLE`) be run? Defaults to `true`.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1858-1861)

Should DML data modification commands (e.g. `INSERT` and `COPY`) be run? Defaults to `true`

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1864-1867)

Should Statements such as (e.g. `SET VARIABLE and` BEGIN TRANSACTION `...`) be run?. Defaults to `true`

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1871-1874)

Return an error if the [`LogicalPlan`](../../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") has any nodes that are incompatible with this [`SQLOptions`](struct.SQLOptions.html "struct datafusion::execution::context::SQLOptions").

[§](#impl-Freeze-for-SQLOptions)

[§](#impl-RefUnwindSafe-for-SQLOptions)

[§](#impl-Send-for-SQLOptions)

[§](#impl-Sync-for-SQLOptions)

[§](#impl-Unpin-for-SQLOptions)

[§](#impl-UnwindSafe-for-SQLOptions)
