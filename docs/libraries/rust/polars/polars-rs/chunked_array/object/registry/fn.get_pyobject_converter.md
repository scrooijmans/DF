# Function get_pyobject_converter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#156" class="src">Source</a>

``` rust
pub fn get_pyobject_converter() -> Arc<dyn Fn(AnyValue<'_>) -> Box<dyn Any> + Send + Sync>
```
