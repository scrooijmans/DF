# Function buffer_unary_not Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/buffer/ops.rs.html#206" class="src">Source</a>

``` rust
pub fn buffer_unary_not(
    left: &Buffer,
    offset_in_bits: usize,
    len_in_bits: usize,
) -> Buffer
```

Expand description

Apply a bitwise not to one input and return the result as a Buffer. The input is treated as a bitmap, meaning that offset and length are specified in number of bits.
