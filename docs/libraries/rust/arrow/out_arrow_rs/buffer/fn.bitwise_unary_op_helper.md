# Function bitwise_unary_op_helper Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/ops.rs.html#96-103" class="src">Source</a>

``` rust
pub fn bitwise_unary_op_helper<F>(
    left: &Buffer,
    offset_in_bits: usize,
    len_in_bits: usize,
    op: F,
) -> Bufferwhere
    F: FnMut(u64) -> u64,
```

Expand description

Apply a bitwise operation `op` to one input and return the result as a Buffer. The input is treated as a bitmap, meaning that offset and length are specified in number of bits.
