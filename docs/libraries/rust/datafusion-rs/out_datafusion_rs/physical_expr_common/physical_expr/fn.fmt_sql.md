# Function fmt_sql Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/physical_expr.rs.html#510" class="src">Source</a>

``` rust
pub fn fmt_sql(expr: &(dyn PhysicalExpr + 'static)) -> impl Display
```

Expand description

Prints a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") in a SQL-like format

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.fmt_sql.html#example" class="doc-anchor">§</a>Example

``` rust
use std::collections::HashMap;
let expr: Arc<dyn PhysicalExpr> = make_physical_expr();
// wrap the expression in `sql_fmt` which can be used with
// `format!`, `to_string()`, etc
let expr_as_sql = fmt_sql(expr.as_ref());
assert_eq!(
  "The SQL: CASE a > b THEN 1 ELSE 0 END",
  format!("The SQL: {expr_as_sql}")
);
```
