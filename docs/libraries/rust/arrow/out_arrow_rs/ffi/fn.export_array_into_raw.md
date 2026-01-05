# Function export_array_into_raw Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/ffi.rs.html#128-132" class="src">Source</a>

``` rust
pub unsafe fn export_array_into_raw(
    src: Arc<dyn Array>,
    out_array: *mut FFI_ArrowArray,
    out_schema: *mut FFI_ArrowSchema,
) -> Result<(), ArrowError>
```

👎Deprecated since 52.0.0: Use FFI_ArrowArray::new and FFI_ArrowSchema::try_from

Available on **crate feature `ffi`** only.

Expand description

Exports an array to raw pointers of the C Data Interface provided by the consumer.

## <a href="https://docs.rs/arrow/latest/arrow/ffi/fn.export_array_into_raw.html#safety" class="doc-anchor">§</a>Safety

Assumes that these pointers represent valid C Data Interfaces, both in memory representation and lifetime via the `release` mechanism.

This function copies the content of two FFI structs [arrow_data::ffi::FFI_ArrowArray](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html "struct arrow::ffi::FFI_ArrowArray") and [arrow_schema::ffi::FFI_ArrowSchema](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html "struct arrow::ffi::FFI_ArrowSchema") in the array to the location pointed by the raw pointers. Usually the raw pointers are provided by the array data consumer.
