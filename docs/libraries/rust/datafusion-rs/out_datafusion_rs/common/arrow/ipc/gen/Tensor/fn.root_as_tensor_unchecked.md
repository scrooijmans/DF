# Function root_as_tensor_unchecked Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Tensor.rs.html#1184" class="src">Source</a>

``` rust
pub unsafe fn root_as_tensor_unchecked(buf: &[u8]) -> Tensor<'_>
```

Expand description

Assumes, without verification, that a buffer of bytes contains a Tensor and returns it.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/fn.root_as_tensor_unchecked.html#safety" class="doc-anchor">§</a>Safety

Callers must trust the given bytes do indeed contain a valid `Tensor`.
