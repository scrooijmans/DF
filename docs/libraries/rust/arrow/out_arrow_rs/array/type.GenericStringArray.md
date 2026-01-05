# Type Alias GenericStringArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/string_array.rs.html#23" class="src">Source</a>

``` rust
pub type GenericStringArray<OffsetSize> = GenericByteArray<GenericStringType<OffsetSize>>;
```

Expand description

A [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for storing `str`

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericStringArray<OffsetSize> { /* private fields */ }
```
