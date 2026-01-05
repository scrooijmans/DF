# Function find_in_set_general Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/unicode/find_in_set.rs.html#260-267" class="src">Source</a>

``` rust
pub fn find_in_set_general<'a, T, V>(
    string_array: V,
    str_list_array: V,
) -> Result<Arc<dyn Array>, DataFusionError>where
    T: ArrowPrimitiveType,
    <T as ArrowPrimitiveType>::Native: OffsetSizeTrait,
    V: ArrayAccessor<Item = &'a str>,
```
