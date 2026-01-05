# Function from_ffi_and_data_type Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/ffi.rs.html#283-286" class="src">Source</a>

``` rust
pub unsafe fn from_ffi_and_data_type(
    array: FFI_ArrowArray,
    data_type: DataType,
) -> Result<ArrayData, ArrowError>
```

Available on **crate feature `ffi`** only.

Expand description

Import [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from the C Data Interface

## <a href="https://docs.rs/arrow/latest/arrow/ffi/fn.from_ffi_and_data_type.html#safety" class="doc-anchor">§</a>Safety

This struct assumes that the incoming data agrees with the C data interface.
