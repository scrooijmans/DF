# Trait AsofJoinBy Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/asof/groups.rs.html#494" class="src">Source</a>

``` rust
pub trait AsofJoinBy: IntoDf {
    // Provided method
    fn join_asof_by<I, S>(
        &self,
        other: &DataFrame,
        left_on: &str,
        right_on: &str,
        left_by: I,
        right_by: I,
        strategy: AsofStrategy,
        tolerance: Option<AnyValue<'static>>,
        allow_eq: bool,
        check_sortedness: bool,
    ) -> Result<DataFrame, PolarsError>
       where I: IntoIterator<Item = S>,
             S: AsRef<str> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#method.join_asof_by" class="fn">join_asof_by</a>\<I, S\>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, right_on: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, left_by: I, right_by: I, strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AsofStrategy.html" class="enum" title="enum polars::prelude::AsofStrategy">AsofStrategy</a>, tolerance: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>\>, allow_eq: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, check_sortedness: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

This is similar to a left-join except that we match on nearest key rather than equal keys. The keys must be sorted to perform an asof join. This is a special implementation of an asof join that searches for the nearest keys within a subgroup set by `by`.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#impl-AsofJoinBy-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html" class="trait" title="trait polars::prelude::AsofJoinBy">AsofJoinBy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
