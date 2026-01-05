# Trait RoundSeries Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/round.rs.html#20" class="src">Source</a>

``` rust
pub trait RoundSeries: SeriesSealed {
    // Provided methods
    fn round(
        &self,
        decimals: u32,
        mode: RoundMode,
    ) -> Result<Series, PolarsError> { ... }
    fn round_sig_figs(&self, digits: i32) -> Result<Series, PolarsError> { ... }
    fn floor(&self) -> Result<Series, PolarsError> { ... }
    fn ceil(&self) -> Result<Series, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.round" class="fn">round</a>(&self, decimals: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, mode: <a href="https://docs.rs/polars/latest/polars/prelude/enum.RoundMode.html" class="enum" title="enum polars::prelude::RoundMode">RoundMode</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Round underlying floating point array to given decimal.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.round_sig_figs" class="fn">round_sig_figs</a>(&self, digits: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.floor" class="fn">floor</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Floor underlying floating point array to the lowest integers smaller or equal to the float value.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.ceil" class="fn">ceil</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Ceil underlying floating point array to the highest integers smaller or equal to the float value.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#impl-RoundSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html" class="trait" title="trait polars::prelude::RoundSeries">RoundSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
