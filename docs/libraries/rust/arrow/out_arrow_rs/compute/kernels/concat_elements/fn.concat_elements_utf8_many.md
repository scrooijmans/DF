# Function concat_elements_utf8_many Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/concat_elements.rs.html#109-111" class="src">Source</a>

``` rust
pub fn concat_elements_utf8_many<Offset>(
    arrays: &[&GenericByteArray<GenericStringType<Offset>>],
) -> Result<GenericByteArray<GenericStringType<Offset>>, ArrowError>where
    Offset: OffsetSizeTrait,
```

Expand description

Returns the elementwise concatenation of [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray").

``` text
e.g:
  ["a", "b"] + [None, "c"] + [None, "d"] = [None, "bcd"]
```

An error will be returned if the [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") are of different lengths
