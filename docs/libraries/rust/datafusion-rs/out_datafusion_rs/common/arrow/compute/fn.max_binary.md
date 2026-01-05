# Function max_binary Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#507" class="src">Source</a>

``` rust
pub fn max_binary<T>(
    array: &GenericByteArray<GenericBinaryType<T>>,
) -> Option<&[u8]>where
    T: OffsetSizeTrait,
```

Expand description

Returns the maximum value in the binary array, according to the natural order.
