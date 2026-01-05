# Type Alias BinaryArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/binary_array.rs.html#170" class="src">Source</a>

``` rust
pub type BinaryArray = GenericByteArray<GenericBinaryType<i32>>;
```

Expand description

A [`GenericBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray") of `[u8]` using `i32` offsets

The byte length of each element is represented by an i32.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.BinaryArray.html#examples" class="doc-anchor">§</a>Examples

Create a BinaryArray from a vector of byte slices.

``` rust
use arrow_array::{Array, BinaryArray};
let values: Vec<&[u8]> =
    vec![b"one", b"two", b"", b"three"];
let array = BinaryArray::from_vec(values);
assert_eq!(4, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(2));
assert_eq!(b"three", array.value(3));
```

Create a BinaryArray from a vector of Optional (null) byte slices.

``` rust
use arrow_array::{Array, BinaryArray};
let values: Vec<Option<&[u8]>> =
    vec![Some(b"one"), Some(b"two"), None, Some(b""), Some(b"three")];
let array = BinaryArray::from_opt_vec(values);
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

See [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") for more information and examples

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.BinaryArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BinaryArray { /* private fields */ }
```
