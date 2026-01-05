# Trait NumOpsDispatchChecked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#167" class="src">Source</a>

``` rust
pub trait NumOpsDispatchChecked {
    // Required methods
    fn checked_div(&self, rhs: &Series) -> Result<Series, PolarsError>;
    fn checked_div_num<T>(&self, _rhs: T) -> Result<Series, PolarsError>
       where T: ToPrimitive;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#tymethod.checked_div" class="fn">checked_div</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checked integer division. Computes self / rhs, returning None if rhs == 0 or the division results in overflow.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#tymethod.checked_div_num" class="fn">checked_div_num</a>\<T\>(&self, \_rhs: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#impl-NumOpsDispatchChecked-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html" class="trait" title="trait polars::prelude::NumOpsDispatchChecked">NumOpsDispatchChecked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#impl-NumOpsDispatchChecked-for-ChunkedArray%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html" class="trait" title="trait polars::prelude::NumOpsDispatchChecked">NumOpsDispatchChecked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<S\>

where S: <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html" class="trait" title="trait polars::series::arithmetic::checked::NumOpsDispatchCheckedInner">NumOpsDispatchCheckedInner</a>,
