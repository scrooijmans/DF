# Type Alias FieldsNameMapper Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/name.rs.html#95" class="src">Source</a>

``` rust
pub type FieldsNameMapper = Arc<dyn Fn(&str) -> PlSmallStr + Send + Sync>;
```

Available on **crate feature `lazy`** only.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.FieldsNameMapper.html#aliased-type" class="anchor">§</a>

``` rust
pub struct FieldsNameMapper { /* private fields */ }
```
