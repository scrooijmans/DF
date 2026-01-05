# Function concat_elements_utf8 Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/concat_elements.rs.html#87-90" class="src">Source</a>

``` rust
pub fn concat_elements_utf8<Offset>(
    left: &GenericByteArray<GenericStringType<Offset>>,
    right: &GenericByteArray<GenericStringType<Offset>>,
) -> Result<GenericByteArray<GenericStringType<Offset>>, ArrowError>where
    Offset: OffsetSizeTrait,
```

Expand description

Returns the elementwise concatenation of a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray").

An index of the resulting [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") is null if any of `StringArray` are null at that location.

``` text
e.g:

  ["Hello"] + ["World"] = ["HelloWorld"]

  ["a", "b"] + [None, "c"] = [None, "bc"]
```

An error will be returned if `left` and `right` have different lengths
