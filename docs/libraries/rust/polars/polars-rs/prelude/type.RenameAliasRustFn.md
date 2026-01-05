# Type Alias RenameAliasRustFn Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/mod.rs.html#713" class="src">Source</a>

``` rust
pub type RenameAliasRustFn = dyn Fn(&PlSmallStr) -> Result<PlSmallStr, PolarsError> + Send + Sync;
```

Available on **crate feature `lazy`** only.
