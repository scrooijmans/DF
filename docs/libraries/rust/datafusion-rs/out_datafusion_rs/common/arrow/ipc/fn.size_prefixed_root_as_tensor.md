# Function size_prefixed_root_as_tensor Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Tensor.rs.html#1151" class="src">Source</a>

``` rust
pub fn size_prefixed_root_as_tensor(
    buf: &[u8],
) -> Result<Tensor<'_>, InvalidFlatbuffer>
```

Expand description

Verifies that a buffer of bytes contains a size prefixed `Tensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_tensor_unchecked`.
