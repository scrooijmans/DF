# Function from_ffi Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/ffi.rs.html#267" class="src">Source</a>

``` rust
pub unsafe fn from_ffi(
    array: FFI_ArrowArray,
    schema: &FFI_ArrowSchema,
) -> Result<ArrayData, ArrowError>
```

Available on **crate feature `ffi`** only.

Expand description

Import [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from the C Data Interface

## <a href="https://docs.rs/arrow/latest/arrow/ffi/fn.from_ffi.html#safety" class="doc-anchor">§</a>Safety

This struct assumes that the incoming data agrees with the C data interface.
