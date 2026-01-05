# Function to_ffi Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/ffi.rs.html#256" class="src">Source</a>

``` rust
pub fn to_ffi(
    data: &ArrayData,
) -> Result<(FFI_ArrowArray, FFI_ArrowSchema), ArrowError>
```

Expand description

Export to the C Data Interface
