# Type Alias SeriesPhysIter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/iterator.rs.html#74" class="src">Source</a>

``` rust
pub type SeriesPhysIter<'a> = Box<dyn ExactSizeIterator<Item = AnyValue<'a>> + 'a>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/series/type.SeriesPhysIter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct SeriesPhysIter<'a>(/* private fields */);
```
