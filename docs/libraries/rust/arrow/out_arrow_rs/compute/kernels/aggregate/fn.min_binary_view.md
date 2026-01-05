# Function min_binary_view Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#527" class="src">Source</a>

``` rust
pub fn min_binary_view(
    array: &GenericByteViewArray<BinaryViewType>,
) -> Option<&[u8]>
```

Expand description

Returns the minimum value in the binary view array, according to the natural order.
