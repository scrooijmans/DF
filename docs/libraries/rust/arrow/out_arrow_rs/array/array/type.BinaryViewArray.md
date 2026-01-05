# Type Alias BinaryViewArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/byte_view_array.rs.html#988" class="src">Source</a>

``` rust
pub type BinaryViewArray = GenericByteViewArray<BinaryViewType>;
```

Expand description

A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") of `[u8]`

See [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") for format and layout details.

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.BinaryViewArray.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::BinaryViewArray;
let array = BinaryViewArray::from_iter_values(vec![b"hello" as &[u8], b"world", b"lulu", b"large payload over 12 bytes"]);
assert_eq!(array.value(0), b"hello");
assert_eq!(array.value(3), b"large payload over 12 bytes");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.BinaryViewArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BinaryViewArray { /* private fields */ }
```
