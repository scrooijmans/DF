# Function col Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#67" class="src">Source</a>

``` rust
pub fn col(ident: impl Into<Column>) -> Expr
```

Expand description

Create a column expression based on a qualified or unqualified column name. Will normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

For example:

``` rust
let c1 = col("a");
let c2 = col("A");
assert_eq!(c1, c2);

// note how quoting with double quotes preserves the case
let c3 = col(r#""A""#);
assert_ne!(c1, c3);
```
