# Trait PolarsPhysicalType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#97" class="src">Source</a>

``` rust
pub trait PolarsPhysicalType: PolarsDataType {
    // Required method
    fn ca_into_series(ca: ChunkedArray<Self>) -> Series;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#tymethod.ca_into_series" class="fn">ca_into_series</a>(ca: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-BinaryOffsetType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-BinaryType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-BooleanType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-FixedSizeListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Int8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-Int128Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-ListType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-StringType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-StructType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-UInt8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-UInt16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-UInt32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html#impl-PolarsPhysicalType-for-ObjectType%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,
