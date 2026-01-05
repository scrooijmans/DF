# Function concat_elements_bytes Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/concat_elements.rs.html#29-32" class="src">Source</a>

``` rust
pub fn concat_elements_bytes<T>(
    left: &GenericByteArray<T>,
    right: &GenericByteArray<T>,
) -> Result<GenericByteArray<T>, ArrowError>where
    T: ByteArrayType,
```

Expand description

Returns the elementwise concatenation of a [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray").
