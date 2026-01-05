# Trait ArgAgg Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/arg_min_max.rs.html#12" class="src">Source</a>

``` rust
pub trait ArgAgg {
    // Required methods
    fn arg_min(&self) -> Option<usize>;
    fn arg_max(&self) -> Option<usize>;
}
```

Available on **crate feature `polars-ops`** only.

Expand description

Argmin/ Argmax

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#tymethod.arg_min" class="fn">arg_min</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the index of the minimal value

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#tymethod.arg_max" class="fn">arg_max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the index of the maximal value

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#impl-ArgAgg-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html" class="trait" title="trait polars::prelude::ArgAgg">ArgAgg</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
