# Type Alias StringRunBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_byte_run_builder.rs.html#333" class="src">Source</a>

``` rust
pub type StringRunBuilder<K> = GenericByteRunBuilder<K, GenericStringType<i32>>;
```

Expand description

Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray")

``` rust
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded values are Strings.

let mut builder = StringRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value("abc");
builder.append_null();
builder.extend([Some("def"), Some("def"), Some("abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_string::<i32>();

assert_eq!(ava.value(0), "abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), "def");
assert_eq!(ava.value(3), "abc");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/builder/type.StringRunBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StringRunBuilder<K> { /* private fields */ }
```
