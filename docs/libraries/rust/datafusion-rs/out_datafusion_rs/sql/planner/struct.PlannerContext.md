# Struct PlannerContext Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/planner.rs.html#256" class="src">Source</a>

``` rust
pub struct PlannerContext { /* private fields */ }
```

Expand description

Struct to store the states used by the Planner. The Planner will leverage the states to resolve CTEs, Views, subqueries and PREPARE statements. The states include Common Table Expression (CTE) provided with WITH clause and Parameter Data Types provided with PREPARE statement and the query schema of the outer query plan.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#cloning" class="doc-anchor">§</a>Cloning

Only the `ctes` are truly cloned when the `PlannerContext` is cloned. This helps resolve scoping issues of CTEs. By using cloning, a subquery can inherit CTEs from the outer query and can also define its own private CTEs without affecting the outer query.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#impl-PlannerContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

Create an empty PlannerContext

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.with_prepare_param_data_types" class="fn">with_prepare_param_data_types</a>( self, prepare_param_data_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

Update the PlannerContext with provided prepare_param_data_types

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.outer_query_schema" class="fn">outer_query_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.set_outer_query_schema" class="fn">set_outer_query_schema</a>( &mut self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>

Sets the outer query schema, returning the existing one, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.set_table_schema" class="fn">set_table_schema</a>( &mut self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.table_schema" class="fn">table_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.outer_from_schema" class="fn">outer_from_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.set_outer_from_schema" class="fn">set_outer_from_schema</a>( &mut self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>\>

Sets the outer FROM schema, returning the existing one, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.extend_outer_from_schema" class="fn">extend_outer_from_schema</a>( &mut self, schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extends the FROM schema, returning the existing one, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.prepare_param_data_types" class="fn">prepare_param_data_types</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\]

Return the types of parameters (`$1`, `$2`, etc) if known

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.contains_cte" class="fn">contains_cte</a>(&self, cte_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there is a Common Table Expression (CTE) / Subquery for the specified name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.insert_cte" class="fn">insert_cte</a>(&mut self, cte_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>)

Inserts a LogicalPlan for the Common Table Expression (CTE) / Subquery for the specified name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.get_cte" class="fn">get_cte</a>(&self, cte_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Return a plan for the Common Table Expression (CTE) / Subquery for the specified name

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#impl-Clone-for-PlannerContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#impl-Debug-for-PlannerContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#impl-Default-for-PlannerContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html#blanket-implementations" class="anchor">§</a>
