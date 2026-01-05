# Function try_list_from_par_iter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/from_iterator_par.rs.html#218-220" class="src">Source</a>

``` rust
pub fn try_list_from_par_iter<I>(
    par_iter: I,
    name: PlSmallStr,
) -> Result<ChunkedArray<ListType>, PolarsError>where
    I: IntoParallelIterator<Item = Result<Option<Series>, PolarsError>>,
```
