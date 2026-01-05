# Struct CteWorkTable Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/cte_worktable.rs.html#39" class="src">Source</a>

``` rust
pub struct CteWorkTable { /* private fields */ }
```

Expand description

The temporary working table where the previous iteration of a recursive query is stored Naming is based on PostgreSQL’s implementation. See here for more details: www.postgresql.org/docs/11/queries-with.html#id-1.5.6.12.5.4

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#impl-CteWorkTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.new" class="fn">new</a>(name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>

construct a new CteWorkTable with the given name and schema This schema must match the schema of the recursive term of the query Since the scan method will contain an physical plan that assumes this schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

The user-provided name of the CTE

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

The schema of the recursive term of the query

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#impl-Debug-for-CteWorkTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#impl-TableProvider-for-CteWorkTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the table provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.get_logical_plan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") of this table, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.schema-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.scan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan" class="fn">scan</a>\<'life0, 'life1, 'life2, 'life3, 'async_trait\>( &'life0 self, \_state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, \_projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life2 <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, \_filters: &'life3 \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html" class="struct" title="struct datafusion::datasource::cte_worktable::CteWorkTable">CteWorkTable</a>: 'async_trait,

Create an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for scanning the table with optionally specified `projection`, `filter` and `limit`, described below. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.supports_filters_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filters: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Specify if DataFusion should provide filter expressions to the TableProvider to apply *during* the scan. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get a reference to the constraints of the table. Returns: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.constraints)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.get_table_definition" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_table_definition" class="fn">get_table_definition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the create statement used to create this table, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.get_column_default" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_column_default" class="fn">get_column_default</a>(&self, \_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>

Get statistics for this table, if available Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules to perform operations such as re-ordering of joins.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#method.insert_into" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into" class="fn">insert_into</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, \_state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, \_input: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, \_insert_op: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp">InsertOp</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Return an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to insert data into this table, if supported. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/struct.CteWorkTable.html#blanket-implementations" class="anchor">§</a>
