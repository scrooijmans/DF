# Function cum_max_with_init Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/cum_agg.rs.html#347-351" class="src">Source</a>

``` rust
pub fn cum_max_with_init(
    s: &Series,
    reverse: bool,
    init: &AnyValue<'static>,
) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.
