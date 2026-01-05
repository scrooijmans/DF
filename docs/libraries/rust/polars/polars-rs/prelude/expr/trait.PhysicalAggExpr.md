# Trait PhysicalAggExpr Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/expr.rs.html#3" class="src">Source</a>

``` rust
pub trait PhysicalAggExpr {
    // Required methods
    fn evaluate_on_groups(
        &self,
        df: &DataFrame,
        groups: &GroupPositions,
    ) -> Result<Series, PolarsError>;
    fn root_name(&self) -> Result<&PlSmallStr, PolarsError>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/expr/trait.PhysicalAggExpr.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/expr/trait.PhysicalAggExpr.html#tymethod.evaluate_on_groups" class="fn">evaluate_on_groups</a>( &self, df: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, groups: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/expr/trait.PhysicalAggExpr.html#tymethod.root_name" class="fn">root_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/expr/trait.PhysicalAggExpr.html#implementors" class="anchor">§</a>
