# Function new_int_range Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/int_range.rs.html#4-12" class="src">Source</a>

``` rust
pub fn new_int_range<T>(
    start: <T as PolarsNumericType>::Native,
    end: <T as PolarsNumericType>::Native,
    step: i64,
    name: PlSmallStr,
) -> Result<Series, PolarsError>where
    T: PolarsIntegerType,
    Range<<T as PolarsNumericType>::Native>: DoubleEndedIterator<Item = <T as PolarsNumericType>::Native>,
```

Available on **crate feature `polars-ops`** only.
