# Function concat Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/dsl/functions.rs.html#75" class="src">Source</a>

``` rust
pub fn concat<L>(inputs: L, args: UnionArgs) -> Result<LazyFrame, PolarsError>where
    L: AsRef<[LazyFrame]>,
```

Available on **crate feature `lazy`** only.

Expand description

Concat multiple [`LazyFrame`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame")s vertically.
