# Function bitwise_bin_op_helper Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/ops.rs.html#63-72" class="src">Source</a>

``` rust
pub fn bitwise_bin_op_helper<F>(
    left: &Buffer,
    left_offset_in_bits: usize,
    right: &Buffer,
    right_offset_in_bits: usize,
    len_in_bits: usize,
    op: F,
) -> Bufferwhere
    F: FnMut(u64, u64) -> u64,
```

Expand description

Apply a bitwise operation `op` to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.
