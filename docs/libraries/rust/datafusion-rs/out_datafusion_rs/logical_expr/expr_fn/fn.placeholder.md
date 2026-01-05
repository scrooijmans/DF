# Function placeholder Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#110" class="src">Source</a>

``` rust
pub fn placeholder(id: impl Into<String>) -> Expr
```

Expand description

Create placeholder value that will be filled in (such as `$1`)

Note the parameter type can be inferred using [`Expr::infer_placeholder_types`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.infer_placeholder_types "method datafusion::prelude::Expr::infer_placeholder_types")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.placeholder.html#example" class="doc-anchor">§</a>Example

``` rust
let p = placeholder("$0"); // $0, refers to parameter 1
assert_eq!(p.to_string(), "$0")
```
