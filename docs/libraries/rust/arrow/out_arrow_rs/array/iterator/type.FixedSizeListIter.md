# Type Alias FixedSizeListIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#141" class="src">Source</a>

``` rust
pub type FixedSizeListIter<'a> = ArrayIter<&'a FixedSizeListArray>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any FixedSizeListArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/iterator/type.FixedSizeListIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct FixedSizeListIter<'a> { /* private fields */ }
```
