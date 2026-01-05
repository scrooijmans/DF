# Trait IntoVec Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/utils/mod.rs.html#1138" class="src">Source</a>

``` rust
pub trait IntoVec<T> {
    // Required method
    fn into_vec(self) -> Vec<T>;
}
```

Expand description

Convenience for `x.into_iter().map(Into::into).collect()` using an `into_vec()` function.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html#tymethod.into_vec" class="fn">into_vec</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html#impl-IntoVec%3CPlSmallStr%3E-for-I" class="anchor">§</a>

### impl\<I, S\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\> for I

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,
