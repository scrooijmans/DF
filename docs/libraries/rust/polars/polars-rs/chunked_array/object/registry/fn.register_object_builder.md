# Function register_object_builder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#120-125" class="src">Source</a>

``` rust
pub fn register_object_builder(
    builder_constructor: Box<dyn Fn(PlSmallStr, usize) -> Box<dyn AnonymousObjectBuilder> + Send + Sync>,
    object_converter: Arc<dyn Fn(AnyValue<'_>) -> Box<dyn Any> + Send + Sync>,
    pyobject_converter: Arc<dyn Fn(AnyValue<'_>) -> Box<dyn Any> + Send + Sync>,
    physical_dtype: ArrowDataType,
)
```
