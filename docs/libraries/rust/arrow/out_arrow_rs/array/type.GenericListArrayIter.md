# Type Alias GenericListArrayIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#143" class="src">Source</a>

``` rust
pub type GenericListArrayIter<'a, O> = ArrayIter<&'a GenericListArray<O>>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any ListArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericListArrayIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GenericListArrayIter<'a, O> { /* private fields */ }
```
