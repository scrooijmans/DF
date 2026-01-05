# Trait MinMaxHorizontal Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/horizontal.rs.html#25" class="src">Source</a>

``` rust
pub trait MinMaxHorizontal {
    // Required methods
    fn min_horizontal(&self) -> Result<Option<Column>, PolarsError>;
    fn max_horizontal(&self) -> Result<Option<Column>, PolarsError>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#tymethod.min_horizontal" class="fn">min_horizontal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate the column horizontally to their min values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#tymethod.max_horizontal" class="fn">max_horizontal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate the column horizontally to their max values.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#impl-MinMaxHorizontal-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html" class="trait" title="trait polars::prelude::MinMaxHorizontal">MinMaxHorizontal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
