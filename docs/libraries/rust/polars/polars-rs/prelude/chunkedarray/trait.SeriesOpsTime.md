# Trait SeriesOpsTime Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/rolling_window/dispatch.rs.html#156" class="src">Source</a>

``` rust
pub trait SeriesOpsTime: AsSeries {
Show 14 methods    // Provided methods
    fn rolling_mean_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_mean(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_sum_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_sum(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_quantile_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_quantile(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_min_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_min(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_max_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_max(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_var_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_var(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_std_by(
        &self,
        by: &Series,
        options: RollingOptionsDynamicWindow,
    ) -> Result<Series, PolarsError> { ... }
    fn rolling_std(
        &self,
        options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
}
```

Available on **crate feature `temporal`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_mean_by" class="fn">rolling_mean_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling mean to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_mean" class="fn">rolling_mean</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling mean to a Series.

See: \[`RollingAgg::rolling_mean`\]

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_sum_by" class="fn">rolling_sum_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling sum to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_sum" class="fn">rolling_sum</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling sum to a Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_quantile_by" class="fn">rolling_quantile_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling quantile to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_quantile" class="fn">rolling_quantile</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling quantile to a Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_min_by" class="fn">rolling_min_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling min to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_min" class="fn">rolling_min</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling min to a Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_max_by" class="fn">rolling_max_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling max to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_max" class="fn">rolling_max</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling max to a Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_var_by" class="fn">rolling_var_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling variance to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_var" class="fn">rolling_var</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling variance to a Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_std_by" class="fn">rolling_std_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling std_dev to a Series based on another Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#method.rolling_std" class="fn">rolling_std</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling std_dev to a Series.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.SeriesOpsTime.html#impl-SeriesOpsTime-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html" class="trait" title="trait polars::prelude::SeriesOpsTime">SeriesOpsTime</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
