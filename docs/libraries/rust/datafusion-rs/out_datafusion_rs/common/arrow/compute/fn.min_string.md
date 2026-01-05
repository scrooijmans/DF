# Function min_string Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#547" class="src">Source</a>

``` rust
pub fn min_string<T>(
    array: &GenericByteArray<GenericStringType<T>>,
) -> Option<&str>where
    T: OffsetSizeTrait,
```

Expand description

Returns the minimum value in the string array, according to the natural order.
