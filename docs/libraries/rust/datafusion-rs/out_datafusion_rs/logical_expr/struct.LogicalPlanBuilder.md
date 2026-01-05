# Struct LogicalPlanBuilder Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#125" class="src">Source</a>

``` rust
pub struct LogicalPlanBuilder { /* private fields */ }
```

Expand description

Builder for logical plans

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#example-building-a-simple-plan" class="doc-anchor">§</a>Example building a simple plan

``` rust
// Create a plan similar to
// SELECT last_name
// FROM employees
// WHERE salary < 1000
let plan = table_scan(Some("employee"), &employee_schema(), None)?
 // Keep only rows where salary < 1000
 .filter(col("salary").lt(lit(1000)))?
 // only show "last_name" in the final results
 .project(vec![col("last_name")])?
 .build()?;

// Convert from plan back to builder
let builder = LogicalPlanBuilder::from(plan);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#impl-LogicalPlanBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.new" class="fn">new</a>(plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Create a builder from an existing plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.new_from_arc" class="fn">new_from_arc</a>(plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Create a builder from an existing plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.with_options" class="fn">with_options</a>( self, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilderOptions.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilderOptions">LogicalPlanBuilderOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>

Return the output schema of the plan build so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.plan" class="fn">plan</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

Return the LogicalPlan of the plan build so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.empty" class="fn">empty</a>(produce_one_row: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Create an empty relation.

`produce_one_row` set to true means this empty node needs to produce a placeholder row.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.to_recursive_query" class="fn">to_recursive_query</a>( self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, recursive_term: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, is_distinct: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a regular plan into a recursive query. `is_distinct` indicates whether the recursive term should be de-duplicated (`UNION`) after each iteration or not (`UNION ALL`).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.values" class="fn">values</a>( values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a values list based relation, and the schema is inferred from data, consuming `value`. See the [Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html) documentation for more details.

so it’s usually better to override the default names with a table alias list.

If the values include params/binders such as \$1, \$2, \$3, etc, then the `param_data_types` should be provided.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.values_with_schema" class="fn">values_with_schema</a>( values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a values list based relation, and the schema is inferred from data itself or table schema if provided, consuming `value`. See the [Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html) documentation for more details.

By default, it assigns the names column1, column2, etc. to the columns of a VALUES table. The column names are not specified by the SQL standard and different database systems do it differently, so it’s usually better to override the default names with a table alias list.

If the values include params/binders such as \$1, \$2, \$3, etc, then the `param_data_types` should be provided.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.scan" class="fn">scan</a>( table_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_source: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a table provider into a builder with a TableScan

Note that if you pass a string as `table_name`, it is treated as a SQL identifier, as described on [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") and thus is normalized

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#example" class="doc-anchor">§</a>Example:

``` rust
// Scan table_source with the name "mytable" (after normalization)
let scan = LogicalPlanBuilder::scan("MyTable", table, None);

// Scan table_source with the name "MyTable" by enclosing in quotes
let scan = LogicalPlanBuilder::scan(r#""MyTable""#, table, None);

// Scan table_source with the name "MyTable" by forming the table reference
let table_reference = TableReference::bare("MyTable");
let scan = LogicalPlanBuilder::scan(table_reference, table, None);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.copy_to" class="fn">copy_to</a>( input: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, output_url: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, file_type: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html" class="trait" title="trait datafusion::common::file_options::file_type::FileType">FileType</a>\>, options: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, partition_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a [CopyTo](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/struct.CopyTo.html "struct datafusion::logical_expr::logical_plan::dml::CopyTo") for copying the contents of this builder to the specified file(s)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.insert_into" class="fn">insert_into</a>( input: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, table_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, target: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, insert_op: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp">InsertOp</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a [`DmlStatement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DmlStatement.html "struct datafusion::logical_expr::DmlStatement") for inserting the contents of this builder into the named table.

Note, use a [`DefaultTableSource`](https://docs.rs/datafusion/latest/datafusion/datasource/default_table_source/struct.DefaultTableSource.html) to insert into a [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html)

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#example-1" class="doc-anchor">§</a>Example:

``` rust
// VALUES (1), (2)
let input = LogicalPlanBuilder::values(vec![vec![lit(1)], vec![lit(2)]])?
  .build()?;
// INSERT INTO MyTable VALUES (1), (2)
let insert_plan = LogicalPlanBuilder::insert_into(
  input,
  "MyTable",
  table_source,
  InsertOp::Append,
)?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.scan_with_filters" class="fn">scan_with_filters</a>( table_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_source: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a table provider into a builder with a TableScan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.scan_with_filters_fetch" class="fn">scan_with_filters_fetch</a>( table_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, table_source: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a table provider into a builder with a TableScan with filter and fetch

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.window_plan" class="fn">window_plan</a>( input: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, window_exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Wrap a plan in a window

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.project" class="fn">project</a>( self, expr: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a projection without alias.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.project_with_validation" class="fn">project_with_validation</a>( self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a projection without alias with optional validation (true to validate, false to not validate)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.select" class="fn">select</a>( self, indices: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Select the given column indices

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.filter" class="fn">filter</a>( self, expr: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a filter

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.having" class="fn">having</a>( self, expr: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a filter which is used for a having clause

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.prepare" class="fn">prepare</a>( self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, data_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Make a builder for a prepare logical plan from the builder’s plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.limit" class="fn">limit</a>( self, skip: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Limit the number of rows returned

`skip` - Number of rows to skip before fetch any row.

`fetch` - Maximum number of rows to fetch, after skipping `skip` rows, if specified.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.limit_by_expr" class="fn">limit_by_expr</a>( self, skip: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Limit the number of rows returned

Similar to `limit` but uses expressions for `skip` and `fetch`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.alias" class="fn">alias</a>( self, alias: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply an alias

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.sort_by" class="fn">sort_by</a>( self, expr: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a sort by provided expressions with default direction

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.sort" class="fn">sort</a>( self, sorts: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.sort_with_limit" class="fn">sort_with_limit</a>( self, sorts: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a sort

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.union" class="fn">union</a>( self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a union, preserving duplicate rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.union_by_name" class="fn">union_by_name</a>( self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a union by name, preserving duplicate rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.union_by_name_distinct" class="fn">union_by_name_distinct</a>( self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a union by name, removing duplicate rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.union_distinct" class="fn">union_distinct</a>( self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a union, removing duplicate rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.distinct" class="fn">distinct</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply deduplication: Only distinct (different) values are returned)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.distinct_on" class="fn">distinct_on</a>( self, on_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, select_expr: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, sort_expr: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Project first values of the specified expression list according to the provided sorting expressions grouped by the `DISTINCT ON` clause expressions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join" class="fn">join</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, join_keys: (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>\>), filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a join to `right` using explicitly specified columns and an optional filter expression.

See [`join_on`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_on "method datafusion::logical_expr::LogicalPlanBuilder::join_on") for a more concise way to specify the join condition. Since DataFusion will automatically identify and optimize equality predicates there is no performance difference between this function and `join_on`

`left_cols` and `right_cols` are used to form “equijoin” predicates (see example below), which are then combined with the optional `filter` expression.

Note that in case of outer join, the `filter` is applied to only matched rows.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_on" class="fn">join_on</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, on_exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a join using the specified expressions.

Note that DataFusion automatically optimizes joins, including identifying and optimizing equality predicates.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#example-2" class="doc-anchor">§</a>Example

``` rust
let example_schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Int32, false),
    Field::new("c", DataType::Int32, false),
]));
let table_source = Arc::new(LogicalTableSource::new(example_schema));
let left_table = table_source.clone();
let right_table = table_source.clone();

let right_plan = LogicalPlanBuilder::scan("right", right_table, None)?.build()?;

// Form the expression `(left.a != right.a)` AND `(left.b != right.b)`
let exprs = vec![
    col("left.a").eq(col("right.a")),
    col("left.b").not_eq(col("right.b"))
 ];

// Perform the equivalent of `left INNER JOIN right ON (a != a2 AND b != b2)`
// finding all pairs of rows from `left` and `right` where
// where `a = a2` and `b != b2`.
let plan = LogicalPlanBuilder::scan("left", left_table, None)?
    .join_on(right_plan, JoinType::Inner, exprs)?
    .build()?;
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_detailed" class="fn">join_detailed</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, join_keys: (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>\>), filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, null_equality: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a join with on constraint and specified null equality.

The behavior is the same as [`join`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join "method datafusion::logical_expr::LogicalPlanBuilder::join") except that it allows specifying the null equality behavior.

The `null_equality` dictates how `null` values are joined.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_using" class="fn">join_using</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, using_keys: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a join with using constraint, which duplicates all join columns in output schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.cross_join" class="fn">cross_join</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a cross join

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.repartition" class="fn">repartition</a>( self, partitioning_scheme: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Partitioning.html" class="enum" title="enum datafusion::prelude::Partitioning">Partitioning</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Repartition

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.window" class="fn">window</a>( self, window_expr: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a window functions to extend the schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.aggregate" class="fn">aggregate</a>( self, group_expr: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, aggr_expr: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply an aggregate: grouping on the `group_expr` expressions and calculating `aggr_expr` aggregates for each distinct value of the `group_expr`;

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.explain" class="fn">explain</a>( self, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, analyze: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an expression to represent the explanation of the plan

if `analyze` is true, runs the actual plan and produces information about metrics during run.

if `verbose` is true, prints out additional details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.explain_option_format" class="fn">explain_option_format</a>( self, explain_option: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExplainOption.html" class="struct" title="struct datafusion::logical_expr::ExplainOption">ExplainOption</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an expression to represent the explanation of the plan The`explain_option` is used to specify the format and verbosity of the explanation. Details see [`ExplainOption`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExplainOption.html "struct datafusion::logical_expr::ExplainOption").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.intersect" class="fn">intersect</a>( left_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, right_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, is_all: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Process intersect set operator

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.except" class="fn">except</a>( left_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, right_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, is_all: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Process except set operator

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Build the plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_with_expr_keys" class="fn">join_with_expr_keys</a>( self, right: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, equi_exprs: (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>), filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply a join with both explicit equijoin and non equijoin predicates.

Note this is a low level API that requires identifying specific predicate types. Most users should use [`join_on`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.join_on "method datafusion::logical_expr::LogicalPlanBuilder::join_on") that automatically identifies predicates appropriately.

`equi_exprs` defines equijoin predicates, of the form `l = r)` for each `(l, r)` tuple. `l`, the first element of the tuple, must only refer to columns from the existing input. `r`, the second element of the tuple, must only refer to columns from the right input.

`filter` contains any other filter expression to apply during the join. Note that `equi_exprs` predicates are evaluated more efficiently than the filter expressions, so they are preferred.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.unnest_column" class="fn">unnest_column</a>( self, column: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Unnest the given column.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.unnest_column_with_options" class="fn">unnest_column_with_options</a>( self, column: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Unnest the given column given [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.unnest_columns_with_options" class="fn">unnest_columns_with_options</a>( self, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Unnest the given columns with the given [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#impl-Clone-for-LogicalPlanBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#impl-Debug-for-LogicalPlanBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#impl-From%3CArc%3CLogicalPlan%3E%3E-for-LogicalPlanBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#impl-From%3CLogicalPlan%3E-for-LogicalPlanBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::LogicalPlanBuilder">LogicalPlanBuilder</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html#blanket-implementations" class="anchor">§</a>
