# Function columnize_expr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#721" class="src">Source</a>

``` rust
pub fn columnize_expr(
    e: Expr,
    input: &LogicalPlan,
) -> Result<Expr, DataFusionError>
```

Expand description

Convert an expression into Column expression if it’s already provided as input plan.

For example, it rewrites:

``` text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), sum(col("c2"))?
```

Into:

``` text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), col("SUM(c2)")?
```
