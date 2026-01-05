# Type Alias StringViewArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/byte_view_array.rs.html#1029" class="src">Source</a>

``` rust
pub type StringViewArray = GenericByteViewArray<StringViewType>;
```

Expand description

A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") that stores utf8 data

See [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") for format and layout details.

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringViewArray.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::StringViewArray;
let array = StringViewArray::from_iter_values(vec!["hello", "world", "lulu", "large payload over 12 bytes"]);
assert_eq!(array.value(0), "hello");
assert_eq!(array.value(3), "large payload over 12 bytes");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringViewArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StringViewArray { /* private fields */ }
```
