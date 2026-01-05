# Function bitwise_quaternary_op_helper Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/ops.rs.html#23-30" class="src">Source</a>

``` rust
pub fn bitwise_quaternary_op_helper<F>(
    buffers: [&Buffer; 4],
    offsets: [usize; 4],
    len_in_bits: usize,
    op: F,
) -> Bufferwhere
    F: Fn(u64, u64, u64, u64) -> u64,
```

Expand description

Apply a bitwise operation `op` to four inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.
