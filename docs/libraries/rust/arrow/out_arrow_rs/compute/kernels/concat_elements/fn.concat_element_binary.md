# Function concat_element_binary Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/concat_elements.rs.html#95-98" class="src">Source</a>

``` rust
pub fn concat_element_binary<Offset>(
    left: &GenericByteArray<GenericBinaryType<Offset>>,
    right: &GenericByteArray<GenericBinaryType<Offset>>,
) -> Result<GenericByteArray<GenericBinaryType<Offset>>, ArrowError>where
    Offset: OffsetSizeTrait,
```

Expand description

Returns the elementwise concatenation of a [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray").
