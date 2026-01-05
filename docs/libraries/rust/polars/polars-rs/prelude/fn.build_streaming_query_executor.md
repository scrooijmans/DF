# Function build_streaming_query_executor Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#2464-2468" class="src">Source</a>

``` rust
pub fn build_streaming_query_executor(
    node: Node,
    ir_arena: &mut Arena<IR>,
    expr_arena: &mut Arena<AExpr>,
) -> Result<Box<dyn Executor>, PolarsError>
```

Available on **crate feature `lazy`** only.
