# Enum Distribution Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/statistics.rs.html#38" class="src">Source</a>

``` rust
pub enum Distribution {
    Uniform(UniformDistribution),
    Exponential(ExponentialDistribution),
    Gaussian(GaussianDistribution),
    Bernoulli(BernoulliDistribution),
    Generic(GenericDistribution),
}
```

Expand description

This object defines probabilistic distributions that encode uncertain information about a single, scalar value. Currently, we support five core statistical distributions. New variants will be added over time.

This object is the lowest-level object in the statistics hierarchy, and it is the main unit of calculus when evaluating expressions in a statistical context. Notions like column and table statistics are built on top of this object and the operations it supports.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform" class="anchor">§</a>

### Uniform(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/struct.UniformDistribution.html" class="struct" title="struct datafusion::logical_expr::statistics::UniformDistribution">UniformDistribution</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential" class="anchor">§</a>

### Exponential(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/struct.ExponentialDistribution.html" class="struct" title="struct datafusion::logical_expr::statistics::ExponentialDistribution">ExponentialDistribution</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian" class="anchor">§</a>

### Gaussian(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/struct.GaussianDistribution.html" class="struct" title="struct datafusion::logical_expr::statistics::GaussianDistribution">GaussianDistribution</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli" class="anchor">§</a>

### Bernoulli(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/struct.BernoulliDistribution.html" class="struct" title="struct datafusion::logical_expr::statistics::BernoulliDistribution">BernoulliDistribution</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic" class="anchor">§</a>

### Generic(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/struct.GenericDistribution.html" class="struct" title="struct datafusion::logical_expr::statistics::GenericDistribution">GenericDistribution</a>)

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#impl-Distribution" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_uniform" class="fn">new_uniform</a>(interval: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Uniform`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform "variant datafusion::logical_expr::statistics::Distribution::Uniform") distribution from the given [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_exponential" class="fn">new_exponential</a>( rate: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, offset: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, positive_tail: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Exponential`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential "variant datafusion::logical_expr::statistics::Distribution::Exponential") distribution from the given rate/offset pair, and validates the given parameters.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_gaussian" class="fn">new_gaussian</a>( mean: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, variance: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Gaussian`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian "variant datafusion::logical_expr::statistics::Distribution::Gaussian") distribution from the given mean/variance pair, and validates the given parameters.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_bernoulli" class="fn">new_bernoulli</a>(p: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Bernoulli`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli "variant datafusion::logical_expr::statistics::Distribution::Bernoulli") distribution from the given success probability, and validates the given parameters.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_generic" class="fn">new_generic</a>( mean: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, median: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, variance: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, range: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution from the given mean, median, variance, and range values after validating the given parameters.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.new_from_interval" class="fn">new_from_interval</a>( range: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution from the given range. Other parameters (mean, median and variance) are initialized with null values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.mean" class="fn">mean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extracts the mean value of this uncertain quantity, depending on its distribution:

- A [`Uniform`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform "variant datafusion::logical_expr::statistics::Distribution::Uniform") distribution’s interval determines its mean value, which is the arithmetic average of the interval endpoints.
- An [`Exponential`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential "variant datafusion::logical_expr::statistics::Distribution::Exponential") distribution’s mean is calculable by the formula `offset + 1 / λ`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian "variant datafusion::logical_expr::statistics::Distribution::Gaussian") distribution contains the mean explicitly.
- A [`Bernoulli`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli "variant datafusion::logical_expr::statistics::Distribution::Bernoulli") distribution’s mean is equal to its success probability `p`.
- A [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution *may* have it explicitly, or this information may be absent.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extracts the median value of this uncertain quantity, depending on its distribution:

- A [`Uniform`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform "variant datafusion::logical_expr::statistics::Distribution::Uniform") distribution’s interval determines its median value, which is the arithmetic average of the interval endpoints.
- An [`Exponential`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential "variant datafusion::logical_expr::statistics::Distribution::Exponential") distribution’s median is calculable by the formula `offset + ln(2) / λ`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian "variant datafusion::logical_expr::statistics::Distribution::Gaussian") distribution’s median is equal to its mean, which is specified explicitly.
- A [`Bernoulli`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli "variant datafusion::logical_expr::statistics::Distribution::Bernoulli") distribution’s median is `1` if `p > 0.5` and `0` otherwise, where `p` is the success probability.
- A [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution *may* have it explicitly, or this information may be absent.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.variance" class="fn">variance</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extracts the variance value of this uncertain quantity, depending on its distribution:

- A [`Uniform`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform "variant datafusion::logical_expr::statistics::Distribution::Uniform") distribution’s interval determines its variance value, which is calculable by the formula `(upper - lower) ^ 2 / 12`.
- An [`Exponential`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential "variant datafusion::logical_expr::statistics::Distribution::Exponential") distribution’s variance is calculable by the formula `1 / (λ ^ 2)`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian "variant datafusion::logical_expr::statistics::Distribution::Gaussian") distribution’s variance is specified explicitly.
- A [`Bernoulli`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli "variant datafusion::logical_expr::statistics::Distribution::Bernoulli") distribution’s median is given by the formula `p * (1 - p)` where `p` is the success probability.
- A [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution *may* have it explicitly, or this information may be absent.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.range" class="fn">range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extracts the range of this uncertain quantity, depending on its distribution:

- A [`Uniform`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Uniform "variant datafusion::logical_expr::statistics::Distribution::Uniform") distribution’s range is simply its interval.
- An [`Exponential`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Exponential "variant datafusion::logical_expr::statistics::Distribution::Exponential") distribution’s range is `[offset, +∞)`.
- A [`Gaussian`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Gaussian "variant datafusion::logical_expr::statistics::Distribution::Gaussian") distribution’s range is unbounded.
- A [`Bernoulli`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Bernoulli "variant datafusion::logical_expr::statistics::Distribution::Bernoulli") distribution’s range is [`Interval::UNCERTAIN`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html#associatedconstant.UNCERTAIN "associated constant datafusion::logical_expr::interval_arithmetic::Interval::UNCERTAIN"), if `p` is neither `0` nor `1`. Otherwise, it is [`Interval::CERTAINLY_FALSE`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html#associatedconstant.CERTAINLY_FALSE "associated constant datafusion::logical_expr::interval_arithmetic::Interval::CERTAINLY_FALSE") and [`Interval::CERTAINLY_TRUE`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html#associatedconstant.CERTAINLY_TRUE "associated constant datafusion::logical_expr::interval_arithmetic::Interval::CERTAINLY_TRUE"), respectively.
- A [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution is unbounded by default, but more information may be present.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.data_type" class="fn">data_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns the data type of the statistical parameters comprising this distribution.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.target_type" class="fn">target_type</a>(args: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#impl-Clone-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#impl-Debug-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#impl-PartialEq-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#impl-StructuralPartialEq-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#blanket-implementations" class="anchor">§</a>
