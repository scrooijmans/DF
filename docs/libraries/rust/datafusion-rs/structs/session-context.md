# SessionContext in datafusion::execution::context - Rust

## Struct SessionContext 

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#275-282)

```
pub struct SessionContext { /* private fields */ }
```

Expand description

Main interface for executing queries with DataFusion. Maintains the state of the connection between a user and an instance of the DataFusion engine.

See examples below for how to use the `SessionContext` to execute queries and how to configure the session.

## [§](#overview)Overview

[`SessionContext`](struct.SessionContext.html "struct datafusion::execution::context::SessionContext") provides the following functionality:

- Create a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from a CSV or Parquet data source.
- Register a CSV or Parquet data source as a table that can be referenced from a SQL query.
- Register a custom data source that can be referenced from a SQL query.
- Execution a SQL query

## [§](#example-dataframe-api)Example: DataFrame API

The following example demonstrates how to use the context to execute a query against a CSV data source using the [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") API:

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;
let results = df
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+----------------+",
   "| a | min(?table?.b) |",
   "+---+----------------+",
   "| 1 | 2              |",
   "+---+----------------+",
 ],
 &results
);
```

## [§](#example-sql-api)Example: SQL API

The following example demonstrates how to execute the same query using SQL:

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;
let results = ctx
  .sql("SELECT a, min(b) FROM example GROUP BY a LIMIT 100")
  .await?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+----------------+",
   "| a | min(example.b) |",
   "+---+----------------+",
   "| 1 | 2              |",
   "+---+----------------+",
 ],
 &results
);
```

## [§](#example-configuring-sessioncontext)Example: Configuring `SessionContext`

The `SessionContext` can be configured by creating a [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") using [`SessionStateBuilder`](../session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder"):

```
// Configure a 4k batch size
let config = SessionConfig::new().with_batch_size(4 * 1024);

// configure a memory limit of 1GB with 20%  slop
 let runtime_env = RuntimeEnvBuilder::new()
    .with_memory_limit(1024 * 1024 * 1024, 0.80)
    .build_arc()
    .unwrap();

// Create a SessionState using the config and runtime_env
let state = SessionStateBuilder::new()
  .with_config(config)
  .with_runtime_env(runtime_env)
  // include support for built in functions and configurations
  .with_default_features()
  .build();

// Create a SessionContext
let ctx = SessionContext::from(state);
```

## [§](#relationship-between-sessioncontext-sessionstate-and-taskcontext)Relationship between `SessionContext`, `SessionState`, and `TaskContext`

The state required to optimize, and evaluate queries is broken into three levels to allow tailoring

The objects are:

1.  [`SessionContext`](struct.SessionContext.html "struct datafusion::execution::context::SessionContext"): Most users should use a `SessionContext`. It contains all information required to execute queries including high level APIs such as [`SessionContext::sql`](about:blank/struct.SessionContext.html#method.sql "method datafusion::execution::context::SessionContext::sql"). All queries run with the same `SessionContext` share the same configuration and resources (e.g. memory limits).
2.  [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"): contains information required to plan and execute an individual query (e.g. creating a [`LogicalPlan`](../../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") or [`ExecutionPlan`](../../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")). Each query is planned and executed using its own `SessionState`, which can be created with [`SessionContext::state`](about:blank/struct.SessionContext.html#method.state "method datafusion::execution::context::SessionContext::state"). `SessionState` allows finer grained control over query execution, for example disallowing DDL operations such as `CREATE TABLE`.
3.  [`TaskContext`](../struct.TaskContext.html "struct datafusion::execution::TaskContext") contains the state required for query execution (e.g. [`ExecutionPlan::execute`](about:blank/physical_plan/trait.ExecutionPlan.html#tymethod.execute "method datafusion::physical_plan::ExecutionPlan::execute")). It contains a subset of information in [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"). `TaskContext` allows executing [`ExecutionPlan`](../../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s [`PhysicalExpr`](../../physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s without requiring a full [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState").

[Source](about:blank/src/datafusion/execution/context/csv.rs.html#25-87)
[§](#impl-SessionContext)

[Source](about:blank/src/datafusion/execution/context/csv.rs.html#46-52)

Creates a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a CSV data source.

For more control such as reading multiple files, you can use [`read_table`](about:blank/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](../../datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

Example usage is given below:

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
// You can read a single file using `read_csv`
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// you can also read multiple files:
let df = ctx.read_csv(vec!["tests/data/example.csv", "tests/data/example.csv"], CsvReadOptions::new()).await?;
```

[Source](about:blank/src/datafusion/execution/context/csv.rs.html#56-77)

Registers a CSV file as a table which can referenced from SQL statements executed against this context.

[Source](about:blank/src/datafusion/execution/context/csv.rs.html#80-86)

Executes a query and writes the results to a partitioned CSV file.

[Source](about:blank/src/datafusion/execution/context/json.rs.html#25-72)
[§](#impl-SessionContext-1)

[Source](about:blank/src/datafusion/execution/context/json.rs.html#32-38)

[Source](about:blank/src/datafusion/execution/context/json.rs.html#42-62)

Registers a JSON file as a table that it can be referenced from SQL statements executed against this context.

[Source](about:blank/src/datafusion/execution/context/json.rs.html#65-71)

Executes a query and writes the results to a partitioned JSON file.

[Source](about:blank/src/datafusion/execution/context/parquet.rs.html#27-97)
[§](#impl-SessionContext-2)

[Source](about:blank/src/datafusion/execution/context/parquet.rs.html#49-55)

Available on **crate feature `parquet`** only.

Creates a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a Parquet data source.

For more control such as reading multiple files, you can use [`read_table`](about:blank/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](../../datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

For an example, see [`read_csv`](about:blank/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv")

##### [§](#note-statistics)Note: Statistics

NOTE: by default, statistics are collected when reading the Parquet files This can slow down the initial DataFrame creation while greatly accelerating queries with certain filters.

To disable statistics collection, set the [config option](https://datafusion.apache.org/user-guide/configs.html) `datafusion.execution.collect_statistics` to `false`. See [`ConfigOptions`](../../config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") and [`ExecutionOptions::collect_statistics`](about:blank/config/struct.ExecutionOptions.html#structfield.collect_statistics "field datafusion::config::ExecutionOptions::collect_statistics") for more details.

[Source](about:blank/src/datafusion/execution/context/parquet.rs.html#66-86)

Available on **crate feature `parquet`** only.

Registers a Parquet file as a table that can be referenced from SQL statements executed against this context.

##### [§](#note-statistics-1)Note: Statistics

Statistics are not collected by default. See [`read_parquet`](about:blank/struct.SessionContext.html#method.read_parquet "method datafusion::execution::context::SessionContext::read_parquet") for more details and how to enable them.

[Source](about:blank/src/datafusion/execution/context/parquet.rs.html#89-96)

Available on **crate feature `parquet`** only.

Executes a query and writes the results to a partitioned Parquet file.

[Source](about:blank/src/datafusion/execution/context/avro.rs.html#23-61)
[§](#impl-SessionContext-3)

[Source](about:blank/src/datafusion/execution/context/avro.rs.html#30-36)

Available on **crate feature `avro`** only.

[Source](about:blank/src/datafusion/execution/context/avro.rs.html#40-60)

Available on **crate feature `avro`** only.

Registers an Avro file as a table that can be referenced from SQL statements executed against this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#290-1679)
[§](#impl-SessionContext-4)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#292-294)

Creates a new `SessionContext` using the default [`SessionConfig`](../../prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig").

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#297-314)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#321-324)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#339-346)

Creates a new `SessionContext` using the provided [`SessionConfig`](../../prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") and a [`RuntimeEnv`](../runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv").

##### [§](#resource-limits)Resource Limits

By default, each new `SessionContext` creates a new `RuntimeEnv`, and therefore will not enforce memory or disk limits for queries run on different `SessionContext`s.

To enforce resource limits (e.g. to limit the total amount of memory used) across all DataFusion queries in a process, all `SessionContext`’s should be configured with the same `RuntimeEnv`.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#349-355)

Creates a new `SessionContext` using the provided [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#396-414)

Enable querying local files as tables.

This feature is security sensitive and should only be enabled for systems that wish to permit direct access to the file system from SQL.

When enabled, this feature permits direct access to arbitrary files via SQL like

```
SELECT * from 'my_file.parquet'
```

See [DynamicFileCatalog](../../catalog/struct.DynamicFileCatalog.html "struct datafusion::catalog::DynamicFileCatalog") for more details

```
let ctx = SessionContext::new()
  .enable_url_table(); // permit local file access
let results = ctx
  .sql("SELECT a, MIN(b) FROM 'tests/data/example.csv' as example GROUP BY a LIMIT 100")
  .await?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+----------------+",
   "| a | min(example.b) |",
   "+---+----------------+",
   "| 1 | 2              |",
   "+---+----------------+",
 ],
 &results
);
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#440-451)

Convert the current `SessionContext` into a [`SessionStateBuilder`](../session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder")

This is useful to switch back to `SessionState` with custom settings such as [`Self::enable_url_table`](about:blank/struct.SessionContext.html#method.enable_url_table "method datafusion::execution::context::SessionContext::enable_url_table").

Avoids cloning the SessionState if possible.

##### [§](#example)Example

```
let my_rule = PushDownFilter{}; // pretend it is a new rule
// Create a new builder with a custom optimizer rule
let context: SessionContext = SessionStateBuilder::new()
  .with_optimizer_rule(Arc::new(my_rule))
  .build()
  .into();
// Enable local file access and convert context back to a builder
let builder = context
  .enable_url_table()
  .into_state_builder();
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#454-456)

Returns the time this `SessionContext` was created

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#459-465)

Registers a [`FunctionFactory`](trait.FunctionFactory.html "trait datafusion::execution::context::FunctionFactory") to handle `CREATE FUNCTION` statements

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#470-475)

Adds an optimizer rule to the end of the existing rules.

See [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") for more control of when the rule is applied.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#480-482)

Adds an analyzer rule to the end of the existing rules.

See [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") for more control of when the rule is applied.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#499-505)

Registers an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") to be used with a specific URL prefix.

See [`RuntimeEnv::register_object_store`](about:blank/runtime_env/struct.RuntimeEnv.html#method.register_object_store "method datafusion::execution::runtime_env::RuntimeEnv::register_object_store") for more details.

##### [§](#example-register-a-local-object-store-for-the-file-url-prefix)Example: register a local object store for the “file://” URL prefix

```
let object_store_url = ObjectStoreUrl::parse("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
let ctx = SessionContext::new();
// All files with the file:// url prefix will be read from the local file system
ctx.register_object_store(object_store_url.as_ref(), Arc::new(object_store));
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#508-520)

Registers the [`RecordBatch`](../../common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") as the specified table name

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#523-525)

Return the [RuntimeEnv](../runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") used to run queries with this `SessionContext`

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#528-530)

Returns an id that uniquely identifies this `SessionContext`.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#534-539)

Return the [`TableProviderFactory`](../../catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory") that is registered for the specified file type, if any.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#542-549)

Return the `enable_ident_normalization` of this Session

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#552-554)

Return a copied version of config for this Session

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#557-559)

Return a copied version of table options for this Session

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#588-590)

Creates a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from SQL query text.

Note: This API implements DDL statements such as `CREATE TABLE` and `CREATE VIEW` and DML statements such as `INSERT INTO` with in-memory default implementations. See [`Self::sql_with_options`](about:blank/struct.SessionContext.html#method.sql_with_options "method datafusion::execution::context::SessionContext::sql_with_options").

##### [§](#example-running-sql-queries)Example: Running SQL queries

See the example on [`Self`](struct.SessionContext.html "struct datafusion::execution::context::SessionContext")

##### [§](#example-creating-a-table-with-sql)Example: Creating a Table with SQL

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
ctx
  .sql("CREATE TABLE foo (x INTEGER)")
  .await?
  .collect()
  .await?;
assert!(ctx.table_exist("foo").unwrap());
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#618-627)

Creates a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from SQL query text, first validating that the queries are allowed by `options`

##### [§](#example-preventing-creating-a-table-with-sql)Example: Preventing Creating a Table with SQL

If you want to avoid creating tables, or modifying data or the session, set [`SQLOptions`](struct.SQLOptions.html "struct datafusion::execution::context::SQLOptions") appropriately:

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
let options = SQLOptions::new()
  .with_allow_ddl(false);
let err = ctx.sql_with_options("CREATE TABLE foo (x INTEGER)", options)
  .await
  .unwrap_err();
assert!(
  err.to_string().starts_with("Error during planning: DDL not supported: CreateMemoryTable")
);
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#651-653)

Creates logical expressions from SQL query text.

##### [§](#example-parsing-sql-queries)Example: Parsing SQL queries

```
// datafusion will parse number as i64 first.
let sql = "a > 10";
let expected = col("a").gt(lit(10 as i64));
// provide type information that `a` is an Int32
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
let expr = SessionContext::new()
 .parse_sql_expr(sql, &df_schema)?;
assert_eq!(expected, expr);
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#662-739)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#768-774)

Create a [`PhysicalExpr`](../../physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](../../prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion and function rewrites.

Note: The expression is not [simplified](../../optimizer/simplify_expressions/index.html "mod datafusion::optimizer::simplify_expressions") or otherwise optimized: `a = 1 + 2` will not be simplified to `a = 3` as this is a more involved process. See the [expr_api](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example for how to simplify expressions.

##### [§](#example-1)Example

```
// a = 1 (i64)
let expr = col("a").eq(lit(1i64));
// provide type information that `a` is an Int32
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// Create a PhysicalExpr. Note DataFusion automatically coerces (casts) `1i64` to `1i32`
let physical_expr = SessionContext::new()
  .create_physical_expr(expr, &df_schema).unwrap();
```

##### [§](#see-also)See Also

- [`SessionState::create_physical_expr`](about:blank/session_state/struct.SessionState.html#method.create_physical_expr "method datafusion::execution::session_state::SessionState::create_physical_expr") for a lower level API

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1099-1115)

Parse memory limit from string to number of bytes Supports formats like ‘1.5G’, ‘100M’, ‘512K’

##### [§](#examples)Examples

```
use datafusion::execution::context::SessionContext;

assert_eq!(SessionContext::parse_memory_limit("1M").unwrap(), 1024 * 1024);
assert_eq!(SessionContext::parse_memory_limit("1.5G").unwrap(), (1.5 * 1024.0 * 1024.0 * 1024.0) as usize);
```

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1259-1268)

Registers a variable provider within this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1271-1273)

Register a table UDF with this context

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1284-1287)

Registers a scalar UDF within this context.

Note in SQL queries, function names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_FUNC(x)...` will look for a function named `"my_func"`
- `SELECT "my_FUNC"(x)` will look for a function named `"my_FUNC"`

Any functions registered with the udf name or its aliases will be overwritten with this new function

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1296-1298)

Registers an aggregate UDF within this context.

Note in SQL queries, aggregate names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_UDAF(x)...` will look for an aggregate named `"my_udaf"`
- `SELECT "my_UDAF"(x)` will look for an aggregate named `"my_UDAF"`

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1307-1309)

Registers a window UDF within this context.

Note in SQL queries, window function names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_UDWF(x)...` will look for a window function named `"my_udwf"`
- `SELECT "my_UDWF"(x)` will look for a window function named `"my_UDWF"`

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1312-1314)

Deregisters a UDF within this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1317-1319)

Deregisters a UDAF within this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1322-1324)

Deregisters a UDWF within this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1327-1329)

Deregisters a UDTF within this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1379-1385)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1388-1393)

Creates an empty DataFrame.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1397-1403)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1406-1417)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1419-1440)

Create a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a \[`Vec[`RecordBatch`]`\]

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1448-1467)

Registers a [`ListingTable`](../../datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") that can assemble multiple files from locations in an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance into a single table.

This method is `async` because it might need to resolve the schema.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1494-1512)

Registers an Arrow file as a table that can be referenced from SQL statements executed against this context.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1520-1530)

Registers a named catalog using a custom `CatalogProvider` so that it can be referenced from SQL statements executed against this context.

Returns the [`CatalogProvider`](../../catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") previously registered for this name, if any

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1533-1535)

Retrieves the list of available catalog names.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1538-1540)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1547-1558)

Registers a [`TableProvider`](../../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") as a table that can be referenced from SQL statements executed against this context.

If a table of the same name was already registered, returns “Table already exists” error.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1563-1573)

Deregisters the given table.

Returns the registered provider, if any

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1576-1585)

Return `true` if the specified table exists in the schema provider.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1594-1604)

Retrieves a [`DataFrame`](../../dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") representing a table previously registered by calling the [`register_table`](about:blank/struct.SessionContext.html#method.register_table "method datafusion::execution::context::SessionContext::register_table") function.

Returns an error if no table has been registered with the provided reference.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1611-1618)

Retrieves a [`TableFunction`](../../catalog/struct.TableFunction.html "struct datafusion::catalog::TableFunction") reference by name.

Returns an error if no table function has been registered with the provided name.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1621-1632)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1635-1637)

Get a new TaskContext to run in this session

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1651-1655)

Return a new [`SessionState`](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") suitable for executing a single query.

Notes:

1.  `query_execution_start_time` is set to the current time for the returned state.
2.  The returned state is not shared with the current session state and this changes to the returned `SessionState` such as changing [`ConfigOptions`](../../config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") will not be reflected in this `SessionContext`.

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1658-1660)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1663-1665)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1668-1670)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1674-1678)

Registers a [`ConfigExtension`](../../config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") as a table option extension that can be referenced from SQL statements executed against this context.

[§](#impl-Freeze-for-SessionContext)

[§](#impl-RefUnwindSafe-for-SessionContext)

[§](#impl-Send-for-SessionContext)

[§](#impl-Sync-for-SessionContext)

[§](#impl-Unpin-for-SessionContext)

[§](#impl-UnwindSafe-for-SessionContext)
