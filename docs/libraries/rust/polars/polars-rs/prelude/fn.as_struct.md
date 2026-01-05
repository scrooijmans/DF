# Function as_struct Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/coerce.rs.html#6" class="src">Source</a>

``` rust
pub fn as_struct(exprs: Vec<Expr>) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Take several expressions and collect them into a [`StructChunked`](https://docs.rs/polars/latest/polars/prelude/type.StructChunked.html "type polars::prelude::StructChunked").

## <a href="https://docs.rs/polars/latest/polars/prelude/fn.as_struct.html#panics" class="doc-anchor">§</a>Panics

panics if `exprs` is empty.
