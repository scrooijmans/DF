# Function as_time32_millisecond_array Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cast.rs.html#204" class="src">Source</a>

``` rust
pub fn as_time32_millisecond_array(
    array: &dyn Array,
) -> Result<&PrimitiveArray<Time32MillisecondType>, DataFusionError>
```
