# Trait SeriesRank Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/rank.rs.html#196" class="src">Source</a>

``` rust
pub trait SeriesRank: SeriesSealed {
    // Provided method
    fn rank(&self, options: RankOptions, seed: Option<u64>) -> Series { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html#method.rank" class="fn">rank</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RankOptions.html" class="struct" title="struct polars::prelude::RankOptions">RankOptions</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html#impl-SeriesRank-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html" class="trait" title="trait polars::prelude::SeriesRank">SeriesRank</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
