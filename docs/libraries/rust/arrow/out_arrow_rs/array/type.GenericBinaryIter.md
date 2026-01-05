# Type Alias GenericBinaryIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#137" class="src">Source</a>

``` rust
pub type GenericBinaryIter<'a, T> = ArrayIter<&'a GenericByteArray<GenericBinaryType<T>>>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any BinaryArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericBinaryIter<'a, T> { /* private fields */ }
```
