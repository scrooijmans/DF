# Function concat_lf_diagonal Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/dsl/functions.rs.html#43-46" class="src">Source</a>

``` rust
pub fn concat_lf_diagonal<L>(
    inputs: L,
    args: UnionArgs,
) -> Result<LazyFrame, PolarsError>where
    L: AsRef<[LazyFrame]>,
```

Available on **crate feature `lazy`** only.

Expand description

Concat [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame")s diagonally. Calls [`concat`](https://docs.rs/polars/latest/polars/prelude/fn.concat.html "fn polars::prelude::concat") internally.
