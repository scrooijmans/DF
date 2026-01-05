# Constant BUILD_STREAMING_EXECUTOR Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#2438" class="src">Source</a>

``` rust
pub const BUILD_STREAMING_EXECUTOR: Option<fn(Node, &mut Arena<IR>, &mut Arena<AExpr>) -> Result<Box<dyn Executor>, PolarsError>>;
```

Available on **crate feature `lazy`** only.
