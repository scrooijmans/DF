# Struct DataFrame Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/dataframe/mod.rs.html#213-233" class="src">Source</a>

``` rust
pub struct DataFrame { /* private fields */ }
```

Expand description

Represents a logical set of rows with the same named columns.

Similar to a [Pandas DataFrame](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html) or [Spark DataFrame](https://spark.apache.org/docs/latest/sql-programming-guide.html), a DataFusion DataFrame represents a 2 dimensional table of rows and columns.

The typical workflow using DataFrames looks like

1.  Create a DataFrame via methods on [SessionContext](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext"), such as [`read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv") and [`read_parquet`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_parquet "method datafusion::execution::context::SessionContext::read_parquet").

2.  Build a desired calculation by calling methods such as [`filter`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.filter "method datafusion::dataframe::DataFrame::filter"), [`select`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.select "method datafusion::dataframe::DataFrame::select"), [`aggregate`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.aggregate "method datafusion::dataframe::DataFrame::aggregate"), and [`limit`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.limit "method datafusion::dataframe::DataFrame::limit")

3.  Execute into [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es by calling [`collect`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.collect "method datafusion::dataframe::DataFrame::collect")

A `DataFrame` is a wrapper around a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") and the [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") required for execution.

DataFrames are “lazy” in the sense that most methods do not actually compute anything, they just build up a plan. Calling [`collect`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.collect "method datafusion::dataframe::DataFrame::collect") executes the plan using the same DataFusion planning and execution process used to execute SQL and other queries.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
// Read the data from a csv file
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// create a new dataframe that computes the equivalent of
// `SELECT a, MIN(b) FROM df WHERE a <= b GROUP BY a LIMIT 100;`
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;
// Perform the actual computation
let results = df.collect();

// Create a new dataframe with in-memory data
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, true),
    Field::new("name", DataType::Utf8, true),
]);
let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![
        Arc::new(Int32Array::from(vec![1, 2, 3])),
        Arc::new(StringArray::from(vec!["foo", "bar", "baz"])),
    ],
)?;
let df = ctx.read_batch(batch)?;
df.show().await?;

// Create a new dataframe with in-memory data using macro
let df = dataframe!(
    "id" => [1, 2, 3],
    "name" => ["foo", "bar", "baz"]
 )?;
df.show().await?;
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#impl-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.write_parquet" class="fn">write_parquet</a>( self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>, writer_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Available on **crate feature `parquet`** only.

Execute the `DataFrame` and write the results to Parquet file(s).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-1" class="doc-anchor">§</a>Example

``` rust
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?
  .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
  .write_parquet(
    "output.parquet",
    DataFrameWriteOptions::new(),
    None, // can also specify parquet writing options here
).await?;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#impl-DataFrame-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.new" class="fn">new</a>(session_state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> Self

Create a new `DataFrame ` based on an existing `LogicalPlan`

This is a low-level method and is not typically used by end users. See [`SessionContext::read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv") and other methods for creating a `DataFrame` from an existing datasource.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.parse_sql_expr" class="fn">parse_sql_expr</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Creates logical expression from a SQL query text. The expression is created and processed against the current schema.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-parsing-sql-queries" class="doc-anchor">§</a>Example: Parsing SQL queries

``` rust
// datafusion will parse number as i64 first.
let sql = "a > 1 and b in (1, 10)";
let expected = col("a").gt(lit(1 as i64))
  .and(col("b").in_list(vec![lit(1 as i64), lit(10 as i64)], false));
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let expr = df.parse_sql_expr(sql)?;
assert_eq!(expected, expr);
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.create_physical_plan" class="fn">create_physical_plan</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Consume the DataFrame and produce a physical plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.select_columns" class="fn">select_columns</a>(self, columns: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Filter the DataFrame by column. Returns a new DataFrame only containing the specified columns.

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.select_columns(&["a", "b"])?;
let expected = vec![
    "+---+---+",
    "| a | b |",
    "+---+---+",
    "| 1 | 2 |",
    "+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.select_exprs" class="fn">select_exprs</a>(self, exprs: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Project arbitrary list of expression strings into a new `DataFrame`. Method will parse string expressions into logical plan expressions.

The output `DataFrame` has one column for each element in `exprs`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-2" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df : DataFrame = df.select_exprs(&["a * b", "c"])?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.select" class="fn">select</a>( self, expr_list: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Project arbitrary expressions (like SQL SELECT expressions) into a new `DataFrame`.

The output `DataFrame` has one column for each element in `expr_list`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-3" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.select(vec![col("a"), col("b") * col("c")])?;
let expected = vec![
    "+---+-----------------------+",
    "| a | ?table?.b * ?table?.c |",
    "+---+-----------------------+",
    "| 1 | 6                     |",
    "+---+-----------------------+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.drop_columns" class="fn">drop_columns</a>(self, columns: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Returns a new DataFrame containing all columns except the specified columns.

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// +----+----+----+
// | a  | b  | c  |
// +----+----+----+
// | 1  | 2  | 3  |
// +----+----+----+
let df = df.drop_columns(&["a"])?;
let expected = vec![
    "+---+---+",
    "| b | c |",
    "+---+---+",
    "| 2 | 3 |",
    "+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.unnest_columns" class="fn">unnest_columns</a>(self, columns: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Expand multiple list/struct columns into a set of rows and new columns.

See also: [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions") documentation for the behavior of `unnest`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-4" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_json("tests/data/unnest.json", NdJsonReadOptions::default()).await?;
// expand into multiple columns if it's json array, flatten field name if it's nested structure
let df = df.unnest_columns(&["b","c","d"])?;
let expected = vec![
    "+---+------+-------+-----+-----+",
    "| a | b    | c     | d.e | d.f |",
    "+---+------+-------+-----+-----+",
    "| 1 | 2.0  | false | 1   | 2   |",
    "| 1 | 1.3  | true  | 1   | 2   |",
    "| 1 | -6.1 |       | 1   | 2   |",
    "| 2 | 3.0  | false |     |     |",
    "| 2 | 2.3  | true  |     |     |",
    "| 2 | -7.1 |       |     |     |",
    "+---+------+-------+-----+-----+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.unnest_columns_with_options" class="fn">unnest_columns_with_options</a>( self, columns: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Expand multiple list columns into a set of rows, with behavior controlled by [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions").

Please see the documentation on [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions") for more details about the meaning of unnest.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.filter" class="fn">filter</a>(self, predicate: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a DataFrame with only rows for which `predicate` evaluates to `true`.

Rows for which `predicate` evaluates to `false` or `null` are filtered out.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-5" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df = df.filter(col("a").lt_eq(col("b")))?;
// all rows where a <= b are returned
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.aggregate" class="fn">aggregate</a>( self, group_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, aggr_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a new `DataFrame` that aggregates the rows of the current `DataFrame`, first optionally grouping by the given expressions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-6" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;

// The following use is the equivalent of "SELECT MIN(b) GROUP BY a"
let df1 = df.clone().aggregate(vec![col("a")], vec![min(col("b"))])?;
let expected1 = vec![
    "+---+----------------+",
    "| a | min(?table?.b) |",
    "+---+----------------+",
    "| 1 | 2              |",
    "| 4 | 5              |",
    "| 7 | 8              |",
    "+---+----------------+"
];
assert_batches_sorted_eq!(expected1, &df1.collect().await?);
// The following use is the equivalent of "SELECT MIN(b)"
let df2 = df.aggregate(vec![], vec![min(col("b"))])?;
let expected2 = vec![
    "+----------------+",
    "| min(?table?.b) |",
    "+----------------+",
    "| 2              |",
    "+----------------+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.window" class="fn">window</a>(self, window_exprs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a new DataFrame that adds the result of evaluating one or more window functions ([`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")) to the existing columns

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.limit" class="fn">limit</a>(self, skip: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Returns a new `DataFrame` with a limited number of rows.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#arguments" class="doc-anchor">§</a>Arguments

`skip` - Number of rows to skip before fetch any row `fetch` - Maximum number of rows to return, after skipping `skip` rows.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-7" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df = df.limit(1, Some(2))?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.union" class="fn">union</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the union of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s, preserving duplicate rows.

The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-8" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?   ;
let d2 = df.clone();
let df = df.union(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.union_by_name" class="fn">union_by_name</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the union of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s using column names, preserving duplicate rows.

The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s are combined using column names rather than position, filling missing columns with null.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-9" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let d2 = df.clone().select_columns(&["b", "c", "a"])?.with_column("d", lit("77"))?;
let df = df.union_by_name(d2)?;
let expected = vec![
    "+---+---+---+----+",
    "| a | b | c | d  |",
    "+---+---+---+----+",
    "| 1 | 2 | 3 |    |",
    "| 1 | 2 | 3 | 77 |",
    "+---+---+---+----+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.union_distinct" class="fn">union_distinct</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the distinct union of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s.

The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema. Any duplicate rows are discarded.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-10" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let d2 = df.clone();
let df = df.union_distinct(d2)?;
// df2 are duplicate of df
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.union_by_name_distinct" class="fn">union_by_name_distinct</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the union of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s using column names with all duplicated rows removed.

The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s are combined using column names rather than position, filling missing columns with null.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-11" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let d2 = df.clone().select_columns(&["b", "c", "a"])?;
let df = df.union_by_name_distinct(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.distinct" class="fn">distinct</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a new `DataFrame` with all duplicated rows removed.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-12" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.distinct()?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.distinct_on" class="fn">distinct_on</a>( self, on_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, select_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, sort_expr: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a new `DataFrame` with duplicated rows removed as per the specified expression list according to the provided sorting expressions grouped by the `DISTINCT ON` clause expressions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-13" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?
  // Return a single row (a, b) for each distinct value of a
  .distinct_on(vec![col("a")], vec![col("a"), col("b")], None)?;
let expected = vec![
    "+---+---+",
    "| a | b |",
    "+---+---+",
    "| 1 | 2 |",
    "+---+---+"
];
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.describe" class="fn">describe</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Return a new `DataFrame` that has statistics for a DataFrame.

Only summarizes numeric datatypes at the moment and returns nulls for non numeric datatypes. The output format is modeled after pandas

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-14" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/tpch-csv/customer.csv", CsvReadOptions::new()).await?;
let stat = df.describe().await?;
let expected = vec![
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+",
    "| describe   | c_custkey          | c_name             | c_address                          | c_nationkey        | c_phone         | c_acctbal          | c_mktsegment | c_comment                                                                                                |",
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+",
    "| count      | 9.0                | 9                  | 9                                  | 9.0                | 9               | 9.0                | 9            | 9                                                                                                        |",
    "| max        | 10.0               | Customer#000000010 | xKiAFTjUsCuxfeleNqefumTrjS         | 20.0               | 30-114-968-4951 | 9561.95            | MACHINERY    | tions. even deposits boost according to the slyly bold packages. final accounts cajole requests. furious |",
    "| mean       | 6.0                | null               | null                               | 9.88888888888889   | null            | 5153.2155555555555 | null         | null                                                                                                     |",
    "| median     | 6.0                | null               | null                               | 8.0                | null            | 6819.74            | null         | null                                                                                                     |",
    "| min        | 2.0                | Customer#000000002 | 6LrEaV6KR6PLVcgl2ArL Q3rqzLzcT1 v2 | 1.0                | 11-719-748-3364 | 121.65             | AUTOMOBILE   |  deposits eat slyly ironic, even instructions. express foxes detect slyly. blithely even accounts abov   |",
    "| null_count | 0.0                | 0                  | 0                                  | 0.0                | 0               | 0.0                | 0            | 0                                                                                                        |",
    "| std        | 2.7386127875258306 | null               | null                               | 7.2188026092359046 | null            | 3522.169804254585  | null         | null                                                                                                     |",
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+"];
assert_batches_sorted_eq!(expected, &stat.collect().await?);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.sort_by" class="fn">sort_by</a>(self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Apply a sort by provided expressions with default direction

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.sort" class="fn">sort</a>(self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Sort the DataFrame by the specified sorting expressions.

Note that any expression can be turned into a sort expression by calling its [sort](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.sort "method datafusion::prelude::Expr::sort") method.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-15" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df = df.sort(vec![
  col("a").sort(false, true),   // a DESC, nulls first
  col("b").sort(true, false), // b ASC, nulls last
 ])?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.join" class="fn">join</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, left_cols: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], right_cols: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Join this `DataFrame` with another `DataFrame` using explicitly specified columns and an optional filter expression.

See [`join_on`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.join_on "method datafusion::dataframe::DataFrame::join_on") for a more concise way to specify the join condition. Since DataFusion will automatically identify and optimize equality predicates there is no performance difference between this function and `join_on`

`left_cols` and `right_cols` are used to form “equijoin” predicates (see example below), which are then combined with the optional `filter` expression. If `left_cols` and `right_cols` contain ambiguous column references, they will be disambiguated by prioritizing the left relation for `left_cols` and the right relation for `right_cols`.

Note that in case of outer join, the `filter` is applied to only matched rows.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-16" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let left = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let right = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?
  .select(vec![
    col("a").alias("a2"),
    col("b").alias("b2"),
    col("c").alias("c2")])?;
// Perform the equivalent of `left INNER JOIN right ON (a = a2 AND b = b2)`
// finding all pairs of rows from `left` and `right` where `a = a2` and `b = b2`.
let join = left.join(right, JoinType::Inner, &["a", "b"], &["a2", "b2"], None)?;
let expected = vec![
    "+---+---+---+----+----+----+",
    "| a | b | c | a2 | b2 | c2 |",
    "+---+---+---+----+----+----+",
    "| 1 | 2 | 3 | 1  | 2  | 3  |",
    "+---+---+---+----+----+----+"
];
assert_batches_sorted_eq!(expected, &join.collect().await?);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.join_on" class="fn">join_on</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, on_exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Join this `DataFrame` with another `DataFrame` using the specified expressions.

Note that DataFusion automatically optimizes joins, including identifying and optimizing equality predicates.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-17" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let left = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let right = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .select(vec![
        col("a").alias("a2"),
        col("b").alias("b2"),
        col("c").alias("c2"),
    ])?;

// Perform the equivalent of `left INNER JOIN right ON (a != a2 AND b != b2)`
// finding all pairs of rows from `left` and `right` where
// where `a != a2` and `b != b2`.
let join_on = left.join_on(
    right,
    JoinType::Inner,
    [col("a").not_eq(col("a2")), col("b").not_eq(col("b2"))],
)?;
let expected = vec![
    "+---+---+---+----+----+----+",
    "| a | b | c | a2 | b2 | c2 |",
    "+---+---+---+----+----+----+",
    "+---+---+---+----+----+----+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.repartition" class="fn">repartition</a>(self, partitioning_scheme: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Partitioning.html" class="enum" title="enum datafusion::prelude::Partitioning">Partitioning</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Repartition a DataFrame based on a logical partitioning scheme.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-18" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df1 = df.repartition(Partitioning::RoundRobinBatch(4))?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+"
];
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.count" class="fn">count</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Return the total number of rows in this `DataFrame`.

Note that this method will actually run a plan to calculate the count, which may be slow for large or complicated DataFrames.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-19" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let count = df.count().await?; // 1
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.collect" class="fn">collect</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>\>

Execute this `DataFrame` and buffer all resulting `RecordBatch`es into memory.

Prior to calling `collect`, modifying a DataFrame simply updates a plan (no actual computation is performed). `collect` triggers the computation.

See [`Self::execute_stream`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.execute_stream "method datafusion::dataframe::DataFrame::execute_stream") to execute a DataFrame without buffering.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-20" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let batches = df.collect().await?;
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.show" class="fn">show</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Execute the `DataFrame` and print the results to the console.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-21" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
df.show().await?;
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.to_string" class="fn">to_string</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Execute the `DataFrame` and return a string representation of the results.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-22" class="doc-anchor">§</a>Example

``` rust

let cfg = SessionConfig::new()
    .set_str("datafusion.format.null", "no-value");
let session_state = SessionStateBuilder::new()
    .with_config(cfg)
    .with_default_features()
    .build();
let ctx = SessionContext::new_with_state(session_state);
let df = ctx.sql("select null as 'null-column'").await?;
let result = df.to_string().await?;
assert_eq!(result,
"+-------------+
| null-column |
+-------------+
| no-value    |
+-------------+"
);
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.show_limit" class="fn">show_limit</a>(self, num: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Execute the `DataFrame` and print only the first `num` rows of the result to the console.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-23" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
df.show_limit(10).await?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.task_ctx" class="fn">task_ctx</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

Return a new [`TaskContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html "struct datafusion::execution::TaskContext") which would be used to execute this DataFrame

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.execute_stream" class="fn">execute_stream</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html" class="type" title="type datafusion::execution::SendableRecordBatchStream">SendableRecordBatchStream</a>\>

Executes this DataFrame and returns a stream over a single partition

See [Self::collect](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.collect "method datafusion::dataframe::DataFrame::collect") to buffer the `RecordBatch`es in memory.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-24" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let stream = df.execute_stream().await?;
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#aborting-execution" class="doc-anchor">§</a>Aborting Execution

Dropping the stream will abort the execution of the query, and free up any allocated resources

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.collect_partitioned" class="fn">collect_partitioned</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>\>\>

Executes this DataFrame and collects all results into a vector of vector of RecordBatch maintaining the input partitioning.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-25" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let batches = df.collect_partitioned().await?;
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.execute_stream_partitioned" class="fn">execute_stream_partitioned</a>( self, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html" class="type" title="type datafusion::execution::SendableRecordBatchStream">SendableRecordBatchStream</a>\>\>

Executes this DataFrame and returns one stream per partition.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-26" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let batches = df.execute_stream_partitioned().await?;
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#aborting-execution-1" class="doc-anchor">§</a>Aborting Execution

Dropping the stream will abort the execution of the query, and free up any allocated resources

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Returns the `DFSchema` describing the output of this DataFrame.

The output `DFSchema` contains information on the name, data type, and nullability for each column.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-27" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let schema = df.schema();
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.logical_plan" class="fn">logical_plan</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

Return a reference to the unoptimized [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") that comprises this DataFrame.

See [`Self::into_unoptimized_plan`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_unoptimized_plan "method datafusion::dataframe::DataFrame::into_unoptimized_plan") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>)

Returns both the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") and [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") that comprise this [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_unoptimized_plan" class="fn">into_unoptimized_plan</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

Return the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") represented by this DataFrame without running any optimizers

Note: This method should not be used outside testing, as it loses the snapshot of the [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") attached to this [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") and consequently subsequent operations may take place against a different state (e.g. a different value of `now()`)

See [`Self::into_parts`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_parts "method datafusion::dataframe::DataFrame::into_parts") to retrieve the owned [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") and corresponding [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_optimized_plan" class="fn">into_optimized_plan</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Return the optimized [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") represented by this DataFrame.

Note: This method should not be used outside testing – see [`Self::into_unoptimized_plan`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_unoptimized_plan "method datafusion::dataframe::DataFrame::into_unoptimized_plan") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.into_view" class="fn">into_view</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>

Converts this [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") into a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") that can be registered as a table view using [`SessionContext::register_table`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.register_table "method datafusion::execution::context::SessionContext::register_table").

Note: This discards the [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") associated with this [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") in favour of the one passed to [`TableProvider::scan`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan "method datafusion::datasource::TableProvider::scan")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.explain" class="fn">explain</a>(self, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, analyze: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a DataFrame with the explanation of its plan so far.

if `analyze` is specified, runs the plan and reports metrics if `verbose` is true, prints out additional details. The default format is Indent format.

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let batches = df.limit(0, Some(100))?.explain(false, false)?.collect().await?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.explain_with_options" class="fn">explain_with_options</a>( self, explain_option: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExplainOption.html" class="struct" title="struct datafusion::logical_expr::ExplainOption">ExplainOption</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Return a DataFrame with the explanation of its plan so far.

`opt` is used to specify the options for the explain operation. Details of the options can be found in [`ExplainOption`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExplainOption.html "struct datafusion::logical_expr::ExplainOption").

``` rust
use datafusion_expr::{Explain, ExplainOption};
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let batches = df.limit(0, Some(100))?.explain_with_options(ExplainOption::default().with_verbose(false).with_analyze(false))?.collect().await?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.registry" class="fn">registry</a>(&self) -\> &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a>

Return a `FunctionRegistry` used to plan udf’s calls

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-28" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let f = df.registry();
// use f.udf("name", vec![...]) to use the udf
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.intersect" class="fn">intersect</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the intersection of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s. The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let d2 = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df = df.intersect(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.intersect_distinct" class="fn">intersect_distinct</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the distinct intersection of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s. The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let d2 = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let df = df.intersect_distinct(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.except" class="fn">except</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the exception of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s. The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let d2 = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let result = df.except(d2)?;
// those columns are not in example.csv, but in example_long.csv
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+"
];
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.except_distinct" class="fn">except_distinct</a>(self, dataframe: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Calculate the distinct exception of two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s. The two [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame")s must have exactly the same schema

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example_long.csv", CsvReadOptions::new()).await?;
let d2 = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let result = df.except_distinct(d2)?;
// those columns are not in example.csv, but in example_long.csv
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+"
];
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.write_table" class="fn">write_table</a>( self, table_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, write_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Execute this `DataFrame` and write the results to `table_name`.

Returns a single [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") containing a single column and row representing the count of total rows written.

Unlike most other `DataFrame` methods, this method executes eagerly. Data is written to the table using the [`TableProvider::insert_into`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into "method datafusion::datasource::TableProvider::insert_into") method. This is the same underlying implementation used by SQL `INSERT INTO` statements.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.write_csv" class="fn">write_csv</a>( self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>, writer_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Execute the `DataFrame` and write the results to CSV file(s).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-29" class="doc-anchor">§</a>Example

``` rust
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?
  .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
  .write_csv(
    "output.csv",
    DataFrameWriteOptions::new(),
    None, // can also specify CSV writing options here
).await?;
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.write_json" class="fn">write_json</a>( self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>, writer_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Execute the `DataFrame` and write the results to JSON file(s).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-30" class="doc-anchor">§</a>Example

``` rust
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?
  .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
  .write_json(
    "output.json",
    DataFrameWriteOptions::new(),
    None
).await?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.with_column" class="fn">with_column</a>(self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Add or replace a column in the DataFrame.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-31" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.with_column("ab_sum", col("a") + col("b"))?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.with_column_renamed" class="fn">with_column_renamed</a>( self, old_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, new_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Rename one column by applying a new projection. This is a no-op if the column to be renamed does not exist.

The method supports case sensitive rename with wrapping column name into one of following symbols ( “ or ’ or \` )

Alternatively setting DataFusion param `datafusion.sql_parser.enable_ident_normalization` to `false` will enable case sensitive rename without need to wrap column name into special symbols

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-32" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.with_column_renamed("ab_sum", "total")?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.with_param_values" class="fn">with_param_values</a>( self, query_values: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ParamValues.html" class="enum" title="enum datafusion::common::ParamValues">ParamValues</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Replace all parameters in logical plan with the specified values, in preparation for execution.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-33" class="doc-anchor">§</a>Example

``` rust
use datafusion::prelude::*;
let ctx = SessionContext::new();
let results = ctx
  .sql("SELECT a FROM example WHERE b = $1")
  .await?
   // replace $1 with value 2
  .with_param_values(vec![
     // value at index 0 --> $1
     ScalarValue::from(2i64)
   ])?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+",
   "| a |",
   "+---+",
   "| 1 |",
   "+---+",
 ],
 &results
);
// Note you can also provide named parameters
let results = ctx
  .sql("SELECT a FROM example WHERE b = $my_param")
  .await?
   // replace $my_param with value 2
   // Note you can also use a HashMap as well
  .with_param_values(vec![
      ("my_param", ScalarValue::from(2i64))
   ])?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+",
   "| a |",
   "+---+",
   "| 1 |",
   "+---+",
 ],
 &results
);
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.cache" class="fn">cache</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Cache DataFrame as a memory table.

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.cache().await?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.alias" class="fn">alias</a>(self, alias: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Apply an alias to the DataFrame.

This method replaces the qualifiers of output columns with the given alias.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.fill_null" class="fn">fill_null</a>( &self, value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>\>

Fill null values in specified columns with a given value If no columns are specified (empty vector), applies to all columns Only fills if the value can be cast to the column’s type

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#arguments-1" class="doc-anchor">§</a>Arguments

- `value` - Value to fill nulls with
- `columns` - List of column names to fill. If empty, fills all columns.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-34" class="doc-anchor">§</a>Example

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// Fill nulls in only columns "a" and "c":
let df = df.fill_null(ScalarValue::from(0), vec!["a".to_owned(), "c".to_owned()])?;
// Fill nulls across all columns:
let df = df.fill_null(ScalarValue::from(0), vec![])?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.from_columns" class="fn">from_columns</a>(columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html" class="type" title="type datafusion::common::arrow::array::ArrayRef">ArrayRef</a>)\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Helper for creating DataFrame.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#example-35" class="doc-anchor">§</a>Example

``` rust
use std::sync::Arc;
use arrow::array::{ArrayRef, Int32Array, StringArray};
use datafusion::prelude::DataFrame;
let id: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3]));
let name: ArrayRef = Arc::new(StringArray::from(vec!["foo", "bar", "baz"]));
let df = DataFrame::from_columns(vec![("id", id), ("name", name)]).unwrap();
// +----+------+,
// | id | name |,
// +----+------+,
// | 1  | foo  |,
// | 2  | bar  |,
// | 3  | baz  |,
// +----+------+,
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#impl-Clone-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#impl-Debug-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame">DataFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html#blanket-implementations" class="anchor">§</a>
