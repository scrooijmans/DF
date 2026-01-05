# Function make_view Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_view_builder.rs.html#547" class="src">Source</a>

``` rust
pub fn make_view(data: &[u8], block_id: u32, offset: u32) -> u128
```

Expand description

Create a view based on the given data, block id and offset.

Note that the code below is carefully examined with x86_64 assembly code: <https://godbolt.org/z/685YPsd5G> The goal is to avoid calling into `ptr::copy_non_interleave`, which makes function call (i.e., not inlined), which slows down things.
