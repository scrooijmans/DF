# Type Alias ObjectConverter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#23" class="src">Source</a>

``` rust
pub type ObjectConverter = Arc<dyn Fn(AnyValue<'_>) -> Box<dyn Any> + Send + Sync>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/type.ObjectConverter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ObjectConverter { /* private fields */ }
```
