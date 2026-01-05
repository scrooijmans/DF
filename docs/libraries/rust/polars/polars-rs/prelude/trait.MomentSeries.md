# Trait MomentSeries Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/moment.rs.html#6" class="src">Source</a>

``` rust
pub trait MomentSeries: SeriesSealed {
    // Provided methods
    fn skew(&self, bias: bool) -> Result<Option<f64>, PolarsError> { ... }
    fn kurtosis(
        &self,
        fisher: bool,
        bias: bool,
    ) -> Result<Option<f64>, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.skew" class="fn">skew</a>(&self, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the sample skewness of a data set.

For normally distributed data, the skewness should be about zero. For uni-modal continuous distributions, a skewness value greater than zero means that there is more weight in the right tail of the distribution. The function `skewtest` can be used to determine if the skewness value is close enough to zero, statistically speaking.

see: [scipy](https://github.com/scipy/scipy/blob/47bb6febaa10658c72962b9615d5d5aa2513fa3a/scipy/stats/stats.py#L1024)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.kurtosis" class="fn">kurtosis</a>(&self, fisher: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the kurtosis (Fisher or Pearson) of a dataset.

Kurtosis is the fourth central moment divided by the square of the variance. If Fisher’s definition is used, then 3.0 is subtracted from the result to give 0.0 for a normal distribution. If bias is `false` then the kurtosis is calculated using k statistics to eliminate bias coming from biased moment estimators

see: [scipy](https://github.com/scipy/scipy/blob/47bb6febaa10658c72962b9615d5d5aa2513fa3a/scipy/stats/stats.py#L1027)

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#impl-MomentSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html" class="trait" title="trait polars::prelude::MomentSeries">MomentSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
