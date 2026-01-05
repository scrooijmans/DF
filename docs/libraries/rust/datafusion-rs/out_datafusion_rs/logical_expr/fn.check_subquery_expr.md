# Function check_subquery_expr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/invariants.rs.html#158-162" class="src">Source</a>

``` rust
pub fn check_subquery_expr(
    outer_plan: &LogicalPlan,
    inner_plan: &LogicalPlan,
    expr: &Expr,
) -> Result<(), DataFusionError>
```

Expand description

Do necessary check on subquery expressions and fail the invalid plan

1.  Check whether the outer plan is in the allowed outer plans list to use subquery expressions, the allowed while list: \[Projection, Filter, Window, Aggregate, Join\].
2.  Check whether the inner plan is in the allowed inner plans list to use correlated(outer) expressions.
3.  Check and validate unsupported cases to use the correlated(outer) expressions inside the subquery(inner) plans/inner expressions. For example, we do not want to support to use correlated expressions as the Join conditions in the subquery plan when the Join is a Full Out Join
