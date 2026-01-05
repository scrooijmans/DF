# Function cum_prod Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/cum_agg.rs.html#256" class="src">Source</a>

``` rust
pub fn cum_prod(s: &Series, reverse: bool) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Get an array with the cumulative product computed at every element.

If the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is one of `{Int8, UInt8, Int16, UInt16, Int32, UInt32}` the `Series` is first cast to `Int64` to prevent overflow issues.
