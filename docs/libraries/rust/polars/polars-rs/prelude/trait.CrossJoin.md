# Trait CrossJoin Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/cross_join.rs.html#45" class="src">Source</a>

``` rust
pub trait CrossJoin: IntoDf {
    // Provided method
    fn cross_join(
        &self,
        other: &DataFrame,
        suffix: Option<PlSmallStr>,
        slice: Option<(i64, usize)>,
    ) -> Result<DataFrame, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html#method.cross_join" class="fn">cross_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, suffix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Creates the Cartesian product from both frames, preserves the order of the left keys.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html#impl-CrossJoin-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html" class="trait" title="trait polars::prelude::CrossJoin">CrossJoin</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
