# Trait QuantileAggSeries Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/aggregate/quantile.rs.html#5" class="src">Source</a>

``` rust
pub trait QuantileAggSeries {
    // Required methods
    fn median_reduce(&self) -> Scalar;
    fn quantile_reduce(
        &self,
        _quantile: f64,
        _method: QuantileMethod,
    ) -> Result<Scalar, PolarsError>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#tymethod.median_reduce" class="fn">median_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the median of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#tymethod.quantile_reduce" class="fn">quantile_reduce</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the quantile of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#impl-QuantileAggSeries-for-ChunkedArray%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html" class="trait" title="trait polars::prelude::QuantileAggSeries">QuantileAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#impl-QuantileAggSeries-for-ChunkedArray%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html" class="trait" title="trait polars::prelude::QuantileAggSeries">QuantileAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html#impl-QuantileAggSeries-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.QuantileAggSeries.html" class="trait" title="trait polars::prelude::QuantileAggSeries">QuantileAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::PolarsIntegerType">PolarsIntegerType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/sum/trait.WrappingSum.html" class="trait" title="trait polars_compute::sum::WrappingSum">WrappingSum</a>,
