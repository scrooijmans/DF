# Function set_bit_raw Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_util.rs.html#66" class="src">Source</a>

``` rust
pub unsafe fn set_bit_raw(data: *mut u8, i: usize)
```

Expand description

Sets bit at position `i` for `data`

## <a href="https://docs.rs/arrow/latest/arrow/util/bit_util/fn.set_bit_raw.html#safety" class="doc-anchor">§</a>Safety

Note this doesn’t do any bound checking, for performance reason. The caller is responsible to guarantee that `i` is within bounds.
