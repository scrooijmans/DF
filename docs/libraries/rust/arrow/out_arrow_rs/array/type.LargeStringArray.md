# Type Alias LargeStringArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/string_array.rs.html#154" class="src">Source</a>

``` rust
pub type LargeStringArray = GenericByteArray<GenericStringType<i64>>;
```

Expand description

A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i64` offsets

## <a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html#examples" class="doc-anchor">§</a>Examples

Construction

``` rust
// Create from Vec<Option<&str>>
let arr = LargeStringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = LargeStringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: LargeStringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

``` rust
use arrow_array::LargeStringArray;
let array = LargeStringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(2), "bar");
```

See [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeStringArray { /* private fields */ }
```
