# Function union Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#1778" class="src">Source</a>

``` rust
pub fn union(
    left_plan: LogicalPlan,
    right_plan: LogicalPlan,
) -> Result<LogicalPlan, DataFusionError>
```

Expand description

Union two [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s.

Constructs the UNION plan, but does not perform type-coercion. Therefore the subtree expressions will not be properly typed until the optimizer pass.

If a properly typed UNION plan is needed, refer to [`TypeCoercionRewriter::coerce_union`](https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union) or alternatively, merge the union input schema using [`coerce_union_schema`](https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/fn.coerce_union_schema.html) and apply the expression rewrite with [`coerce_plan_expr_for_schema`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.coerce_plan_expr_for_schema.html "fn datafusion::logical_expr::expr_rewriter::coerce_plan_expr_for_schema").
