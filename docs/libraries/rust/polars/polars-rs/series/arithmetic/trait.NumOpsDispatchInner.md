# Trait NumOpsDispatchInner Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#4" class="src">Source</a>

``` rust
pub trait NumOpsDispatchInner: Sized + PolarsDataType {
    // Provided methods
    fn subtract(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
    fn add_to(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
    fn multiply(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
    fn divide(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
    fn remainder(
        lhs: &ChunkedArray<Self>,
        rhs: &Series,
    ) -> Result<Series, PolarsError> { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#method.subtract" class="fn">subtract</a>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#method.add_to" class="fn">add_to</a>(lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#method.multiply" class="fn">multiply</a>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#method.divide" class="fn">divide</a>(lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#method.remainder" class="fn">remainder</a>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-BinaryType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-BooleanType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-FixedSizeListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-ListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-StringType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html#impl-NumOpsDispatchInner-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.NumOpsDispatchInner.html" class="trait" title="trait polars::series::arithmetic::NumOpsDispatchInner">NumOpsDispatchInner</a> for T

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
