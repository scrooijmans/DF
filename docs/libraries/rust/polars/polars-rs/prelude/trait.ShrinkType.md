# Trait ShrinkType Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/eager.rs.html#5" class="src">Source</a>

``` rust
pub trait ShrinkType {
    // Required method
    fn shrink_type(&self) -> Result<Series, PolarsError>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html#tymethod.shrink_type" class="fn">shrink_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html#impl-ShrinkType-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html" class="trait" title="trait polars::prelude::ShrinkType">ShrinkType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
