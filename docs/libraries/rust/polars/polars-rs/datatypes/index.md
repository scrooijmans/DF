# Module datatypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/lib.rs.html#16" class="src">Source</a>

Expand description

## <a href="https://docs.rs/polars/latest/polars/datatypes/index.html#data-types-supported-by-polars" class="doc-anchor">§</a>Data types supported by Polars.

At the moment Polars doesn’t include all data types available by Arrow. The goal is to incrementally support more data types and prioritize these by usability.

[See the AnyValue variants](https://docs.rs/polars/latest/polars/datatypes/enum.AnyValue.html#variants) for the data types that are currently supported.

## Modules<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/index.html" class="mod" title="mod polars::datatypes::categorical">categorical</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/time_unit/index.html" class="mod" title="mod polars::datatypes::time_unit">time_unit</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/time_zone/index.html" class="mod" title="mod polars::datatypes::time_zone">time_zone</a>

## Structs<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.BinaryOffsetType.html" class="struct" title="struct polars::datatypes::BinaryOffsetType">BinaryOffsetType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.BinaryType.html" class="struct" title="struct polars::datatypes::BinaryType">BinaryType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.BooleanType.html" class="struct" title="struct polars::datatypes::BooleanType">BooleanType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Categorical8Type.html" class="struct" title="struct polars::datatypes::Categorical8Type">Categorical8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Categorical16Type.html" class="struct" title="struct polars::datatypes::Categorical16Type">Categorical16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Categorical32Type.html" class="struct" title="struct polars::datatypes::Categorical32Type">Categorical32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html" class="struct" title="struct polars::datatypes::CategoricalMapping">CategoricalMapping</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalType.html" class="struct" title="struct polars::datatypes::CategoricalType">CategoricalType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Categories.html" class="struct" title="struct polars::datatypes::Categories">Categories</a>

A (named) object which is used to indicate which categorical data types have the same mapping.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CompatLevel.html" class="struct" title="struct polars::datatypes::CompatLevel">CompatLevel</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.DateType.html" class="struct" title="struct polars::datatypes::DateType">DateType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.DatetimeType.html" class="struct" title="struct polars::datatypes::DatetimeType">DatetimeType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.DecimalType.html" class="struct" title="struct polars::datatypes::DecimalType">DecimalType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Dimension.html" class="struct" title="struct polars::datatypes::Dimension">Dimension</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.DurationType.html" class="struct" title="struct polars::datatypes::DurationType">DurationType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FalseT.html" class="struct" title="struct polars::datatypes::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html" class="struct" title="struct polars::datatypes::Field">Field</a>

Characterizes the name and the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") of a column.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FixedSizeListType.html" class="struct" title="struct polars::datatypes::FixedSizeListType">FixedSizeListType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Float32Type.html" class="struct" title="struct polars::datatypes::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Float64Type.html" class="struct" title="struct polars::datatypes::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html" class="struct" title="struct polars::datatypes::FrozenCategories">FrozenCategories</a>

An ordered collection of unique strings with an associated pre-computed mapping to go from string \<-\> index.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Int8Type.html" class="struct" title="struct polars::datatypes::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Int16Type.html" class="struct" title="struct polars::datatypes::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Int32Type.html" class="struct" title="struct polars::datatypes::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Int64Type.html" class="struct" title="struct polars::datatypes::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Int128Type.html" class="struct" title="struct polars::datatypes::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.ListType.html" class="struct" title="struct polars::datatypes::ListType">ListType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Logical.html" class="struct" title="struct polars::datatypes::Logical">Logical</a>

Maps a logical type to a chunked array implementation of the physical type. This saves a lot of compiler bloat and allows us to reuse functionality.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.ObjectType.html" class="struct" title="struct polars::datatypes::ObjectType">ObjectType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html" class="struct" title="struct polars::datatypes::OwnedObject">OwnedObject</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.PlSmallStr.html" class="struct" title="struct polars::datatypes::PlSmallStr">PlSmallStr</a>

String type that inlines small strings.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.StringType.html" class="struct" title="struct polars::datatypes::StringType">StringType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.StructType.html" class="struct" title="struct polars::datatypes::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeType.html" class="struct" title="struct polars::datatypes::TimeType">TimeType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html" class="struct" title="struct polars::datatypes::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TrueT.html" class="struct" title="struct polars::datatypes::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.UInt8Type.html" class="struct" title="struct polars::datatypes::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.UInt16Type.html" class="struct" title="struct polars::datatypes::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.UInt32Type.html" class="struct" title="struct polars::datatypes::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.UInt64Type.html" class="struct" title="struct polars::datatypes::UInt64Type">UInt64Type</a>

## Enums<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.AnyValue.html" class="enum" title="enum polars::datatypes::AnyValue">AnyValue</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html" class="enum" title="enum polars::datatypes::ArrowDataType">ArrowDataType</a>

The set of supported logical types in this crate.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowTimeUnit.html" class="enum" title="enum polars::datatypes::ArrowTimeUnit">ArrowTimeUnit</a>

The time units defined in Arrow.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.CategoricalPhysical.html" class="enum" title="enum polars::datatypes::CategoricalPhysical">CategoricalPhysical</a>

The physical datatype backing a categorical / enum.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.DataType.html" class="enum" title="enum polars::datatypes::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ReshapeDimension.html" class="enum" title="enum polars::datatypes::ReshapeDimension">ReshapeDimension</a>

A dimension in a reshape.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.TimeUnit.html" class="enum" title="enum polars::datatypes::TimeUnit">TimeUnit</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.UnknownKind.html" class="enum" title="enum polars::datatypes::UnknownKind">UnknownKind</a>

## Constants<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/constant.IDX_DTYPE.html" class="constant" title="constant polars::datatypes::IDX_DTYPE">IDX_DTYPE</a>

## Statics<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#statics" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/static.EXTENSION_NAME.html" class="static" title="static polars::datatypes::EXTENSION_NAME">EXTENSION_NAME</a>

## Traits<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayCollectIterExt.html" class="trait" title="trait polars::datatypes::ArrayCollectIterExt">ArrayCollectIterExt</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html" class="trait" title="trait polars::datatypes::ArrayFromIter">ArrayFromIter</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::datatypes::ArrayFromIterDtype">ArrayFromIterDtype</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.AsRefDataType.html" class="trait" title="trait polars::datatypes::AsRefDataType">AsRefDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html" class="trait" title="trait polars::datatypes::CatNative">CatNative</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CategoricalPhysicalDtypeExt.html" class="trait" title="trait polars::datatypes::CategoricalPhysicalDtypeExt">CategoricalPhysicalDtypeExt</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.GetAnyValue.html" class="trait" title="trait polars::datatypes::GetAnyValue">GetAnyValue</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html" class="trait" title="trait polars::datatypes::InitHashMaps">InitHashMaps</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps2.html" class="trait" title="trait polars::datatypes::InitHashMaps2">InitHashMaps2</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.IntoMetadata.html" class="trait" title="trait polars::datatypes::IntoMetadata">IntoMetadata</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.IntoScalar.html" class="trait" title="trait polars::datatypes::IntoScalar">IntoScalar</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.LogicalType.html" class="trait" title="trait polars::datatypes::LogicalType">LogicalType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.MetaDataExt.html" class="trait" title="trait polars::datatypes::MetaDataExt">MetaDataExt</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html" class="trait" title="trait polars::datatypes::NumericNative">NumericNative</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsCategoricalType.html" class="trait" title="trait polars::datatypes::PolarsCategoricalType">PolarsCategoricalType</a>

Safety

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsDataType.html" class="trait" title="trait polars::datatypes::PolarsDataType">PolarsDataType</a>

Safety

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsFloatType.html" class="trait" title="trait polars::datatypes::PolarsFloatType">PolarsFloatType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsIntegerType.html" class="trait" title="trait polars::datatypes::PolarsIntegerType">PolarsIntegerType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html" class="trait" title="trait polars::datatypes::PolarsNumericType">PolarsNumericType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsPhysicalType.html" class="trait" title="trait polars::datatypes::PolarsPhysicalType">PolarsPhysicalType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.SchemaExtPl.html" class="trait" title="trait polars::datatypes::SchemaExtPl">SchemaExtPl</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.StaticArray.html" class="trait" title="trait polars::datatypes::StaticArray">StaticArray</a>

## Functions<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/fn.ensure_same_categories.html" class="fn" title="fn polars::datatypes::ensure_same_categories">ensure_same_categories</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/fn.ensure_same_frozen_categories.html" class="fn" title="fn polars::datatypes::ensure_same_frozen_categories">ensure_same_frozen_categories</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/fn.merge_dtypes.html" class="fn" title="fn polars::datatypes::merge_dtypes">merge_dtypes</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/fn.unpack_dtypes.html" class="fn" title="fn polars::datatypes::unpack_dtypes">unpack_dtypes</a>

## Type Aliases<a href="https://docs.rs/polars/latest/polars/datatypes/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.ArrayChunked.html" class="type" title="type polars::datatypes::ArrayChunked">ArrayChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.BinaryChunked.html" class="type" title="type polars::datatypes::BinaryChunked">BinaryChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.BinaryOffsetChunked.html" class="type" title="type polars::datatypes::BinaryOffsetChunked">BinaryOffsetChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.BooleanChunked.html" class="type" title="type polars::datatypes::BooleanChunked">BooleanChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.CatSize.html" class="type" title="type polars::datatypes::CatSize">CatSize</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Categorical8Chunked.html" class="type" title="type polars::datatypes::Categorical8Chunked">Categorical8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Categorical16Chunked.html" class="type" title="type polars::datatypes::Categorical16Chunked">Categorical16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Categorical32Chunked.html" class="type" title="type polars::datatypes::Categorical32Chunked">Categorical32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.CategoricalChunked.html" class="type" title="type polars::datatypes::CategoricalChunked">CategoricalChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.DateChunked.html" class="type" title="type polars::datatypes::DateChunked">DateChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.DatetimeChunked.html" class="type" title="type polars::datatypes::DatetimeChunked">DatetimeChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.DecimalChunked.html" class="type" title="type polars::datatypes::DecimalChunked">DecimalChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.DurationChunked.html" class="type" title="type polars::datatypes::DurationChunked">DurationChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.FieldRef.html" class="type" title="type polars::datatypes::FieldRef">FieldRef</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Float32Chunked.html" class="type" title="type polars::datatypes::Float32Chunked">Float32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Float64Chunked.html" class="type" title="type polars::datatypes::Float64Chunked">Float64Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.IdxArr.html" class="type" title="type polars::datatypes::IdxArr">IdxArr</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.IdxCa.html" class="type" title="type polars::datatypes::IdxCa">IdxCa</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.IdxType.html" class="type" title="type polars::datatypes::IdxType">IdxType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Int8Chunked.html" class="type" title="type polars::datatypes::Int8Chunked">Int8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Int16Chunked.html" class="type" title="type polars::datatypes::Int16Chunked">Int16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Int32Chunked.html" class="type" title="type polars::datatypes::Int32Chunked">Int32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Int64Chunked.html" class="type" title="type polars::datatypes::Int64Chunked">Int64Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.Int128Chunked.html" class="type" title="type polars::datatypes::Int128Chunked">Int128Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.ListChunked.html" class="type" title="type polars::datatypes::ListChunked">ListChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.ObjectChunked.html" class="type" title="type polars::datatypes::ObjectChunked">ObjectChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlHashMap.html" class="type" title="type polars::datatypes::PlHashMap">PlHashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlHashSet.html" class="type" title="type polars::datatypes::PlHashSet">PlHashSet</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlIdHashMap.html" class="type" title="type polars::datatypes::PlIdHashMap">PlIdHashMap</a>

This hashmap uses an IdHasher

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlIndexMap.html" class="type" title="type polars::datatypes::PlIndexMap">PlIndexMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlIndexSet.html" class="type" title="type polars::datatypes::PlIndexSet">PlIndexSet</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.PlRandomState.html" class="type" title="type polars::datatypes::PlRandomState">PlRandomState</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.StringChunked.html" class="type" title="type polars::datatypes::StringChunked">StringChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.TimeChunked.html" class="type" title="type polars::datatypes::TimeChunked">TimeChunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.UInt8Chunked.html" class="type" title="type polars::datatypes::UInt8Chunked">UInt8Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.UInt16Chunked.html" class="type" title="type polars::datatypes::UInt16Chunked">UInt16Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.UInt32Chunked.html" class="type" title="type polars::datatypes::UInt32Chunked">UInt32Chunked</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/type.UInt64Chunked.html" class="type" title="type polars::datatypes::UInt64Chunked">UInt64Chunked</a>
