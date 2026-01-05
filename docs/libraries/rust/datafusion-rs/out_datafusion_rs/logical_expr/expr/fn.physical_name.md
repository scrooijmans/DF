# Function physical_name Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#3545" class="src">Source</a>

``` rust
pub fn physical_name(expr: &Expr) -> Result<String, DataFusionError>
```

Expand description

The name of the column (field) that this `Expr` will produce in the physical plan. The difference from [Expr::schema_name](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name "method datafusion::prelude::Expr::schema_name") is that top-level columns are unqualified.
