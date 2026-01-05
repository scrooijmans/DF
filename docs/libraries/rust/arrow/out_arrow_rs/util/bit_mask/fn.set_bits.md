# Function set_bits Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_mask.rs.html#28-34" class="src">Source</a>

``` rust
pub fn set_bits(
    write_data: &mut [u8],
    data: &[u8],
    offset_write: usize,
    offset_read: usize,
    len: usize,
) -> usize
```

Expand description

Util function to set bits in a slice of bytes.

This will sets all bits on `write_data` in the range `[offset_write..offset_write+len]` to be equal to the bits in `data` in the range `[offset_read..offset_read+len]` returns the number of `0` bits `data[offset_read..offset_read+len]` `offset_write`, `offset_read`, and `len` are in terms of bits
