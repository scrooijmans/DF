# Function get_bit_raw Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_util.rs.html#49" class="src">Source</a>

``` rust
pub unsafe fn get_bit_raw(data: *const u8, i: usize) -> bool
```

Expand description

Returns whether bit at position `i` in `data` is set or not.

## <a href="https://docs.rs/arrow/latest/arrow/util/bit_util/fn.get_bit_raw.html#safety" class="doc-anchor">§</a>Safety

Note this doesn’t do any bound checking, for performance reason. The caller is responsible to guarantee that `i` is within bounds.
