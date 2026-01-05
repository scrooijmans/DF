# Function buffer_bin_and_not Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/ops.rs.html#187-193" class="src">Source</a>

``` rust
pub fn buffer_bin_and_not(
    left: &Buffer,
    left_offset_in_bits: usize,
    right: &Buffer,
    right_offset_in_bits: usize,
    len_in_bits: usize,
) -> Buffer
```

Expand description

Apply a bitwise and_not to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.
