# Function as_float16_array Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cast.rs.html#111" class="src">Source</a>

``` rust
pub fn as_float16_array(
    array: &dyn Array,
) -> Result<&PrimitiveArray<Float16Type>, DataFusionError>
```
