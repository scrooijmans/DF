# Type Alias LargeBinaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/binary_array.rs.html#210" class="src">Source</a>

``` rust
pub type LargeBinaryArray = GenericByteArray<GenericBinaryType<i64>>;
```

Expand description

A [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") of `[u8]` using `i64` offsets

## <a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html#examples" class="doc-anchor">§</a>Examples

Create a LargeBinaryArray from a vector of byte slices.

``` rust
use arrow_array::{Array, LargeBinaryArray};
let values: Vec<&[u8]> =
    vec![b"one", b"two", b"", b"three"];
let array = LargeBinaryArray::from_vec(values);
assert_eq!(4, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(2));
assert_eq!(b"three", array.value(3));
```

Create a LargeBinaryArray from a vector of Optional (null) byte slices.

``` rust
use arrow_array::{Array, LargeBinaryArray};
let values: Vec<Option<&[u8]>> =
    vec![Some(b"one"), Some(b"two"), None, Some(b""), Some(b"three")];
let array = LargeBinaryArray::from_opt_vec(values);
assert_eq!(5, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(3));
assert_eq!(b"three", array.value(4));
assert!(!array.is_null(0));
assert!(!array.is_null(1));
assert!(array.is_null(2));
assert!(!array.is_null(3));
assert!(!array.is_null(4));
```

See [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for more information and examples

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeBinaryArray { /* private fields */ }
```
