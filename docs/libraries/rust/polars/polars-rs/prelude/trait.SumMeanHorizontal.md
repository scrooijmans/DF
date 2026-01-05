# Trait SumMeanHorizontal Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/horizontal.rs.html#47" class="src">Source</a>

``` rust
pub trait SumMeanHorizontal {
    // Required methods
    fn sum_horizontal(
        &self,
        null_strategy: NullStrategy,
    ) -> Result<Option<Column>, PolarsError>;
    fn mean_horizontal(
        &self,
        null_strategy: NullStrategy,
    ) -> Result<Option<Column>, PolarsError>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#tymethod.sum_horizontal" class="fn">sum_horizontal</a>( &self, null_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.NullStrategy.html" class="enum" title="enum polars::prelude::NullStrategy">NullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sum all values horizontally across columns.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#tymethod.mean_horizontal" class="fn">mean_horizontal</a>( &self, null_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.NullStrategy.html" class="enum" title="enum polars::prelude::NullStrategy">NullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the mean of all numeric values horizontally across columns.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#impl-SumMeanHorizontal-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html" class="trait" title="trait polars::prelude::SumMeanHorizontal">SumMeanHorizontal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
