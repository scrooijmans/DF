# Module ffi Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#253" class="src">Source</a>

Available on **crate feature `ffi`** only.

Expand description

Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

Generally, this module is divided in two main interfaces: One interface maps C ABI to native Rust types, i.e. convert c-pointers, c_char, to native rust. This is handled by [FFI_ArrowSchema](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html "struct arrow::ffi::FFI_ArrowSchema") and [FFI_ArrowArray](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html "struct arrow::ffi::FFI_ArrowArray").

The second interface maps native Rust types to the Rust-specific implementation of Arrow such as `format` to `Datatype`, `Buffer`, etc. This is handled by `from_ffi` and `to_ffi`.

Export to FFI

``` rust
// create an array natively

let array = Int32Array::from(vec![Some(1), None, Some(3)]);
let data = array.into_data();

// Export it
let (out_array, out_schema) = to_ffi(&data)?;

// import it
let data = unsafe { from_ffi(out_array, &out_schema) }?;
let array = Int32Array::from(data);

// verify
assert_eq!(array, Int32Array::from(vec![Some(1), None, Some(3)]));
```

Import from FFI

``` rust
/// A foreign data container that can export to C Data interface
struct ForeignArray {};

impl ForeignArray {
    /// Export from foreign array representation to C Data interface
    /// e.g. <https://github.com/apache/arrow/blob/fc1f9ebbc4c3ae77d5cfc2f9322f4373d3d19b8a/python/pyarrow/array.pxi#L1552>
    fn export_to_c(&self, array: *mut FFI_ArrowArray, schema: *mut FFI_ArrowSchema) {
        // ...
    }
}

/// Import an [`ArrayRef`] from a [`ForeignArray`]
fn import_array(foreign: &ForeignArray) -> Result<ArrayRef, ArrowError> {
    let mut schema = FFI_ArrowSchema::empty();
    let mut array = FFI_ArrowArray::empty();
    foreign.export_to_c(addr_of_mut!(array), addr_of_mut!(schema));
    Ok(make_array(unsafe { from_ffi(array, &schema) }?))
}
```

## Structs<a href="https://docs.rs/arrow/latest/arrow/ffi/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>  
ABI-compatible struct for ArrowArray from C Data Interface See <https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>  
ABI-compatible struct for `ArrowSchema` from C Data Interface See <https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions>

## Functions<a href="https://docs.rs/arrow/latest/arrow/ffi/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/fn.export_array_into_raw.html" class="fn" title="fn arrow::ffi::export_array_into_raw">export_array_into_raw</a><sup>⚠</sup>Deprecated  
Exports an array to raw pointers of the C Data Interface provided by the consumer.

<a href="https://docs.rs/arrow/latest/arrow/ffi/fn.from_ffi.html" class="fn" title="fn arrow::ffi::from_ffi">from_ffi</a><sup>⚠</sup>  
Import [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from the C Data Interface

<a href="https://docs.rs/arrow/latest/arrow/ffi/fn.from_ffi_and_data_type.html" class="fn" title="fn arrow::ffi::from_ffi_and_data_type">from_ffi_and_data_type</a><sup>⚠</sup>  
Import [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from the C Data Interface

<a href="https://docs.rs/arrow/latest/arrow/ffi/fn.to_ffi.html" class="fn" title="fn arrow::ffi::to_ffi">to_ffi</a>  
Export to the C Data Interface
