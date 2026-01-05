# Trait NumOpsDispatchCheckedInner Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#154" class="src">Source</a>

``` rust
pub trait NumOpsDispatchCheckedInner: Sized + PolarsDataType {
    // Provided methods
    fn checked_div(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
    fn checked_div_num<T>(
        lhs: &ChunkedArray<Self>,
        _rhs: T,
    ) -> Result<Series, PolarsError>
       where T: ToPrimitive { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#method.checked_div" class="fn">checked_div</a>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checked integer division. Computes self / rhs, returning None if rhs == 0 or the division results in overflow.

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#method.checked_div_num" class="fn">checked_div_num</a>\<T\>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, \_rhs: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#impl-NumOpsDispatchCheckedInner-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html" class="trait" title="trait polars::series::arithmetic::checked::NumOpsDispatchCheckedInner">NumOpsDispatchCheckedInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#impl-NumOpsDispatchCheckedInner-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html" class="trait" title="trait polars::series::arithmetic::checked::NumOpsDispatchCheckedInner">NumOpsDispatchCheckedInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#impl-NumOpsDispatchCheckedInner-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html" class="trait" title="trait polars::series::arithmetic::checked::NumOpsDispatchCheckedInner">NumOpsDispatchCheckedInner</a> for T

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/ops/checked/trait.CheckedDiv.html" class="trait" title="trait num_traits::ops::checked::CheckedDiv">CheckedDiv</a>\<Output = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>, Output = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/ops/checked/trait.CheckedDiv.html" class="trait" title="trait num_traits::ops::checked::CheckedDiv">CheckedDiv</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/identities/trait.Zero.html" class="trait" title="trait num_traits::identities::Zero">Zero</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/identities/trait.One.html" class="trait" title="trait num_traits::identities::One">One</a>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::PolarsIntegerType">PolarsIntegerType</a>,
