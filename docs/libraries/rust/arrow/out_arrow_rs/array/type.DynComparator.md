# Type Alias DynComparator Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/ord.rs.html#28" class="src">Source</a>

``` rust
pub type DynComparator = Box<dyn Fn(usize, usize) -> Ordering + Send + Sync>;
```

Expand description

Compare the values at two arbitrary indices in two arrays.

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.DynComparator.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DynComparator(/* private fields */);
```
