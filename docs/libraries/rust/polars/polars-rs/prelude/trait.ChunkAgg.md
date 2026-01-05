# Trait ChunkAgg Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#240" class="src">Source</a>

``` rust
pub trait ChunkAgg<T> {
    // Required method
    fn _sum_as_f64(&self) -> f64;

    // Provided methods
    fn sum(&self) -> Option<T> { ... }
    fn min(&self) -> Option<T> { ... }
    fn max(&self) -> Option<T> { ... }
    fn min_max(&self) -> Option<(T, T)> { ... }
    fn mean(&self) -> Option<f64> { ... }
}
```

Expand description

Aggregation operations.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#tymethod._sum_as_f64" class="fn">_sum_as_f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#method.sum" class="fn">sum</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Aggregate the sum of the ChunkedArray. Returns `None` if not implemented for `T`. If the array is empty, `0` is returned

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#method.min" class="fn">min</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#method.max" class="fn">max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Returns the maximum value in the array, according to the natural order. Returns `None` if the array is empty or only contains null values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#method.min_max" class="fn">min_max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(T, T)</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#method.mean" class="fn">mean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the mean value in the array. Returns `None` if the array is empty or only contains null values.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html#impl-ChunkAgg%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html" class="trait" title="trait polars::prelude::ChunkAgg">ChunkAgg</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/sum/trait.WrappingSum.html" class="trait" title="trait polars_compute::sum::WrappingSum">WrappingSum</a>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>: for\<'a\> <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/min_max/trait.MinMaxKernel.html" class="trait" title="trait polars_compute::min_max::MinMaxKernel">MinMaxKernel</a>\<Scalar\<'a\> = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,
