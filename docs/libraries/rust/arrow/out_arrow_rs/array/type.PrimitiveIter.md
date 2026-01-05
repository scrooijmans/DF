# Type Alias PrimitiveIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#131" class="src">Source</a>

``` rust
pub type PrimitiveIter<'a, T> = ArrayIter<&'a PrimitiveArray<T>>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any PrimitiveArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.PrimitiveIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct PrimitiveIter<'a, T> { /* private fields */ }
```
