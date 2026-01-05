# Function min_binary Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#522" class="src">Source</a>

``` rust
pub fn min_binary<T>(
    array: &GenericByteArray<GenericBinaryType<T>>,
) -> Option<&[u8]>where
    T: OffsetSizeTrait,
```

Expand description

Returns the minimum value in the binary array, according to the natural order.
