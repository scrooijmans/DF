# Type Alias FixedSizeBinaryIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#139" class="src">Source</a>

``` rust
pub type FixedSizeBinaryIter<'a> = ArrayIter<&'a FixedSizeBinaryArray>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any FixedSizeBinaryArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/iterator/type.FixedSizeBinaryIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct FixedSizeBinaryIter<'a> { /* private fields */ }
```
