# Type Alias GenericBinaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_builder.rs.html#395" class="src">Source</a>

``` rust
pub type GenericBinaryBuilder<O> = GenericByteBuilder<GenericBinaryType<O>>;
```

Expand description

Array builder for [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray")

Values can be appended using [`GenericByteBuilder::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.append_value "method arrow::array::GenericByteBuilder::append_value"), and nulls with [`GenericByteBuilder::append_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.append_null "method arrow::array::GenericByteBuilder::append_null").

## <a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data
builder.append_value("foo");

// Write second value
builder.append_value(&[0,1,2]);

let array = builder.finish();
// binary values
assert_eq!(array.value(0), b"foo");
assert_eq!(array.value(1), b"\x00\x01\x02");
```

## <a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryBuilder.html#example-incrementally-writing-bytes-with-write_bytes" class="doc-anchor">§</a>Example incrementally writing bytes with `write_bytes`

``` rust
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data in multiple `write_bytes` calls
write!(builder, "foo").unwrap();
write!(builder, "bar").unwrap();
// The next call to append_value finishes the current string
// including all previously written strings.
builder.append_value("baz");

// Write second value with a single write call
write!(builder, "v2").unwrap();
// finish the value by calling append_value with an empty string
builder.append_value("");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz".as_bytes());
assert_eq!(array.value(1), "v2".as_bytes());
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericBinaryBuilder<O> { /* private fields */ }
```
