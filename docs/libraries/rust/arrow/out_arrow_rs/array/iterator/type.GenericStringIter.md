# Type Alias GenericStringIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#135" class="src">Source</a>

``` rust
pub type GenericStringIter<'a, T> = ArrayIter<&'a GenericByteArray<GenericStringType<T>>>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any Utf8Array

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/iterator/type.GenericStringIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericStringIter<'a, T> { /* private fields */ }
```
