# Function prep_null_mask_filter Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/filter.rs.html#116" class="src">Source</a>

``` rust
pub fn prep_null_mask_filter(filter: &BooleanArray) -> BooleanArray
```

Expand description

Remove null values by do a bitmask AND operation with null bits and the boolean bits.
