# Module planner Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#60" class="src">Source</a>

Expand description

[`ContextProvider`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html "trait datafusion::logical_expr::planner::ContextProvider") and [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") APIs to customize SQL query planning

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawAggregateExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawAggregateExpr">RawAggregateExpr</a>  
This structure is used by `AggregateFunctionPlanner` to plan operators with custom expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawBinaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawBinaryExpr">RawBinaryExpr</a>  
An operator with two arguments to plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawDictionaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawDictionaryExpr">RawDictionaryExpr</a>  
A Dictionary literal expression `{ key: value, ...}`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>  
An expression with GetFieldAccess to plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawWindowExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawWindowExpr">RawWindowExpr</a>  
This structure is used by `WindowFunctionPlanner` to plan operators with custom expressions.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>  
Result of planning a raw expr with [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html" class="trait" title="trait datafusion::logical_expr::planner::ContextProvider">ContextProvider</a>  
Provides the `SQL` query planner meta-data about tables and functions referenced in SQL statements, without a direct dependency on the `datafusion` Catalog structures such as [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>  
Customize planning of SQL AST expressions to [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html" class="trait" title="trait datafusion::logical_expr::planner::TypePlanner">TypePlanner</a>  
Customize planning SQL types to DataFusion (Arrow) types.
