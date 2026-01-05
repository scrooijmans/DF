# Type Alias StringViewBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_view_builder.rs.html#508" class="src">Source</a>

``` rust
pub type StringViewBuilder = GenericByteViewBuilder<StringViewType>;
```

Expand description

Array builder for [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray")

Values can be appended using [`GenericByteViewBuilder::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_value "method arrow::array::GenericByteViewBuilder::append_value"), and nulls with [`GenericByteViewBuilder::append_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html#method.append_null "method arrow::array::GenericByteViewBuilder::append_null") as normal.

## <a href="https://docs.rs/arrow/latest/arrow/array/type.StringViewBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = StringViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected = vec![Some("hello"), None, Some("world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.StringViewBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StringViewBuilder { /* private fields */ }
```
