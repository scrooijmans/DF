# Type Alias BinaryViewBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_view_builder.rs.html#530" class="src">Source</a>

``` rust
pub type BinaryViewBuilder = GenericByteViewBuilder<BinaryViewType>;
```

Expand description

Array builder for [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray")

Values can be appended using [`GenericByteViewBuilder::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_value "method arrow::array::GenericByteViewBuilder::append_value"), and nulls with [`GenericByteViewBuilder::append_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_null "method arrow::array::GenericByteViewBuilder::append_null") as normal.

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/type.BinaryViewBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::BinaryViewArray;
let mut builder = BinaryViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected: Vec<Option<&[u8]>> = vec![Some(b"hello"), None, Some(b"world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/builder/type.BinaryViewBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BinaryViewBuilder { /* private fields */ }
```
