# Module datatypes Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/datatypes/mod.rs.html#18-32" class="src">Source</a>

Expand description

Defines the logical data types of Arrow arrays.

The most important things you might be looking for are:

- [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema") to describe a schema.
- [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") to describe one field within a schema.
- [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") to describe the type of a field.

## Structs<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>  
[`BinaryViewType`](https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html "struct arrow::datatypes::BinaryViewType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BooleanType.html" class="struct" title="struct arrow::datatypes::BooleanType">BooleanType</a>  
A boolean datatype

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>  
32-bit date type: the elapsed time since UNIX epoch in days (32 bits).

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>  
64-bit date type: the elapsed time since UNIX epoch in milliseconds (64 bits). Values must be divisible by `86_400_000`. See [`DataType::Date64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Date64 "variant arrow::datatypes::DataType::Date64") for more details.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>  
The decimal type for a Decimal32Array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>  
The decimal type for a Decimal64Array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>  
The decimal type for a Decimal128Array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>  
The decimal type for a Decimal256Array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>  
Elapsed time type: microseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>  
Elapsed time type: milliseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>  
Elapsed time type: nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>  
Elapsed time type: seconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>  
Describes a single column in a [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema").

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>  
A cheaply cloneable, owned slice of [`FieldRef`](https://docs.rs/arrow/latest/arrow/datatypes/type.FieldRef.html "type arrow::datatypes::FieldRef")

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>  
16-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>  
32-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>  
64-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>  
[`ByteArrayType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html "trait arrow::datatypes::ByteArrayType") for binary arrays

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>  
[`ByteArrayType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html "trait arrow::datatypes::ByteArrayType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>  
A signed 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>  
Signed 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>  
Signed 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>  
Signed 64-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>  
Value of an IntervalDayTime array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>  
“Calendar” interval type: days and milliseconds. See [`IntervalDayTime`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html "struct arrow::datatypes::IntervalDayTime") for more details.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>  
Value of an IntervalMonthDayNano array

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>  
“Calendar” interval type: months, days, and nanoseconds. See [`IntervalMonthDayNano`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html "struct arrow::datatypes::IntervalMonthDayNano") for more details.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>  
32-bit “calendar” interval type: the number of whole months.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>  
Describes the meta-data of an ordered sequence of relative types.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct arrow::datatypes::SchemaBuilder">SchemaBuilder</a>  
A builder to facilitate building a [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema") from iteratively from [`FieldRef`](https://docs.rs/arrow/latest/arrow/datatypes/type.FieldRef.html "type arrow::datatypes::FieldRef")

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>  
[`ByteViewType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html "trait arrow::datatypes::ByteViewType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>  
32-bit time type: the elapsed time since midnight in milliseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>  
32-bit time type: the elapsed time since midnight in seconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>  
64-bit time type: the elapsed time since midnight in microseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>  
64-bit time type: the elapsed time since midnight in nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>  
Timestamp microsecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>  
Timestamp millisecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>  
Timestamp nanosecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>  
Timestamp second type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>  
Unsigned 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>  
Unsigned 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>  
Unsigned 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>  
Unsigned 64-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct arrow::datatypes::UnionFields">UnionFields</a>  
A cheaply cloneable, owned collection of [`FieldRef`](https://docs.rs/arrow/latest/arrow/datatypes/type.FieldRef.html "type arrow::datatypes::FieldRef") and their corresponding type ids

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>  
A signed 256-bit integer

## Enums<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>  
Datatypes supported by this implementation of Apache Arrow.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.IntervalUnit.html" class="enum" title="enum arrow::datatypes::IntervalUnit">IntervalUnit</a>  
YEAR_MONTH, DAY_TIME, MONTH_DAY_NANO interval in SQL style.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a>  
An absolute length of time in seconds, milliseconds, microseconds or nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.UnionMode.html" class="enum" title="enum arrow::datatypes::UnionMode">UnionMode</a>  
Sparse or Dense union layouts

## Constants<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL32_DEFAULT_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL32_DEFAULT_SCALE">DECIMAL32_DEFAULT_SCALE</a>  
The default scale for [DataType::Decimal32](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL32_MAX_PRECISION.html" class="constant" title="constant arrow::datatypes::DECIMAL32_MAX_PRECISION">DECIMAL32_MAX_PRECISION</a>  
The maximum precision for [DataType::Decimal32](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL32_MAX_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL32_MAX_SCALE">DECIMAL32_MAX_SCALE</a>  
The maximum scale for [DataType::Decimal32](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL64_DEFAULT_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL64_DEFAULT_SCALE">DECIMAL64_DEFAULT_SCALE</a>  
The default scale for [DataType::Decimal64](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL64_MAX_PRECISION.html" class="constant" title="constant arrow::datatypes::DECIMAL64_MAX_PRECISION">DECIMAL64_MAX_PRECISION</a>  
The maximum precision for [DataType::Decimal64](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL64_MAX_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL64_MAX_SCALE">DECIMAL64_MAX_SCALE</a>  
The maximum scale for [DataType::Decimal64](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL128_MAX_PRECISION.html" class="constant" title="constant arrow::datatypes::DECIMAL128_MAX_PRECISION">DECIMAL128_MAX_PRECISION</a>  
The maximum precision for [DataType::Decimal128](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL128_MAX_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL128_MAX_SCALE">DECIMAL128_MAX_SCALE</a>  
The maximum scale for [DataType::Decimal128](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL256_MAX_PRECISION.html" class="constant" title="constant arrow::datatypes::DECIMAL256_MAX_PRECISION">DECIMAL256_MAX_PRECISION</a>  
The maximum precision for [DataType::Decimal256](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL256_MAX_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL256_MAX_SCALE">DECIMAL256_MAX_SCALE</a>  
The maximum scale for [DataType::Decimal256](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.DECIMAL_DEFAULT_SCALE.html" class="constant" title="constant arrow::datatypes::DECIMAL_DEFAULT_SCALE">DECIMAL_DEFAULT_SCALE</a>  
The default scale for [DataType::Decimal128](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") and [DataType::Decimal256](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") values

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL32_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MAX_DECIMAL32_FOR_EACH_PRECISION">MAX_DECIMAL32_FOR_EACH_PRECISION</a>  
`MAX_DECIMAL32_FOR_EACH_PRECISION[p]` holds the maximum `i32` value that can be stored in [`Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL64_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MAX_DECIMAL64_FOR_EACH_PRECISION">MAX_DECIMAL64_FOR_EACH_PRECISION</a>  
`MAX_DECIMAL64_FOR_EACH_PRECISION[p]` holds the maximum `i64` value that can be stored in [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL128_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MAX_DECIMAL128_FOR_EACH_PRECISION">MAX_DECIMAL128_FOR_EACH_PRECISION</a>  
`MAX_DECIMAL128_FOR_EACH_PRECISION[p]` holds the maximum `i128` value that can be stored in [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL256_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MAX_DECIMAL256_FOR_EACH_PRECISION">MAX_DECIMAL256_FOR_EACH_PRECISION</a>  
`MAX_DECIMAL256_FOR_EACH_PRECISION[p]` holds the maximum [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") value that can be stored in a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MAX_DECIMAL_FOR_EACH_PRECISION">MAX_DECIMAL_FOR_EACH_PRECISION</a>Deprecated  
`MAX_DECIMAL_FOR_EACH_PRECISION[p-1]` holds the maximum `i128` value that can be stored in a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL32_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MIN_DECIMAL32_FOR_EACH_PRECISION">MIN_DECIMAL32_FOR_EACH_PRECISION</a>  
`MIN_DECIMAL32_FOR_EACH_PRECISION[p]` holds the minimum `ialue that can be stored in a [`Decimal32`] value of precision `p\`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL64_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MIN_DECIMAL64_FOR_EACH_PRECISION">MIN_DECIMAL64_FOR_EACH_PRECISION</a>  
`MIN_DECIMAL64_FOR_EACH_PRECISION[p]` holds the minimum `i64` value that can be stored in a [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL128_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MIN_DECIMAL128_FOR_EACH_PRECISION">MIN_DECIMAL128_FOR_EACH_PRECISION</a>  
`MIN_DECIMAL_FOR_EACH_PRECISION[p]` holds the minimum `i128` value that can be stored in a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL256_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MIN_DECIMAL256_FOR_EACH_PRECISION">MIN_DECIMAL256_FOR_EACH_PRECISION</a>  
`MIN_DECIMAL256_FOR_EACH_PRECISION[p]` holds the minimum [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") value that can be stored in a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") value of precision `p`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL_FOR_EACH_PRECISION.html" class="constant" title="constant arrow::datatypes::MIN_DECIMAL_FOR_EACH_PRECISION">MIN_DECIMAL_FOR_EACH_PRECISION</a>Deprecated  
`MIN_DECIMAL_FOR_EACH_PRECISION[p-1]` holds the minimum `i128` value that can be stored in a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`

## Traits<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>  
A subtype of primitive type that represents legal dictionary keys. See <https://arrow.apache.org/docs/format/Columnar.html>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>  
Trait expressing a Rust type that has the same in-memory representation as Arrow.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::datatypes::ArrowNativeTypeOp">ArrowNativeTypeOp</a>  
Trait for [`ArrowNativeType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") that adds checked and unchecked arithmetic operations, and totally ordered comparison operations

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNumericType.html" class="trait" title="trait arrow::datatypes::ArrowNumericType">ArrowNumericType</a>  
A subtype of primitive type that represents numeric values.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::datatypes::ArrowPrimitiveType">ArrowPrimitiveType</a>  
Trait for [primitive values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout).

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a>  
A subtype of primitive type that represents temporal values.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a>  
A timestamp type allows us to create array builders that take a timestamp.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>  
A trait over the variable-size byte array types

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>  
A trait over the variable length bytes view array types

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a>  
A trait over the decimal types, used by [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") to provide a generic implementation across the various decimal types

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>  
A subtype of primitive type that is used as run-ends index in `RunArray`. See <https://arrow.apache.org/docs/format/Columnar.html>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html" class="trait" title="trait arrow::datatypes::ToByteSlice">ToByteSlice</a>  
Allows conversion from supported Arrow types to a byte slice.

## Functions<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.is_validate_decimal32_precision.html" class="fn" title="fn arrow::datatypes::is_validate_decimal32_precision">is_validate_decimal32_precision</a>  
Returns true if the specified `i32` value can be properly interpreted as a [`Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.is_validate_decimal64_precision.html" class="fn" title="fn arrow::datatypes::is_validate_decimal64_precision">is_validate_decimal64_precision</a>  
Returns true if the specified `i64` value can be properly interpreted as a [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.is_validate_decimal256_precision.html" class="fn" title="fn arrow::datatypes::is_validate_decimal256_precision">is_validate_decimal256_precision</a>  
Return true if the specified `i256` value can be properly interpreted as a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.is_validate_decimal_precision.html" class="fn" title="fn arrow::datatypes::is_validate_decimal_precision">is_validate_decimal_precision</a>  
Returns true if the specified `i128` value can be properly interpreted as a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal32_precision.html" class="fn" title="fn arrow::datatypes::validate_decimal32_precision">validate_decimal32_precision</a>  
Validates that the specified `i32` value can be properly interpreted as a [`Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal64_precision.html" class="fn" title="fn arrow::datatypes::validate_decimal64_precision">validate_decimal64_precision</a>  
Validates that the specified `i64` value can be properly interpreted as a [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal256_precision.html" class="fn" title="fn arrow::datatypes::validate_decimal256_precision">validate_decimal256_precision</a>  
Validates that the specified `i256` of value can be properly interpreted as a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal_precision.html" class="fn" title="fn arrow::datatypes::validate_decimal_precision">validate_decimal_precision</a>  
Validates that the specified `i128` value can be properly interpreted as a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") number with precision `precision`

<a href="https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal_precision_and_scale.html" class="fn" title="fn arrow::datatypes::validate_decimal_precision_and_scale">validate_decimal_precision_and_scale</a>  
Validate that `precision` and `scale` are valid for `T`

## Type Aliases<a href="https://docs.rs/arrow/latest/arrow/datatypes/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.BinaryType.html" class="type" title="type arrow::datatypes::BinaryType">BinaryType</a>  
An arrow binary array with i32 offsets

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.FieldRef.html" class="type" title="type arrow::datatypes::FieldRef">FieldRef</a>  
A reference counted [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field")

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.LargeBinaryType.html" class="type" title="type arrow::datatypes::LargeBinaryType">LargeBinaryType</a>  
An arrow binary array with i64 offsets

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.LargeUtf8Type.html" class="type" title="type arrow::datatypes::LargeUtf8Type">LargeUtf8Type</a>  
An arrow utf8 array with i64 offsets

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.SchemaRef.html" class="type" title="type arrow::datatypes::SchemaRef">SchemaRef</a>  
A reference-counted reference to a [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema").

<a href="https://docs.rs/arrow/latest/arrow/datatypes/type.Utf8Type.html" class="type" title="type arrow::datatypes::Utf8Type">Utf8Type</a>  
An arrow utf8 array with i32 offsets
