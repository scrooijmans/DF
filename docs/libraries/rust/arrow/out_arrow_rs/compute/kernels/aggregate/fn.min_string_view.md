# Function min_string_view Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#552" class="src">Source</a>

``` rust
pub fn min_string_view(
    array: &GenericByteViewArray<StringViewType>,
) -> Option<&str>
```

Expand description

Returns the minimum value in the string view array, according to the natural order.
