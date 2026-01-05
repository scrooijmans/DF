# Function get_list_builder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/list/mod.rs.html#79-84" class="src">Source</a>

``` rust
pub fn get_list_builder(
    inner_type_logical: &DataType,
    value_capacity: usize,
    list_capacity: usize,
    name: PlSmallStr,
) -> Box<dyn ListBuilderTrait>
```
