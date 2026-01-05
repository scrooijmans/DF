# Function cast Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/syntactic_sugar.rs.html#60" class="src">Source</a>

``` rust
pub fn cast(expr: Expr, dtype: impl Into<DataTypeExpr>) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Casts the column given by `Expr` to a different type.

Follows the rules of Rust casting, with the exception that integers and floats can be cast to `DataType::Date` and `DataType::DateTime(_, _)`. A column consisting entirely of `Null` can be cast to any type, regardless of the nominal type of the column.
