# Trait LogicalType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/mod.rs.html#62" class="src">Source</a>

``` rust
pub trait LogicalType {
    // Required methods
    fn dtype(&self) -> &DataType;
    fn cast_with_options(
        &self,
        dtype: &DataType,
        options: CastOptions,
    ) -> Result<Series, PolarsError>;

    // Provided methods
    fn get_any_value(&self, _i: usize) -> Result<AnyValue<'_>, PolarsError> { ... }
    unsafe fn get_any_value_unchecked(&self, _i: usize) -> AnyValue<'_> { ... }
    fn cast(&self, dtype: &DataType) -> Result<Series, PolarsError> { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, \_i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, \_i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

##### <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#safety" class="doc-anchor">§</a>Safety

Does not do any bound checks.

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html#impl-LogicalType-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,
