# Type Alias MapArrayIter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/iterator.rs.html#145" class="src">Source</a>

``` rust
pub type MapArrayIter<'a> = ArrayIter<&'a MapArray>;
```

Expand description

an iterator that returns Some(T) or None, that can be used on any MapArray

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/iterator/type.MapArrayIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct MapArrayIter<'a> { /* private fields */ }
```
