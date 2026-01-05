# Struct SessionContext Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#275-282" class="src">Source</a>

``` rust
pub struct SessionContext { /* private fields */ }
```

Expand description

Main interface for executing queries with DataFusion. Maintains the state of the connection between a user and an instance of the DataFusion engine.

See examples below for how to use the `SessionContext` to execute queries and how to configure the session.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#overview" class="doc-anchor">§</a>Overview

[`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") provides the following functionality:

- Create a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from a CSV or Parquet data source.
- Register a CSV or Parquet data source as a table that can be referenced from a SQL query.
- Register a custom data source that can be referenced from a SQL query.
- Execution a SQL query

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-dataframe-api" class="doc-anchor">§</a>Example: DataFrame API

The following example demonstrates how to use the context to execute a query against a CSV data source using the [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") API:

``` rust
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

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-sql-api" class="doc-anchor">§</a>Example: SQL API

The following example demonstrates how to execute the same query using SQL:

``` rust
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

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-configuring-sessioncontext" class="doc-anchor">§</a>Example: Configuring `SessionContext`

The `SessionContext` can be configured by creating a [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") using [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder"):

``` rust
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

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#relationship-between-sessioncontext-sessionstate-and-taskcontext" class="doc-anchor">§</a>Relationship between `SessionContext`, `SessionState`, and `TaskContext`

The state required to optimize, and evaluate queries is broken into three levels to allow tailoring

The objects are:

1.  [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext"): Most users should use a `SessionContext`. It contains all information required to execute queries including high level APIs such as [`SessionContext::sql`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql "method datafusion::execution::context::SessionContext::sql"). All queries run with the same `SessionContext` share the same configuration and resources (e.g. memory limits).

2.  [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"): contains information required to plan and execute an individual query (e.g. creating a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") or [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")). Each query is planned and executed using its own `SessionState`, which can be created with [`SessionContext::state`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.state "method datafusion::execution::context::SessionContext::state"). `SessionState` allows finer grained control over query execution, for example disallowing DDL operations such as `CREATE TABLE`.

3.  [`TaskContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html "struct datafusion::execution::TaskContext") contains the state required for query execution (e.g. [`ExecutionPlan::execute`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute "method datafusion::physical_plan::ExecutionPlan::execute")). It contains a subset of information in [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"). `TaskContext` allows executing [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s without requiring a full [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState").

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-SessionContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv" class="fn">read_csv</a>\<P: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>\>( &self, table_paths: P, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a CSV data source.

For more control such as reading multiple files, you can use [`read_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

Example usage is given below:

``` rust
use datafusion::prelude::*;
let ctx = SessionContext::new();
// You can read a single file using `read_csv`
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// you can also read multiple files:
let df = ctx.read_csv(vec!["tests/data/example.csv", "tests/data/example.csv"], CsvReadOptions::new()).await?;
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_csv" class="fn">register_csv</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a CSV file as a table which can referenced from SQL statements executed against this context.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.write_csv" class="fn">write_csv</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Executes a query and writes the results to a partitioned CSV file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-SessionContext-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_json" class="fn">read_json</a>\<P: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>\>( &self, table_paths: P, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading an JSON data source.

For more control such as reading multiple files, you can use [`read_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

For an example, see [`read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv")

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_json" class="fn">register_json</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a JSON file as a table that it can be referenced from SQL statements executed against this context.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.write_json" class="fn">write_json</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Executes a query and writes the results to a partitioned JSON file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-SessionContext-2" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_parquet" class="fn">read_parquet</a>\<P: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>\>( &self, table_paths: P, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Available on **crate feature `parquet`** only.

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a Parquet data source.

For more control such as reading multiple files, you can use [`read_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

For an example, see [`read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#note-statistics" class="doc-anchor">§</a>Note: Statistics

NOTE: by default, statistics are collected when reading the Parquet files This can slow down the initial DataFrame creation while greatly accelerating queries with certain filters.

To disable statistics collection, set the [config option](https://datafusion.apache.org/user-guide/configs.html) `datafusion.execution.collect_statistics` to `false`. See [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") and [`ExecutionOptions::collect_statistics`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.collect_statistics "field datafusion::config::ExecutionOptions::collect_statistics") for more details.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_parquet" class="fn">register_parquet</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Available on **crate feature `parquet`** only.

Registers a Parquet file as a table that can be referenced from SQL statements executed against this context.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#note-statistics-1" class="doc-anchor">§</a>Note: Statistics

Statistics are not collected by default. See [`read_parquet`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_parquet "method datafusion::execution::context::SessionContext::read_parquet") for more details and how to enable them.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.write_parquet" class="fn">write_parquet</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, writer_properties: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/properties/struct.WriterProperties.html" class="struct" title="struct parquet::file::properties::WriterProperties">WriterProperties</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Available on **crate feature `parquet`** only.

Executes a query and writes the results to a partitioned Parquet file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-SessionContext-3" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_avro" class="fn">read_avro</a>\<P: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>\>( &self, table_paths: P, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.AvroReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::AvroReadOptions">AvroReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Available on **crate feature `avro`** only.

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading an Avro data source.

For more control such as reading multiple files, you can use [`read_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`super::ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

For an example, see [`read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv")

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_avro" class="fn">register_avro</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.AvroReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::AvroReadOptions">AvroReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Available on **crate feature `avro`** only.

Registers an Avro file as a table that can be referenced from SQL statements executed against this context.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-SessionContext-4" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.new" class="fn">new</a>() -\> Self

Creates a new `SessionContext` using the default [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig").

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.refresh_catalogs" class="fn">refresh_catalogs</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Finds any [`ListingSchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html "struct datafusion::catalog::listing_schema::ListingSchemaProvider")s and instructs them to reload tables from “disk”

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.new_with_config" class="fn">new_with_config</a>(config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>) -\> Self

Creates a new `SessionContext` using the provided [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") and a new [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv").

See [`Self::new_with_config_rt`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.new_with_config_rt "associated function datafusion::execution::context::SessionContext::new_with_config_rt") for more details on resource limits.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.new_with_config_rt" class="fn">new_with_config_rt</a>( config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, runtime: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, ) -\> Self

Creates a new `SessionContext` using the provided [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") and a [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#resource-limits" class="doc-anchor">§</a>Resource Limits

By default, each new `SessionContext` creates a new `RuntimeEnv`, and therefore will not enforce memory or disk limits for queries run on different `SessionContext`s.

To enforce resource limits (e.g. to limit the total amount of memory used) across all DataFusion queries in a process, all `SessionContext`’s should be configured with the same `RuntimeEnv`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.new_with_state" class="fn">new_with_state</a>(state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Creates a new `SessionContext` using the provided [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.enable_url_table" class="fn">enable_url_table</a>(self) -\> Self

Enable querying local files as tables.

This feature is security sensitive and should only be enabled for systems that wish to permit direct access to the file system from SQL.

When enabled, this feature permits direct access to arbitrary files via SQL like

``` sql
SELECT * from 'my_file.parquet'
```

See [DynamicFileCatalog](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html "struct datafusion::catalog::DynamicFileCatalog") for more details

``` rust
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

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.into_state_builder" class="fn">into_state_builder</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

Convert the current `SessionContext` into a [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder")

This is useful to switch back to `SessionState` with custom settings such as [`Self::enable_url_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.enable_url_table "method datafusion::execution::context::SessionContext::enable_url_table").

Avoids cloning the SessionState if possible.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example" class="doc-anchor">§</a>Example

``` rust
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

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.session_start_time" class="fn">session_start_time</a>(&self) -\> <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>

Returns the time this `SessionContext` was created

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.with_function_factory" class="fn">with_function_factory</a>( self, function_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>\>, ) -\> Self

Registers a [`FunctionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html "trait datafusion::execution::context::FunctionFactory") to handle `CREATE FUNCTION` statements

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.add_optimizer_rule" class="fn">add_optimizer_rule</a>( &self, optimizer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, )

Adds an optimizer rule to the end of the existing rules.

See [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") for more control of when the rule is applied.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.add_analyzer_rule" class="fn">add_analyzer_rule</a>( &self, analyzer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, )

Adds an analyzer rule to the end of the existing rules.

See [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") for more control of when the rule is applied.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_object_store" class="fn">register_object_store</a>( &self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

Registers an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") to be used with a specific URL prefix.

See [`RuntimeEnv::register_object_store`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.register_object_store "method datafusion::execution::runtime_env::RuntimeEnv::register_object_store") for more details.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-register-a-local-object-store-for-the-file-url-prefix" class="doc-anchor">§</a>Example: register a local object store for the “file://” URL prefix

``` rust
let object_store_url = ObjectStoreUrl::parse("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
let ctx = SessionContext::new();
// All files with the file:// url prefix will be read from the local file system
ctx.register_object_store(object_store_url.as_ref(), Arc::new(object_store));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_batch" class="fn">register_batch</a>( &self, table_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>\>

Registers the [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") as the specified table name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.runtime_env" class="fn">runtime_env</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>

Return the [RuntimeEnv](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") used to run queries with this `SessionContext`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.session_id" class="fn">session_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns an id that uniquely identifies this `SessionContext`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.table_factory" class="fn">table_factory</a>( &self, file_type: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>

Return the [`TableProviderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory") that is registered for the specified file type, if any.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.enable_ident_normalization" class="fn">enable_ident_normalization</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return the `enable_ident_normalization` of this Session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.copied_config" class="fn">copied_config</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Return a copied version of config for this Session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.copied_table_options" class="fn">copied_table_options</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Return a copied version of table options for this Session

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql" class="fn">sql</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from SQL query text.

Note: This API implements DDL statements such as `CREATE TABLE` and `CREATE VIEW` and DML statements such as `INSERT INTO` with in-memory default implementations. See [`Self::sql_with_options`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql_with_options "method datafusion::execution::context::SessionContext::sql_with_options").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-running-sql-queries" class="doc-anchor">§</a>Example: Running SQL queries

See the example on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-creating-a-table-with-sql" class="doc-anchor">§</a>Example: Creating a Table with SQL

``` rust
use datafusion::prelude::*;
let ctx = SessionContext::new();
ctx
  .sql("CREATE TABLE foo (x INTEGER)")
  .await?
  .collect()
  .await?;
assert!(ctx.table_exist("foo").unwrap());
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql_with_options" class="fn">sql_with_options</a>( &self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") from SQL query text, first validating that the queries are allowed by `options`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-preventing-creating-a-table-with-sql" class="doc-anchor">§</a>Example: Preventing Creating a Table with SQL

If you want to avoid creating tables, or modifying data or the session, set [`SQLOptions`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html "struct datafusion::execution::context::SQLOptions") appropriately:

``` rust
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

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.parse_sql_expr" class="fn">parse_sql_expr</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Creates logical expressions from SQL query text.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-parsing-sql-queries" class="doc-anchor">§</a>Example: Parsing SQL queries

``` rust
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

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.execute_logical_plan" class="fn">execute_logical_plan</a>(&self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Execute the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan"), return a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame"). This API is not featured limited (so all SQL such as `CREATE TABLE` and `COPY` will be run).

If you wish to limit the type of plan that can be run from SQL, see [`Self::sql_with_options`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql_with_options "method datafusion::execution::context::SessionContext::sql_with_options") and [`SQLOptions::verify_plan`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.verify_plan "method datafusion::execution::context::SQLOptions::verify_plan").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Create a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion and function rewrites.

Note: The expression is not [simplified](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html "mod datafusion::optimizer::simplify_expressions") or otherwise optimized: `a = 1 + 2` will not be simplified to `a = 3` as this is a more involved process. See the [expr_api](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example for how to simplify expressions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#example-1" class="doc-anchor">§</a>Example

``` rust
// a = 1 (i64)
let expr = col("a").eq(lit(1i64));
// provide type information that `a` is an Int32
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// Create a PhysicalExpr. Note DataFusion automatically coerces (casts) `1i64` to `1i32`
let physical_expr = SessionContext::new()
  .create_physical_expr(expr, &df_schema).unwrap();
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#see-also" class="doc-anchor">§</a>See Also

- [`SessionState::create_physical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_physical_expr "method datafusion::execution::session_state::SessionState::create_physical_expr") for a lower level API

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.parse_memory_limit" class="fn">parse_memory_limit</a>(limit: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Parse memory limit from string to number of bytes Supports formats like ‘1.5G’, ‘100M’, ‘512K’

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#examples" class="doc-anchor">§</a>Examples

``` rust
use datafusion::execution::context::SessionContext;

assert_eq!(SessionContext::parse_memory_limit("1M").unwrap(), 1024 * 1024);
assert_eq!(SessionContext::parse_memory_limit("1.5G").unwrap(), (1.5 * 1024.0 * 1024.0 * 1024.0) as usize);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_variable" class="fn">register_variable</a>( &self, variable_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/enum.VarType.html" class="enum" title="enum datafusion::variable::VarType">VarType</a>, provider: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, )

Registers a variable provider within this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udtf" class="fn">register_udtf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, fun: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>\>)

Register a table UDF with this context

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udf" class="fn">register_udf</a>(&self, f: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>)

Registers a scalar UDF within this context.

Note in SQL queries, function names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_FUNC(x)...` will look for a function named `"my_func"`
- `SELECT "my_FUNC"(x)` will look for a function named `"my_FUNC"`

Any functions registered with the udf name or its aliases will be overwritten with this new function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udaf" class="fn">register_udaf</a>(&self, f: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>)

Registers an aggregate UDF within this context.

Note in SQL queries, aggregate names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_UDAF(x)...` will look for an aggregate named `"my_udaf"`
- `SELECT "my_UDAF"(x)` will look for an aggregate named `"my_UDAF"`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udwf" class="fn">register_udwf</a>(&self, f: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>)

Registers a window UDF within this context.

Note in SQL queries, window function names are looked up using lowercase unless the query uses quotes. For example,

- `SELECT MY_UDWF(x)...` will look for a window function named `"my_udwf"`
- `SELECT "my_UDWF"(x)` will look for a window function named `"my_UDWF"`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udf" class="fn">deregister_udf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Deregisters a UDF within this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udaf" class="fn">deregister_udaf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Deregisters a UDAF within this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udwf" class="fn">deregister_udwf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Deregisters a UDWF within this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udtf" class="fn">deregister_udtf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Deregisters a UDTF within this context.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_arrow" class="fn">read_arrow</a>\<P: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>\>( &self, table_paths: P, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading an Arrow data source.

For more control such as reading multiple files, you can use [`read_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table "method datafusion::execution::context::SessionContext::read_table") with a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

For an example, see [`read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_empty" class="fn">read_empty</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates an empty DataFrame.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_table" class="fn">read_table</a>(&self, provider: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") such as a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") or a custom user defined provider.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_batch" class="fn">read_batch</a>(&self, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Creates a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_batches" class="fn">read_batches</a>( &self, batches: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Create a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") for reading a \[`Vec[`RecordBatch`]`\]

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_listing_table" class="fn">register_listing_table</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>, provided_schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>, sql_definition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") that can assemble multiple files from locations in an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance into a single table.

This method is `async` because it might need to resolve the schema.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_arrow" class="fn">register_arrow</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, table_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'\_\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers an Arrow file as a table that can be referenced from SQL statements executed against this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_catalog" class="fn">register_catalog</a>( &self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, catalog: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Registers a named catalog using a custom `CatalogProvider` so that it can be referenced from SQL statements executed against this context.

Returns the [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") previously registered for this name, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.catalog_names" class="fn">catalog_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available catalog names.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.catalog" class="fn">catalog</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Retrieves a [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") instance by name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_table" class="fn">register_table</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, provider: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>\>

Registers a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") as a table that can be referenced from SQL statements executed against this context.

If a table of the same name was already registered, returns “Table already exists” error.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_table" class="fn">deregister_table</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>\>

Deregisters the given table.

Returns the registered provider, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.table_exist" class="fn">table_exist</a>(&self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Return `true` if the specified table exists in the schema provider.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.table" class="fn">table</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Retrieves a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") representing a table previously registered by calling the [`register_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_table "method datafusion::execution::context::SessionContext::register_table") function.

Returns an error if no table has been registered with the provided reference.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.table_function" class="fn">table_function</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>

Retrieves a [`TableFunction`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html "struct datafusion::catalog::TableFunction") reference by name.

Returns an error if no table function has been registered with the provided name.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.table_provider" class="fn">table_provider</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>

Return a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") for the specified table.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.task_ctx" class="fn">task_ctx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>

Get a new TaskContext to run in this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.state" class="fn">state</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

Return a new [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") suitable for executing a single query.

Notes:

1.  `query_execution_start_time` is set to the current time for the returned state.

2.  The returned state is not shared with the current session state and this changes to the returned `SessionState` such as changing [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") will not be reflected in this `SessionContext`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.state_ref" class="fn">state_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/parking_lot/0.12.4/x86_64-unknown-linux-gnu/parking_lot/rwlock/type.RwLock.html" class="type" title="type parking_lot::rwlock::RwLock">RwLock</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#" class="tooltip" data-notable-ty="Arc&lt;RwLock&lt;SessionState&gt;&gt;">ⓘ</a>

Get reference to [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.state_weak_ref" class="fn">state_weak_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html" class="struct" title="struct alloc::sync::Weak">Weak</a>\<<a href="https://docs.rs/parking_lot/0.12.4/x86_64-unknown-linux-gnu/parking_lot/rwlock/type.RwLock.html" class="type" title="type parking_lot::rwlock::RwLock">RwLock</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\>\>

Get weak reference to [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_catalog_list" class="fn">register_catalog_list</a>(&self, catalog_list: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>\>)

Register [`CatalogProviderList`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList") in [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_table_options_extension" class="fn">register_table_options_extension</a>\<T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>\>(&self, extension: T)

Registers a [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") as a table option extension that can be referenced from SQL statements executed against this context.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-Clone-for-SessionContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-Default-for-SessionContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-From%3C%26SessionContext%3E-for-TaskContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

Create a new task context instance from SessionContext

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(session: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-From%3CSessionContext%3E-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(session: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-From%3CSessionState%3E-for-SessionContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#impl-FunctionRegistry-for-SessionContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udfs" class="fn">udfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available scalar user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udf" class="fn">udf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Returns a reference to the user defined scalar function (udf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udaf" class="fn">udaf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Returns a reference to the user defined aggregate function (udaf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udwf" class="fn">udwf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Returns a reference to the user defined window function (udwf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf" class="fn">register_udf</a>( &mut self, udf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>\>

Registers a new [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udaf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf" class="fn">register_udaf</a>( &mut self, udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>\>

Registers a new [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_udwf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf" class="fn">register_udwf</a>( &mut self, udwf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>\>

Registers a new [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_function_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite" class="fn">register_function_rewrite</a>( &mut self, rewrite: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a new [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite") with the registry. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.expr_planners" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.expr_planners" class="fn">expr_planners</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>

Set of all registered [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_expr_planner" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_expr_planner" class="fn">register_expr_planner</a>( &mut self, expr_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a new [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") with the registry.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udafs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udafs" class="fn">udafs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available aggregate user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.udwfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udwfs" class="fn">udwfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available window user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf" class="fn">deregister_udf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udaf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf" class="fn">deregister_udaf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.deregister_udwf-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf" class="fn">deregister_udwf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#blanket-implementations" class="anchor">§</a>
