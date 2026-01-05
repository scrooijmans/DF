# Type Alias BinaryRunBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_byte_run_builder.rs.html#369" class="src">Source</a>

``` rust
pub type BinaryRunBuilder<K> = GenericByteRunBuilder<K, GenericBinaryType<i32>>;
```

Expand description

Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray")

``` rust
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded data is binary values.


let mut builder = BinaryRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value(b"abc");
builder.append_null();
builder.extend([Some(b"def"), Some(b"def"), Some(b"abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), b"def");
assert_eq!(ava.value(3), b"abc");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/builder/type.BinaryRunBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BinaryRunBuilder<K> { /* private fields */ }
```
