# Module datatypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/lib.rs.html#16" class="src">Source</a>

Expand description

## <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#data-types-supported-by-polars" class="doc-anchor">§</a>Data types supported by Polars.

At the moment Polars doesn’t include all data types available by Arrow. The goal is to incrementally support more data types and prioritize these by usability.

[See the AnyValue variants](https://docs.rs/polars/latest/polars/prelude/datatypes/enum.AnyValue.html#variants) for the data types that are currently supported.

## Modules<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/categorical/index.html" class="mod" title="mod polars::prelude::datatypes::categorical">categorical</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/time_unit/index.html" class="mod" title="mod polars::prelude::datatypes::time_unit">time_unit</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/time_zone/index.html" class="mod" title="mod polars::prelude::datatypes::time_zone">time_zone</a>

## Structs<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::datatypes::BinaryOffsetType">BinaryOffsetType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.BinaryType.html" class="struct" title="struct polars::prelude::datatypes::BinaryType">BinaryType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.BooleanType.html" class="struct" title="struct polars::prelude::datatypes::BooleanType">BooleanType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::datatypes::Categorical8Type">Categorical8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::datatypes::Categorical16Type">Categorical16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::datatypes::Categorical32Type">Categorical32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::datatypes::CategoricalMapping">CategoricalMapping</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.CategoricalType.html" class="struct" title="struct polars::prelude::datatypes::CategoricalType">CategoricalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html" class="struct" title="struct polars::prelude::datatypes::Categories">Categories</a>

A (named) object which is used to indicate which categorical data types have the same mapping.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.CompatLevel.html" class="struct" title="struct polars::prelude::datatypes::CompatLevel">CompatLevel</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.DateType.html" class="struct" title="struct polars::prelude::datatypes::DateType">DateType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.DatetimeType.html" class="struct" title="struct polars::prelude::datatypes::DatetimeType">DatetimeType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.DecimalType.html" class="struct" title="struct polars::prelude::datatypes::DecimalType">DecimalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Dimension.html" class="struct" title="struct polars::prelude::datatypes::Dimension">Dimension</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.DurationType.html" class="struct" title="struct polars::prelude::datatypes::DurationType">DurationType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.FalseT.html" class="struct" title="struct polars::prelude::datatypes::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Field.html" class="struct" title="struct polars::prelude::datatypes::Field">Field</a>

Characterizes the name and the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") of a column.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::datatypes::FixedSizeListType">FixedSizeListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html" class="struct" title="struct polars::prelude::datatypes::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float64Type.html" class="struct" title="struct polars::prelude::datatypes::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::datatypes::FrozenCategories">FrozenCategories</a>

An ordered collection of unique strings with an associated pre-computed mapping to go from string \<-\> index.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Int8Type.html" class="struct" title="struct polars::prelude::datatypes::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Int16Type.html" class="struct" title="struct polars::prelude::datatypes::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Int32Type.html" class="struct" title="struct polars::prelude::datatypes::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Int64Type.html" class="struct" title="struct polars::prelude::datatypes::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Int128Type.html" class="struct" title="struct polars::prelude::datatypes::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.ListType.html" class="struct" title="struct polars::prelude::datatypes::ListType">ListType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html" class="struct" title="struct polars::prelude::datatypes::Logical">Logical</a>

Maps a logical type to a chunked array implementation of the physical type. This saves a lot of compiler bloat and allows us to reuse functionality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.ObjectType.html" class="struct" title="struct polars::prelude::datatypes::ObjectType">ObjectType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.OwnedObject.html" class="struct" title="struct polars::prelude::datatypes::OwnedObject">OwnedObject</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::datatypes::PlSmallStr">PlSmallStr</a>

String type that inlines small strings.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StringType.html" class="struct" title="struct polars::prelude::datatypes::StringType">StringType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html" class="struct" title="struct polars::prelude::datatypes::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.TimeType.html" class="struct" title="struct polars::prelude::datatypes::TimeType">TimeType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.TimeZone.html" class="struct" title="struct polars::prelude::datatypes::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.TrueT.html" class="struct" title="struct polars::prelude::datatypes::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.UInt8Type.html" class="struct" title="struct polars::prelude::datatypes::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.UInt16Type.html" class="struct" title="struct polars::prelude::datatypes::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.UInt32Type.html" class="struct" title="struct polars::prelude::datatypes::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.UInt64Type.html" class="struct" title="struct polars::prelude::datatypes::UInt64Type">UInt64Type</a>

## Enums<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.AnyValue.html" class="enum" title="enum polars::prelude::datatypes::AnyValue">AnyValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::datatypes::ArrowDataType">ArrowDataType</a>

The set of supported logical types in this crate.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.ArrowTimeUnit.html" class="enum" title="enum polars::prelude::datatypes::ArrowTimeUnit">ArrowTimeUnit</a>

The time units defined in Arrow.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::datatypes::CategoricalPhysical">CategoricalPhysical</a>

The physical datatype backing a categorical / enum.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.DataType.html" class="enum" title="enum polars::prelude::datatypes::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.ReshapeDimension.html" class="enum" title="enum polars::prelude::datatypes::ReshapeDimension">ReshapeDimension</a>

A dimension in a reshape.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.TimeUnit.html" class="enum" title="enum polars::prelude::datatypes::TimeUnit">TimeUnit</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/enum.UnknownKind.html" class="enum" title="enum polars::prelude::datatypes::UnknownKind">UnknownKind</a>

## Constants<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/constant.IDX_DTYPE.html" class="constant" title="constant polars::prelude::datatypes::IDX_DTYPE">IDX_DTYPE</a>

## Statics<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#statics" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/static.EXTENSION_NAME.html" class="static" title="static polars::prelude::datatypes::EXTENSION_NAME">EXTENSION_NAME</a>

## Traits<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayCollectIterExt.html" class="trait" title="trait polars::prelude::datatypes::ArrayCollectIterExt">ArrayCollectIterExt</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::datatypes::ArrayFromIter">ArrayFromIter</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::datatypes::ArrayFromIterDtype">ArrayFromIterDtype</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.AsRefDataType.html" class="trait" title="trait polars::prelude::datatypes::AsRefDataType">AsRefDataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.CatNative.html" class="trait" title="trait polars::prelude::datatypes::CatNative">CatNative</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.CategoricalPhysicalDtypeExt.html" class="trait" title="trait polars::prelude::datatypes::CategoricalPhysicalDtypeExt">CategoricalPhysicalDtypeExt</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.GetAnyValue.html" class="trait" title="trait polars::prelude::datatypes::GetAnyValue">GetAnyValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::datatypes::InitHashMaps">InitHashMaps</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.InitHashMaps2.html" class="trait" title="trait polars::prelude::datatypes::InitHashMaps2">InitHashMaps2</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoMetadata.html" class="trait" title="trait polars::prelude::datatypes::IntoMetadata">IntoMetadata</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html" class="trait" title="trait polars::prelude::datatypes::IntoScalar">IntoScalar</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.LogicalType.html" class="trait" title="trait polars::prelude::datatypes::LogicalType">LogicalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html" class="trait" title="trait polars::prelude::datatypes::MetaDataExt">MetaDataExt</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.NumericNative.html" class="trait" title="trait polars::prelude::datatypes::NumericNative">NumericNative</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::datatypes::PolarsCategoricalType">PolarsCategoricalType</a>

Safety

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::datatypes::PolarsDataType">PolarsDataType</a>

Safety

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsFloatType.html" class="trait" title="trait polars::prelude::datatypes::PolarsFloatType">PolarsFloatType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::datatypes::PolarsIntegerType">PolarsIntegerType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::datatypes::PolarsNumericType">PolarsNumericType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::datatypes::PolarsPhysicalType">PolarsPhysicalType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.SchemaExtPl.html" class="trait" title="trait polars::prelude::datatypes::SchemaExtPl">SchemaExtPl</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.StaticArray.html" class="trait" title="trait polars::prelude::datatypes::StaticArray">StaticArray</a>

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/fn.ensure_same_categories.html" class="fn" title="fn polars::prelude::datatypes::ensure_same_categories">ensure_same_categories</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/fn.ensure_same_frozen_categories.html" class="fn" title="fn polars::prelude::datatypes::ensure_same_frozen_categories">ensure_same_frozen_categories</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/fn.merge_dtypes.html" class="fn" title="fn polars::prelude::datatypes::merge_dtypes">merge_dtypes</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/fn.unpack_dtypes.html" class="fn" title="fn polars::prelude::datatypes::unpack_dtypes">unpack_dtypes</a>

## Type Aliases<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.ArrayChunked.html" class="type" title="type polars::prelude::datatypes::ArrayChunked">ArrayChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.BinaryChunked.html" class="type" title="type polars::prelude::datatypes::BinaryChunked">BinaryChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.BinaryOffsetChunked.html" class="type" title="type polars::prelude::datatypes::BinaryOffsetChunked">BinaryOffsetChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.BooleanChunked.html" class="type" title="type polars::prelude::datatypes::BooleanChunked">BooleanChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.CatSize.html" class="type" title="type polars::prelude::datatypes::CatSize">CatSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Categorical8Chunked.html" class="type" title="type polars::prelude::datatypes::Categorical8Chunked">Categorical8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Categorical16Chunked.html" class="type" title="type polars::prelude::datatypes::Categorical16Chunked">Categorical16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Categorical32Chunked.html" class="type" title="type polars::prelude::datatypes::Categorical32Chunked">Categorical32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.CategoricalChunked.html" class="type" title="type polars::prelude::datatypes::CategoricalChunked">CategoricalChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DateChunked.html" class="type" title="type polars::prelude::datatypes::DateChunked">DateChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DatetimeChunked.html" class="type" title="type polars::prelude::datatypes::DatetimeChunked">DatetimeChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DecimalChunked.html" class="type" title="type polars::prelude::datatypes::DecimalChunked">DecimalChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DurationChunked.html" class="type" title="type polars::prelude::datatypes::DurationChunked">DurationChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.FieldRef.html" class="type" title="type polars::prelude::datatypes::FieldRef">FieldRef</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Float32Chunked.html" class="type" title="type polars::prelude::datatypes::Float32Chunked">Float32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Float64Chunked.html" class="type" title="type polars::prelude::datatypes::Float64Chunked">Float64Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.IdxArr.html" class="type" title="type polars::prelude::datatypes::IdxArr">IdxArr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.IdxCa.html" class="type" title="type polars::prelude::datatypes::IdxCa">IdxCa</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.IdxType.html" class="type" title="type polars::prelude::datatypes::IdxType">IdxType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Int8Chunked.html" class="type" title="type polars::prelude::datatypes::Int8Chunked">Int8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Int16Chunked.html" class="type" title="type polars::prelude::datatypes::Int16Chunked">Int16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Int32Chunked.html" class="type" title="type polars::prelude::datatypes::Int32Chunked">Int32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Int64Chunked.html" class="type" title="type polars::prelude::datatypes::Int64Chunked">Int64Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.Int128Chunked.html" class="type" title="type polars::prelude::datatypes::Int128Chunked">Int128Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.ListChunked.html" class="type" title="type polars::prelude::datatypes::ListChunked">ListChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.ObjectChunked.html" class="type" title="type polars::prelude::datatypes::ObjectChunked">ObjectChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlHashMap.html" class="type" title="type polars::prelude::datatypes::PlHashMap">PlHashMap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlHashSet.html" class="type" title="type polars::prelude::datatypes::PlHashSet">PlHashSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlIdHashMap.html" class="type" title="type polars::prelude::datatypes::PlIdHashMap">PlIdHashMap</a>

This hashmap uses an IdHasher

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlIndexMap.html" class="type" title="type polars::prelude::datatypes::PlIndexMap">PlIndexMap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlIndexSet.html" class="type" title="type polars::prelude::datatypes::PlIndexSet">PlIndexSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.PlRandomState.html" class="type" title="type polars::prelude::datatypes::PlRandomState">PlRandomState</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.StringChunked.html" class="type" title="type polars::prelude::datatypes::StringChunked">StringChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.TimeChunked.html" class="type" title="type polars::prelude::datatypes::TimeChunked">TimeChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.UInt8Chunked.html" class="type" title="type polars::prelude::datatypes::UInt8Chunked">UInt8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.UInt16Chunked.html" class="type" title="type polars::prelude::datatypes::UInt16Chunked">UInt16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.UInt32Chunked.html" class="type" title="type polars::prelude::datatypes::UInt32Chunked">UInt32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.UInt64Chunked.html" class="type" title="type polars::prelude::datatypes::UInt64Chunked">UInt64Chunked</a>
