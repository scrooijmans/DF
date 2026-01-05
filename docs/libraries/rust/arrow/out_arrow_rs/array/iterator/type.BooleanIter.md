# Type Alias BooleanIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#133" class="src">Source</a>

``` rust
pub type BooleanIter<'a> = ArrayIter<&'a BooleanArray>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any BooleanArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/iterator/type.BooleanIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BooleanIter<'a> { /* private fields */ }
```
