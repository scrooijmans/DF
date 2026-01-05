# Function as_timestamp_nanosecond_array Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cast.rs.html#219-221" class="src">Source</a>

``` rust
pub fn as_timestamp_nanosecond_array(
    array: &dyn Array,
) -> Result<&PrimitiveArray<TimestampNanosecondType>, DataFusionError>
```
