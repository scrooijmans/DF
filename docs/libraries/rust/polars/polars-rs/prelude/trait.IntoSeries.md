# Trait IntoSeries Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/from.rs.html#878" class="src">Source</a>

``` rust
pub unsafe trait IntoSeries {
    // Required method
    fn into_series(self) -> Series
       where Self: Sized;

    // Provided method
    fn is_series() -> bool { ... }
}
```

Expand description

Used to convert a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), `&dyn SeriesTrait` and [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") into a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

## <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#safety" class="doc-anchor">§</a>Safety

This trait is marked `unsafe` as the `is_series` return is used to transmute to `Series`. This must always return `false` except for `Series` structs.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Arc%3Cdyn+SeriesTrait%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#impl-IntoSeries-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,
