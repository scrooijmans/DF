# Trait ToDummies Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/to_dummies.rs.html#15" class="src">Source</a>

``` rust
pub trait ToDummies {
    // Required method
    fn to_dummies(
        &self,
        separator: Option<&str>,
        drop_first: bool,
        drop_nulls: bool,
    ) -> Result<DataFrame, PolarsError>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html#tymethod.to_dummies" class="fn">to_dummies</a>( &self, separator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, drop_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, drop_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html#impl-ToDummies-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html" class="trait" title="trait polars::prelude::ToDummies">ToDummies</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
