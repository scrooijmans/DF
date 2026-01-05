# Type Alias StringArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/string_array.rs.html#127" class="src">Source</a>

``` rust
pub type StringArray = GenericByteArray<GenericStringType<i32>>;
```

Expand description

A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i32` offsets

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringArray.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<&str>>
let arr = StringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = StringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: StringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

``` rust
let array = StringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(0), "foo");
```

See [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StringArray { /* private fields */ }
```
