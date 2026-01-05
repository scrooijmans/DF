# Function collect_all Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/dsl/functions.rs.html#80-82" class="src">Source</a>

``` rust
pub fn collect_all<I>(lfs: I) -> Result<Vec<DataFrame>, PolarsError>where
    I: IntoParallelIterator<Item = LazyFrame>,
```

Available on **crate feature `lazy`** only.

Expand description

Collect all [`LazyFrame`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame") computations.
