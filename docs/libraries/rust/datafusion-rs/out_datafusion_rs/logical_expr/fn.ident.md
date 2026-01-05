# Function ident Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#95" class="src">Source</a>

``` rust
pub fn ident(name: impl Into<String>) -> Expr
```

Expand description

Create an unqualified column expression from the provided name, without normalizing the column.

For example:

``` rust
let c1 = ident("A"); // not normalized staying as column 'A'
let c2 = col("A"); // normalized via SQL rules becoming column 'a'
assert_ne!(c1, c2);

let c3 = col(r#""A""#);
assert_eq!(c1, c3);

let c4 = col("t1.a"); // parses as relation 't1' column 'a'
let c5 = ident("t1.a"); // parses as column 't1.a'
assert_ne!(c4, c5);
```
