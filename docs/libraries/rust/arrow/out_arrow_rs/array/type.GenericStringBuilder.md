# Type Alias GenericStringBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_builder.rs.html#341" class="src">Source</a>

``` rust
pub type GenericStringBuilder<O> = GenericByteBuilder<GenericStringType<O>>;
```

Expand description

Array builder for [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray")

Values can be appended using [`GenericByteBuilder::append_value`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.append_value "method arrow::array::GenericByteBuilder::append_value"), and nulls with [`GenericByteBuilder::append_null`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html#method.append_null "method arrow::array::GenericByteBuilder::append_null").

This builder also implements [`std::fmt::Write`](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write") with any written data included in the next appended value. This allows using [`std::fmt::Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") with standard Rust idioms like `write!` and `writeln!` to write data directly to the builder without intermediate allocations.

## <a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html#example-writing-strings-with-append_value" class="doc-anchor">§</a>Example writing strings with `append_value`

``` rust
let mut builder = GenericStringBuilder::<i32>::new();

// Write one string value
builder.append_value("foobarbaz");

// Write a second string
builder.append_value("v2");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

## <a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html#example-incrementally-writing-strings-with-stdfmtwrite" class="doc-anchor">§</a>Example incrementally writing strings with `std::fmt::Write`

``` rust
let mut builder = GenericStringBuilder::<i32>::new();

// Write data in multiple `write!` calls
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
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericStringBuilder<O> { /* private fields */ }
```
