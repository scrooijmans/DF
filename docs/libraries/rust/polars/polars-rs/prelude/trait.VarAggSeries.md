# Trait VarAggSeries Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/aggregate/var.rs.html#5" class="src">Source</a>

``` rust
pub trait VarAggSeries {
    // Required methods
    fn var_reduce(&self, ddof: u8) -> Scalar;
    fn std_reduce(&self, ddof: u8) -> Scalar;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#tymethod.var_reduce" class="fn">var_reduce</a>(&self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the variance of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#tymethod.std_reduce" class="fn">std_reduce</a>(&self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the standard deviation of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#impl-VarAggSeries-for-ChunkedArray%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html" class="trait" title="trait polars::prelude::VarAggSeries">VarAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#impl-VarAggSeries-for-ChunkedArray%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html" class="trait" title="trait polars::prelude::VarAggSeries">VarAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html#impl-VarAggSeries-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.VarAggSeries.html" class="trait" title="trait polars::prelude::VarAggSeries">VarAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::PolarsIntegerType">PolarsIntegerType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkVar.html" class="trait" title="trait polars::prelude::ChunkVar">ChunkVar</a>,
